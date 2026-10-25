#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeImageData_ {
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
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    licenses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_disk: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_snapshot: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_os_features: std::option::Option<std::vec::Vec<ComputeImage_GuestOsFeatures>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_encryption_key: std::option::Option<std::vec::Vec<ComputeImage_ImageEncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<ComputeImage_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_disk: std::option::Option<std::vec::Vec<ComputeImage_RawDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_initial_state: std::option::Option<std::vec::Vec<ComputeImage_ShieldedInstanceInitialState>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_disk_encryption_key: std::option::Option<std::vec::Vec<ComputeImage_SourceDiskEncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_image_encryption_key: std::option::Option<std::vec::Vec<ComputeImage_SourceImageEncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_snapshot_encryption_key: std::option::Option<std::vec::Vec<ComputeImage_SourceSnapshotEncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeImage_Timeouts>,
    dynamic: ComputeImageDynamic,
}

struct ComputeImage_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeImageData_>,
}

#[derive(Clone)]
pub struct ComputeImage(std::rc::Rc<ComputeImage_>);

impl ComputeImage {
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
        "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_size_gb`.\nSize of the image when restored onto a persistent disk (in GB)."]
    pub fn set_disk_size_gb(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `family`.\nThe name of the image family to which this image belongs. You can\ncreate disks by specifying an image family instead of a specific\nimage name. The image family always returns its latest image that is\nnot deprecated. The name of the image family must comply with\nRFC1035."]
    pub fn set_family(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().family = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels to apply to this Image.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `licenses`.\nAny applicable license URI."]
    pub fn set_licenses(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().licenses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_disk`.\nThe source disk to create this image based on.\nYou must provide either this property or the\nrawDisk.source property but not both to create an image."]
    pub fn set_source_disk(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_disk = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_image`.\nURL of the source image used to create this image. In order to create an image, you must provide the full or partial\nURL of one of the following:\n\n* The selfLink URL\n* This property\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn set_source_image(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_snapshot`.\nURL of the source snapshot used to create this image.\n\nIn order to create an image, you must provide the full or partial URL of one of the following:\n\n* The selfLink URL\n* This property\n* The sourceImage URL\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn set_source_snapshot(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_snapshot = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_locations`.\nCloud Storage bucket storage location of the image\n(regional or multi-regional).\nReference link: https://cloud.google.com/compute/docs/reference/rest/v1/images"]
    pub fn set_storage_locations(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().storage_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guest_os_features`.\n"]
    pub fn set_guest_os_features(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_GuestOsFeatures>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().guest_os_features = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.guest_os_features = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image_encryption_key`.\n"]
    pub fn set_image_encryption_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_ImageEncryptionKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().image_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.image_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_Params>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `raw_disk`.\n"]
    pub fn set_raw_disk(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_RawDisk>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().raw_disk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.raw_disk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shielded_instance_initial_state`.\n"]
    pub fn set_shielded_instance_initial_state(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_ShieldedInstanceInitialState>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shielded_instance_initial_state = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shielded_instance_initial_state = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_disk_encryption_key`.\n"]
    pub fn set_source_disk_encryption_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_SourceDiskEncryptionKey>>,
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

    #[doc = "Set the field `source_image_encryption_key`.\n"]
    pub fn set_source_image_encryption_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_SourceImageEncryptionKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_image_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_image_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_snapshot_encryption_key`.\n"]
    pub fn set_source_snapshot_encryption_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_SourceSnapshotEncryptionKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_snapshot_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_snapshot_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeImage_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `archive_size_bytes` after provisioning.\nSize of the image tar.gz archive stored in Google Cloud Storage (in\nbytes)."]
    pub fn archive_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archive_size_bytes", self.extract_ref()))
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the image when restored onto a persistent disk (in GB)."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `family` after provisioning.\nThe name of the image family to which this image belongs. You can\ncreate disks by specifying an image family instead of a specific\nimage name. The image family always returns its latest image that is\nnot deprecated. The name of the image family must comply with\nRFC1035."]
    pub fn family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this Image.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `licenses` after provisioning.\nAny applicable license URI."]
    pub fn licenses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.licenses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
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
        "Get a reference to the value of field `source_disk` after provisioning.\nThe source disk to create this image based on.\nYou must provide either this property or the\nrawDisk.source property but not both to create an image."]
    pub fn source_disk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_image` after provisioning.\nURL of the source image used to create this image. In order to create an image, you must provide the full or partial\nURL of one of the following:\n\n* The selfLink URL\n* This property\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_snapshot` after provisioning.\nURL of the source snapshot used to create this image.\n\nIn order to create an image, you must provide the full or partial URL of one of the following:\n\n* The selfLink URL\n* This property\n* The sourceImage URL\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_snapshot(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket storage location of the image\n(regional or multi-regional).\nReference link: https://cloud.google.com/compute/docs/reference/rest/v1/images"]
    pub fn storage_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_locations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_encryption_key` after provisioning.\n"]
    pub fn image_encryption_key(&self) -> terrars::ListRef<ComputeImage_ImageEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeImage_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `raw_disk` after provisioning.\n"]
    pub fn raw_disk(&self) -> terrars::ListRef<ComputeImage_RawDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.raw_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_initial_state` after provisioning.\n"]
    pub fn shielded_instance_initial_state(&self) -> terrars::ListRef<ComputeImage_ShieldedInstanceInitialStateRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_initial_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_disk_encryption_key` after provisioning.\n"]
    pub fn source_disk_encryption_key(&self) -> terrars::ListRef<ComputeImage_SourceDiskEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_disk_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(&self) -> terrars::ListRef<ComputeImage_SourceImageEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_image_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(&self) -> terrars::ListRef<ComputeImage_SourceSnapshotEncryptionKeyRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.source_snapshot_encryption_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeImage_TimeoutsRef {
        ComputeImage_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeImage {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeImage { }

impl terrars::ToListMappable for ComputeImage {
    type O = terrars::ListRef<ComputeImageRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeImage_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_image".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeImage {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeImage {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeImage {
        let out = ComputeImage(std::rc::Rc::new(ComputeImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeImageData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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
                params: core::default::Default::default(),
                raw_disk: core::default::Default::default(),
                shielded_instance_initial_state: core::default::Default::default(),
                source_disk_encryption_key: core::default::Default::default(),
                source_image_encryption_key: core::default::Default::default(),
                source_snapshot_encryption_key: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeImageRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_size_bytes` after provisioning.\nSize of the image tar.gz archive stored in Google Cloud Storage (in\nbytes)."]
    pub fn archive_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archive_size_bytes", self.extract_ref()))
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
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the image when restored onto a persistent disk (in GB)."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `family` after provisioning.\nThe name of the image family to which this image belongs. You can\ncreate disks by specifying an image family instead of a specific\nimage name. The image family always returns its latest image that is\nnot deprecated. The name of the image family must comply with\nRFC1035."]
    pub fn family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this Image.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `licenses` after provisioning.\nAny applicable license URI."]
    pub fn licenses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.licenses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
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
        "Get a reference to the value of field `source_disk` after provisioning.\nThe source disk to create this image based on.\nYou must provide either this property or the\nrawDisk.source property but not both to create an image."]
    pub fn source_disk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_image` after provisioning.\nURL of the source image used to create this image. In order to create an image, you must provide the full or partial\nURL of one of the following:\n\n* The selfLink URL\n* This property\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_snapshot` after provisioning.\nURL of the source snapshot used to create this image.\n\nIn order to create an image, you must provide the full or partial URL of one of the following:\n\n* The selfLink URL\n* This property\n* The sourceImage URL\n* The rawDisk.source URL\n* The sourceDisk URL"]
    pub fn source_snapshot(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_locations` after provisioning.\nCloud Storage bucket storage location of the image\n(regional or multi-regional).\nReference link: https://cloud.google.com/compute/docs/reference/rest/v1/images"]
    pub fn storage_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_locations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_encryption_key` after provisioning.\n"]
    pub fn image_encryption_key(&self) -> terrars::ListRef<ComputeImage_ImageEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeImage_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `raw_disk` after provisioning.\n"]
    pub fn raw_disk(&self) -> terrars::ListRef<ComputeImage_RawDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.raw_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_initial_state` after provisioning.\n"]
    pub fn shielded_instance_initial_state(&self) -> terrars::ListRef<ComputeImage_ShieldedInstanceInitialStateRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_initial_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `source_disk_encryption_key` after provisioning.\n"]
    pub fn source_disk_encryption_key(&self) -> terrars::ListRef<ComputeImage_SourceDiskEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_disk_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(&self) -> terrars::ListRef<ComputeImage_SourceImageEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_image_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(&self) -> terrars::ListRef<ComputeImage_SourceSnapshotEncryptionKeyRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.source_snapshot_encryption_key", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeImage_TimeoutsRef {
        ComputeImage_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeImageDynamic {
    guest_os_features: std::option::Option<terrars::DynamicBlock<ComputeImage_GuestOsFeatures>>,
    image_encryption_key: std::option::Option<terrars::DynamicBlock<ComputeImage_ImageEncryptionKey>>,
    params: std::option::Option<terrars::DynamicBlock<ComputeImage_Params>>,
    raw_disk: std::option::Option<terrars::DynamicBlock<ComputeImage_RawDisk>>,
    shielded_instance_initial_state: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeImage_ShieldedInstanceInitialState>>,
    source_disk_encryption_key: std::option::Option<terrars::DynamicBlock<ComputeImage_SourceDiskEncryptionKey>>,
    source_image_encryption_key: std::option::Option<terrars::DynamicBlock<ComputeImage_SourceImageEncryptionKey>>,
    source_snapshot_encryption_key: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeImage_SourceSnapshotEncryptionKey>>,
}

#[derive(Serialize)]
pub struct ComputeImage_GuestOsFeatures {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeImage_GuestOsFeatures { }

impl terrars::ToListMappable for ComputeImage_GuestOsFeatures {
    type O = terrars::BlockAssignable<ComputeImage_GuestOsFeatures>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_GuestOsFeatures {
    #[doc =
        "The type of supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options. Possible values: [\"MULTI_IP_SUBNET\", \"SECURE_BOOT\", \"SEV_CAPABLE\", \"UEFI_COMPATIBLE\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"GVNIC\", \"IDPF\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"TDX_CAPABLE\", \"SEV_LIVE_MIGRATABLE_V2\", \"SNP_SVSM_CAPABLE\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeImage_GuestOsFeatures {
    pub fn build(self) -> ComputeImage_GuestOsFeatures {
        ComputeImage_GuestOsFeatures { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_GuestOsFeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_GuestOsFeaturesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeImage_GuestOsFeaturesRef {
        ComputeImage_GuestOsFeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_GuestOsFeaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of supported feature. Read [Enabling guest operating system features](https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features) to see a list of available options. Possible values: [\"MULTI_IP_SUBNET\", \"SECURE_BOOT\", \"SEV_CAPABLE\", \"UEFI_COMPATIBLE\", \"VIRTIO_SCSI_MULTIQUEUE\", \"WINDOWS\", \"GVNIC\", \"IDPF\", \"SEV_LIVE_MIGRATABLE\", \"SEV_SNP_CAPABLE\", \"SUSPEND_RESUME_COMPATIBLE\", \"TDX_CAPABLE\", \"SEV_LIVE_MIGRATABLE_V2\", \"SNP_SVSM_CAPABLE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_ImageEncryptionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa_encrypted_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeImage_ImageEncryptionKey {
    #[doc =
        "Set the field `kms_key_self_link`.\nThe self link of the encryption key that is stored in Google Cloud\nKMS."]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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

    #[doc =
        "Set the field `rsa_encrypted_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn set_rsa_encrypted_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rsa_encrypted_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeImage_ImageEncryptionKey {
    type O = terrars::BlockAssignable<ComputeImage_ImageEncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_ImageEncryptionKey {}

impl BuildComputeImage_ImageEncryptionKey {
    pub fn build(self) -> ComputeImage_ImageEncryptionKey {
        ComputeImage_ImageEncryptionKey {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_ImageEncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_ImageEncryptionKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeImage_ImageEncryptionKeyRef {
        ComputeImage_ImageEncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_ImageEncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in Google Cloud\nKMS."]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn rsa_encrypted_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rsa_encrypted_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeImage_Params {
    #[doc =
        "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the image. Tag keys and values have the\nsame definition as resource manager tags. Keys and values can be either in numeric format,\nsuch as tagKeys/{tag_key_id} and tagValues/{tag_value_id} or in namespaced format such as\n{org_id|projectId}/{tag_key_short_name} and {tag_value_short_name}. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeImage_Params {
    type O = terrars::BlockAssignable<ComputeImage_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_Params {}

impl BuildComputeImage_Params {
    pub fn build(self) -> ComputeImage_Params {
        ComputeImage_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeImage_ParamsRef {
        ComputeImage_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the image. Tag keys and values have the\nsame definition as resource manager tags. Keys and values can be either in numeric format,\nsuch as tagKeys/{tag_key_id} and tagValues/{tag_value_id} or in namespaced format such as\n{org_id|projectId}/{tag_key_short_name} and {tag_value_short_name}. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_RawDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha1: std::option::Option<terrars::PrimField<std::string::String>>,
    source: terrars::PrimField<std::string::String>,
}

impl ComputeImage_RawDisk {
    #[doc =
        "Set the field `container_type`.\nThe format used to encode and transmit the block device, which\nshould be TAR. This is just a container and transmission format\nand not a runtime format. Provided by the client when the disk\nimage is created. Default value: \"TAR\" Possible values: [\"TAR\"]"]
    pub fn set_container_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sha1`.\nAn optional SHA1 checksum of the disk image before unpackaging.\nThis is provided by the client when the disk image is created."]
    pub fn set_sha1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha1 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeImage_RawDisk {
    type O = terrars::BlockAssignable<ComputeImage_RawDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_RawDisk {
    #[doc =
        "The full Google Cloud Storage URL where disk storage is stored\nYou must provide either this property or the sourceDisk property\nbut not both."]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildComputeImage_RawDisk {
    pub fn build(self) -> ComputeImage_RawDisk {
        ComputeImage_RawDisk {
            container_type: core::default::Default::default(),
            sha1: core::default::Default::default(),
            source: self.source,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_RawDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_RawDiskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeImage_RawDiskRef {
        ComputeImage_RawDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_RawDiskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_type` after provisioning.\nThe format used to encode and transmit the block device, which\nshould be TAR. This is just a container and transmission format\nand not a runtime format. Provided by the client when the disk\nimage is created. Default value: \"TAR\" Possible values: [\"TAR\"]"]
    pub fn container_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sha1` after provisioning.\nAn optional SHA1 checksum of the disk image before unpackaging.\nThis is provided by the client when the disk image is created."]
    pub fn sha1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha1", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nThe full Google Cloud Storage URL where disk storage is stored\nYou must provide either this property or the sourceDisk property\nbut not both."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_ShieldedInstanceInitialState_Dbs {
    content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeImage_ShieldedInstanceInitialState_Dbs {
    #[doc = "Set the field `file_type`.\nThe file type of source file."]
    pub fn set_file_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeImage_ShieldedInstanceInitialState_Dbs {
    type O = terrars::BlockAssignable<ComputeImage_ShieldedInstanceInitialState_Dbs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_ShieldedInstanceInitialState_Dbs {
    #[doc = "The raw content in the secure keys file.\n\nA base64-encoded string."]
    pub content: terrars::PrimField<std::string::String>,
}

impl BuildComputeImage_ShieldedInstanceInitialState_Dbs {
    pub fn build(self) -> ComputeImage_ShieldedInstanceInitialState_Dbs {
        ComputeImage_ShieldedInstanceInitialState_Dbs {
            content: self.content,
            file_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_ShieldedInstanceInitialState_DbsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_ShieldedInstanceInitialState_DbsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeImage_ShieldedInstanceInitialState_DbsRef {
        ComputeImage_ShieldedInstanceInitialState_DbsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_ShieldedInstanceInitialState_DbsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe raw content in the secure keys file.\n\nA base64-encoded string."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\nThe file type of source file."]
    pub fn file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_ShieldedInstanceInitialState_Dbxs {
    content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeImage_ShieldedInstanceInitialState_Dbxs {
    #[doc = "Set the field `file_type`.\nThe file type of source file."]
    pub fn set_file_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeImage_ShieldedInstanceInitialState_Dbxs {
    type O = terrars::BlockAssignable<ComputeImage_ShieldedInstanceInitialState_Dbxs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_ShieldedInstanceInitialState_Dbxs {
    #[doc = "The raw content in the secure keys file.\n\nA base64-encoded string."]
    pub content: terrars::PrimField<std::string::String>,
}

impl BuildComputeImage_ShieldedInstanceInitialState_Dbxs {
    pub fn build(self) -> ComputeImage_ShieldedInstanceInitialState_Dbxs {
        ComputeImage_ShieldedInstanceInitialState_Dbxs {
            content: self.content,
            file_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_ShieldedInstanceInitialState_DbxsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_ShieldedInstanceInitialState_DbxsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeImage_ShieldedInstanceInitialState_DbxsRef {
        ComputeImage_ShieldedInstanceInitialState_DbxsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_ShieldedInstanceInitialState_DbxsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe raw content in the secure keys file.\n\nA base64-encoded string."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\nThe file type of source file."]
    pub fn file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_ShieldedInstanceInitialState_Keks {
    content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeImage_ShieldedInstanceInitialState_Keks {
    #[doc = "Set the field `file_type`.\nThe file type of source file."]
    pub fn set_file_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeImage_ShieldedInstanceInitialState_Keks {
    type O = terrars::BlockAssignable<ComputeImage_ShieldedInstanceInitialState_Keks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_ShieldedInstanceInitialState_Keks {
    #[doc = "The raw content in the secure keys file.\n\nA base64-encoded string."]
    pub content: terrars::PrimField<std::string::String>,
}

impl BuildComputeImage_ShieldedInstanceInitialState_Keks {
    pub fn build(self) -> ComputeImage_ShieldedInstanceInitialState_Keks {
        ComputeImage_ShieldedInstanceInitialState_Keks {
            content: self.content,
            file_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_ShieldedInstanceInitialState_KeksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_ShieldedInstanceInitialState_KeksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeImage_ShieldedInstanceInitialState_KeksRef {
        ComputeImage_ShieldedInstanceInitialState_KeksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_ShieldedInstanceInitialState_KeksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe raw content in the secure keys file.\n\nA base64-encoded string."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\nThe file type of source file."]
    pub fn file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_ShieldedInstanceInitialState_Pk {
    content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeImage_ShieldedInstanceInitialState_Pk {
    #[doc = "Set the field `file_type`.\nThe file type of source file."]
    pub fn set_file_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeImage_ShieldedInstanceInitialState_Pk {
    type O = terrars::BlockAssignable<ComputeImage_ShieldedInstanceInitialState_Pk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_ShieldedInstanceInitialState_Pk {
    #[doc = "The raw content in the secure keys file.\n\nA base64-encoded string."]
    pub content: terrars::PrimField<std::string::String>,
}

impl BuildComputeImage_ShieldedInstanceInitialState_Pk {
    pub fn build(self) -> ComputeImage_ShieldedInstanceInitialState_Pk {
        ComputeImage_ShieldedInstanceInitialState_Pk {
            content: self.content,
            file_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_ShieldedInstanceInitialState_PkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_ShieldedInstanceInitialState_PkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeImage_ShieldedInstanceInitialState_PkRef {
        ComputeImage_ShieldedInstanceInitialState_PkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_ShieldedInstanceInitialState_PkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe raw content in the secure keys file.\n\nA base64-encoded string."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\nThe file type of source file."]
    pub fn file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeImage_ShieldedInstanceInitialStateDynamic {
    dbs: std::option::Option<terrars::DynamicBlock<ComputeImage_ShieldedInstanceInitialState_Dbs>>,
    dbxs: std::option::Option<terrars::DynamicBlock<ComputeImage_ShieldedInstanceInitialState_Dbxs>>,
    keks: std::option::Option<terrars::DynamicBlock<ComputeImage_ShieldedInstanceInitialState_Keks>>,
    pk: std::option::Option<terrars::DynamicBlock<ComputeImage_ShieldedInstanceInitialState_Pk>>,
}

#[derive(Serialize)]
pub struct ComputeImage_ShieldedInstanceInitialState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dbs: std::option::Option<std::vec::Vec<ComputeImage_ShieldedInstanceInitialState_Dbs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dbxs: std::option::Option<std::vec::Vec<ComputeImage_ShieldedInstanceInitialState_Dbxs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keks: std::option::Option<std::vec::Vec<ComputeImage_ShieldedInstanceInitialState_Keks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pk: std::option::Option<std::vec::Vec<ComputeImage_ShieldedInstanceInitialState_Pk>>,
    dynamic: ComputeImage_ShieldedInstanceInitialStateDynamic,
}

impl ComputeImage_ShieldedInstanceInitialState {
    #[doc = "Set the field `dbs`.\n"]
    pub fn set_dbs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_ShieldedInstanceInitialState_Dbs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dbs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dbs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dbxs`.\n"]
    pub fn set_dbxs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_ShieldedInstanceInitialState_Dbxs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dbxs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dbxs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `keks`.\n"]
    pub fn set_keks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_ShieldedInstanceInitialState_Keks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.keks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.keks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pk`.\n"]
    pub fn set_pk(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeImage_ShieldedInstanceInitialState_Pk>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pk = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeImage_ShieldedInstanceInitialState {
    type O = terrars::BlockAssignable<ComputeImage_ShieldedInstanceInitialState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_ShieldedInstanceInitialState {}

impl BuildComputeImage_ShieldedInstanceInitialState {
    pub fn build(self) -> ComputeImage_ShieldedInstanceInitialState {
        ComputeImage_ShieldedInstanceInitialState {
            dbs: core::default::Default::default(),
            dbxs: core::default::Default::default(),
            keks: core::default::Default::default(),
            pk: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_ShieldedInstanceInitialStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_ShieldedInstanceInitialStateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeImage_ShieldedInstanceInitialStateRef {
        ComputeImage_ShieldedInstanceInitialStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_ShieldedInstanceInitialStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dbs` after provisioning.\n"]
    pub fn dbs(&self) -> terrars::ListRef<ComputeImage_ShieldedInstanceInitialState_DbsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dbs", self.base))
    }

    #[doc = "Get a reference to the value of field `dbxs` after provisioning.\n"]
    pub fn dbxs(&self) -> terrars::ListRef<ComputeImage_ShieldedInstanceInitialState_DbxsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dbxs", self.base))
    }

    #[doc = "Get a reference to the value of field `keks` after provisioning.\n"]
    pub fn keks(&self) -> terrars::ListRef<ComputeImage_ShieldedInstanceInitialState_KeksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keks", self.base))
    }

    #[doc = "Get a reference to the value of field `pk` after provisioning.\n"]
    pub fn pk(&self) -> terrars::ListRef<ComputeImage_ShieldedInstanceInitialState_PkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pk", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_SourceDiskEncryptionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa_encrypted_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeImage_SourceDiskEncryptionKey {
    #[doc =
        "Set the field `kms_key_self_link`.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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

    #[doc =
        "Set the field `rsa_encrypted_key`.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit\ncustomer-supplied encryption key to either encrypt or decrypt\nthis resource. You can provide either the rawKey or the rsaEncryptedKey."]
    pub fn set_rsa_encrypted_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rsa_encrypted_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeImage_SourceDiskEncryptionKey {
    type O = terrars::BlockAssignable<ComputeImage_SourceDiskEncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_SourceDiskEncryptionKey {}

impl BuildComputeImage_SourceDiskEncryptionKey {
    pub fn build(self) -> ComputeImage_SourceDiskEncryptionKey {
        ComputeImage_SourceDiskEncryptionKey {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_SourceDiskEncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_SourceDiskEncryptionKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeImage_SourceDiskEncryptionKeyRef {
        ComputeImage_SourceDiskEncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_SourceDiskEncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit\ncustomer-supplied encryption key to either encrypt or decrypt\nthis resource. You can provide either the rawKey or the rsaEncryptedKey."]
    pub fn rsa_encrypted_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rsa_encrypted_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_SourceImageEncryptionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa_encrypted_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeImage_SourceImageEncryptionKey {
    #[doc =
        "Set the field `kms_key_self_link`.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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

    #[doc =
        "Set the field `rsa_encrypted_key`.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit\ncustomer-supplied encryption key to either encrypt or decrypt\nthis resource. You can provide either the rawKey or the rsaEncryptedKey."]
    pub fn set_rsa_encrypted_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rsa_encrypted_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeImage_SourceImageEncryptionKey {
    type O = terrars::BlockAssignable<ComputeImage_SourceImageEncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_SourceImageEncryptionKey {}

impl BuildComputeImage_SourceImageEncryptionKey {
    pub fn build(self) -> ComputeImage_SourceImageEncryptionKey {
        ComputeImage_SourceImageEncryptionKey {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_SourceImageEncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_SourceImageEncryptionKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeImage_SourceImageEncryptionKeyRef {
        ComputeImage_SourceImageEncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_SourceImageEncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit\ncustomer-supplied encryption key to either encrypt or decrypt\nthis resource. You can provide either the rawKey or the rsaEncryptedKey."]
    pub fn rsa_encrypted_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rsa_encrypted_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_SourceSnapshotEncryptionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa_encrypted_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeImage_SourceSnapshotEncryptionKey {
    #[doc =
        "Set the field `kms_key_self_link`.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
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

    #[doc =
        "Set the field `rsa_encrypted_key`.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit\ncustomer-supplied encryption key to either encrypt or decrypt\nthis resource. You can provide either the rawKey or the rsaEncryptedKey."]
    pub fn set_rsa_encrypted_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rsa_encrypted_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeImage_SourceSnapshotEncryptionKey {
    type O = terrars::BlockAssignable<ComputeImage_SourceSnapshotEncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_SourceSnapshotEncryptionKey {}

impl BuildComputeImage_SourceSnapshotEncryptionKey {
    pub fn build(self) -> ComputeImage_SourceSnapshotEncryptionKey {
        ComputeImage_SourceSnapshotEncryptionKey {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_SourceSnapshotEncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_SourceSnapshotEncryptionKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeImage_SourceSnapshotEncryptionKeyRef {
        ComputeImage_SourceSnapshotEncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_SourceSnapshotEncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key used to decrypt this resource. Also called KmsKeyName\nin the cloud console. Your project's Compute Engine System service account\n('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nSee https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys"]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the\ngiven KMS key. If absent, the Compute Engine default service\naccount is used."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in\nRFC 4648 base64 to either encrypt or decrypt this resource."]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit\ncustomer-supplied encryption key to either encrypt or decrypt\nthis resource. You can provide either the rawKey or the rsaEncryptedKey."]
    pub fn rsa_encrypted_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rsa_encrypted_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeImage_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeImage_Timeouts {
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

impl terrars::ToListMappable for ComputeImage_Timeouts {
    type O = terrars::BlockAssignable<ComputeImage_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeImage_Timeouts {}

impl BuildComputeImage_Timeouts {
    pub fn build(self) -> ComputeImage_Timeouts {
        ComputeImage_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeImage_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeImage_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeImage_TimeoutsRef {
        ComputeImage_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeImage_TimeoutsRef {
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
