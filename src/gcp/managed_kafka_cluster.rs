#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ManagedKafkaClusterData_ {
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
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    broker_capacity_config: std::option::Option<std::vec::Vec<ManagedKafkaCluster_BrokerCapacityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_config: std::option::Option<std::vec::Vec<ManagedKafkaCluster_CapacityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_config: std::option::Option<std::vec::Vec<ManagedKafkaCluster_GcpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rebalance_config: std::option::Option<std::vec::Vec<ManagedKafkaCluster_RebalanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ManagedKafkaCluster_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<std::vec::Vec<ManagedKafkaCluster_TlsConfig>>,
    dynamic: ManagedKafkaClusterDynamic,
}

struct ManagedKafkaCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ManagedKafkaClusterData_>,
}

#[derive(Clone)]
pub struct ManagedKafkaCluster(std::rc::Rc<ManagedKafkaCluster_>);

impl ManagedKafkaCluster {
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
        "Set the field `kafka_version`.\nThe Apache Kafka version of the cluster. Specifically, this value is the finalized metadata version.\nIf not specified during cluster creation, defaults to '3.7.x'.\nIf you manage Kafka versions outside of Terraform, leave this field unset to avoid a permanent diff."]
    pub fn set_kafka_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kafka_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `broker_capacity_config`.\n"]
    pub fn set_broker_capacity_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaCluster_BrokerCapacityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().broker_capacity_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.broker_capacity_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `capacity_config`.\n"]
    pub fn set_capacity_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaCluster_CapacityConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaCluster_GcpConfig>>,
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

    #[doc = "Set the field `rebalance_config`.\n"]
    pub fn set_rebalance_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaCluster_RebalanceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rebalance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rebalance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ManagedKafkaCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaCluster_TlsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tls_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tls_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `bootstrap_address` after provisioning.\nThe bootstrap address of the Kafka cluster. Use port :9092 for SASL connection and :9192 for mTLS connection"]
    pub fn bootstrap_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID to use for the cluster, which will become the final component of the cluster's name. The ID must be 1-63 characters long, and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' to comply with RFC 1035. This value is structured like: 'my-cluster-id'."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
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
        "Get a reference to the value of field `kafka_version` after provisioning.\nThe Apache Kafka version of the cluster. Specifically, this value is the finalized metadata version.\nIf not specified during cluster creation, defaults to '3.7.x'.\nIf you manage Kafka versions outside of Terraform, leave this field unset to avoid a permanent diff."]
    pub fn kafka_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kafka_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nList of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers. Values must contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster. Structured like: 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_cluster_details` after provisioning.\nDetails of the public cluster feature for the Kafka cluster."]
    pub fn public_cluster_details(&self) -> terrars::ListRef<ManagedKafkaCluster_PublicClusterDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_cluster_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the cluster. Possible values: 'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'DELETING'."]
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

    #[doc = "Get a reference to the value of field `broker_capacity_config` after provisioning.\n"]
    pub fn broker_capacity_config(&self) -> terrars::ListRef<ManagedKafkaCluster_BrokerCapacityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_capacity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_config` after provisioning.\n"]
    pub fn capacity_config(&self) -> terrars::ListRef<ManagedKafkaCluster_CapacityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcp_config` after provisioning.\n"]
    pub fn gcp_config(&self) -> terrars::ListRef<ManagedKafkaCluster_GcpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rebalance_config` after provisioning.\n"]
    pub fn rebalance_config(&self) -> terrars::ListRef<ManagedKafkaCluster_RebalanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rebalance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaCluster_TimeoutsRef {
        ManagedKafkaCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<ManagedKafkaCluster_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.extract_ref()))
    }
}

impl terrars::Referable for ManagedKafkaCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ManagedKafkaCluster { }

impl terrars::ToListMappable for ManagedKafkaCluster {
    type O = terrars::ListRef<ManagedKafkaClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ManagedKafkaCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_managed_kafka_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildManagedKafkaCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the cluster, which will become the final component of the cluster's name. The ID must be 1-63 characters long, and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' to comply with RFC 1035. This value is structured like: 'my-cluster-id'."]
    pub cluster_id: terrars::PrimField<std::string::String>,
    #[doc =
        "ID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildManagedKafkaCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> ManagedKafkaCluster {
        let out = ManagedKafkaCluster(std::rc::Rc::new(ManagedKafkaCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ManagedKafkaClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_id: self.cluster_id,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                kafka_version: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                broker_capacity_config: core::default::Default::default(),
                capacity_config: core::default::Default::default(),
                gcp_config: core::default::Default::default(),
                rebalance_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                tls_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ManagedKafkaClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ManagedKafkaClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bootstrap_address` after provisioning.\nThe bootstrap address of the Kafka cluster. Use port :9092 for SASL connection and :9192 for mTLS connection"]
    pub fn bootstrap_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID to use for the cluster, which will become the final component of the cluster's name. The ID must be 1-63 characters long, and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' to comply with RFC 1035. This value is structured like: 'my-cluster-id'."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
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
        "Get a reference to the value of field `kafka_version` after provisioning.\nThe Apache Kafka version of the cluster. Specifically, this value is the finalized metadata version.\nIf not specified during cluster creation, defaults to '3.7.x'.\nIf you manage Kafka versions outside of Terraform, leave this field unset to avoid a permanent diff."]
    pub fn kafka_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kafka_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nList of label KEY=VALUE pairs to add. Keys must start with a lowercase character and contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers. Values must contain only hyphens (-), underscores (\u{a0}), lowercase characters, and numbers.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nID of the location of the Kafka resource. See https://cloud.google.com/managed-kafka/docs/locations for a list of supported locations."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the cluster. Structured like: 'projects/PROJECT_ID/locations/LOCATION/clusters/CLUSTER_ID'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_cluster_details` after provisioning.\nDetails of the public cluster feature for the Kafka cluster."]
    pub fn public_cluster_details(&self) -> terrars::ListRef<ManagedKafkaCluster_PublicClusterDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_cluster_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the cluster. Possible values: 'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'DELETING'."]
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

    #[doc = "Get a reference to the value of field `broker_capacity_config` after provisioning.\n"]
    pub fn broker_capacity_config(&self) -> terrars::ListRef<ManagedKafkaCluster_BrokerCapacityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_capacity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_config` after provisioning.\n"]
    pub fn capacity_config(&self) -> terrars::ListRef<ManagedKafkaCluster_CapacityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcp_config` after provisioning.\n"]
    pub fn gcp_config(&self) -> terrars::ListRef<ManagedKafkaCluster_GcpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rebalance_config` after provisioning.\n"]
    pub fn rebalance_config(&self) -> terrars::ListRef<ManagedKafkaCluster_RebalanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rebalance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ManagedKafkaCluster_TimeoutsRef {
        ManagedKafkaCluster_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<ManagedKafkaCluster_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ManagedKafkaClusterDynamic {
    broker_capacity_config: std::option::Option<terrars::DynamicBlock<ManagedKafkaCluster_BrokerCapacityConfig>>,
    capacity_config: std::option::Option<terrars::DynamicBlock<ManagedKafkaCluster_CapacityConfig>>,
    gcp_config: std::option::Option<terrars::DynamicBlock<ManagedKafkaCluster_GcpConfig>>,
    rebalance_config: std::option::Option<terrars::DynamicBlock<ManagedKafkaCluster_RebalanceConfig>>,
    tls_config: std::option::Option<terrars::DynamicBlock<ManagedKafkaCluster_TlsConfig>>,
}

#[derive(Serialize)]
pub struct ManagedKafkaCluster_PublicClusterDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discovery_dns_records: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ip_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ManagedKafkaCluster_PublicClusterDetails {
    #[doc = "Set the field `discovery_dns_records`.\n"]
    pub fn set_discovery_dns_records(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.discovery_dns_records = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_ip_addresses`.\n"]
    pub fn set_external_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.external_ip_addresses = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ManagedKafkaCluster_PublicClusterDetails {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_PublicClusterDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_PublicClusterDetails {}

impl BuildManagedKafkaCluster_PublicClusterDetails {
    pub fn build(self) -> ManagedKafkaCluster_PublicClusterDetails {
        ManagedKafkaCluster_PublicClusterDetails {
            discovery_dns_records: core::default::Default::default(),
            external_ip_addresses: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_PublicClusterDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_PublicClusterDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaCluster_PublicClusterDetailsRef {
        ManagedKafkaCluster_PublicClusterDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_PublicClusterDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `discovery_dns_records` after provisioning.\n"]
    pub fn discovery_dns_records(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discovery_dns_records", self.base))
    }

    #[doc = "Get a reference to the value of field `external_ip_addresses` after provisioning.\n"]
    pub fn external_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct ManagedKafkaCluster_BrokerCapacityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gib: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ManagedKafkaCluster_BrokerCapacityConfig {
    #[doc = "Set the field `disk_size_gib`.\nThe disk to provision for each broker in Gibibytes. Minimum: 100 GiB."]
    pub fn set_disk_size_gib(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_size_gib = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ManagedKafkaCluster_BrokerCapacityConfig {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_BrokerCapacityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_BrokerCapacityConfig {}

impl BuildManagedKafkaCluster_BrokerCapacityConfig {
    pub fn build(self) -> ManagedKafkaCluster_BrokerCapacityConfig {
        ManagedKafkaCluster_BrokerCapacityConfig { disk_size_gib: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_BrokerCapacityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_BrokerCapacityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaCluster_BrokerCapacityConfigRef {
        ManagedKafkaCluster_BrokerCapacityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_BrokerCapacityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gib` after provisioning.\nThe disk to provision for each broker in Gibibytes. Minimum: 100 GiB."]
    pub fn disk_size_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct ManagedKafkaCluster_CapacityConfig {
    memory_bytes: terrars::PrimField<std::string::String>,
    vcpu_count: terrars::PrimField<std::string::String>,
}

impl ManagedKafkaCluster_CapacityConfig { }

impl terrars::ToListMappable for ManagedKafkaCluster_CapacityConfig {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_CapacityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_CapacityConfig {
    #[doc =
        "The memory to provision for the cluster in bytes. The value must be between 1 GiB and 8 GiB per vCPU. Ex. 1024Mi, 4Gi."]
    pub memory_bytes: terrars::PrimField<std::string::String>,
    #[doc = "The number of vCPUs to provision for the cluster. The minimum is 3."]
    pub vcpu_count: terrars::PrimField<std::string::String>,
}

impl BuildManagedKafkaCluster_CapacityConfig {
    pub fn build(self) -> ManagedKafkaCluster_CapacityConfig {
        ManagedKafkaCluster_CapacityConfig {
            memory_bytes: self.memory_bytes,
            vcpu_count: self.vcpu_count,
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_CapacityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_CapacityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaCluster_CapacityConfigRef {
        ManagedKafkaCluster_CapacityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_CapacityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `memory_bytes` after provisioning.\nThe memory to provision for the cluster in bytes. The value must be between 1 GiB and 8 GiB per vCPU. Ex. 1024Mi, 4Gi."]
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
pub struct ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs {
    subnet: terrars::PrimField<std::string::String>,
}

impl ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs { }

impl terrars::ToListMappable for ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs {
    #[doc =
        "Name of the VPC subnet from which the cluster is accessible. Both broker and bootstrap server IP addresses and DNS entries are automatically created in the subnet. There can only be one subnet per network, and the subnet must be located in the same region as the cluster. The project may differ. The name of the subnet must be in the format 'projects/PROJECT_ID/regions/REGION/subnetworks/SUBNET'."]
    pub subnet: terrars::PrimField<std::string::String>,
}

impl BuildManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs {
    pub fn build(self) -> ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs {
        ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs { subnet: self.subnet }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigsRef {
        ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `subnet` after provisioning.\nName of the VPC subnet from which the cluster is accessible. Both broker and bootstrap server IP addresses and DNS entries are automatically created in the subnet. There can only be one subnet per network, and the subnet must be located in the same region as the cluster. The project may differ. The name of the subnet must be in the format 'projects/PROJECT_ID/regions/REGION/subnetworks/SUBNET'."]
    pub fn subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}

#[derive(Serialize)]
pub struct ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig {
    allowed_source_ip_ranges: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig { }

impl terrars::ToListMappable for ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig {
    #[doc =
        "A list of IPv4 addresses or CIDR ranges that are allowed to connect to the cluster. To protect your cluster, allow access from only trusted external IP ranges. Don't expose your cluster to untrusted ranges."]
    pub allowed_source_ip_ranges: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig {
    pub fn build(self) -> ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig {
        ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig {
            allowed_source_ip_ranges: self.allowed_source_ip_ranges,
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfigRef {
        ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_source_ip_ranges` after provisioning.\nA list of IPv4 addresses or CIDR ranges that are allowed to connect to the cluster. To protect your cluster, allow access from only trusted external IP ranges. Don't expose your cluster to untrusted ranges."]
    pub fn allowed_source_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_source_ip_ranges", self.base))
    }
}

#[derive(Serialize, Default)]
struct ManagedKafkaCluster_GcpConfig_AccessConfigDynamic {
    network_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs>>,
    public_cluster_config: std
    ::option
    ::Option<terrars::DynamicBlock<ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig>>,
}

#[derive(Serialize)]
pub struct ManagedKafkaCluster_GcpConfig_AccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configs: std::option::Option<std::vec::Vec<ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_cluster_config: std
    ::option
    ::Option<std::vec::Vec<ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig>>,
    dynamic: ManagedKafkaCluster_GcpConfig_AccessConfigDynamic,
}

impl ManagedKafkaCluster_GcpConfig_AccessConfig {
    #[doc = "Set the field `network_configs`.\n"]
    pub fn set_network_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigs>>,
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

    #[doc = "Set the field `public_cluster_config`.\n"]
    pub fn set_public_cluster_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ManagedKafkaCluster_GcpConfig_AccessConfig {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_GcpConfig_AccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_GcpConfig_AccessConfig {}

impl BuildManagedKafkaCluster_GcpConfig_AccessConfig {
    pub fn build(self) -> ManagedKafkaCluster_GcpConfig_AccessConfig {
        ManagedKafkaCluster_GcpConfig_AccessConfig {
            network_configs: core::default::Default::default(),
            public_cluster_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_GcpConfig_AccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_GcpConfig_AccessConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaCluster_GcpConfig_AccessConfigRef {
        ManagedKafkaCluster_GcpConfig_AccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_GcpConfig_AccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_configs` after provisioning.\n"]
    pub fn network_configs(&self) -> terrars::ListRef<ManagedKafkaCluster_GcpConfig_AccessConfig_NetworkConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `public_cluster_config` after provisioning.\n"]
    pub fn public_cluster_config(
        &self,
    ) -> terrars::ListRef<ManagedKafkaCluster_GcpConfig_AccessConfig_PublicClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_cluster_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ManagedKafkaCluster_GcpConfigDynamic {
    access_config: std::option::Option<terrars::DynamicBlock<ManagedKafkaCluster_GcpConfig_AccessConfig>>,
}

#[derive(Serialize)]
pub struct ManagedKafkaCluster_GcpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_config: std::option::Option<std::vec::Vec<ManagedKafkaCluster_GcpConfig_AccessConfig>>,
    dynamic: ManagedKafkaCluster_GcpConfigDynamic,
}

impl ManagedKafkaCluster_GcpConfig {
    #[doc =
        "Set the field `kms_key`.\nThe Cloud KMS Key name to use for encryption. The key must be located in the same region as the cluster and cannot be changed. Must be in the format 'projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY'."]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaCluster_GcpConfig_AccessConfig>>,
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

impl terrars::ToListMappable for ManagedKafkaCluster_GcpConfig {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_GcpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_GcpConfig {}

impl BuildManagedKafkaCluster_GcpConfig {
    pub fn build(self) -> ManagedKafkaCluster_GcpConfig {
        ManagedKafkaCluster_GcpConfig {
            kms_key: core::default::Default::default(),
            access_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_GcpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_GcpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaCluster_GcpConfigRef {
        ManagedKafkaCluster_GcpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_GcpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS Key name to use for encryption. The key must be located in the same region as the cluster and cannot be changed. Must be in the format 'projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY'."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> terrars::ListRef<ManagedKafkaCluster_GcpConfig_AccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ManagedKafkaCluster_RebalanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ManagedKafkaCluster_RebalanceConfig {
    #[doc =
        "Set the field `mode`.\nThe rebalance behavior for the cluster. When not specified, defaults to 'NO_REBALANCE'. Possible values: 'MODE_UNSPECIFIED', 'NO_REBALANCE', 'AUTO_REBALANCE_ON_SCALE_UP'."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ManagedKafkaCluster_RebalanceConfig {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_RebalanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_RebalanceConfig {}

impl BuildManagedKafkaCluster_RebalanceConfig {
    pub fn build(self) -> ManagedKafkaCluster_RebalanceConfig {
        ManagedKafkaCluster_RebalanceConfig { mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_RebalanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_RebalanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaCluster_RebalanceConfigRef {
        ManagedKafkaCluster_RebalanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_RebalanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe rebalance behavior for the cluster. When not specified, defaults to 'NO_REBALANCE'. Possible values: 'MODE_UNSPECIFIED', 'NO_REBALANCE', 'AUTO_REBALANCE_ON_SCALE_UP'."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ManagedKafkaCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ManagedKafkaCluster_Timeouts {
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

impl terrars::ToListMappable for ManagedKafkaCluster_Timeouts {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_Timeouts {}

impl BuildManagedKafkaCluster_Timeouts {
    pub fn build(self) -> ManagedKafkaCluster_Timeouts {
        ManagedKafkaCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaCluster_TimeoutsRef {
        ManagedKafkaCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_TimeoutsRef {
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
pub struct ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs {
    ca_pool: terrars::PrimField<std::string::String>,
}

impl ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs { }

impl terrars::ToListMappable for ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs {
    #[doc =
        "The name of the CA pool to pull CA certificates from. The CA pool does not need to be in the same project or location as the Kafka cluster. Must be in the format 'projects/PROJECT_ID/locations/LOCATION/caPools/CA_POOL_ID."]
    pub ca_pool: terrars::PrimField<std::string::String>,
}

impl BuildManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs {
    pub fn build(self) -> ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs {
        ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs { ca_pool: self.ca_pool }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigsRef {
        ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_pool` after provisioning.\nThe name of the CA pool to pull CA certificates from. The CA pool does not need to be in the same project or location as the Kafka cluster. Must be in the format 'projects/PROJECT_ID/locations/LOCATION/caPools/CA_POOL_ID."]
    pub fn ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_pool", self.base))
    }
}

#[derive(Serialize, Default)]
struct ManagedKafkaCluster_TlsConfig_TrustConfigDynamic {
    cas_configs: std::option::Option<terrars::DynamicBlock<ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs>>,
}

#[derive(Serialize)]
pub struct ManagedKafkaCluster_TlsConfig_TrustConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cas_configs: std::option::Option<std::vec::Vec<ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs>>,
    dynamic: ManagedKafkaCluster_TlsConfig_TrustConfigDynamic,
}

impl ManagedKafkaCluster_TlsConfig_TrustConfig {
    #[doc = "Set the field `cas_configs`.\n"]
    pub fn set_cas_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cas_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cas_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ManagedKafkaCluster_TlsConfig_TrustConfig {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_TlsConfig_TrustConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_TlsConfig_TrustConfig {}

impl BuildManagedKafkaCluster_TlsConfig_TrustConfig {
    pub fn build(self) -> ManagedKafkaCluster_TlsConfig_TrustConfig {
        ManagedKafkaCluster_TlsConfig_TrustConfig {
            cas_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_TlsConfig_TrustConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_TlsConfig_TrustConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaCluster_TlsConfig_TrustConfigRef {
        ManagedKafkaCluster_TlsConfig_TrustConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_TlsConfig_TrustConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cas_configs` after provisioning.\n"]
    pub fn cas_configs(&self) -> terrars::ListRef<ManagedKafkaCluster_TlsConfig_TrustConfig_CasConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cas_configs", self.base))
    }
}

#[derive(Serialize, Default)]
struct ManagedKafkaCluster_TlsConfigDynamic {
    trust_config: std::option::Option<terrars::DynamicBlock<ManagedKafkaCluster_TlsConfig_TrustConfig>>,
}

#[derive(Serialize)]
pub struct ManagedKafkaCluster_TlsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_principal_mapping_rules: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_config: std::option::Option<std::vec::Vec<ManagedKafkaCluster_TlsConfig_TrustConfig>>,
    dynamic: ManagedKafkaCluster_TlsConfigDynamic,
}

impl ManagedKafkaCluster_TlsConfig {
    #[doc =
        "Set the field `ssl_principal_mapping_rules`.\nThe rules for mapping mTLS certificate Distinguished Names (DNs) to shortened principal names for Kafka ACLs. This field corresponds exactly to the ssl.principal.mapping.rules broker config and matches the format and syntax defined in the Apache Kafka documentation. Setting or modifying this field will trigger a rolling restart of the Kafka brokers to apply the change. An empty string means that the default Kafka behavior is used. Example: 'RULE:^CN=(.?),OU=ServiceUsers.$/$1@example.com/,DEFAULT'"]
    pub fn set_ssl_principal_mapping_rules(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_principal_mapping_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_config`.\n"]
    pub fn set_trust_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ManagedKafkaCluster_TlsConfig_TrustConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trust_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trust_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ManagedKafkaCluster_TlsConfig {
    type O = terrars::BlockAssignable<ManagedKafkaCluster_TlsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildManagedKafkaCluster_TlsConfig {}

impl BuildManagedKafkaCluster_TlsConfig {
    pub fn build(self) -> ManagedKafkaCluster_TlsConfig {
        ManagedKafkaCluster_TlsConfig {
            ssl_principal_mapping_rules: core::default::Default::default(),
            trust_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ManagedKafkaCluster_TlsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ManagedKafkaCluster_TlsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ManagedKafkaCluster_TlsConfigRef {
        ManagedKafkaCluster_TlsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ManagedKafkaCluster_TlsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ssl_principal_mapping_rules` after provisioning.\nThe rules for mapping mTLS certificate Distinguished Names (DNs) to shortened principal names for Kafka ACLs. This field corresponds exactly to the ssl.principal.mapping.rules broker config and matches the format and syntax defined in the Apache Kafka documentation. Setting or modifying this field will trigger a rolling restart of the Kafka brokers to apply the change. An empty string means that the default Kafka behavior is used. Example: 'RULE:^CN=(.?),OU=ServiceUsers.$/$1@example.com/,DEFAULT'"]
    pub fn ssl_principal_mapping_rules(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_principal_mapping_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `trust_config` after provisioning.\n"]
    pub fn trust_config(&self) -> terrars::ListRef<ManagedKafkaCluster_TlsConfig_TrustConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_config", self.base))
    }
}
