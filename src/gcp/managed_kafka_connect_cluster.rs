#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ManagedKafkaConnectClusterData_ {
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
    connect_cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    kafka_cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_config: std::option::Option<std::vec::Vec<ManagedKafkaConnectCluster_CapacityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_config: std::option::Option<std::vec::Vec<ManagedKafkaConnectCluster_GcpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ManagedKafkaConnectCluster_Timeouts>,
    dynamic: ManagedKafkaConnectClusterDynamic,
}

struct ManagedKafkaConnectCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ManagedKafkaConnectClusterData_>,
}

#[derive(Clone)]
pub struct ManagedKafkaConnectCluster(std::rc::Rc<ManagedKafkaConnectCluster_>);

impl ManagedKafkaConnectCluster {
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
        "Set the field `labels`.\nList of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers. Values must contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_config`.\n"]
    pub fn set_capacity_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaConnectCluster_CapacityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcp_config`.\n"]
    pub fn set_gcp_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaConnectCluster_GcpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gcp_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gcp_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ManagedKafkaConnectCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `connect_cluster_id` after provisioning.\nThe ID to use for the Connect Cluster, which will become the final component of the connect cluster's name. This value is structured like: 'my-connect-cluster-id'."]
    pub fn connect_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kafka_cluster` after provisioning.\nThe name of the Kafka cluster this Kafka Connect cluster is attached to. Structured like: 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID'."]
    pub fn kafka_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kafka_cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nList of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers. Values must contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka Connect resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the connect cluster. Structured like: 'projects/PROJECT_ID/locations/LOCATION/connectClusters/CONNECT_CLUSTER_ID'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the connect cluster. Possible values: 'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'DELETING'."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the cluster was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_config` after provisioning.\n"]
    pub fn capacity_config(&self) -> terrars::ListRef<ManagedKafkaConnectCluster_CapacityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcp_config` after provisioning.\n"]
    pub fn gcp_config(&self) -> terrars::ListRef<ManagedKafkaConnectCluster_GcpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaConnectCluster_TimeoutsRef {
        ManagedKafkaConnectCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ManagedKafkaConnectCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ManagedKafkaConnectCluster { }

impl terrars::ToListMappable for ManagedKafkaConnectCluster {
    type O = terrars::ListRef<ManagedKafkaConnectClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ManagedKafkaConnectCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_managed_kafka_connect_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildManagedKafkaConnectCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the Connect Cluster, which will become the final component of the connect cluster's name. This value is structured like: 'my-connect-cluster-id'."]
    pub connect_cluster_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the Kafka cluster this Kafka Connect cluster is attached to. Structured like: 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID'."]
    pub kafka_cluster: terrars::PrimField<std::string::String>,
    #[doc =
        "ID of the location of the Kafka Connect resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildManagedKafkaConnectCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> ManagedKafkaConnectCluster {
        let out = ManagedKafkaConnectCluster(std::rc::Rc::new(ManagedKafkaConnectCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ManagedKafkaConnectClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connect_cluster_id: self.connect_cluster_id,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                kafka_cluster: self.kafka_cluster,
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                capacity_config: core::default::Default::default(),
                gcp_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ManagedKafkaConnectClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaConnectClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ManagedKafkaConnectClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connect_cluster_id` after provisioning.\nThe ID to use for the Connect Cluster, which will become the final component of the connect cluster's name. This value is structured like: 'my-connect-cluster-id'."]
    pub fn connect_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kafka_cluster` after provisioning.\nThe name of the Kafka cluster this Kafka Connect cluster is attached to. Structured like: 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID'."]
    pub fn kafka_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kafka_cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nList of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers. Values must contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka Connect resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the connect cluster. Structured like: 'projects/PROJECT_ID/locations/LOCATION/connectClusters/CONNECT_CLUSTER_ID'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the connect cluster. Possible values: 'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'DELETING'."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the cluster was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_config` after provisioning.\n"]
    pub fn capacity_config(&self) -> terrars::ListRef<ManagedKafkaConnectCluster_CapacityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcp_config` after provisioning.\n"]
    pub fn gcp_config(&self) -> terrars::ListRef<ManagedKafkaConnectCluster_GcpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaConnectCluster_TimeoutsRef {
        ManagedKafkaConnectCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ManagedKafkaConnectClusterDynamic {
    capacity_config: std::option::Option<terrars::DynamicBlock<ManagedKafkaConnectCluster_CapacityConfig>>,
    gcp_config: std::option::Option<terrars::DynamicBlock<ManagedKafkaConnectCluster_GcpConfig>>,
}

#[derive(Serialize)]
pub struct ManagedKafkaConnectCluster_CapacityConfig {
    memory_bytes: terrars::PrimField<std::string::String>,
    vcpu_count: terrars::PrimField<std::string::String>,
}

impl ManagedKafkaConnectCluster_CapacityConfig { }

impl terrars::ToListMappable for ManagedKafkaConnectCluster_CapacityConfig {
    type O = terrars::BlockAssignable<ManagedKafkaConnectCluster_CapacityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaConnectCluster_CapacityConfig {
    #[doc =
        "The memory to provision for the cluster in bytes. The CPU:memory ratio (vCPU:GiB) must be between 1:1 and 1:8. Minimum: 3221225472 (3 GiB)."]
    pub memory_bytes: terrars::PrimField<std::string::String>,
    #[doc = "The number of vCPUs to provision for the cluster. The minimum is 3."]
    pub vcpu_count: terrars::PrimField<std::string::String>,
}

impl BuildManagedKafkaConnectCluster_CapacityConfig {
    pub fn build(self) -> ManagedKafkaConnectCluster_CapacityConfig {
        ManagedKafkaConnectCluster_CapacityConfig {
            memory_bytes: self.memory_bytes,
            vcpu_count: self.vcpu_count,
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaConnectCluster_CapacityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaConnectCluster_CapacityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaConnectCluster_CapacityConfigRef {
        ManagedKafkaConnectCluster_CapacityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaConnectCluster_CapacityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `memory_bytes` after provisioning.\nThe memory to provision for the cluster in bytes. The CPU:memory ratio (vCPU:GiB) must be between 1:1 and 1:8. Minimum: 3221225472 (3 GiB)."]
    pub fn memory_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vcpu_count` after provisioning.\nThe number of vCPUs to provision for the cluster. The minimum is 3."]
    pub fn vcpu_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_subnets: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_domain_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    primary_subnet: terrars::PrimField<std::string::String>,
}

impl ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs {
    #[doc =
        "Set the field `additional_subnets`.\nAdditional subnets may be specified. They may be in another region, but must be in the same VPC network. The Connect workers can communicate with network endpoints in either the primary or additional subnets."]
    pub fn set_additional_subnets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_subnets = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dns_domain_names`.\nAdditional DNS domain names from the subnet's network to be made visible to the Connect Cluster. When using MirrorMaker2, it's necessary to add the bootstrap address's dns domain name of the target cluster to make it visible to the connector. For example: my-kafka-cluster.us-central1.managedkafka.my-project.cloud.goog"]
    pub fn set_dns_domain_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_domain_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs {
    type O = terrars::BlockAssignable<ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs {
    #[doc =
        "VPC subnet to make available to the Kafka Connect cluster. Structured like: projects/{project}/regions/{region}/subnetworks/{subnet_id}. It is used to create a Private Service Connect (PSC) interface for the Kafka Connect workers. It must be located in the same region as the Kafka Connect cluster. The CIDR range of the subnet must be within the IPv4 address ranges for private networks, as specified in RFC 1918. The primary subnet CIDR range must have a minimum size of /22 (1024 addresses)."]
    pub primary_subnet: terrars::PrimField<std::string::String>,
}

impl BuildManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs {
    pub fn build(self) -> ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs {
        ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs {
            additional_subnets: core::default::Default::default(),
            dns_domain_names: core::default::Default::default(),
            primary_subnet: self.primary_subnet,
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigsRef {
        ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_subnets` after provisioning.\nAdditional subnets may be specified. They may be in another region, but must be in the same VPC network. The Connect workers can communicate with network endpoints in either the primary or additional subnets."]
    pub fn additional_subnets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_subnets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_domain_names` after provisioning.\nAdditional DNS domain names from the subnet's network to be made visible to the Connect Cluster. When using MirrorMaker2, it's necessary to add the bootstrap address's dns domain name of the target cluster to make it visible to the connector. For example: my-kafka-cluster.us-central1.managedkafka.my-project.cloud.goog"]
    pub fn dns_domain_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_domain_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_subnet` after provisioning.\nVPC subnet to make available to the Kafka Connect cluster. Structured like: projects/{project}/regions/{region}/subnetworks/{subnet_id}. It is used to create a Private Service Connect (PSC) interface for the Kafka Connect workers. It must be located in the same region as the Kafka Connect cluster. The CIDR range of the subnet must be within the IPv4 address ranges for private networks, as specified in RFC 1918. The primary subnet CIDR range must have a minimum size of /22 (1024 addresses)."]
    pub fn primary_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_subnet", self.base))
    }
}

#[derive(Serialize, Default)]
struct ManagedKafkaConnectCluster_GcpConfig_AccessConfigDynamic {
    network_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs>>,
}

#[derive(Serialize)]
pub struct ManagedKafkaConnectCluster_GcpConfig_AccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configs: std::option::Option<std::vec::Vec<ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs>>,
    dynamic: ManagedKafkaConnectCluster_GcpConfig_AccessConfigDynamic,
}

impl ManagedKafkaConnectCluster_GcpConfig_AccessConfig {
    #[doc = "Set the field `network_configs`.\n"]
    pub fn set_network_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ManagedKafkaConnectCluster_GcpConfig_AccessConfig {
    type O = terrars::BlockAssignable<ManagedKafkaConnectCluster_GcpConfig_AccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaConnectCluster_GcpConfig_AccessConfig {}

impl BuildManagedKafkaConnectCluster_GcpConfig_AccessConfig {
    pub fn build(self) -> ManagedKafkaConnectCluster_GcpConfig_AccessConfig {
        ManagedKafkaConnectCluster_GcpConfig_AccessConfig {
            network_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaConnectCluster_GcpConfig_AccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaConnectCluster_GcpConfig_AccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ManagedKafkaConnectCluster_GcpConfig_AccessConfigRef {
        ManagedKafkaConnectCluster_GcpConfig_AccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaConnectCluster_GcpConfig_AccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_configs` after provisioning.\n"]
    pub fn network_configs(
        &self,
    ) -> terrars::ListRef<ManagedKafkaConnectCluster_GcpConfig_AccessConfig_NetworkConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configs", self.base))
    }
}

#[derive(Serialize, Default)]
struct ManagedKafkaConnectCluster_GcpConfigDynamic {
    access_config: std::option::Option<terrars::DynamicBlock<ManagedKafkaConnectCluster_GcpConfig_AccessConfig>>,
}

#[derive(Serialize)]
pub struct ManagedKafkaConnectCluster_GcpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_config: std::option::Option<std::vec::Vec<ManagedKafkaConnectCluster_GcpConfig_AccessConfig>>,
    dynamic: ManagedKafkaConnectCluster_GcpConfigDynamic,
}

impl ManagedKafkaConnectCluster_GcpConfig {
    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaConnectCluster_GcpConfig_AccessConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ManagedKafkaConnectCluster_GcpConfig {
    type O = terrars::BlockAssignable<ManagedKafkaConnectCluster_GcpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaConnectCluster_GcpConfig {}

impl BuildManagedKafkaConnectCluster_GcpConfig {
    pub fn build(self) -> ManagedKafkaConnectCluster_GcpConfig {
        ManagedKafkaConnectCluster_GcpConfig {
            access_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaConnectCluster_GcpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaConnectCluster_GcpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaConnectCluster_GcpConfigRef {
        ManagedKafkaConnectCluster_GcpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaConnectCluster_GcpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> terrars::ListRef<ManagedKafkaConnectCluster_GcpConfig_AccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ManagedKafkaConnectCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ManagedKafkaConnectCluster_Timeouts {
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

impl terrars::ToListMappable for ManagedKafkaConnectCluster_Timeouts {
    type O = terrars::BlockAssignable<ManagedKafkaConnectCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaConnectCluster_Timeouts {}

impl BuildManagedKafkaConnectCluster_Timeouts {
    pub fn build(self) -> ManagedKafkaConnectCluster_Timeouts {
        ManagedKafkaConnectCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaConnectCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaConnectCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaConnectCluster_TimeoutsRef {
        ManagedKafkaConnectCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaConnectCluster_TimeoutsRef {
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
