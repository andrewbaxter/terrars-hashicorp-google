#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SpannerInstancePartitionData_ {
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
    config: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processing_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_config: std::option::Option<std::vec::Vec<SpannerInstancePartition_AutoscalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SpannerInstancePartition_Timeouts>,
    dynamic: SpannerInstancePartitionDynamic,
}

struct SpannerInstancePartition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SpannerInstancePartitionData_>,
}

#[derive(Clone)]
pub struct SpannerInstancePartition(std::rc::Rc<SpannerInstancePartition_>);

impl SpannerInstancePartition {
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
        "Set the field `node_count`.\nThe number of nodes allocated to this instance partition. One node equals\n1000 processing units. Exactly one of either node_count, processing_units,\nor autoscaling_config must be present."]
    pub fn set_node_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `processing_units`.\nThe number of processing units allocated to this instance partition.\nExactly one of either node_count, processing_units, or autoscaling_config\nmust be present."]
    pub fn set_processing_units(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().processing_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpannerInstancePartition_AutoscalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().autoscaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.autoscaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SpannerInstancePartition_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe name of the instance partition's configuration (similar to a region) which\ndefines the geographic placement and replication of data in this instance partition."]
    pub fn config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe descriptive name for this instance partition as it appears in UIs.\nMust be unique per project and between 4 and 30 characters in length."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the instance partition in."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the instance partition, which cannot be changed after\nthe instance partition is created. The name must be between 2 and 64 characters\nand match the regular expression [a-z][a-z0-9\\\\-]{0,61}[a-z0-9]."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes allocated to this instance partition. One node equals\n1000 processing units. Exactly one of either node_count, processing_units,\nor autoscaling_config must be present."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `processing_units` after provisioning.\nThe number of processing units allocated to this instance partition.\nExactly one of either node_count, processing_units, or autoscaling_config\nmust be present."]
    pub fn processing_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processing_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current instance partition state. Possible values are:\nCREATING: The instance partition is being created. Resources are being\nallocated for the instance partition.\nREADY: The instance partition has been allocated resources and is ready for use."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> terrars::ListRef<SpannerInstancePartition_AutoscalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerInstancePartition_TimeoutsRef {
        SpannerInstancePartition_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SpannerInstancePartition {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SpannerInstancePartition { }

impl terrars::ToListMappable for SpannerInstancePartition {
    type O = terrars::ListRef<SpannerInstancePartitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SpannerInstancePartition_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_spanner_instance_partition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpannerInstancePartition {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the instance partition's configuration (similar to a region) which\ndefines the geographic placement and replication of data in this instance partition."]
    pub config: terrars::PrimField<std::string::String>,
    #[doc =
        "The descriptive name for this instance partition as it appears in UIs.\nMust be unique per project and between 4 and 30 characters in length."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The instance to create the instance partition in."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "A unique identifier for the instance partition, which cannot be changed after\nthe instance partition is created. The name must be between 2 and 64 characters\nand match the regular expression [a-z][a-z0-9\\\\-]{0,61}[a-z0-9]."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSpannerInstancePartition {
    pub fn build(self, stack: &mut terrars::Stack) -> SpannerInstancePartition {
        let out = SpannerInstancePartition(std::rc::Rc::new(SpannerInstancePartition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SpannerInstancePartitionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                config: self.config,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                instance: self.instance,
                name: self.name,
                node_count: core::default::Default::default(),
                processing_units: core::default::Default::default(),
                project: core::default::Default::default(),
                autoscaling_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpannerInstancePartitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerInstancePartitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SpannerInstancePartitionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config` after provisioning.\nThe name of the instance partition's configuration (similar to a region) which\ndefines the geographic placement and replication of data in this instance partition."]
    pub fn config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe descriptive name for this instance partition as it appears in UIs.\nMust be unique per project and between 4 and 30 characters in length."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the instance partition in."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the instance partition, which cannot be changed after\nthe instance partition is created. The name must be between 2 and 64 characters\nand match the regular expression [a-z][a-z0-9\\\\-]{0,61}[a-z0-9]."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes allocated to this instance partition. One node equals\n1000 processing units. Exactly one of either node_count, processing_units,\nor autoscaling_config must be present."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `processing_units` after provisioning.\nThe number of processing units allocated to this instance partition.\nExactly one of either node_count, processing_units, or autoscaling_config\nmust be present."]
    pub fn processing_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processing_units", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current instance partition state. Possible values are:\nCREATING: The instance partition is being created. Resources are being\nallocated for the instance partition.\nREADY: The instance partition has been allocated resources and is ready for use."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> terrars::ListRef<SpannerInstancePartition_AutoscalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerInstancePartition_TimeoutsRef {
        SpannerInstancePartition_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SpannerInstancePartitionDynamic {
    autoscaling_config: std::option::Option<terrars::DynamicBlock<SpannerInstancePartition_AutoscalingConfig>>,
}

#[derive(Serialize)]
pub struct SpannerInstancePartition_AutoscalingConfig_AutoscalingLimits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_processing_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_nodes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_processing_units: std::option::Option<terrars::PrimField<f64>>,
}

impl SpannerInstancePartition_AutoscalingConfig_AutoscalingLimits {
    #[doc =
        "Set the field `max_nodes`.\nSpecifies maximum number of nodes allocated to the instance partition. If set, this number\nshould be greater than or equal to min_nodes."]
    pub fn set_max_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_processing_units`.\nSpecifies maximum number of processing units allocated to the instance partition.\nIf set, this number should be multiples of 1000 and be greater than or equal to\nmin_processing_units."]
    pub fn set_max_processing_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_processing_units = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_nodes`.\nSpecifies number of nodes allocated to the instance partition. If set, this number\nshould be greater than or equal to 1."]
    pub fn set_min_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_nodes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_processing_units`.\nSpecifies minimum number of processing units allocated to the instance partition.\nIf set, this number should be multiples of 1000."]
    pub fn set_min_processing_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_processing_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpannerInstancePartition_AutoscalingConfig_AutoscalingLimits {
    type O = terrars::BlockAssignable<SpannerInstancePartition_AutoscalingConfig_AutoscalingLimits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerInstancePartition_AutoscalingConfig_AutoscalingLimits {}

impl BuildSpannerInstancePartition_AutoscalingConfig_AutoscalingLimits {
    pub fn build(self) -> SpannerInstancePartition_AutoscalingConfig_AutoscalingLimits {
        SpannerInstancePartition_AutoscalingConfig_AutoscalingLimits {
            max_nodes: core::default::Default::default(),
            max_processing_units: core::default::Default::default(),
            min_nodes: core::default::Default::default(),
            min_processing_units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpannerInstancePartition_AutoscalingConfig_AutoscalingLimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerInstancePartition_AutoscalingConfig_AutoscalingLimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpannerInstancePartition_AutoscalingConfig_AutoscalingLimitsRef {
        SpannerInstancePartition_AutoscalingConfig_AutoscalingLimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerInstancePartition_AutoscalingConfig_AutoscalingLimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_nodes` after provisioning.\nSpecifies maximum number of nodes allocated to the instance partition. If set, this number\nshould be greater than or equal to min_nodes."]
    pub fn max_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_processing_units` after provisioning.\nSpecifies maximum number of processing units allocated to the instance partition.\nIf set, this number should be multiples of 1000 and be greater than or equal to\nmin_processing_units."]
    pub fn max_processing_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_processing_units", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_nodes` after provisioning.\nSpecifies number of nodes allocated to the instance partition. If set, this number\nshould be greater than or equal to 1."]
    pub fn min_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_processing_units` after provisioning.\nSpecifies minimum number of processing units allocated to the instance partition.\nIf set, this number should be multiples of 1000."]
    pub fn min_processing_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_processing_units", self.base))
    }
}

#[derive(Serialize)]
pub struct SpannerInstancePartition_AutoscalingConfig_AutoscalingTargets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high_priority_cpu_utilization_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_utilization_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_cpu_utilization_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl SpannerInstancePartition_AutoscalingConfig_AutoscalingTargets {
    #[doc =
        "Set the field `high_priority_cpu_utilization_percent`.\nSpecifies the target high priority cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn set_high_priority_cpu_utilization_percent(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.high_priority_cpu_utilization_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_utilization_percent`.\nSpecifies the target storage utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn set_storage_utilization_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_utilization_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `total_cpu_utilization_percent`.\nSpecifies the target total cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization). The valid range is [10, 90] inclusive.\nIf not specified or set to 0, the autoscaler will skip scaling based on total cpu utilization.\nThe value should be higher than high_priority_cpu_utilization_percent if present."]
    pub fn set_total_cpu_utilization_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_cpu_utilization_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpannerInstancePartition_AutoscalingConfig_AutoscalingTargets {
    type O = terrars::BlockAssignable<SpannerInstancePartition_AutoscalingConfig_AutoscalingTargets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerInstancePartition_AutoscalingConfig_AutoscalingTargets {}

impl BuildSpannerInstancePartition_AutoscalingConfig_AutoscalingTargets {
    pub fn build(self) -> SpannerInstancePartition_AutoscalingConfig_AutoscalingTargets {
        SpannerInstancePartition_AutoscalingConfig_AutoscalingTargets {
            high_priority_cpu_utilization_percent: core::default::Default::default(),
            storage_utilization_percent: core::default::Default::default(),
            total_cpu_utilization_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpannerInstancePartition_AutoscalingConfig_AutoscalingTargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerInstancePartition_AutoscalingConfig_AutoscalingTargetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpannerInstancePartition_AutoscalingConfig_AutoscalingTargetsRef {
        SpannerInstancePartition_AutoscalingConfig_AutoscalingTargetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerInstancePartition_AutoscalingConfig_AutoscalingTargetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `high_priority_cpu_utilization_percent` after provisioning.\nSpecifies the target high priority cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn high_priority_cpu_utilization_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.high_priority_cpu_utilization_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_utilization_percent` after provisioning.\nSpecifies the target storage utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization)."]
    pub fn storage_utilization_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_utilization_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_cpu_utilization_percent` after provisioning.\nSpecifies the target total cpu utilization percentage that the autoscaler\nshould be trying to achieve for the instance partition.\nThis number is on a scale from 0 (no utilization) to 100 (full utilization). The valid range is [10, 90] inclusive.\nIf not specified or set to 0, the autoscaler will skip scaling based on total cpu utilization.\nThe value should be higher than high_priority_cpu_utilization_percent if present."]
    pub fn total_cpu_utilization_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_cpu_utilization_percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct SpannerInstancePartition_AutoscalingConfigDynamic {
    autoscaling_limits: std
    ::option
    ::Option<terrars::DynamicBlock<SpannerInstancePartition_AutoscalingConfig_AutoscalingLimits>>,
    autoscaling_targets: std
    ::option
    ::Option<terrars::DynamicBlock<SpannerInstancePartition_AutoscalingConfig_AutoscalingTargets>>,
}

#[derive(Serialize)]
pub struct SpannerInstancePartition_AutoscalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_limits: std::option::Option<std::vec::Vec<SpannerInstancePartition_AutoscalingConfig_AutoscalingLimits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_targets: std
    ::option
    ::Option<std::vec::Vec<SpannerInstancePartition_AutoscalingConfig_AutoscalingTargets>>,
    dynamic: SpannerInstancePartition_AutoscalingConfigDynamic,
}

impl SpannerInstancePartition_AutoscalingConfig {
    #[doc = "Set the field `autoscaling_limits`.\n"]
    pub fn set_autoscaling_limits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SpannerInstancePartition_AutoscalingConfig_AutoscalingLimits>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `autoscaling_targets`.\n"]
    pub fn set_autoscaling_targets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SpannerInstancePartition_AutoscalingConfig_AutoscalingTargets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_targets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SpannerInstancePartition_AutoscalingConfig {
    type O = terrars::BlockAssignable<SpannerInstancePartition_AutoscalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerInstancePartition_AutoscalingConfig {}

impl BuildSpannerInstancePartition_AutoscalingConfig {
    pub fn build(self) -> SpannerInstancePartition_AutoscalingConfig {
        SpannerInstancePartition_AutoscalingConfig {
            autoscaling_limits: core::default::Default::default(),
            autoscaling_targets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpannerInstancePartition_AutoscalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerInstancePartition_AutoscalingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpannerInstancePartition_AutoscalingConfigRef {
        SpannerInstancePartition_AutoscalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerInstancePartition_AutoscalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_limits` after provisioning.\n"]
    pub fn autoscaling_limits(
        &self,
    ) -> terrars::ListRef<SpannerInstancePartition_AutoscalingConfig_AutoscalingLimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_limits", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_targets` after provisioning.\n"]
    pub fn autoscaling_targets(
        &self,
    ) -> terrars::ListRef<SpannerInstancePartition_AutoscalingConfig_AutoscalingTargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_targets", self.base))
    }
}

#[derive(Serialize)]
pub struct SpannerInstancePartition_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpannerInstancePartition_Timeouts {
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

impl terrars::ToListMappable for SpannerInstancePartition_Timeouts {
    type O = terrars::BlockAssignable<SpannerInstancePartition_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpannerInstancePartition_Timeouts {}

impl BuildSpannerInstancePartition_Timeouts {
    pub fn build(self) -> SpannerInstancePartition_Timeouts {
        SpannerInstancePartition_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SpannerInstancePartition_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpannerInstancePartition_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpannerInstancePartition_TimeoutsRef {
        SpannerInstancePartition_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpannerInstancePartition_TimeoutsRef {
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
