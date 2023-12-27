use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeImageData {
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
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    licenses: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_disk: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_locations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_os_features: Option<Vec<ComputeImageGuestOsFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_encryption_key: Option<Vec<ComputeImageImageEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_disk: Option<Vec<ComputeImageRawDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeImageTimeoutsEl>,
    dynamic: ComputeImageDynamic,
}

struct ComputeImage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeImageData>,
}

#[derive(Clone)]
pub struct ComputeImage(Rc<ComputeImage_>);

impl ComputeImage {
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

    #[doc= "Set the field `disk_size_gb`.\nSize of the image when restored onto a persistent disk (in GB)."]
    pub fn set_disk_size_gb(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().disk_size_gb = Some(v.into());
        self
    }

    #[doc= "Set the field `family`.\nThe name of the image family to which this image belongs. You can\ncreate disks by specifying an image family instead of a specific\nimage name. The image family always returns its latest image that is\nnot deprecated. The name of the image family must comply with\nRFC1035."]
    pub fn set_family(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().family = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nLabels to apply to this Image.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `licenses`.\nAny applicable license URI."]
    pub fn set_licenses(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().licenses = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `source_disk`.\nThe source disk to create this image based on.\nYou must provide either this property or the\nrawDisk.source property but not both to create an image."]
    pub fn set_source_disk(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_disk = Some(v.into());
        self
    }

    #[doc= "Set the field `source_image`.\nURL of the source image used to create this image. In order to create an image, you must provide the full or partial\nURL of one of the following:\n\n* The selfLink URL\n* This property\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn set_source_image(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_image = Some(v.into());
        self
    }

    #[doc= "Set the field `source_snapshot`.\nURL of the source snapshot used to create this image.\n\nIn order to create an image, you must provide the full or partial URL of one of the following:\n\n* The selfLink URL\n* This property\n* The sourceImage URL\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn set_source_snapshot(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_snapshot = Some(v.into());
        self
    }

    #[doc= "Set the field `storage_locations`.\nCloud Storage bucket storage location of the image\n(regional or multi-regional).\nReference link: https://cloud.google.com/compute/docs/reference/rest/v1/images"]
    pub fn set_storage_locations(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().storage_locations = Some(v.into());
        self
    }

    #[doc= "Set the field `guest_os_features`.\n"]
    pub fn set_guest_os_features(self, v: impl Into<BlockAssignable<ComputeImageGuestOsFeaturesEl>>) -> Self {
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

    #[doc= "Set the field `image_encryption_key`.\n"]
    pub fn set_image_encryption_key(self, v: impl Into<BlockAssignable<ComputeImageImageEncryptionKeyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_encryption_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_encryption_key = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `raw_disk`.\n"]
    pub fn set_raw_disk(self, v: impl Into<BlockAssignable<ComputeImageRawDiskEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().raw_disk = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.raw_disk = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeImageTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `archive_size_bytes` after provisioning.\nSize of the image tar.gz archive stored in Google Cloud Storage (in\nbytes)."]
    pub fn archive_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.archive_size_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the image when restored onto a persistent disk (in GB)."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `family` after provisioning.\nThe name of the image family to which this image belongs. You can\ncreate disks by specifying an image family instead of a specific\nimage name. The image family always returns its latest image that is\nnot deprecated. The name of the image family must comply with\nRFC1035."]
    pub fn family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource. Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this Image.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `licenses` after provisioning.\nAny applicable license URI."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.licenses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_disk` after provisioning.\nThe source disk to create this image based on.\nYou must provide either this property or the\nrawDisk.source property but not both to create an image."]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_image` after provisioning.\nURL of the source image used to create this image. In order to create an image, you must provide the full or partial\nURL of one of the following:\n\n* The selfLink URL\n* This property\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_snapshot` after provisioning.\nURL of the source snapshot used to create this image.\n\nIn order to create an image, you must provide the full or partial URL of one of the following:\n\n* The selfLink URL\n* This property\n* The sourceImage URL\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket storage location of the image\n(regional or multi-regional).\nReference link: https://cloud.google.com/compute/docs/reference/rest/v1/images"]
    pub fn storage_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.storage_locations", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image_encryption_key` after provisioning.\n"]
    pub fn image_encryption_key(&self) -> ListRef<ComputeImageImageEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_encryption_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `raw_disk` after provisioning.\n"]
    pub fn raw_disk(&self) -> ListRef<ComputeImageRawDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.raw_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeImageTimeoutsElRef {
        ComputeImageTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ComputeImage {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeImage { }

impl ToListMappable for ComputeImage {
    type O = ListRef<ComputeImageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeImage_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_image".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeImage {
    pub tf_id: String,
    #[doc= "Name of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: PrimField<String>,
}

impl BuildComputeImage {
    pub fn build(self, stack: &mut Stack) -> ComputeImage {
        let out = ComputeImage(Rc::new(ComputeImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeImageData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                disk_size_gb: core::default::Default::default(),
                family: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                licenses: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                source_disk: core::default::Default::default(),
                source_image: core::default::Default::default(),
                source_snapshot: core::default::Default::default(),
                storage_locations: core::default::Default::default(),
                guest_os_features: core::default::Default::default(),
                image_encryption_key: core::default::Default::default(),
                raw_disk: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeImageRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeImageRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `archive_size_bytes` after provisioning.\nSize of the image tar.gz archive stored in Google Cloud Storage (in\nbytes)."]
    pub fn archive_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.archive_size_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the image when restored onto a persistent disk (in GB)."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `family` after provisioning.\nThe name of the image family to which this image belongs. You can\ncreate disks by specifying an image family instead of a specific\nimage name. The image family always returns its latest image that is\nnot deprecated. The name of the image family must comply with\nRFC1035."]
    pub fn family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource. Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this Image.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `licenses` after provisioning.\nAny applicable license URI."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.licenses", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_disk` after provisioning.\nThe source disk to create this image based on.\nYou must provide either this property or the\nrawDisk.source property but not both to create an image."]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_image` after provisioning.\nURL of the source image used to create this image. In order to create an image, you must provide the full or partial\nURL of one of the following:\n\n* The selfLink URL\n* This property\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_snapshot` after provisioning.\nURL of the source snapshot used to create this image.\n\nIn order to create an image, you must provide the full or partial URL of one of the following:\n\n* The selfLink URL\n* This property\n* The sourceImage URL\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket storage location of the image\n(regional or multi-regional).\nReference link: https://cloud.google.com/compute/docs/reference/rest/v1/images"]
    pub fn storage_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.storage_locations", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image_encryption_key` after provisioning.\n"]
    pub fn image_encryption_key(&self) -> ListRef<ComputeImageImageEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.image_encryption_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `raw_disk` after provisioning.\n"]
    pub fn raw_disk(&self) -> ListRef<ComputeImageRawDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.raw_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeImageTimeoutsElRef {
        ComputeImageTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeImageGuestOsFeaturesEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl ComputeImageGuestOsFeaturesEl { }

impl ToListMappable for ComputeImageGuestOsFeaturesEl {
    type O = BlockAssignable<ComputeImageGuestOsFeaturesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImageGuestOsFeaturesEl {
    #[doc= "The type of supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options. Possible values: [\"MULTI_IP_SUBNET\", \"SECURE_BOOT\", \"SEV_CAPABLE\", \"UEFI_COMPATIBLE\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"GVNIC\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"TDX_CAPABLE\", \"SEV_LIVE_MIGRATABLE_V2\"]"]
    pub type_: PrimField<String>,
}

impl BuildComputeImageGuestOsFeaturesEl {
    pub fn build(self) -> ComputeImageGuestOsFeaturesEl {
        ComputeImageGuestOsFeaturesEl { type_: self.type_ }
    }
}

pub struct ComputeImageGuestOsFeaturesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeImageGuestOsFeaturesElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageGuestOsFeaturesElRef {
        ComputeImageGuestOsFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImageGuestOsFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options. Possible values: [\"MULTI_IP_SUBNET\", \"SECURE_BOOT\", \"SEV_CAPABLE\", \"UEFI_COMPATIBLE\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"GVNIC\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"TDX_CAPABLE\", \"SEV_LIVE_MIGRATABLE_V2\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImageImageEncryptionKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
}

impl ComputeImageImageEncryptionKeyEl {
    #[doc= "Set the field `kms_key_self_link`.\nThe self link of the encryption key that is stored in Google Cloud\nKMS."]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeImageImageEncryptionKeyEl {
    type O = BlockAssignable<ComputeImageImageEncryptionKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImageImageEncryptionKeyEl {}

impl BuildComputeImageImageEncryptionKeyEl {
    pub fn build(self) -> ComputeImageImageEncryptionKeyEl {
        ComputeImageImageEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
        }
    }
}

pub struct ComputeImageImageEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeImageImageEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageImageEncryptionKeyElRef {
        ComputeImageImageEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImageImageEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in Google Cloud\nKMS."]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImageRawDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    container_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha1: Option<PrimField<String>>,
    source: PrimField<String>,
}

impl ComputeImageRawDiskEl {
    #[doc= "Set the field `container_type`.\nThe format used to encode and transmit the block device, which\nshould be TAR. This is just a container and transmission format\nand not a runtime format. Provided by the client when the disk\nimage is created. Default value: \"TAR\" Possible values: [\"TAR\"]"]
    pub fn set_container_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_type = Some(v.into());
        self
    }

    #[doc= "Set the field `sha1`.\nAn optional SHA1 checksum of the disk image before unpackaging.\nThis is provided by the client when the disk image is created."]
    pub fn set_sha1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sha1 = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeImageRawDiskEl {
    type O = BlockAssignable<ComputeImageRawDiskEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImageRawDiskEl {
    #[doc= "The full Google Cloud Storage URL where disk storage is stored\nYou must provide either this property or the sourceDisk property\nbut not both."]
    pub source: PrimField<String>,
}

impl BuildComputeImageRawDiskEl {
    pub fn build(self) -> ComputeImageRawDiskEl {
        ComputeImageRawDiskEl {
            container_type: core::default::Default::default(),
            sha1: core::default::Default::default(),
            source: self.source,
        }
    }
}

pub struct ComputeImageRawDiskElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeImageRawDiskElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageRawDiskElRef {
        ComputeImageRawDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImageRawDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `container_type` after provisioning.\nThe format used to encode and transmit the block device, which\nshould be TAR. This is just a container and transmission format\nand not a runtime format. Provided by the client when the disk\nimage is created. Default value: \"TAR\" Possible values: [\"TAR\"]"]
    pub fn container_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.container_type", self.base))
    }

    #[doc= "Get a reference to the value of field `sha1` after provisioning.\nAn optional SHA1 checksum of the disk image before unpackaging.\nThis is provided by the client when the disk image is created."]
    pub fn sha1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sha1", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nThe full Google Cloud Storage URL where disk storage is stored\nYou must provide either this property or the sourceDisk property\nbut not both."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImageTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeImageTimeoutsEl {
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

impl ToListMappable for ComputeImageTimeoutsEl {
    type O = BlockAssignable<ComputeImageTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImageTimeoutsEl {}

impl BuildComputeImageTimeoutsEl {
    pub fn build(self) -> ComputeImageTimeoutsEl {
        ComputeImageTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeImageTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeImageTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeImageTimeoutsElRef {
        ComputeImageTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImageTimeoutsElRef {
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
struct ComputeImageDynamic {
    guest_os_features: Option<DynamicBlock<ComputeImageGuestOsFeaturesEl>>,
    image_encryption_key: Option<DynamicBlock<ComputeImageImageEncryptionKeyEl>>,
    raw_disk: Option<DynamicBlock<ComputeImageRawDiskEl>>,
}
