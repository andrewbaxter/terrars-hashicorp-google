use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigtableInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster: Option<Vec<BigtableInstanceClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigtableInstanceTimeoutsEl>,
    dynamic: BigtableInstanceDynamic,
}

struct BigtableInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigtableInstanceData>,
}

#[derive(Clone)]
pub struct BigtableInstance(Rc<BigtableInstance_>);

impl BigtableInstance {
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

    #[doc= "Set the field `deletion_protection`.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `display_name`.\nThe human-readable display name of the Bigtable instance. Defaults to the instance name."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_type`.\nThe instance type to create. One of \"DEVELOPMENT\" or \"PRODUCTION\". Defaults to \"PRODUCTION\"."]
    pub fn set_instance_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_type = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nA mapping of labels to assign to the resource.\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `cluster`.\n"]
    pub fn set_cluster(self, v: impl Into<BlockAssignable<BigtableInstanceClusterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigtableInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable display name of the Bigtable instance. Defaults to the instance name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\nThe instance type to create. One of \"DEVELOPMENT\" or \"PRODUCTION\". Defaults to \"PRODUCTION\"."]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA mapping of labels to assign to the resource.\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name (also called Instance Id in the Cloud Console) of the Cloud Bigtable instance. Must be 6-33 characters and must only contain hyphens, lowercase letters and numbers."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> ListRef<BigtableInstanceClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableInstanceTimeoutsElRef {
        BigtableInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for BigtableInstance {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BigtableInstance { }

impl ToListMappable for BigtableInstance {
    type O = ListRef<BigtableInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BigtableInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_bigtable_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigtableInstance {
    pub tf_id: String,
    #[doc= "The name (also called Instance Id in the Cloud Console) of the Cloud Bigtable instance. Must be 6-33 characters and must only contain hyphens, lowercase letters and numbers."]
    pub name: PrimField<String>,
}

impl BuildBigtableInstance {
    pub fn build(self, stack: &mut Stack) -> BigtableInstance {
        let out = BigtableInstance(Rc::new(BigtableInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigtableInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_protection: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                cluster: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigtableInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigtableInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigtableInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable display name of the Bigtable instance. Defaults to the instance name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\nThe instance type to create. One of \"DEVELOPMENT\" or \"PRODUCTION\". Defaults to \"PRODUCTION\"."]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA mapping of labels to assign to the resource.\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name (also called Instance Id in the Cloud Console) of the Cloud Bigtable instance. Must be 6-33 characters and must only contain hyphens, lowercase letters and numbers."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> ListRef<BigtableInstanceClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableInstanceTimeoutsElRef {
        BigtableInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BigtableInstanceClusterElAutoscalingConfigEl {
    cpu_target: PrimField<f64>,
    max_nodes: PrimField<f64>,
    min_nodes: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_target: Option<PrimField<f64>>,
}

impl BigtableInstanceClusterElAutoscalingConfigEl {
    #[doc= "Set the field `storage_target`.\nThe target storage utilization for autoscaling, in GB, for each node in a cluster. This number is limited between 2560 (2.5TiB) and 5120 (5TiB) for a SSD cluster and between 8192 (8TiB) and 16384 (16 TiB) for an HDD cluster. If not set, whatever is already set for the cluster will not change, or if the cluster is just being created, it will use the default value of 2560 for SSD clusters and 8192 for HDD clusters."]
    pub fn set_storage_target(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_target = Some(v.into());
        self
    }
}

impl ToListMappable for BigtableInstanceClusterElAutoscalingConfigEl {
    type O = BlockAssignable<BigtableInstanceClusterElAutoscalingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableInstanceClusterElAutoscalingConfigEl {
    #[doc= "The target CPU utilization for autoscaling. Value must be between 10 and 80."]
    pub cpu_target: PrimField<f64>,
    #[doc= "The maximum number of nodes for autoscaling."]
    pub max_nodes: PrimField<f64>,
    #[doc= "The minimum number of nodes for autoscaling."]
    pub min_nodes: PrimField<f64>,
}

impl BuildBigtableInstanceClusterElAutoscalingConfigEl {
    pub fn build(self) -> BigtableInstanceClusterElAutoscalingConfigEl {
        BigtableInstanceClusterElAutoscalingConfigEl {
            cpu_target: self.cpu_target,
            max_nodes: self.max_nodes,
            min_nodes: self.min_nodes,
            storage_target: core::default::Default::default(),
        }
    }
}

pub struct BigtableInstanceClusterElAutoscalingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigtableInstanceClusterElAutoscalingConfigElRef {
    fn new(shared: StackShared, base: String) -> BigtableInstanceClusterElAutoscalingConfigElRef {
        BigtableInstanceClusterElAutoscalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableInstanceClusterElAutoscalingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cpu_target` after provisioning.\nThe target CPU utilization for autoscaling. Value must be between 10 and 80."]
    pub fn cpu_target(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_target", self.base))
    }

    #[doc= "Get a reference to the value of field `max_nodes` after provisioning.\nThe maximum number of nodes for autoscaling."]
    pub fn max_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `min_nodes` after provisioning.\nThe minimum number of nodes for autoscaling."]
    pub fn min_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `storage_target` after provisioning.\nThe target storage utilization for autoscaling, in GB, for each node in a cluster. This number is limited between 2560 (2.5TiB) and 5120 (5TiB) for a SSD cluster and between 8192 (8TiB) and 16384 (16 TiB) for an HDD cluster. If not set, whatever is already set for the cluster will not change, or if the cluster is just being created, it will use the default value of 2560 for SSD clusters and 8192 for HDD clusters."]
    pub fn storage_target(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_target", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigtableInstanceClusterElDynamic {
    autoscaling_config: Option<DynamicBlock<BigtableInstanceClusterElAutoscalingConfigEl>>,
}

#[derive(Serialize)]
pub struct BigtableInstanceClusterEl {
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_nodes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_config: Option<Vec<BigtableInstanceClusterElAutoscalingConfigEl>>,
    dynamic: BigtableInstanceClusterElDynamic,
}

impl BigtableInstanceClusterEl {
    #[doc= "Set the field `kms_key_name`.\nDescribes the Cloud KMS encryption key that will be used to protect the destination Bigtable cluster. The requirements for this key are: 1) The Cloud Bigtable service account associated with the project that contains this cluster must be granted the cloudkms.cryptoKeyEncrypterDecrypter role on the CMEK key. 2) Only regional keys can be used and the region of the CMEK key must match the region of the cluster. 3) All clusters within an instance must use the same CMEK key. Values are of the form projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }

    #[doc= "Set the field `num_nodes`.\nThe number of nodes in the cluster. If no value is set, Cloud Bigtable automatically allocates nodes based on your data footprint and optimized for 50% storage utilization."]
    pub fn set_num_nodes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_nodes = Some(v.into());
        self
    }

    #[doc= "Set the field `storage_type`.\nThe storage type to use. One of \"SSD\" or \"HDD\". Defaults to \"SSD\"."]
    pub fn set_storage_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_type = Some(v.into());
        self
    }

    #[doc= "Set the field `zone`.\nThe zone to create the Cloud Bigtable cluster in. Each cluster must have a different zone in the same region. Zones that support Bigtable instances are noted on the Cloud Bigtable locations page."]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }

    #[doc= "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        mut self,
        v: impl Into<BlockAssignable<BigtableInstanceClusterElAutoscalingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BigtableInstanceClusterEl {
    type O = BlockAssignable<BigtableInstanceClusterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableInstanceClusterEl {
    #[doc= "The ID of the Cloud Bigtable cluster. Must be 6-30 characters and must only contain hyphens, lowercase letters and numbers."]
    pub cluster_id: PrimField<String>,
}

impl BuildBigtableInstanceClusterEl {
    pub fn build(self) -> BigtableInstanceClusterEl {
        BigtableInstanceClusterEl {
            cluster_id: self.cluster_id,
            kms_key_name: core::default::Default::default(),
            num_nodes: core::default::Default::default(),
            storage_type: core::default::Default::default(),
            zone: core::default::Default::default(),
            autoscaling_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BigtableInstanceClusterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigtableInstanceClusterElRef {
    fn new(shared: StackShared, base: String) -> BigtableInstanceClusterElRef {
        BigtableInstanceClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableInstanceClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID of the Cloud Bigtable cluster. Must be 6-30 characters and must only contain hyphens, lowercase letters and numbers."]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nDescribes the Cloud KMS encryption key that will be used to protect the destination Bigtable cluster. The requirements for this key are: 1) The Cloud Bigtable service account associated with the project that contains this cluster must be granted the cloudkms.cryptoKeyEncrypterDecrypter role on the CMEK key. 2) Only regional keys can be used and the region of the CMEK key must match the region of the cluster. 3) All clusters within an instance must use the same CMEK key. Values are of the form projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc= "Get a reference to the value of field `num_nodes` after provisioning.\nThe number of nodes in the cluster. If no value is set, Cloud Bigtable automatically allocates nodes based on your data footprint and optimized for 50% storage utilization."]
    pub fn num_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_nodes", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe state of the cluster"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `storage_type` after provisioning.\nThe storage type to use. One of \"SSD\" or \"HDD\". Defaults to \"SSD\"."]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe zone to create the Cloud Bigtable cluster in. Each cluster must have a different zone in the same region. Zones that support Bigtable instances are noted on the Cloud Bigtable locations page."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }

    #[doc= "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> ListRef<BigtableInstanceClusterElAutoscalingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BigtableInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BigtableInstanceTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for BigtableInstanceTimeoutsEl {
    type O = BlockAssignable<BigtableInstanceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableInstanceTimeoutsEl {}

impl BuildBigtableInstanceTimeoutsEl {
    pub fn build(self) -> BigtableInstanceTimeoutsEl {
        BigtableInstanceTimeoutsEl {
            create: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BigtableInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigtableInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigtableInstanceTimeoutsElRef {
        BigtableInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableInstanceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigtableInstanceDynamic {
    cluster: Option<DynamicBlock<BigtableInstanceClusterEl>>,
}
