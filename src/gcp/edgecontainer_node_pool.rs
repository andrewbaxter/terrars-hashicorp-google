use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct EdgecontainerNodePoolData {
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
    cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    node_count: terrars::PrimField<f64>,
    node_location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_disk_encryption: std::option::Option<std::vec::Vec<EdgecontainerNodePoolLocalDiskEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<EdgecontainerNodePoolNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EdgecontainerNodePoolTimeoutsEl>,
    dynamic: EdgecontainerNodePoolDynamic,
}

struct EdgecontainerNodePool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EdgecontainerNodePoolData>,
}

#[derive(Clone)]
pub struct EdgecontainerNodePool(std::rc::Rc<EdgecontainerNodePool_>);

impl EdgecontainerNodePool {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels associated with this resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_filter`.\nOnly machines matching this filter will be allowed to join the node pool.\nThe filtering language accepts strings like \"name=<name>\", and is\ndocumented in more detail in [AIP-160](https://google.aip.dev/160)."]
    pub fn set_machine_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().machine_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_disk_encryption`.\n"]
    pub fn set_local_disk_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerNodePoolLocalDiskEncryptionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().local_disk_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.local_disk_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EdgecontainerNodePoolNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EdgecontainerNodePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe name of the target Distributed Cloud Edge Cluster."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the node pool was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels associated with this resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `machine_filter` after provisioning.\nOnly machines matching this filter will be allowed to join the node pool.\nThe filtering language accepts strings like \"name=<name>\", and is\ndocumented in more detail in [AIP-160](https://google.aip.dev/160)."]
    pub fn machine_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the node pool."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes in the pool."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_location` after provisioning.\nName of the Google Distributed Cloud Edge zone where this node pool will be created. For example: 'us-central1-edge-customer-a'."]
    pub fn node_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_version` after provisioning.\nThe lowest release version among all worker nodes."]
    pub fn node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the node pool was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_disk_encryption` after provisioning.\n"]
    pub fn local_disk_encryption(&self) -> terrars::ListRef<EdgecontainerNodePoolLocalDiskEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_disk_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<EdgecontainerNodePoolNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerNodePoolTimeoutsElRef {
        EdgecontainerNodePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EdgecontainerNodePool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EdgecontainerNodePool { }

impl terrars::ToListMappable for EdgecontainerNodePool {
    type O = terrars::ListRef<EdgecontainerNodePoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EdgecontainerNodePool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_edgecontainer_node_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEdgecontainerNodePool {
    pub tf_id: std::string::String,
    #[doc = "The name of the target Distributed Cloud Edge Cluster."]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The resource name of the node pool."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The number of nodes in the pool."]
    pub node_count: terrars::PrimField<f64>,
    #[doc =
        "Name of the Google Distributed Cloud Edge zone where this node pool will be created. For example: 'us-central1-edge-customer-a'."]
    pub node_location: terrars::PrimField<std::string::String>,
}

impl BuildEdgecontainerNodePool {
    pub fn build(self, stack: &mut terrars::Stack) -> EdgecontainerNodePool {
        let out = EdgecontainerNodePool(std::rc::Rc::new(EdgecontainerNodePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EdgecontainerNodePoolData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster: self.cluster,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                machine_filter: core::default::Default::default(),
                name: self.name,
                node_count: self.node_count,
                node_location: self.node_location,
                project: core::default::Default::default(),
                local_disk_encryption: core::default::Default::default(),
                node_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EdgecontainerNodePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerNodePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EdgecontainerNodePoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe name of the target Distributed Cloud Edge Cluster."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the node pool was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels associated with this resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `machine_filter` after provisioning.\nOnly machines matching this filter will be allowed to join the node pool.\nThe filtering language accepts strings like \"name=<name>\", and is\ndocumented in more detail in [AIP-160](https://google.aip.dev/160)."]
    pub fn machine_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the node pool."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes in the pool."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_location` after provisioning.\nName of the Google Distributed Cloud Edge zone where this node pool will be created. For example: 'us-central1-edge-customer-a'."]
    pub fn node_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `node_version` after provisioning.\nThe lowest release version among all worker nodes."]
    pub fn node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the node pool was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_disk_encryption` after provisioning.\n"]
    pub fn local_disk_encryption(&self) -> terrars::ListRef<EdgecontainerNodePoolLocalDiskEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_disk_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<EdgecontainerNodePoolNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EdgecontainerNodePoolTimeoutsElRef {
        EdgecontainerNodePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerNodePoolLocalDiskEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerNodePoolLocalDiskEncryptionEl {
    #[doc =
        "Set the field `kms_key`.\nThe Cloud KMS CryptoKey e.g. projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey} to use for protecting node local disks.\nIf not specified, a Google-managed key will be used instead."]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerNodePoolLocalDiskEncryptionEl {
    type O = terrars::BlockAssignable<EdgecontainerNodePoolLocalDiskEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerNodePoolLocalDiskEncryptionEl {}

impl BuildEdgecontainerNodePoolLocalDiskEncryptionEl {
    pub fn build(self) -> EdgecontainerNodePoolLocalDiskEncryptionEl {
        EdgecontainerNodePoolLocalDiskEncryptionEl { kms_key: core::default::Default::default() }
    }
}

pub struct EdgecontainerNodePoolLocalDiskEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerNodePoolLocalDiskEncryptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerNodePoolLocalDiskEncryptionElRef {
        EdgecontainerNodePoolLocalDiskEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerNodePoolLocalDiskEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS CryptoKey e.g. projects/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{cryptoKey} to use for protecting node local disks.\nIf not specified, a Google-managed key will be used instead."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_active_version` after provisioning.\nThe Cloud KMS CryptoKeyVersion currently in use for protecting node local disks. Only applicable if kmsKey is set."]
    pub fn kms_key_active_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_active_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_state` after provisioning.\nAvailability of the Cloud KMS CryptoKey. If not KEY_AVAILABLE, then nodes may go offline as they cannot access their local data.\nThis can be caused by a lack of permissions to use the key, or if the key is disabled or deleted."]
    pub fn kms_key_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_state", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerNodePoolNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl EdgecontainerNodePoolNodeConfigEl {
    #[doc = "Set the field `labels`.\n\"The Kubernetes node labels\""]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EdgecontainerNodePoolNodeConfigEl {
    type O = terrars::BlockAssignable<EdgecontainerNodePoolNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerNodePoolNodeConfigEl {}

impl BuildEdgecontainerNodePoolNodeConfigEl {
    pub fn build(self) -> EdgecontainerNodePoolNodeConfigEl {
        EdgecontainerNodePoolNodeConfigEl { labels: core::default::Default::default() }
    }
}

pub struct EdgecontainerNodePoolNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerNodePoolNodeConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerNodePoolNodeConfigElRef {
        EdgecontainerNodePoolNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerNodePoolNodeConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n\"The Kubernetes node labels\""]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize)]
pub struct EdgecontainerNodePoolTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EdgecontainerNodePoolTimeoutsEl {
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

impl terrars::ToListMappable for EdgecontainerNodePoolTimeoutsEl {
    type O = terrars::BlockAssignable<EdgecontainerNodePoolTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEdgecontainerNodePoolTimeoutsEl {}

impl BuildEdgecontainerNodePoolTimeoutsEl {
    pub fn build(self) -> EdgecontainerNodePoolTimeoutsEl {
        EdgecontainerNodePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EdgecontainerNodePoolTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EdgecontainerNodePoolTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EdgecontainerNodePoolTimeoutsElRef {
        EdgecontainerNodePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EdgecontainerNodePoolTimeoutsElRef {
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
struct EdgecontainerNodePoolDynamic {
    local_disk_encryption: std::option::Option<terrars::DynamicBlock<EdgecontainerNodePoolLocalDiskEncryptionEl>>,
    node_config: std::option::Option<terrars::DynamicBlock<EdgecontainerNodePoolNodeConfigEl>>,
}
