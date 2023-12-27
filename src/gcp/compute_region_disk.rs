use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionDiskData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    licenses: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_block_size_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    replica_zones: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_disk: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    async_primary_disk: Option<Vec<ComputeRegionDiskAsyncPrimaryDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key: Option<Vec<ComputeRegionDiskDiskEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_os_features: Option<Vec<ComputeRegionDiskGuestOsFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot_encryption_key: Option<Vec<ComputeRegionDiskSourceSnapshotEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionDiskTimeoutsEl>,
    dynamic: ComputeRegionDiskDynamic,
}

struct ComputeRegionDisk_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionDiskData>,
}

#[derive(Clone)]
pub struct ComputeRegionDisk(Rc<ComputeRegionDisk_>);

impl ComputeRegionDisk {
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

    #[doc= "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nLabels to apply to this disk.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `licenses`.\nAny applicable license URI."]
    pub fn set_licenses(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().licenses = Some(v.into());
        self
    }

    #[doc= "Set the field `physical_block_size_bytes`.\nPhysical block size of the persistent disk, in bytes. If not present\nin a request, a default value is used. Currently supported sizes\nare 4096 and 16384, other sizes may be added in the future.\nIf an unsupported value is requested, the error message will list\nthe supported values for the caller's project."]
    pub fn set_physical_block_size_bytes(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().physical_block_size_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nA reference to the region where the disk resides."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\nSize of the persistent disk, specified in GB. You can specify this\nfield when creating a persistent disk using the sourceImage or\nsourceSnapshot parameter, or specify it alone to create an empty\npersistent disk.\n\nIf you specify this field along with sourceImage or sourceSnapshot,\nthe value of sizeGb must not be less than the size of the sourceImage\nor the size of the snapshot."]
    pub fn set_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().size = Some(v.into());
        self
    }

    #[doc= "Set the field `snapshot`.\nThe source snapshot used to create this disk. You can provide this as\na partial or full URL to the resource. For example, the following are\nvalid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot'\n* 'projects/project/global/snapshots/snapshot'\n* 'global/snapshots/snapshot'\n* 'snapshot'"]
    pub fn set_snapshot(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().snapshot = Some(v.into());
        self
    }

    #[doc= "Set the field `source_disk`.\nThe source disk used to create this disk. You can provide this as a partial or full URL to the resource.\nFor example, the following are valid values:\n\n* https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/disks/{disk}\n* https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/disks/{disk}\n* projects/{project}/zones/{zone}/disks/{disk}\n* projects/{project}/regions/{region}/disks/{disk}\n* zones/{zone}/disks/{disk}\n* regions/{region}/disks/{disk}"]
    pub fn set_source_disk(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_disk = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nURL of the disk type resource describing which disk type to use to\ncreate the disk. Provide this when creating the disk."]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `async_primary_disk`.\n"]
    pub fn set_async_primary_disk(self, v: impl Into<BlockAssignable<ComputeRegionDiskAsyncPrimaryDiskEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().async_primary_disk = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.async_primary_disk = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `disk_encryption_key`.\n"]
    pub fn set_disk_encryption_key(self, v: impl Into<BlockAssignable<ComputeRegionDiskDiskEncryptionKeyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_encryption_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_encryption_key = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `guest_os_features`.\n"]
    pub fn set_guest_os_features(self, v: impl Into<BlockAssignable<ComputeRegionDiskGuestOsFeaturesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().guest_os_features = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.guest_os_features = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `source_snapshot_encryption_key`.\n"]
    pub fn set_source_snapshot_encryption_key(
        self,
        v: impl Into<BlockAssignable<ComputeRegionDiskSourceSnapshotEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_snapshot_encryption_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_snapshot_encryption_key = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRegionDiskTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this disk.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_attach_timestamp` after provisioning.\nLast attach timestamp in RFC3339 text format."]
    pub fn last_attach_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_attach_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_detach_timestamp` after provisioning.\nLast detach timestamp in RFC3339 text format."]
    pub fn last_detach_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_detach_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `licenses` after provisioning.\nAny applicable license URI."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.licenses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `physical_block_size_bytes` after provisioning.\nPhysical block size of the persistent disk, in bytes. If not present\nin a request, a default value is used. Currently supported sizes\nare 4096 and 16384, other sizes may be added in the future.\nIf an unsupported value is requested, the error message will list\nthe supported values for the caller's project."]
    pub fn physical_block_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.physical_block_size_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the disk resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `replica_zones` after provisioning.\nURLs of the zones where the disk should be replicated to."]
    pub fn replica_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.replica_zones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nSize of the persistent disk, specified in GB. You can specify this\nfield when creating a persistent disk using the sourceImage or\nsourceSnapshot parameter, or specify it alone to create an empty\npersistent disk.\n\nIf you specify this field along with sourceImage or sourceSnapshot,\nthe value of sizeGb must not be less than the size of the sourceImage\nor the size of the snapshot."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snapshot` after provisioning.\nThe source snapshot used to create this disk. You can provide this as\na partial or full URL to the resource. For example, the following are\nvalid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot'\n* 'projects/project/global/snapshots/snapshot'\n* 'global/snapshots/snapshot'\n* 'snapshot'"]
    pub fn snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_disk` after provisioning.\nThe source disk used to create this disk. You can provide this as a partial or full URL to the resource.\nFor example, the following are valid values:\n\n* https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/disks/{disk}\n* https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/disks/{disk}\n* projects/{project}/zones/{zone}/disks/{disk}\n* projects/{project}/regions/{region}/disks/{disk}\n* zones/{zone}/disks/{disk}\n* regions/{region}/disks/{disk}"]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_disk_id` after provisioning.\nThe ID value of the disk used to create this image. This value may\nbe used to determine whether the image was taken from the current\nor a previous instance of a given disk name."]
    pub fn source_disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_disk_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_snapshot_id` after provisioning.\nThe unique ID of the snapshot used to create this disk. This value\nidentifies the exact snapshot that was used to create this persistent\ndisk. For example, if you created the persistent disk from a snapshot\nthat was later deleted and recreated under the same name, the source\nsnapshot ID would identify the exact version of the snapshot that was\nused."]
    pub fn source_snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nURL of the disk type resource describing which disk type to use to\ncreate the disk. Provide this when creating the disk."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `users` after provisioning.\nLinks to the users of the disk (attached instances) in form:\nproject/zones/zone/instances/instance"]
    pub fn users(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.users", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `async_primary_disk` after provisioning.\n"]
    pub fn async_primary_disk(&self) -> ListRef<ComputeRegionDiskAsyncPrimaryDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.async_primary_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(&self) -> ListRef<ComputeRegionDiskDiskEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk_encryption_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(&self) -> ListRef<ComputeRegionDiskSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_snapshot_encryption_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionDiskTimeoutsElRef {
        ComputeRegionDiskTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ComputeRegionDisk {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeRegionDisk { }

impl ToListMappable for ComputeRegionDisk {
    type O = ListRef<ComputeRegionDiskRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeRegionDisk_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_disk".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRegionDisk {
    pub tf_id: String,
    #[doc= "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc= "URLs of the zones where the disk should be replicated to."]
    pub replica_zones: ListField<PrimField<String>>,
}

impl BuildComputeRegionDisk {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionDisk {
        let out = ComputeRegionDisk(Rc::new(ComputeRegionDisk_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRegionDiskData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                licenses: core::default::Default::default(),
                name: self.name,
                physical_block_size_bytes: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                replica_zones: self.replica_zones,
                size: core::default::Default::default(),
                snapshot: core::default::Default::default(),
                source_disk: core::default::Default::default(),
                type_: core::default::Default::default(),
                async_primary_disk: core::default::Default::default(),
                disk_encryption_key: core::default::Default::default(),
                guest_os_features: core::default::Default::default(),
                source_snapshot_encryption_key: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRegionDiskRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionDiskRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRegionDiskRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this disk.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_attach_timestamp` after provisioning.\nLast attach timestamp in RFC3339 text format."]
    pub fn last_attach_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_attach_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_detach_timestamp` after provisioning.\nLast detach timestamp in RFC3339 text format."]
    pub fn last_detach_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_detach_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `licenses` after provisioning.\nAny applicable license URI."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.licenses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `physical_block_size_bytes` after provisioning.\nPhysical block size of the persistent disk, in bytes. If not present\nin a request, a default value is used. Currently supported sizes\nare 4096 and 16384, other sizes may be added in the future.\nIf an unsupported value is requested, the error message will list\nthe supported values for the caller's project."]
    pub fn physical_block_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.physical_block_size_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the disk resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `replica_zones` after provisioning.\nURLs of the zones where the disk should be replicated to."]
    pub fn replica_zones(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.replica_zones", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nSize of the persistent disk, specified in GB. You can specify this\nfield when creating a persistent disk using the sourceImage or\nsourceSnapshot parameter, or specify it alone to create an empty\npersistent disk.\n\nIf you specify this field along with sourceImage or sourceSnapshot,\nthe value of sizeGb must not be less than the size of the sourceImage\nor the size of the snapshot."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `snapshot` after provisioning.\nThe source snapshot used to create this disk. You can provide this as\na partial or full URL to the resource. For example, the following are\nvalid values:\n\n* 'https://www.googleapis.com/compute/v1/projects/project/global/snapshots/snapshot'\n* 'projects/project/global/snapshots/snapshot'\n* 'global/snapshots/snapshot'\n* 'snapshot'"]
    pub fn snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.snapshot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_disk` after provisioning.\nThe source disk used to create this disk. You can provide this as a partial or full URL to the resource.\nFor example, the following are valid values:\n\n* https://www.googleapis.com/compute/v1/projects/{project}/zones/{zone}/disks/{disk}\n* https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/disks/{disk}\n* projects/{project}/zones/{zone}/disks/{disk}\n* projects/{project}/regions/{region}/disks/{disk}\n* zones/{zone}/disks/{disk}\n* regions/{region}/disks/{disk}"]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_disk_id` after provisioning.\nThe ID value of the disk used to create this image. This value may\nbe used to determine whether the image was taken from the current\nor a previous instance of a given disk name."]
    pub fn source_disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_disk_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_snapshot_id` after provisioning.\nThe unique ID of the snapshot used to create this disk. This value\nidentifies the exact snapshot that was used to create this persistent\ndisk. For example, if you created the persistent disk from a snapshot\nthat was later deleted and recreated under the same name, the source\nsnapshot ID would identify the exact version of the snapshot that was\nused."]
    pub fn source_snapshot_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nURL of the disk type resource describing which disk type to use to\ncreate the disk. Provide this when creating the disk."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `users` after provisioning.\nLinks to the users of the disk (attached instances) in form:\nproject/zones/zone/instances/instance"]
    pub fn users(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.users", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `async_primary_disk` after provisioning.\n"]
    pub fn async_primary_disk(&self) -> ListRef<ComputeRegionDiskAsyncPrimaryDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.async_primary_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(&self) -> ListRef<ComputeRegionDiskDiskEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk_encryption_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(&self) -> ListRef<ComputeRegionDiskSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_snapshot_encryption_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionDiskTimeoutsElRef {
        ComputeRegionDiskTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionDiskAsyncPrimaryDiskEl {
    disk: PrimField<String>,
}

impl ComputeRegionDiskAsyncPrimaryDiskEl { }

impl ToListMappable for ComputeRegionDiskAsyncPrimaryDiskEl {
    type O = BlockAssignable<ComputeRegionDiskAsyncPrimaryDiskEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionDiskAsyncPrimaryDiskEl {
    #[doc= "Primary disk for asynchronous disk replication."]
    pub disk: PrimField<String>,
}

impl BuildComputeRegionDiskAsyncPrimaryDiskEl {
    pub fn build(self) -> ComputeRegionDiskAsyncPrimaryDiskEl {
        ComputeRegionDiskAsyncPrimaryDiskEl { disk: self.disk }
    }
}

pub struct ComputeRegionDiskAsyncPrimaryDiskElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionDiskAsyncPrimaryDiskElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionDiskAsyncPrimaryDiskElRef {
        ComputeRegionDiskAsyncPrimaryDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionDiskAsyncPrimaryDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\nPrimary disk for asynchronous disk replication."]
    pub fn disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionDiskDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
}

impl ComputeRegionDiskDiskEncryptionKeyEl {
    #[doc= "Set the field `kms_key_name`.\nThe name of the encryption key that is stored in Google Cloud KMS."]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }

    #[doc= "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionDiskDiskEncryptionKeyEl {
    type O = BlockAssignable<ComputeRegionDiskDiskEncryptionKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionDiskDiskEncryptionKeyEl {}

impl BuildComputeRegionDiskDiskEncryptionKeyEl {
    pub fn build(self) -> ComputeRegionDiskDiskEncryptionKeyEl {
        ComputeRegionDiskDiskEncryptionKeyEl {
            kms_key_name: core::default::Default::default(),
            raw_key: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionDiskDiskEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionDiskDiskEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionDiskDiskEncryptionKeyElRef {
        ComputeRegionDiskDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionDiskDiskEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nThe name of the encryption key that is stored in Google Cloud KMS."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc= "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc= "Get a reference to the value of field `sha256` after provisioning.\nThe RFC 4648 base64 encoded SHA-256 hash of the customer-supplied\nencryption key that protects this resource."]
    pub fn sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionDiskGuestOsFeaturesEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl ComputeRegionDiskGuestOsFeaturesEl { }

impl ToListMappable for ComputeRegionDiskGuestOsFeaturesEl {
    type O = BlockAssignable<ComputeRegionDiskGuestOsFeaturesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionDiskGuestOsFeaturesEl {
    #[doc= "The type of supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options. Possible values: [\"MULTI_IP_SUBNET\", \"SECURE_BOOT\", \"SEV_CAPABLE\", \"UEFI_COMPATIBLE\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"GVNIC\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"TDX_CAPABLE\"]"]
    pub type_: PrimField<String>,
}

impl BuildComputeRegionDiskGuestOsFeaturesEl {
    pub fn build(self) -> ComputeRegionDiskGuestOsFeaturesEl {
        ComputeRegionDiskGuestOsFeaturesEl { type_: self.type_ }
    }
}

pub struct ComputeRegionDiskGuestOsFeaturesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionDiskGuestOsFeaturesElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionDiskGuestOsFeaturesElRef {
        ComputeRegionDiskGuestOsFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionDiskGuestOsFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options. Possible values: [\"MULTI_IP_SUBNET\", \"SECURE_BOOT\", \"SEV_CAPABLE\", \"UEFI_COMPATIBLE\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"GVNIC\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"TDX_CAPABLE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionDiskSourceSnapshotEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_key: Option<PrimField<String>>,
}

impl ComputeRegionDiskSourceSnapshotEncryptionKeyEl {
    #[doc= "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn set_raw_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raw_key = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionDiskSourceSnapshotEncryptionKeyEl {
    type O = BlockAssignable<ComputeRegionDiskSourceSnapshotEncryptionKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionDiskSourceSnapshotEncryptionKeyEl {}

impl BuildComputeRegionDiskSourceSnapshotEncryptionKeyEl {
    pub fn build(self) -> ComputeRegionDiskSourceSnapshotEncryptionKeyEl {
        ComputeRegionDiskSourceSnapshotEncryptionKeyEl { raw_key: core::default::Default::default() }
    }
}

pub struct ComputeRegionDiskSourceSnapshotEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionDiskSourceSnapshotEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionDiskSourceSnapshotEncryptionKeyElRef {
        ComputeRegionDiskSourceSnapshotEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionDiskSourceSnapshotEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc= "Get a reference to the value of field `sha256` after provisioning.\nThe RFC 4648 base64 encoded SHA-256 hash of the customer-supplied\nencryption key that protects this resource."]
    pub fn sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionDiskTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeRegionDiskTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionDiskTimeoutsEl {
    type O = BlockAssignable<ComputeRegionDiskTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionDiskTimeoutsEl {}

impl BuildComputeRegionDiskTimeoutsEl {
    pub fn build(self) -> ComputeRegionDiskTimeoutsEl {
        ComputeRegionDiskTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionDiskTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionDiskTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionDiskTimeoutsElRef {
        ComputeRegionDiskTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionDiskTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionDiskDynamic {
    async_primary_disk: Option<DynamicBlock<ComputeRegionDiskAsyncPrimaryDiskEl>>,
    disk_encryption_key: Option<DynamicBlock<ComputeRegionDiskDiskEncryptionKeyEl>>,
    guest_os_features: Option<DynamicBlock<ComputeRegionDiskGuestOsFeaturesEl>>,
    source_snapshot_encryption_key: Option<DynamicBlock<ComputeRegionDiskSourceSnapshotEncryptionKeyEl>>,
}
