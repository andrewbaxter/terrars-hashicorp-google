#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PubsubLiteTopicData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_config: std::option::Option<std::vec::Vec<PubsubLiteTopic_PartitionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_config: std::option::Option<std::vec::Vec<PubsubLiteTopic_ReservationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_config: std::option::Option<std::vec::Vec<PubsubLiteTopic_RetentionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PubsubLiteTopic_Timeouts>,
    dynamic: PubsubLiteTopicDynamic,
}

struct PubsubLiteTopic_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PubsubLiteTopicData_>,
}

#[derive(Clone)]
pub struct PubsubLiteTopic(std::rc::Rc<PubsubLiteTopic_>);

impl PubsubLiteTopic {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region of the pubsub lite topic."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nThe zone of the pubsub lite topic."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partition_config`.\n"]
    pub fn set_partition_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubLiteTopic_PartitionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partition_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partition_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reservation_config`.\n"]
    pub fn set_reservation_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubLiteTopic_ReservationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reservation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reservation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retention_config`.\n"]
    pub fn set_retention_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubLiteTopic_RetentionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PubsubLiteTopic_Timeouts>) -> Self {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the pubsub lite topic."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone of the pubsub lite topic."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_config` after provisioning.\n"]
    pub fn partition_config(&self) -> terrars::ListRef<PubsubLiteTopic_PartitionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reservation_config` after provisioning.\n"]
    pub fn reservation_config(&self) -> terrars::ListRef<PubsubLiteTopic_ReservationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_config` after provisioning.\n"]
    pub fn retention_config(&self) -> terrars::ListRef<PubsubLiteTopic_RetentionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubLiteTopic_TimeoutsRef {
        PubsubLiteTopic_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for PubsubLiteTopic {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PubsubLiteTopic { }

impl terrars::ToListMappable for PubsubLiteTopic {
    type O = terrars::ListRef<PubsubLiteTopicRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PubsubLiteTopic_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_pubsub_lite_topic".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPubsubLiteTopic {
    pub tf_id: std::string::String,
    #[doc = "Name of the topic."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildPubsubLiteTopic {
    pub fn build(self, stack: &mut terrars::Stack) -> PubsubLiteTopic {
        let out = PubsubLiteTopic(std::rc::Rc::new(PubsubLiteTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PubsubLiteTopicData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                zone: core::default::Default::default(),
                partition_config: core::default::Default::default(),
                reservation_config: core::default::Default::default(),
                retention_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PubsubLiteTopicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubLiteTopicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PubsubLiteTopicRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the pubsub lite topic."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone of the pubsub lite topic."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_config` after provisioning.\n"]
    pub fn partition_config(&self) -> terrars::ListRef<PubsubLiteTopic_PartitionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reservation_config` after provisioning.\n"]
    pub fn reservation_config(&self) -> terrars::ListRef<PubsubLiteTopic_ReservationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_config` after provisioning.\n"]
    pub fn retention_config(&self) -> terrars::ListRef<PubsubLiteTopic_RetentionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubLiteTopic_TimeoutsRef {
        PubsubLiteTopic_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct PubsubLiteTopicDynamic {
    partition_config: std::option::Option<terrars::DynamicBlock<PubsubLiteTopic_PartitionConfig>>,
    reservation_config: std::option::Option<terrars::DynamicBlock<PubsubLiteTopic_ReservationConfig>>,
    retention_config: std::option::Option<terrars::DynamicBlock<PubsubLiteTopic_RetentionConfig>>,
}

#[derive(Serialize)]
pub struct PubsubLiteTopic_PartitionConfig_Capacity {
    publish_mib_per_sec: terrars::PrimField<f64>,
    subscribe_mib_per_sec: terrars::PrimField<f64>,
}

impl PubsubLiteTopic_PartitionConfig_Capacity { }

impl terrars::ToListMappable for PubsubLiteTopic_PartitionConfig_Capacity {
    type O = terrars::BlockAssignable<PubsubLiteTopic_PartitionConfig_Capacity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubLiteTopic_PartitionConfig_Capacity {
    #[doc = "Subscribe throughput capacity per partition in MiB/s. Must be >= 4 and <= 16."]
    pub publish_mib_per_sec: terrars::PrimField<f64>,
    #[doc = "Publish throughput capacity per partition in MiB/s. Must be >= 4 and <= 16."]
    pub subscribe_mib_per_sec: terrars::PrimField<f64>,
}

impl BuildPubsubLiteTopic_PartitionConfig_Capacity {
    pub fn build(self) -> PubsubLiteTopic_PartitionConfig_Capacity {
        PubsubLiteTopic_PartitionConfig_Capacity {
            publish_mib_per_sec: self.publish_mib_per_sec,
            subscribe_mib_per_sec: self.subscribe_mib_per_sec,
        }
    }
}

#[allow(dead_code)]
pub struct PubsubLiteTopic_PartitionConfig_CapacityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubLiteTopic_PartitionConfig_CapacityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubLiteTopic_PartitionConfig_CapacityRef {
        PubsubLiteTopic_PartitionConfig_CapacityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubLiteTopic_PartitionConfig_CapacityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `publish_mib_per_sec` after provisioning.\nSubscribe throughput capacity per partition in MiB/s. Must be >= 4 and <= 16."]
    pub fn publish_mib_per_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish_mib_per_sec", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subscribe_mib_per_sec` after provisioning.\nPublish throughput capacity per partition in MiB/s. Must be >= 4 and <= 16."]
    pub fn subscribe_mib_per_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscribe_mib_per_sec", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubLiteTopic_PartitionConfigDynamic {
    capacity: std::option::Option<terrars::DynamicBlock<PubsubLiteTopic_PartitionConfig_Capacity>>,
}

#[derive(Serialize)]
pub struct PubsubLiteTopic_PartitionConfig {
    count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity: std::option::Option<std::vec::Vec<PubsubLiteTopic_PartitionConfig_Capacity>>,
    dynamic: PubsubLiteTopic_PartitionConfigDynamic,
}

impl PubsubLiteTopic_PartitionConfig {
    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubLiteTopic_PartitionConfig_Capacity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capacity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PubsubLiteTopic_PartitionConfig {
    type O = terrars::BlockAssignable<PubsubLiteTopic_PartitionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubLiteTopic_PartitionConfig {
    #[doc = "The number of partitions in the topic. Must be at least 1."]
    pub count: terrars::PrimField<f64>,
}

impl BuildPubsubLiteTopic_PartitionConfig {
    pub fn build(self) -> PubsubLiteTopic_PartitionConfig {
        PubsubLiteTopic_PartitionConfig {
            count: self.count,
            capacity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubLiteTopic_PartitionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubLiteTopic_PartitionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubLiteTopic_PartitionConfigRef {
        PubsubLiteTopic_PartitionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubLiteTopic_PartitionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nThe number of partitions in the topic. Must be at least 1."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::ListRef<PubsubLiteTopic_PartitionConfig_CapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubLiteTopic_ReservationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput_reservation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubLiteTopic_ReservationConfig {
    #[doc = "Set the field `throughput_reservation`.\nThe Reservation to use for this topic's throughput capacity."]
    pub fn set_throughput_reservation(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.throughput_reservation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubLiteTopic_ReservationConfig {
    type O = terrars::BlockAssignable<PubsubLiteTopic_ReservationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubLiteTopic_ReservationConfig {}

impl BuildPubsubLiteTopic_ReservationConfig {
    pub fn build(self) -> PubsubLiteTopic_ReservationConfig {
        PubsubLiteTopic_ReservationConfig { throughput_reservation: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct PubsubLiteTopic_ReservationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubLiteTopic_ReservationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubLiteTopic_ReservationConfigRef {
        PubsubLiteTopic_ReservationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubLiteTopic_ReservationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `throughput_reservation` after provisioning.\nThe Reservation to use for this topic's throughput capacity."]
    pub fn throughput_reservation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput_reservation", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubLiteTopic_RetentionConfig {
    per_partition_bytes: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubLiteTopic_RetentionConfig {
    #[doc =
        "Set the field `period`.\nHow long a published message is retained. If unset, messages will be retained as\nlong as the bytes retained for each partition is below perPartitionBytes. A\nduration in seconds with up to nine fractional digits, terminated by 's'.\nExample: \"3.5s\"."]
    pub fn set_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubLiteTopic_RetentionConfig {
    type O = terrars::BlockAssignable<PubsubLiteTopic_RetentionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubLiteTopic_RetentionConfig {
    #[doc =
        "The provisioned storage, in bytes, per partition. If the number of bytes stored\nin any of the topic's partitions grows beyond this value, older messages will be\ndropped to make room for newer ones, regardless of the value of period."]
    pub per_partition_bytes: terrars::PrimField<std::string::String>,
}

impl BuildPubsubLiteTopic_RetentionConfig {
    pub fn build(self) -> PubsubLiteTopic_RetentionConfig {
        PubsubLiteTopic_RetentionConfig {
            per_partition_bytes: self.per_partition_bytes,
            period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubLiteTopic_RetentionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubLiteTopic_RetentionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubLiteTopic_RetentionConfigRef {
        PubsubLiteTopic_RetentionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubLiteTopic_RetentionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `per_partition_bytes` after provisioning.\nThe provisioned storage, in bytes, per partition. If the number of bytes stored\nin any of the topic's partitions grows beyond this value, older messages will be\ndropped to make room for newer ones, regardless of the value of period."]
    pub fn per_partition_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_partition_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nHow long a published message is retained. If unset, messages will be retained as\nlong as the bytes retained for each partition is below perPartitionBytes. A\nduration in seconds with up to nine fractional digits, terminated by 's'.\nExample: \"3.5s\"."]
    pub fn period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubLiteTopic_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubLiteTopic_Timeouts {
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

impl terrars::ToListMappable for PubsubLiteTopic_Timeouts {
    type O = terrars::BlockAssignable<PubsubLiteTopic_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubLiteTopic_Timeouts {}

impl BuildPubsubLiteTopic_Timeouts {
    pub fn build(self) -> PubsubLiteTopic_Timeouts {
        PubsubLiteTopic_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubLiteTopic_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubLiteTopic_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubLiteTopic_TimeoutsRef {
        PubsubLiteTopic_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubLiteTopic_TimeoutsRef {
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
