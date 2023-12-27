use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FilestoreInstanceData {
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
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    tier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_shares: Option<Vec<FilestoreInstanceFileSharesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<Vec<FilestoreInstanceNetworksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FilestoreInstanceTimeoutsEl>,
    dynamic: FilestoreInstanceDynamic,
}

struct FilestoreInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FilestoreInstanceData>,
}

#[derive(Clone)]
pub struct FilestoreInstance(Rc<FilestoreInstance_>);

impl FilestoreInstance {
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

    #[doc= "Set the field `description`.\nA description of the instance."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key_name`.\nKMS key name used for data encryption."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\nThe name of the location of the instance. This can be a region for ENTERPRISE tier instances."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `zone`.\nThe name of the Filestore zone of the instance."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }

    #[doc= "Set the field `file_shares`.\n"]
    pub fn set_file_shares(self, v: impl Into<BlockAssignable<FilestoreInstanceFileSharesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_shares = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_shares = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `networks`.\n"]
    pub fn set_networks(self, v: impl Into<BlockAssignable<FilestoreInstanceNetworksEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FilestoreInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nServer-specified ETag for the instance resource to prevent\nsimultaneous updates from overwriting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nKMS key name used for data encryption."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the instance. This can be a region for ENTERPRISE tier instances."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tier` after provisioning.\nThe service tier of the instance.\nPossible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL and ENTERPRISE"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe name of the Filestore zone of the instance."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_shares` after provisioning.\n"]
    pub fn file_shares(&self) -> ListRef<FilestoreInstanceFileSharesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_shares", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> ListRef<FilestoreInstanceNetworksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FilestoreInstanceTimeoutsElRef {
        FilestoreInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for FilestoreInstance {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FilestoreInstance { }

impl ToListMappable for FilestoreInstance {
    type O = ListRef<FilestoreInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FilestoreInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_filestore_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFilestoreInstance {
    pub tf_id: String,
    #[doc= "The resource name of the instance."]
    pub name: PrimField<String>,
    #[doc= "The service tier of the instance.\nPossible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL and ENTERPRISE"]
    pub tier: PrimField<String>,
}

impl BuildFilestoreInstance {
    pub fn build(self, stack: &mut Stack) -> FilestoreInstance {
        let out = FilestoreInstance(Rc::new(FilestoreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FilestoreInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                tier: self.tier,
                zone: core::default::Default::default(),
                file_shares: core::default::Default::default(),
                networks: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FilestoreInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for FilestoreInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FilestoreInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nServer-specified ETag for the instance resource to prevent\nsimultaneous updates from overwriting each other."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nKMS key name used for data encryption."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the instance. This can be a region for ENTERPRISE tier instances."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the instance."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tier` after provisioning.\nThe service tier of the instance.\nPossible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL and ENTERPRISE"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe name of the Filestore zone of the instance."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_shares` after provisioning.\n"]
    pub fn file_shares(&self) -> ListRef<FilestoreInstanceFileSharesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_shares", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> ListRef<FilestoreInstanceNetworksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FilestoreInstanceTimeoutsElRef {
        FilestoreInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstanceFileSharesElNfsExportOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anon_gid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anon_uid: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    squash_mode: Option<PrimField<String>>,
}

impl FilestoreInstanceFileSharesElNfsExportOptionsEl {
    #[doc= "Set the field `access_mode`.\nEither READ_ONLY, for allowing only read requests on the exported directory,\nor READ_WRITE, for allowing both read and write requests. The default is READ_WRITE. Default value: \"READ_WRITE\" Possible values: [\"READ_ONLY\", \"READ_WRITE\"]"]
    pub fn set_access_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `anon_gid`.\nAn integer representing the anonymous group id with a default value of 65534.\nAnon_gid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn set_anon_gid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.anon_gid = Some(v.into());
        self
    }

    #[doc= "Set the field `anon_uid`.\nAn integer representing the anonymous user id with a default value of 65534.\nAnon_uid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn set_anon_uid(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.anon_uid = Some(v.into());
        self
    }

    #[doc= "Set the field `ip_ranges`.\nList of either IPv4 addresses, or ranges in CIDR notation which may mount the file share.\nOverlapping IP ranges are not allowed, both within and across NfsExportOptions. An error will be returned.\nThe limit is 64 IP ranges/addresses for each FileShareConfig among all NfsExportOptions."]
    pub fn set_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_ranges = Some(v.into());
        self
    }

    #[doc= "Set the field `squash_mode`.\nEither NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH,\nfor not allowing root access. The default is NO_ROOT_SQUASH. Default value: \"NO_ROOT_SQUASH\" Possible values: [\"NO_ROOT_SQUASH\", \"ROOT_SQUASH\"]"]
    pub fn set_squash_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.squash_mode = Some(v.into());
        self
    }
}

impl ToListMappable for FilestoreInstanceFileSharesElNfsExportOptionsEl {
    type O = BlockAssignable<FilestoreInstanceFileSharesElNfsExportOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstanceFileSharesElNfsExportOptionsEl {}

impl BuildFilestoreInstanceFileSharesElNfsExportOptionsEl {
    pub fn build(self) -> FilestoreInstanceFileSharesElNfsExportOptionsEl {
        FilestoreInstanceFileSharesElNfsExportOptionsEl {
            access_mode: core::default::Default::default(),
            anon_gid: core::default::Default::default(),
            anon_uid: core::default::Default::default(),
            ip_ranges: core::default::Default::default(),
            squash_mode: core::default::Default::default(),
        }
    }
}

pub struct FilestoreInstanceFileSharesElNfsExportOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FilestoreInstanceFileSharesElNfsExportOptionsElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceFileSharesElNfsExportOptionsElRef {
        FilestoreInstanceFileSharesElNfsExportOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstanceFileSharesElNfsExportOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_mode` after provisioning.\nEither READ_ONLY, for allowing only read requests on the exported directory,\nor READ_WRITE, for allowing both read and write requests. The default is READ_WRITE. Default value: \"READ_WRITE\" Possible values: [\"READ_ONLY\", \"READ_WRITE\"]"]
    pub fn access_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `anon_gid` after provisioning.\nAn integer representing the anonymous group id with a default value of 65534.\nAnon_gid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn anon_gid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.anon_gid", self.base))
    }

    #[doc= "Get a reference to the value of field `anon_uid` after provisioning.\nAn integer representing the anonymous user id with a default value of 65534.\nAnon_uid may only be set with squashMode of ROOT_SQUASH. An error will be returned\nif this field is specified for other squashMode settings."]
    pub fn anon_uid(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.anon_uid", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_ranges` after provisioning.\nList of either IPv4 addresses, or ranges in CIDR notation which may mount the file share.\nOverlapping IP ranges are not allowed, both within and across NfsExportOptions. An error will be returned.\nThe limit is 64 IP ranges/addresses for each FileShareConfig among all NfsExportOptions."]
    pub fn ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_ranges", self.base))
    }

    #[doc= "Get a reference to the value of field `squash_mode` after provisioning.\nEither NO_ROOT_SQUASH, for allowing root access on the exported directory, or ROOT_SQUASH,\nfor not allowing root access. The default is NO_ROOT_SQUASH. Default value: \"NO_ROOT_SQUASH\" Possible values: [\"NO_ROOT_SQUASH\", \"ROOT_SQUASH\"]"]
    pub fn squash_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct FilestoreInstanceFileSharesElDynamic {
    nfs_export_options: Option<DynamicBlock<FilestoreInstanceFileSharesElNfsExportOptionsEl>>,
}

#[derive(Serialize)]
pub struct FilestoreInstanceFileSharesEl {
    capacity_gb: PrimField<f64>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nfs_export_options: Option<Vec<FilestoreInstanceFileSharesElNfsExportOptionsEl>>,
    dynamic: FilestoreInstanceFileSharesElDynamic,
}

impl FilestoreInstanceFileSharesEl {
    #[doc= "Set the field `nfs_export_options`.\n"]
    pub fn set_nfs_export_options(
        mut self,
        v: impl Into<BlockAssignable<FilestoreInstanceFileSharesElNfsExportOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.nfs_export_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs_export_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for FilestoreInstanceFileSharesEl {
    type O = BlockAssignable<FilestoreInstanceFileSharesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstanceFileSharesEl {
    #[doc= "File share capacity in GiB. This must be at least 1024 GiB\nfor the standard tier, or 2560 GiB for the premium tier."]
    pub capacity_gb: PrimField<f64>,
    #[doc= "The name of the fileshare (16 characters or less)"]
    pub name: PrimField<String>,
}

impl BuildFilestoreInstanceFileSharesEl {
    pub fn build(self) -> FilestoreInstanceFileSharesEl {
        FilestoreInstanceFileSharesEl {
            capacity_gb: self.capacity_gb,
            name: self.name,
            nfs_export_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct FilestoreInstanceFileSharesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FilestoreInstanceFileSharesElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceFileSharesElRef {
        FilestoreInstanceFileSharesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstanceFileSharesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `capacity_gb` after provisioning.\nFile share capacity in GiB. This must be at least 1024 GiB\nfor the standard tier, or 2560 GiB for the premium tier."]
    pub fn capacity_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_gb", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the fileshare (16 characters or less)"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `source_backup` after provisioning.\nThe resource name of the backup, in the format\nprojects/{projectId}/locations/{locationId}/backups/{backupId},\nthat this file share has been restored from."]
    pub fn source_backup(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_backup", self.base))
    }

    #[doc= "Get a reference to the value of field `nfs_export_options` after provisioning.\n"]
    pub fn nfs_export_options(&self) -> ListRef<FilestoreInstanceFileSharesElNfsExportOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.nfs_export_options", self.base))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstanceNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_mode: Option<PrimField<String>>,
    modes: ListField<PrimField<String>>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reserved_ip_range: Option<PrimField<String>>,
}

impl FilestoreInstanceNetworksEl {
    #[doc= "Set the field `connect_mode`.\nThe network connect mode of the Filestore instance.\nIf not provided, the connect mode defaults to\nDIRECT_PEERING. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\"]"]
    pub fn set_connect_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connect_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `reserved_ip_range`.\nA /29 CIDR block that identifies the range of IP\naddresses reserved for this instance."]
    pub fn set_reserved_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reserved_ip_range = Some(v.into());
        self
    }
}

impl ToListMappable for FilestoreInstanceNetworksEl {
    type O = BlockAssignable<FilestoreInstanceNetworksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstanceNetworksEl {
    #[doc= "IP versions for which the instance has\nIP addresses assigned. Possible values: [\"ADDRESS_MODE_UNSPECIFIED\", \"MODE_IPV4\", \"MODE_IPV6\"]"]
    pub modes: ListField<PrimField<String>>,
    #[doc= "The name of the GCE VPC network to which the\ninstance is connected."]
    pub network: PrimField<String>,
}

impl BuildFilestoreInstanceNetworksEl {
    pub fn build(self) -> FilestoreInstanceNetworksEl {
        FilestoreInstanceNetworksEl {
            connect_mode: core::default::Default::default(),
            modes: self.modes,
            network: self.network,
            reserved_ip_range: core::default::Default::default(),
        }
    }
}

pub struct FilestoreInstanceNetworksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FilestoreInstanceNetworksElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceNetworksElRef {
        FilestoreInstanceNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstanceNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `connect_mode` after provisioning.\nThe network connect mode of the Filestore instance.\nIf not provided, the connect mode defaults to\nDIRECT_PEERING. Default value: \"DIRECT_PEERING\" Possible values: [\"DIRECT_PEERING\", \"PRIVATE_SERVICE_ACCESS\"]"]
    pub fn connect_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `ip_addresses` after provisioning.\nA list of IPv4 or IPv6 addresses."]
    pub fn ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }

    #[doc= "Get a reference to the value of field `modes` after provisioning.\nIP versions for which the instance has\nIP addresses assigned. Possible values: [\"ADDRESS_MODE_UNSPECIFIED\", \"MODE_IPV4\", \"MODE_IPV6\"]"]
    pub fn modes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.modes", self.base))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe name of the GCE VPC network to which the\ninstance is connected."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc= "Get a reference to the value of field `reserved_ip_range` after provisioning.\nA /29 CIDR block that identifies the range of IP\naddresses reserved for this instance."]
    pub fn reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reserved_ip_range", self.base))
    }
}

#[derive(Serialize)]
pub struct FilestoreInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FilestoreInstanceTimeoutsEl {
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

impl ToListMappable for FilestoreInstanceTimeoutsEl {
    type O = BlockAssignable<FilestoreInstanceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFilestoreInstanceTimeoutsEl {}

impl BuildFilestoreInstanceTimeoutsEl {
    pub fn build(self) -> FilestoreInstanceTimeoutsEl {
        FilestoreInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FilestoreInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FilestoreInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FilestoreInstanceTimeoutsElRef {
        FilestoreInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FilestoreInstanceTimeoutsElRef {
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
struct FilestoreInstanceDynamic {
    file_shares: Option<DynamicBlock<FilestoreInstanceFileSharesEl>>,
    networks: Option<DynamicBlock<FilestoreInstanceNetworksEl>>,
}
