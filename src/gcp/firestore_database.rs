use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FirestoreDatabaseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_engine_integration_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrency_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_protection_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    point_in_time_recovery_enablement: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirestoreDatabaseTimeoutsEl>,
}

struct FirestoreDatabase_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirestoreDatabaseData>,
}

#[derive(Clone)]
pub struct FirestoreDatabase(Rc<FirestoreDatabase_>);

impl FirestoreDatabase {
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

    #[doc= "Set the field `app_engine_integration_mode`.\nThe App Engine integration mode to use for this database. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn set_app_engine_integration_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().app_engine_integration_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `concurrency_mode`.\nThe concurrency control mode to use for this database. Possible values: [\"OPTIMISTIC\", \"PESSIMISTIC\", \"OPTIMISTIC_WITH_ENTITY_GROUPS\"]"]
    pub fn set_concurrency_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().concurrency_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `delete_protection_state`.\nState of delete protection for the database.\nWhen delete protection is enabled, this database cannot be deleted.\nThe default value is 'DELETE_PROTECTION_STATE_UNSPECIFIED', which is currently equivalent to 'DELETE_PROTECTION_DISABLED'.\n**Note:** Additionally, to delete this database using 'terraform destroy', 'deletion_policy' must be set to 'DELETE'. Possible values: [\"DELETE_PROTECTION_STATE_UNSPECIFIED\", \"DELETE_PROTECTION_ENABLED\", \"DELETE_PROTECTION_DISABLED\"]"]
    pub fn set_delete_protection_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delete_protection_state = Some(v.into());
        self
    }

    #[doc= "Set the field `deletion_policy`.\nDeletion behavior for this database.\nIf the deletion policy is 'ABANDON', the database will be removed from Terraform state but not deleted from Google Cloud upon destruction.\nIf the deletion policy is 'DELETE', the database will both be removed from Terraform state and deleted from Google Cloud upon destruction.\nThe default value is 'ABANDON'.\nSee also 'delete_protection'."]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `point_in_time_recovery_enablement`.\nWhether to enable the PITR feature on this database.\nIf 'POINT_IN_TIME_RECOVERY_ENABLED' is selected, reads are supported on selected versions of the data from within the past 7 days.\nversionRetentionPeriod and earliestVersionTime can be used to determine the supported versions. These include reads against any timestamp within the past hour\nand reads against 1-minute snapshots beyond 1 hour and within 7 days.\nIf 'POINT_IN_TIME_RECOVERY_DISABLED' is selected, reads are supported on any version of the data from within the past 1 hour. Default value: \"POINT_IN_TIME_RECOVERY_DISABLED\" Possible values: [\"POINT_IN_TIME_RECOVERY_ENABLED\", \"POINT_IN_TIME_RECOVERY_DISABLED\"]"]
    pub fn set_point_in_time_recovery_enablement(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().point_in_time_recovery_enablement = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirestoreDatabaseTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `app_engine_integration_mode` after provisioning.\nThe App Engine integration mode to use for this database. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn app_engine_integration_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_engine_integration_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `concurrency_mode` after provisioning.\nThe concurrency control mode to use for this database. Possible values: [\"OPTIMISTIC\", \"PESSIMISTIC\", \"OPTIMISTIC_WITH_ENTITY_GROUPS\"]"]
    pub fn concurrency_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.concurrency_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp at which this database was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_protection_state` after provisioning.\nState of delete protection for the database.\nWhen delete protection is enabled, this database cannot be deleted.\nThe default value is 'DELETE_PROTECTION_STATE_UNSPECIFIED', which is currently equivalent to 'DELETE_PROTECTION_DISABLED'.\n**Note:** Additionally, to delete this database using 'terraform destroy', 'deletion_policy' must be set to 'DELETE'. Possible values: [\"DELETE_PROTECTION_STATE_UNSPECIFIED\", \"DELETE_PROTECTION_ENABLED\", \"DELETE_PROTECTION_DISABLED\"]"]
    pub fn delete_protection_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_protection_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_policy` after provisioning.\nDeletion behavior for this database.\nIf the deletion policy is 'ABANDON', the database will be removed from Terraform state but not deleted from Google Cloud upon destruction.\nIf the deletion policy is 'DELETE', the database will both be removed from Terraform state and deleted from Google Cloud upon destruction.\nThe default value is 'ABANDON'.\nSee also 'delete_protection'."]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `earliest_version_time` after provisioning.\nOutput only. The earliest timestamp at which older versions of the data can be read from the database. See versionRetentionPeriod above; this field is populated with now - versionRetentionPeriod.\nThis value is continuously updated, and becomes stale the moment it is queried. If you are using this value to recover data, make sure to account for the time from the moment when the value is queried to the moment when you initiate the recovery.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn earliest_version_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.earliest_version_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nOutput only. This checksum is computed by the server based on the value of other fields,\nand may be sent on update and delete requests to ensure the client has an\nup-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_prefix` after provisioning.\nOutput only. The keyPrefix for this database.\nThis keyPrefix is used, in combination with the project id (\"~\") to construct the application id\nthat is returned from the Cloud Datastore APIs in Google App Engine first generation runtimes.\nThis value may be empty in which case the appid to use for URL-encoded keys is the project_id (eg: foo instead of v~foo)."]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location_id` after provisioning.\nThe location of the database. Available locations are listed at\nhttps://cloud.google.com/firestore/docs/locations."]
    pub fn location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe ID to use for the database, which will become the final\ncomponent of the database's resource name. This value should be 4-63\ncharacters. Valid characters are /[a-z][0-9]-/ with first character\na letter and the last a letter or a number. Must not be\nUUID-like /[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}/.\n\"(default)\" database id is also valid."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `point_in_time_recovery_enablement` after provisioning.\nWhether to enable the PITR feature on this database.\nIf 'POINT_IN_TIME_RECOVERY_ENABLED' is selected, reads are supported on selected versions of the data from within the past 7 days.\nversionRetentionPeriod and earliestVersionTime can be used to determine the supported versions. These include reads against any timestamp within the past hour\nand reads against 1-minute snapshots beyond 1 hour and within 7 days.\nIf 'POINT_IN_TIME_RECOVERY_DISABLED' is selected, reads are supported on any version of the data from within the past 1 hour. Default value: \"POINT_IN_TIME_RECOVERY_DISABLED\" Possible values: [\"POINT_IN_TIME_RECOVERY_ENABLED\", \"POINT_IN_TIME_RECOVERY_DISABLED\"]"]
    pub fn point_in_time_recovery_enablement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.point_in_time_recovery_enablement", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the database.\nSee https://cloud.google.com/datastore/docs/firestore-or-datastore\nfor information about how to choose. Possible values: [\"FIRESTORE_NATIVE\", \"DATASTORE_MODE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nOutput only. The system-generated UUID4 for this Database."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The timestamp at which this database was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_retention_period` after provisioning.\nOutput only. The period during which past versions of data are retained in the database.\nAny read or query can specify a readTime within this window, and will read the state of the database at that time.\nIf the PITR feature is enabled, the retention period is 7 days. Otherwise, the retention period is 1 hour.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn version_retention_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_retention_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreDatabaseTimeoutsElRef {
        FirestoreDatabaseTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for FirestoreDatabase {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FirestoreDatabase { }

impl ToListMappable for FirestoreDatabase {
    type O = ListRef<FirestoreDatabaseRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FirestoreDatabase_ {
    fn extract_resource_type(&self) -> String {
        "google_firestore_database".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFirestoreDatabase {
    pub tf_id: String,
    #[doc= "The location of the database. Available locations are listed at\nhttps://cloud.google.com/firestore/docs/locations."]
    pub location_id: PrimField<String>,
    #[doc= "The ID to use for the database, which will become the final\ncomponent of the database's resource name. This value should be 4-63\ncharacters. Valid characters are /[a-z][0-9]-/ with first character\na letter and the last a letter or a number. Must not be\nUUID-like /[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}/.\n\"(default)\" database id is also valid."]
    pub name: PrimField<String>,
    #[doc= "The type of the database.\nSee https://cloud.google.com/datastore/docs/firestore-or-datastore\nfor information about how to choose. Possible values: [\"FIRESTORE_NATIVE\", \"DATASTORE_MODE\"]"]
    pub type_: PrimField<String>,
}

impl BuildFirestoreDatabase {
    pub fn build(self, stack: &mut Stack) -> FirestoreDatabase {
        let out = FirestoreDatabase(Rc::new(FirestoreDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirestoreDatabaseData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_engine_integration_mode: core::default::Default::default(),
                concurrency_mode: core::default::Default::default(),
                delete_protection_state: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location_id: self.location_id,
                name: self.name,
                point_in_time_recovery_enablement: core::default::Default::default(),
                project: core::default::Default::default(),
                type_: self.type_,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FirestoreDatabaseRef {
    shared: StackShared,
    base: String,
}

impl Ref for FirestoreDatabaseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FirestoreDatabaseRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `app_engine_integration_mode` after provisioning.\nThe App Engine integration mode to use for this database. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn app_engine_integration_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app_engine_integration_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `concurrency_mode` after provisioning.\nThe concurrency control mode to use for this database. Possible values: [\"OPTIMISTIC\", \"PESSIMISTIC\", \"OPTIMISTIC_WITH_ENTITY_GROUPS\"]"]
    pub fn concurrency_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.concurrency_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp at which this database was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_protection_state` after provisioning.\nState of delete protection for the database.\nWhen delete protection is enabled, this database cannot be deleted.\nThe default value is 'DELETE_PROTECTION_STATE_UNSPECIFIED', which is currently equivalent to 'DELETE_PROTECTION_DISABLED'.\n**Note:** Additionally, to delete this database using 'terraform destroy', 'deletion_policy' must be set to 'DELETE'. Possible values: [\"DELETE_PROTECTION_STATE_UNSPECIFIED\", \"DELETE_PROTECTION_ENABLED\", \"DELETE_PROTECTION_DISABLED\"]"]
    pub fn delete_protection_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_protection_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_policy` after provisioning.\nDeletion behavior for this database.\nIf the deletion policy is 'ABANDON', the database will be removed from Terraform state but not deleted from Google Cloud upon destruction.\nIf the deletion policy is 'DELETE', the database will both be removed from Terraform state and deleted from Google Cloud upon destruction.\nThe default value is 'ABANDON'.\nSee also 'delete_protection'."]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `earliest_version_time` after provisioning.\nOutput only. The earliest timestamp at which older versions of the data can be read from the database. See versionRetentionPeriod above; this field is populated with now - versionRetentionPeriod.\nThis value is continuously updated, and becomes stale the moment it is queried. If you are using this value to recover data, make sure to account for the time from the moment when the value is queried to the moment when you initiate the recovery.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn earliest_version_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.earliest_version_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nOutput only. This checksum is computed by the server based on the value of other fields,\nand may be sent on update and delete requests to ensure the client has an\nup-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_prefix` after provisioning.\nOutput only. The keyPrefix for this database.\nThis keyPrefix is used, in combination with the project id (\"~\") to construct the application id\nthat is returned from the Cloud Datastore APIs in Google App Engine first generation runtimes.\nThis value may be empty in which case the appid to use for URL-encoded keys is the project_id (eg: foo instead of v~foo)."]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location_id` after provisioning.\nThe location of the database. Available locations are listed at\nhttps://cloud.google.com/firestore/docs/locations."]
    pub fn location_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe ID to use for the database, which will become the final\ncomponent of the database's resource name. This value should be 4-63\ncharacters. Valid characters are /[a-z][0-9]-/ with first character\na letter and the last a letter or a number. Must not be\nUUID-like /[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}/.\n\"(default)\" database id is also valid."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `point_in_time_recovery_enablement` after provisioning.\nWhether to enable the PITR feature on this database.\nIf 'POINT_IN_TIME_RECOVERY_ENABLED' is selected, reads are supported on selected versions of the data from within the past 7 days.\nversionRetentionPeriod and earliestVersionTime can be used to determine the supported versions. These include reads against any timestamp within the past hour\nand reads against 1-minute snapshots beyond 1 hour and within 7 days.\nIf 'POINT_IN_TIME_RECOVERY_DISABLED' is selected, reads are supported on any version of the data from within the past 1 hour. Default value: \"POINT_IN_TIME_RECOVERY_DISABLED\" Possible values: [\"POINT_IN_TIME_RECOVERY_ENABLED\", \"POINT_IN_TIME_RECOVERY_DISABLED\"]"]
    pub fn point_in_time_recovery_enablement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.point_in_time_recovery_enablement", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the database.\nSee https://cloud.google.com/datastore/docs/firestore-or-datastore\nfor information about how to choose. Possible values: [\"FIRESTORE_NATIVE\", \"DATASTORE_MODE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nOutput only. The system-generated UUID4 for this Database."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The timestamp at which this database was most recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version_retention_period` after provisioning.\nOutput only. The period during which past versions of data are retained in the database.\nAny read or query can specify a readTime within this window, and will read the state of the database at that time.\nIf the PITR feature is enabled, the retention period is 7 days. Otherwise, the retention period is 1 hour.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn version_retention_period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_retention_period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreDatabaseTimeoutsElRef {
        FirestoreDatabaseTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FirestoreDatabaseTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FirestoreDatabaseTimeoutsEl {
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

impl ToListMappable for FirestoreDatabaseTimeoutsEl {
    type O = BlockAssignable<FirestoreDatabaseTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreDatabaseTimeoutsEl {}

impl BuildFirestoreDatabaseTimeoutsEl {
    pub fn build(self) -> FirestoreDatabaseTimeoutsEl {
        FirestoreDatabaseTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FirestoreDatabaseTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FirestoreDatabaseTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirestoreDatabaseTimeoutsElRef {
        FirestoreDatabaseTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreDatabaseTimeoutsElRef {
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
