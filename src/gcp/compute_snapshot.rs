use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeSnapshotData {
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
    chain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    source_disk: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_encryption_key: std::option::Option<std::vec::Vec<ComputeSnapshotSnapshotEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_disk_encryption_key: std::option::Option<std::vec::Vec<ComputeSnapshotSourceDiskEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeSnapshotTimeoutsEl>,
    dynamic: ComputeSnapshotDynamic,
}

struct ComputeSnapshot_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeSnapshotData>,
}

#[derive(Clone)]
pub struct ComputeSnapshot(std::rc::Rc<ComputeSnapshot_>);

impl ComputeSnapshot {
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
        "Set the field `chain_name`.\nCreates the new snapshot in the snapshot chain labeled with the\nspecified name. The chain name must be 1-63 characters long and\ncomply with RFC1035. This is an uncommon option only for advanced\nservice owners who needs to create separate snapshot chains, for\nexample, for chargeback tracking.  When you describe your snapshot\nresource, this field is visible only if it has a non-empty value."]
    pub fn set_chain_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().chain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
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
        "Set the field `labels`.\nLabels to apply to this Snapshot.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc =
        "Set the field `storage_locations`.\nCloud Storage bucket storage location of the snapshot (regional or multi-regional)."]
    pub fn set_storage_locations(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().storage_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nA reference to the zone where the disk is hosted."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_encryption_key`.\n"]
    pub fn set_snapshot_encryption_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSnapshotSnapshotEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snapshot_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snapshot_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_disk_encryption_key`.\n"]
    pub fn set_source_disk_encryption_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSnapshotSourceDiskEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_disk_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_disk_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeSnapshotTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `chain_name` after provisioning.\nCreates the new snapshot in the snapshot chain labeled with the\nspecified name. The chain name must be 1-63 characters long and\ncomply with RFC1035. This is an uncommon option only for advanced\nservice owners who needs to create separate snapshot chains, for\nexample, for chargeback tracking.  When you describe your snapshot\nresource, this field is visible only if it has a non-empty value."]
    pub fn chain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the snapshot, specified in GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.extract_ref()))
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
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource. Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this Snapshot.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `licenses` after provisioning.\nA list of public visible licenses that apply to this snapshot. This\ncan be because the original image had licenses attached (such as a\nWindows image).  snapshotEncryptionKey nested object Encrypts the\nsnapshot using a customer-supplied encryption key."]
    pub fn licenses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.licenses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `snapshot_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn snapshot_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_disk` after provisioning.\nA reference to the disk used to create this snapshot."]
    pub fn source_disk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_bytes` after provisioning.\nA size of the storage used by the snapshot. As snapshots share\nstorage, this number is expected to change with snapshot\ncreation/deletion."]
    pub fn storage_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket storage location of the snapshot (regional or multi-regional)."]
    pub fn storage_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_locations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nA reference to the zone where the disk is hosted."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_encryption_key` after provisioning.\n"]
    pub fn snapshot_encryption_key(&self) -> terrars::ListRef<ComputeSnapshotSnapshotEncryptionKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_disk_encryption_key` after provisioning.\n"]
    pub fn source_disk_encryption_key(&self) -> terrars::ListRef<ComputeSnapshotSourceDiskEncryptionKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_disk_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSnapshotTimeoutsElRef {
        ComputeSnapshotTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeSnapshot {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeSnapshot { }

impl terrars::ToListMappable for ComputeSnapshot {
    type O = terrars::ListRef<ComputeSnapshotRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeSnapshot_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_snapshot".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeSnapshot {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "A reference to the disk used to create this snapshot."]
    pub source_disk: terrars::PrimField<std::string::String>,
}

impl BuildComputeSnapshot {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeSnapshot {
        let out = ComputeSnapshot(std::rc::Rc::new(ComputeSnapshot_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeSnapshotData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                chain_name: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                source_disk: self.source_disk,
                storage_locations: core::default::Default::default(),
                zone: core::default::Default::default(),
                snapshot_encryption_key: core::default::Default::default(),
                source_disk_encryption_key: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeSnapshotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSnapshotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeSnapshotRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `chain_name` after provisioning.\nCreates the new snapshot in the snapshot chain labeled with the\nspecified name. The chain name must be 1-63 characters long and\ncomply with RFC1035. This is an uncommon option only for advanced\nservice owners who needs to create separate snapshot chains, for\nexample, for chargeback tracking.  When you describe your snapshot\nresource, this field is visible only if it has a non-empty value."]
    pub fn chain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the snapshot, specified in GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.extract_ref()))
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
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource. Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this Snapshot.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `licenses` after provisioning.\nA list of public visible licenses that apply to this snapshot. This\ncan be because the original image had licenses attached (such as a\nWindows image).  snapshotEncryptionKey nested object Encrypts the\nsnapshot using a customer-supplied encryption key."]
    pub fn licenses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.licenses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `snapshot_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn snapshot_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_disk` after provisioning.\nA reference to the disk used to create this snapshot."]
    pub fn source_disk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_bytes` after provisioning.\nA size of the storage used by the snapshot. As snapshots share\nstorage, this number is expected to change with snapshot\ncreation/deletion."]
    pub fn storage_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket storage location of the snapshot (regional or multi-regional)."]
    pub fn storage_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_locations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nA reference to the zone where the disk is hosted."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_encryption_key` after provisioning.\n"]
    pub fn snapshot_encryption_key(&self) -> terrars::ListRef<ComputeSnapshotSnapshotEncryptionKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_disk_encryption_key` after provisioning.\n"]
    pub fn source_disk_encryption_key(&self) -> terrars::ListRef<ComputeSnapshotSourceDiskEncryptionKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_disk_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSnapshotTimeoutsElRef {
        ComputeSnapshotTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeSnapshotSnapshotEncryptionKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSnapshotSnapshotEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\nThe name of the encryption key that is stored in Google Cloud KMS."]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_service_account`.\nThe service account used for the encryption request for the given KMS key.\nIf absent, the Compute Engine Service Agent service account is used."]
    pub fn set_kms_key_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn set_raw_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.raw_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSnapshotSnapshotEncryptionKeyEl {
    type O = terrars::BlockAssignable<ComputeSnapshotSnapshotEncryptionKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSnapshotSnapshotEncryptionKeyEl {}

impl BuildComputeSnapshotSnapshotEncryptionKeyEl {
    pub fn build(self) -> ComputeSnapshotSnapshotEncryptionKeyEl {
        ComputeSnapshotSnapshotEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
        }
    }
}

pub struct ComputeSnapshotSnapshotEncryptionKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSnapshotSnapshotEncryptionKeyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSnapshotSnapshotEncryptionKeyElRef {
        ComputeSnapshotSnapshotEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSnapshotSnapshotEncryptionKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe name of the encryption key that is stored in Google Cloud KMS."]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account used for the encryption request for the given KMS key.\nIf absent, the Compute Engine Service Agent service account is used."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sha256` after provisioning.\nThe RFC 4648 base64 encoded SHA-256 hash of the customer-supplied\nencryption key that protects this resource."]
    pub fn sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSnapshotSourceDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSnapshotSourceDiskEncryptionKeyEl {
    #[doc =
        "Set the field `kms_key_service_account`.\nThe service account used for the encryption request for the given KMS key.\nIf absent, the Compute Engine Service Agent service account is used."]
    pub fn set_kms_key_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn set_raw_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.raw_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSnapshotSourceDiskEncryptionKeyEl {
    type O = terrars::BlockAssignable<ComputeSnapshotSourceDiskEncryptionKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSnapshotSourceDiskEncryptionKeyEl {}

impl BuildComputeSnapshotSourceDiskEncryptionKeyEl {
    pub fn build(self) -> ComputeSnapshotSourceDiskEncryptionKeyEl {
        ComputeSnapshotSourceDiskEncryptionKeyEl {
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
        }
    }
}

pub struct ComputeSnapshotSourceDiskEncryptionKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSnapshotSourceDiskEncryptionKeyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSnapshotSourceDiskEncryptionKeyElRef {
        ComputeSnapshotSourceDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSnapshotSourceDiskEncryptionKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account used for the encryption request for the given KMS key.\nIf absent, the Compute Engine Service Agent service account is used."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSnapshotTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSnapshotTimeoutsEl {
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

impl terrars::ToListMappable for ComputeSnapshotTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeSnapshotTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSnapshotTimeoutsEl {}

impl BuildComputeSnapshotTimeoutsEl {
    pub fn build(self) -> ComputeSnapshotTimeoutsEl {
        ComputeSnapshotTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeSnapshotTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSnapshotTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSnapshotTimeoutsElRef {
        ComputeSnapshotTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSnapshotTimeoutsElRef {
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
struct ComputeSnapshotDynamic {
    snapshot_encryption_key: std::option::Option<terrars::DynamicBlock<ComputeSnapshotSnapshotEncryptionKeyEl>>,
    source_disk_encryption_key: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSnapshotSourceDiskEncryptionKeyEl>>,
}
