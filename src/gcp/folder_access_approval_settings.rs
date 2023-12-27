use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FolderAccessApprovalSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_key_version: Option<PrimField<String>>,
    folder_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_emails: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enrolled_services: Option<Vec<FolderAccessApprovalSettingsEnrolledServicesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FolderAccessApprovalSettingsTimeoutsEl>,
    dynamic: FolderAccessApprovalSettingsDynamic,
}

struct FolderAccessApprovalSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FolderAccessApprovalSettingsData>,
}

#[derive(Clone)]
pub struct FolderAccessApprovalSettings(Rc<FolderAccessApprovalSettings_>);

impl FolderAccessApprovalSettings {
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

    #[doc= "Set the field `active_key_version`.\nThe asymmetric crypto key version to use for signing approval requests.\nEmpty active_key_version indicates that a Google-managed key should be used for signing.\nThis property will be ignored if set by an ancestor of the resource, and new non-empty values may not be set."]
    pub fn set_active_key_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().active_key_version = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `notification_emails`.\nA list of email addresses to which notifications relating to approval requests should be sent.\nNotifications relating to a resource will be sent to all emails in the settings of ancestor\nresources of that resource. A maximum of 50 email addresses are allowed."]
    pub fn set_notification_emails(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().notification_emails = Some(v.into());
        self
    }

    #[doc= "Set the field `enrolled_services`.\n"]
    pub fn set_enrolled_services(
        self,
        v: impl Into<BlockAssignable<FolderAccessApprovalSettingsEnrolledServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enrolled_services = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enrolled_services = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FolderAccessApprovalSettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `active_key_version` after provisioning.\nThe asymmetric crypto key version to use for signing approval requests.\nEmpty active_key_version indicates that a Google-managed key should be used for signing.\nThis property will be ignored if set by an ancestor of the resource, and new non-empty values may not be set."]
    pub fn active_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_key_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ancestor_has_active_key_version` after provisioning.\nIf the field is true, that indicates that an ancestor of this Folder has set active_key_version."]
    pub fn ancestor_has_active_key_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ancestor_has_active_key_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enrolled_ancestor` after provisioning.\nIf the field is true, that indicates that at least one service is enrolled for Access Approval in one or more ancestors of the Folder."]
    pub fn enrolled_ancestor(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enrolled_ancestor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder_id` after provisioning.\nID of the folder of the access approval settings."]
    pub fn folder_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `invalid_key_version` after provisioning.\nIf the field is true, that indicates that there is some configuration issue with the active_key_version\nconfigured on this Folder (e.g. it doesn't exist or the Access Approval service account doesn't have the\ncorrect permissions on it, etc.) This key version is not necessarily the effective key version at this level,\nas key versions are inherited top-down."]
    pub fn invalid_key_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invalid_key_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the settings. Format is \"folders/{folder_id}/accessApprovalSettings\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notification_emails` after provisioning.\nA list of email addresses to which notifications relating to approval requests should be sent.\nNotifications relating to a resource will be sent to all emails in the settings of ancestor\nresources of that resource. A maximum of 50 email addresses are allowed."]
    pub fn notification_emails(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.notification_emails", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FolderAccessApprovalSettingsTimeoutsElRef {
        FolderAccessApprovalSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for FolderAccessApprovalSettings {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FolderAccessApprovalSettings { }

impl ToListMappable for FolderAccessApprovalSettings {
    type O = ListRef<FolderAccessApprovalSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FolderAccessApprovalSettings_ {
    fn extract_resource_type(&self) -> String {
        "google_folder_access_approval_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFolderAccessApprovalSettings {
    pub tf_id: String,
    #[doc= "ID of the folder of the access approval settings."]
    pub folder_id: PrimField<String>,
}

impl BuildFolderAccessApprovalSettings {
    pub fn build(self, stack: &mut Stack) -> FolderAccessApprovalSettings {
        let out = FolderAccessApprovalSettings(Rc::new(FolderAccessApprovalSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FolderAccessApprovalSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                active_key_version: core::default::Default::default(),
                folder_id: self.folder_id,
                id: core::default::Default::default(),
                notification_emails: core::default::Default::default(),
                enrolled_services: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FolderAccessApprovalSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for FolderAccessApprovalSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FolderAccessApprovalSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `active_key_version` after provisioning.\nThe asymmetric crypto key version to use for signing approval requests.\nEmpty active_key_version indicates that a Google-managed key should be used for signing.\nThis property will be ignored if set by an ancestor of the resource, and new non-empty values may not be set."]
    pub fn active_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_key_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ancestor_has_active_key_version` after provisioning.\nIf the field is true, that indicates that an ancestor of this Folder has set active_key_version."]
    pub fn ancestor_has_active_key_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ancestor_has_active_key_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enrolled_ancestor` after provisioning.\nIf the field is true, that indicates that at least one service is enrolled for Access Approval in one or more ancestors of the Folder."]
    pub fn enrolled_ancestor(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enrolled_ancestor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder_id` after provisioning.\nID of the folder of the access approval settings."]
    pub fn folder_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `invalid_key_version` after provisioning.\nIf the field is true, that indicates that there is some configuration issue with the active_key_version\nconfigured on this Folder (e.g. it doesn't exist or the Access Approval service account doesn't have the\ncorrect permissions on it, etc.) This key version is not necessarily the effective key version at this level,\nas key versions are inherited top-down."]
    pub fn invalid_key_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invalid_key_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the settings. Format is \"folders/{folder_id}/accessApprovalSettings\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notification_emails` after provisioning.\nA list of email addresses to which notifications relating to approval requests should be sent.\nNotifications relating to a resource will be sent to all emails in the settings of ancestor\nresources of that resource. A maximum of 50 email addresses are allowed."]
    pub fn notification_emails(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.notification_emails", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FolderAccessApprovalSettingsTimeoutsElRef {
        FolderAccessApprovalSettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct FolderAccessApprovalSettingsEnrolledServicesEl {
    cloud_product: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enrollment_level: Option<PrimField<String>>,
}

impl FolderAccessApprovalSettingsEnrolledServicesEl {
    #[doc= "Set the field `enrollment_level`.\nThe enrollment level of the service. Default value: \"BLOCK_ALL\" Possible values: [\"BLOCK_ALL\"]"]
    pub fn set_enrollment_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enrollment_level = Some(v.into());
        self
    }
}

impl ToListMappable for FolderAccessApprovalSettingsEnrolledServicesEl {
    type O = BlockAssignable<FolderAccessApprovalSettingsEnrolledServicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFolderAccessApprovalSettingsEnrolledServicesEl {
    #[doc= "The product for which Access Approval will be enrolled. Allowed values are listed (case-sensitive):\n  * all\n  * App Engine\n  * BigQuery\n  * Cloud Bigtable\n  * Cloud Key Management Service\n  * Compute Engine\n  * Cloud Dataflow\n  * Cloud Identity and Access Management\n  * Cloud Pub/Sub\n  * Cloud Storage\n  * Persistent Disk\n\nNote: These values are supported as input, but considered a legacy format:\n  * all\n  * appengine.googleapis.com\n  * bigquery.googleapis.com\n  * bigtable.googleapis.com\n  * cloudkms.googleapis.com\n  * compute.googleapis.com\n  * dataflow.googleapis.com\n  * iam.googleapis.com\n  * pubsub.googleapis.com\n  * storage.googleapis.com"]
    pub cloud_product: PrimField<String>,
}

impl BuildFolderAccessApprovalSettingsEnrolledServicesEl {
    pub fn build(self) -> FolderAccessApprovalSettingsEnrolledServicesEl {
        FolderAccessApprovalSettingsEnrolledServicesEl {
            cloud_product: self.cloud_product,
            enrollment_level: core::default::Default::default(),
        }
    }
}

pub struct FolderAccessApprovalSettingsEnrolledServicesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FolderAccessApprovalSettingsEnrolledServicesElRef {
    fn new(shared: StackShared, base: String) -> FolderAccessApprovalSettingsEnrolledServicesElRef {
        FolderAccessApprovalSettingsEnrolledServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FolderAccessApprovalSettingsEnrolledServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cloud_product` after provisioning.\nThe product for which Access Approval will be enrolled. Allowed values are listed (case-sensitive):\n  * all\n  * App Engine\n  * BigQuery\n  * Cloud Bigtable\n  * Cloud Key Management Service\n  * Compute Engine\n  * Cloud Dataflow\n  * Cloud Identity and Access Management\n  * Cloud Pub/Sub\n  * Cloud Storage\n  * Persistent Disk\n\nNote: These values are supported as input, but considered a legacy format:\n  * all\n  * appengine.googleapis.com\n  * bigquery.googleapis.com\n  * bigtable.googleapis.com\n  * cloudkms.googleapis.com\n  * compute.googleapis.com\n  * dataflow.googleapis.com\n  * iam.googleapis.com\n  * pubsub.googleapis.com\n  * storage.googleapis.com"]
    pub fn cloud_product(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloud_product", self.base))
    }

    #[doc= "Get a reference to the value of field `enrollment_level` after provisioning.\nThe enrollment level of the service. Default value: \"BLOCK_ALL\" Possible values: [\"BLOCK_ALL\"]"]
    pub fn enrollment_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enrollment_level", self.base))
    }
}

#[derive(Serialize)]
pub struct FolderAccessApprovalSettingsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FolderAccessApprovalSettingsTimeoutsEl {
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

impl ToListMappable for FolderAccessApprovalSettingsTimeoutsEl {
    type O = BlockAssignable<FolderAccessApprovalSettingsTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFolderAccessApprovalSettingsTimeoutsEl {}

impl BuildFolderAccessApprovalSettingsTimeoutsEl {
    pub fn build(self) -> FolderAccessApprovalSettingsTimeoutsEl {
        FolderAccessApprovalSettingsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FolderAccessApprovalSettingsTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FolderAccessApprovalSettingsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FolderAccessApprovalSettingsTimeoutsElRef {
        FolderAccessApprovalSettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FolderAccessApprovalSettingsTimeoutsElRef {
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
struct FolderAccessApprovalSettingsDynamic {
    enrolled_services: Option<DynamicBlock<FolderAccessApprovalSettingsEnrolledServicesEl>>,
}
