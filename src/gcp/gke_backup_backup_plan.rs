use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeBackupBackupPlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deactivated: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_config: Option<Vec<GkeBackupBackupPlanBackupConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_schedule: Option<Vec<GkeBackupBackupPlanBackupScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention_policy: Option<Vec<GkeBackupBackupPlanRetentionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeBackupBackupPlanTimeoutsEl>,
    dynamic: GkeBackupBackupPlanDynamic,
}

struct GkeBackupBackupPlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeBackupBackupPlanData>,
}

#[derive(Clone)]
pub struct GkeBackupBackupPlan(Rc<GkeBackupBackupPlan_>);

impl GkeBackupBackupPlan {
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

    #[doc= "Set the field `deactivated`.\nThis flag indicates whether this BackupPlan has been deactivated.\nSetting this field to True locks the BackupPlan such that no further updates will be allowed\n(except deletes), including the deactivated field itself. It also prevents any new Backups\nfrom being created via this BackupPlan (including scheduled Backups)."]
    pub fn set_deactivated(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deactivated = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nUser specified descriptive string for this BackupPlan."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `backup_config`.\n"]
    pub fn set_backup_config(self, v: impl Into<BlockAssignable<GkeBackupBackupPlanBackupConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `backup_schedule`.\n"]
    pub fn set_backup_schedule(self, v: impl Into<BlockAssignable<GkeBackupBackupPlanBackupScheduleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_schedule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_schedule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(self, v: impl Into<BlockAssignable<GkeBackupBackupPlanRetentionPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeBackupBackupPlanTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `cluster` after provisioning.\nThe source cluster from which Backups will be created via this BackupPlan."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deactivated` after provisioning.\nThis flag indicates whether this BackupPlan has been deactivated.\nSetting this field to True locks the BackupPlan such that no further updates will be allowed\n(except deletes), including the deactivated field itself. It also prevents any new Backups\nfrom being created via this BackupPlan (including scheduled Backups)."]
    pub fn deactivated(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deactivated", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nUser specified descriptive string for this BackupPlan."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\netag is used for optimistic concurrency control as a way to help prevent simultaneous\nupdates of a backup plan from overwriting each other. It is strongly suggested that\nsystems make use of the 'etag' in the read-modify-write cycle to perform BackupPlan updates\nin order to avoid race conditions: An etag is returned in the response to backupPlans.get,\nand systems are expected to put that etag in the request to backupPlans.patch or\nbackupPlans.delete to ensure that their change will be applied to the same version of the resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe region of the Backup Plan."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe full name of the BackupPlan Resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected_pod_count` after provisioning.\nThe number of Kubernetes Pods backed up in the last successful Backup created via this BackupPlan."]
    pub fn protected_pod_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected_pod_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe State of the BackupPlan."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state_reason` after provisioning.\nDetailed description of why BackupPlan is in its current state."]
    pub fn state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nServer generated, unique identifier of UUID format."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `backup_config` after provisioning.\n"]
    pub fn backup_config(&self) -> ListRef<GkeBackupBackupPlanBackupConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backup_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `backup_schedule` after provisioning.\n"]
    pub fn backup_schedule(&self) -> ListRef<GkeBackupBackupPlanBackupScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backup_schedule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> ListRef<GkeBackupBackupPlanRetentionPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeBackupBackupPlanTimeoutsElRef {
        GkeBackupBackupPlanTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for GkeBackupBackupPlan {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GkeBackupBackupPlan { }

impl ToListMappable for GkeBackupBackupPlan {
    type O = ListRef<GkeBackupBackupPlanRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GkeBackupBackupPlan_ {
    fn extract_resource_type(&self) -> String {
        "google_gke_backup_backup_plan".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeBackupBackupPlan {
    pub tf_id: String,
    #[doc= "The source cluster from which Backups will be created via this BackupPlan."]
    pub cluster: PrimField<String>,
    #[doc= "The region of the Backup Plan."]
    pub location: PrimField<String>,
    #[doc= "The full name of the BackupPlan Resource."]
    pub name: PrimField<String>,
}

impl BuildGkeBackupBackupPlan {
    pub fn build(self, stack: &mut Stack) -> GkeBackupBackupPlan {
        let out = GkeBackupBackupPlan(Rc::new(GkeBackupBackupPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeBackupBackupPlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster: self.cluster,
                deactivated: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                backup_config: core::default::Default::default(),
                backup_schedule: core::default::Default::default(),
                retention_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeBackupBackupPlanRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeBackupBackupPlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeBackupBackupPlanRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster` after provisioning.\nThe source cluster from which Backups will be created via this BackupPlan."]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deactivated` after provisioning.\nThis flag indicates whether this BackupPlan has been deactivated.\nSetting this field to True locks the BackupPlan such that no further updates will be allowed\n(except deletes), including the deactivated field itself. It also prevents any new Backups\nfrom being created via this BackupPlan (including scheduled Backups)."]
    pub fn deactivated(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deactivated", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nUser specified descriptive string for this BackupPlan."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\netag is used for optimistic concurrency control as a way to help prevent simultaneous\nupdates of a backup plan from overwriting each other. It is strongly suggested that\nsystems make use of the 'etag' in the read-modify-write cycle to perform BackupPlan updates\nin order to avoid race conditions: An etag is returned in the response to backupPlans.get,\nand systems are expected to put that etag in the request to backupPlans.patch or\nbackupPlans.delete to ensure that their change will be applied to the same version of the resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe region of the Backup Plan."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe full name of the BackupPlan Resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `protected_pod_count` after provisioning.\nThe number of Kubernetes Pods backed up in the last successful Backup created via this BackupPlan."]
    pub fn protected_pod_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.protected_pod_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe State of the BackupPlan."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state_reason` after provisioning.\nDetailed description of why BackupPlan is in its current state."]
    pub fn state_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\nServer generated, unique identifier of UUID format."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `backup_config` after provisioning.\n"]
    pub fn backup_config(&self) -> ListRef<GkeBackupBackupPlanBackupConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backup_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `backup_schedule` after provisioning.\n"]
    pub fn backup_schedule(&self) -> ListRef<GkeBackupBackupPlanBackupScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backup_schedule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> ListRef<GkeBackupBackupPlanRetentionPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeBackupBackupPlanTimeoutsElRef {
        GkeBackupBackupPlanTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlanBackupConfigElEncryptionKeyEl {
    gcp_kms_encryption_key: PrimField<String>,
}

impl GkeBackupBackupPlanBackupConfigElEncryptionKeyEl { }

impl ToListMappable for GkeBackupBackupPlanBackupConfigElEncryptionKeyEl {
    type O = BlockAssignable<GkeBackupBackupPlanBackupConfigElEncryptionKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlanBackupConfigElEncryptionKeyEl {
    #[doc= "Google Cloud KMS encryption key. Format: projects/*/locations/*/keyRings/*/cryptoKeys/*"]
    pub gcp_kms_encryption_key: PrimField<String>,
}

impl BuildGkeBackupBackupPlanBackupConfigElEncryptionKeyEl {
    pub fn build(self) -> GkeBackupBackupPlanBackupConfigElEncryptionKeyEl {
        GkeBackupBackupPlanBackupConfigElEncryptionKeyEl { gcp_kms_encryption_key: self.gcp_kms_encryption_key }
    }
}

pub struct GkeBackupBackupPlanBackupConfigElEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeBackupBackupPlanBackupConfigElEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> GkeBackupBackupPlanBackupConfigElEncryptionKeyElRef {
        GkeBackupBackupPlanBackupConfigElEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlanBackupConfigElEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `gcp_kms_encryption_key` after provisioning.\nGoogle Cloud KMS encryption key. Format: projects/*/locations/*/keyRings/*/cryptoKeys/*"]
    pub fn gcp_kms_encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcp_kms_encryption_key", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl {
    name: PrimField<String>,
    namespace: PrimField<String>,
}

impl GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl { }

impl ToListMappable for GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl {
    type O = BlockAssignable<GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl {
    #[doc= "The name of a Kubernetes Resource."]
    pub name: PrimField<String>,
    #[doc= "The namespace of a Kubernetes Resource."]
    pub namespace: PrimField<String>,
}

impl BuildGkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl {
    pub fn build(self) -> GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl {
        GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl {
            name: self.name,
            namespace: self.namespace,
        }
    }
}

pub struct GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesElRef {
        GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of a Kubernetes Resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `namespace` after provisioning.\nThe namespace of a Kubernetes Resource."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupBackupPlanBackupConfigElSelectedApplicationsElDynamic {
    namespaced_names: Option<DynamicBlock<GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl>>,
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlanBackupConfigElSelectedApplicationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaced_names: Option<Vec<GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl>>,
    dynamic: GkeBackupBackupPlanBackupConfigElSelectedApplicationsElDynamic,
}

impl GkeBackupBackupPlanBackupConfigElSelectedApplicationsEl {
    #[doc= "Set the field `namespaced_names`.\n"]
    pub fn set_namespaced_names(
        mut self,
        v: impl Into<BlockAssignable<GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.namespaced_names = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.namespaced_names = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GkeBackupBackupPlanBackupConfigElSelectedApplicationsEl {
    type O = BlockAssignable<GkeBackupBackupPlanBackupConfigElSelectedApplicationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlanBackupConfigElSelectedApplicationsEl {}

impl BuildGkeBackupBackupPlanBackupConfigElSelectedApplicationsEl {
    pub fn build(self) -> GkeBackupBackupPlanBackupConfigElSelectedApplicationsEl {
        GkeBackupBackupPlanBackupConfigElSelectedApplicationsEl {
            namespaced_names: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GkeBackupBackupPlanBackupConfigElSelectedApplicationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeBackupBackupPlanBackupConfigElSelectedApplicationsElRef {
    fn new(shared: StackShared, base: String) -> GkeBackupBackupPlanBackupConfigElSelectedApplicationsElRef {
        GkeBackupBackupPlanBackupConfigElSelectedApplicationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlanBackupConfigElSelectedApplicationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `namespaced_names` after provisioning.\n"]
    pub fn namespaced_names(
        &self,
    ) -> ListRef<GkeBackupBackupPlanBackupConfigElSelectedApplicationsElNamespacedNamesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.namespaced_names", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlanBackupConfigElSelectedNamespacesEl {
    namespaces: ListField<PrimField<String>>,
}

impl GkeBackupBackupPlanBackupConfigElSelectedNamespacesEl { }

impl ToListMappable for GkeBackupBackupPlanBackupConfigElSelectedNamespacesEl {
    type O = BlockAssignable<GkeBackupBackupPlanBackupConfigElSelectedNamespacesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlanBackupConfigElSelectedNamespacesEl {
    #[doc= "A list of Kubernetes Namespaces."]
    pub namespaces: ListField<PrimField<String>>,
}

impl BuildGkeBackupBackupPlanBackupConfigElSelectedNamespacesEl {
    pub fn build(self) -> GkeBackupBackupPlanBackupConfigElSelectedNamespacesEl {
        GkeBackupBackupPlanBackupConfigElSelectedNamespacesEl { namespaces: self.namespaces }
    }
}

pub struct GkeBackupBackupPlanBackupConfigElSelectedNamespacesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeBackupBackupPlanBackupConfigElSelectedNamespacesElRef {
    fn new(shared: StackShared, base: String) -> GkeBackupBackupPlanBackupConfigElSelectedNamespacesElRef {
        GkeBackupBackupPlanBackupConfigElSelectedNamespacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlanBackupConfigElSelectedNamespacesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `namespaces` after provisioning.\nA list of Kubernetes Namespaces."]
    pub fn namespaces(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.namespaces", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupBackupPlanBackupConfigElDynamic {
    encryption_key: Option<DynamicBlock<GkeBackupBackupPlanBackupConfigElEncryptionKeyEl>>,
    selected_applications: Option<DynamicBlock<GkeBackupBackupPlanBackupConfigElSelectedApplicationsEl>>,
    selected_namespaces: Option<DynamicBlock<GkeBackupBackupPlanBackupConfigElSelectedNamespacesEl>>,
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlanBackupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_namespaces: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_secrets: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_volume_data: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<Vec<GkeBackupBackupPlanBackupConfigElEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_applications: Option<Vec<GkeBackupBackupPlanBackupConfigElSelectedApplicationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_namespaces: Option<Vec<GkeBackupBackupPlanBackupConfigElSelectedNamespacesEl>>,
    dynamic: GkeBackupBackupPlanBackupConfigElDynamic,
}

impl GkeBackupBackupPlanBackupConfigEl {
    #[doc= "Set the field `all_namespaces`.\nIf True, include all namespaced resources."]
    pub fn set_all_namespaces(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_namespaces = Some(v.into());
        self
    }

    #[doc= "Set the field `include_secrets`.\nThis flag specifies whether Kubernetes Secret resources should be included\nwhen they fall into the scope of Backups."]
    pub fn set_include_secrets(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_secrets = Some(v.into());
        self
    }

    #[doc= "Set the field `include_volume_data`.\nThis flag specifies whether volume data should be backed up when PVCs are\nincluded in the scope of a Backup."]
    pub fn set_include_volume_data(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_volume_data = Some(v.into());
        self
    }

    #[doc= "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(
        mut self,
        v: impl Into<BlockAssignable<GkeBackupBackupPlanBackupConfigElEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_key = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `selected_applications`.\n"]
    pub fn set_selected_applications(
        mut self,
        v: impl Into<BlockAssignable<GkeBackupBackupPlanBackupConfigElSelectedApplicationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selected_applications = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_applications = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `selected_namespaces`.\n"]
    pub fn set_selected_namespaces(
        mut self,
        v: impl Into<BlockAssignable<GkeBackupBackupPlanBackupConfigElSelectedNamespacesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.selected_namespaces = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_namespaces = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GkeBackupBackupPlanBackupConfigEl {
    type O = BlockAssignable<GkeBackupBackupPlanBackupConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlanBackupConfigEl {}

impl BuildGkeBackupBackupPlanBackupConfigEl {
    pub fn build(self) -> GkeBackupBackupPlanBackupConfigEl {
        GkeBackupBackupPlanBackupConfigEl {
            all_namespaces: core::default::Default::default(),
            include_secrets: core::default::Default::default(),
            include_volume_data: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
            selected_applications: core::default::Default::default(),
            selected_namespaces: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GkeBackupBackupPlanBackupConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeBackupBackupPlanBackupConfigElRef {
    fn new(shared: StackShared, base: String) -> GkeBackupBackupPlanBackupConfigElRef {
        GkeBackupBackupPlanBackupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlanBackupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `all_namespaces` after provisioning.\nIf True, include all namespaced resources."]
    pub fn all_namespaces(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all_namespaces", self.base))
    }

    #[doc= "Get a reference to the value of field `include_secrets` after provisioning.\nThis flag specifies whether Kubernetes Secret resources should be included\nwhen they fall into the scope of Backups."]
    pub fn include_secrets(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_secrets", self.base))
    }

    #[doc= "Get a reference to the value of field `include_volume_data` after provisioning.\nThis flag specifies whether volume data should be backed up when PVCs are\nincluded in the scope of a Backup."]
    pub fn include_volume_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_volume_data", self.base))
    }

    #[doc= "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> ListRef<GkeBackupBackupPlanBackupConfigElEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }

    #[doc= "Get a reference to the value of field `selected_applications` after provisioning.\n"]
    pub fn selected_applications(&self) -> ListRef<GkeBackupBackupPlanBackupConfigElSelectedApplicationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.selected_applications", self.base))
    }

    #[doc= "Get a reference to the value of field `selected_namespaces` after provisioning.\n"]
    pub fn selected_namespaces(&self) -> ListRef<GkeBackupBackupPlanBackupConfigElSelectedNamespacesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.selected_namespaces", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlanBackupScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cron_schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    paused: Option<PrimField<bool>>,
}

impl GkeBackupBackupPlanBackupScheduleEl {
    #[doc= "Set the field `cron_schedule`.\nA standard cron string that defines a repeating schedule for\ncreating Backups via this BackupPlan.\nIf this is defined, then backupRetainDays must also be defined."]
    pub fn set_cron_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cron_schedule = Some(v.into());
        self
    }

    #[doc= "Set the field `paused`.\nThis flag denotes whether automatic Backup creation is paused for this BackupPlan."]
    pub fn set_paused(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.paused = Some(v.into());
        self
    }
}

impl ToListMappable for GkeBackupBackupPlanBackupScheduleEl {
    type O = BlockAssignable<GkeBackupBackupPlanBackupScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlanBackupScheduleEl {}

impl BuildGkeBackupBackupPlanBackupScheduleEl {
    pub fn build(self) -> GkeBackupBackupPlanBackupScheduleEl {
        GkeBackupBackupPlanBackupScheduleEl {
            cron_schedule: core::default::Default::default(),
            paused: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupBackupPlanBackupScheduleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeBackupBackupPlanBackupScheduleElRef {
    fn new(shared: StackShared, base: String) -> GkeBackupBackupPlanBackupScheduleElRef {
        GkeBackupBackupPlanBackupScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlanBackupScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cron_schedule` after provisioning.\nA standard cron string that defines a repeating schedule for\ncreating Backups via this BackupPlan.\nIf this is defined, then backupRetainDays must also be defined."]
    pub fn cron_schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cron_schedule", self.base))
    }

    #[doc= "Get a reference to the value of field `paused` after provisioning.\nThis flag denotes whether automatic Backup creation is paused for this BackupPlan."]
    pub fn paused(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.paused", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlanRetentionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_delete_lock_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_retain_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<PrimField<bool>>,
}

impl GkeBackupBackupPlanRetentionPolicyEl {
    #[doc= "Set the field `backup_delete_lock_days`.\nMinimum age for a Backup created via this BackupPlan (in days).\nMust be an integer value between 0-90 (inclusive).\nA Backup created under this BackupPlan will not be deletable\nuntil it reaches Backup's (create time + backup_delete_lock_days).\nUpdating this field of a BackupPlan does not affect existing Backups.\nBackups created after a successful update will inherit this new value."]
    pub fn set_backup_delete_lock_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.backup_delete_lock_days = Some(v.into());
        self
    }

    #[doc= "Set the field `backup_retain_days`.\nThe default maximum age of a Backup created via this BackupPlan.\nThis field MUST be an integer value >= 0 and <= 365. If specified,\na Backup created under this BackupPlan will be automatically deleted\nafter its age reaches (createTime + backupRetainDays).\nIf not specified, Backups created under this BackupPlan will NOT be\nsubject to automatic deletion. Updating this field does NOT affect\nexisting Backups under it. Backups created AFTER a successful update\nwill automatically pick up the new value.\nNOTE: backupRetainDays must be >= backupDeleteLockDays.\nIf cronSchedule is defined, then this must be <= 360 * the creation interval.]"]
    pub fn set_backup_retain_days(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.backup_retain_days = Some(v.into());
        self
    }

    #[doc= "Set the field `locked`.\nThis flag denotes whether the retention policy of this BackupPlan is locked.\nIf set to True, no further update is allowed on this policy, including\nthe locked field itself."]
    pub fn set_locked(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.locked = Some(v.into());
        self
    }
}

impl ToListMappable for GkeBackupBackupPlanRetentionPolicyEl {
    type O = BlockAssignable<GkeBackupBackupPlanRetentionPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlanRetentionPolicyEl {}

impl BuildGkeBackupBackupPlanRetentionPolicyEl {
    pub fn build(self) -> GkeBackupBackupPlanRetentionPolicyEl {
        GkeBackupBackupPlanRetentionPolicyEl {
            backup_delete_lock_days: core::default::Default::default(),
            backup_retain_days: core::default::Default::default(),
            locked: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupBackupPlanRetentionPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeBackupBackupPlanRetentionPolicyElRef {
    fn new(shared: StackShared, base: String) -> GkeBackupBackupPlanRetentionPolicyElRef {
        GkeBackupBackupPlanRetentionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlanRetentionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backup_delete_lock_days` after provisioning.\nMinimum age for a Backup created via this BackupPlan (in days).\nMust be an integer value between 0-90 (inclusive).\nA Backup created under this BackupPlan will not be deletable\nuntil it reaches Backup's (create time + backup_delete_lock_days).\nUpdating this field of a BackupPlan does not affect existing Backups.\nBackups created after a successful update will inherit this new value."]
    pub fn backup_delete_lock_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_delete_lock_days", self.base))
    }

    #[doc= "Get a reference to the value of field `backup_retain_days` after provisioning.\nThe default maximum age of a Backup created via this BackupPlan.\nThis field MUST be an integer value >= 0 and <= 365. If specified,\na Backup created under this BackupPlan will be automatically deleted\nafter its age reaches (createTime + backupRetainDays).\nIf not specified, Backups created under this BackupPlan will NOT be\nsubject to automatic deletion. Updating this field does NOT affect\nexisting Backups under it. Backups created AFTER a successful update\nwill automatically pick up the new value.\nNOTE: backupRetainDays must be >= backupDeleteLockDays.\nIf cronSchedule is defined, then this must be <= 360 * the creation interval.]"]
    pub fn backup_retain_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_retain_days", self.base))
    }

    #[doc= "Get a reference to the value of field `locked` after provisioning.\nThis flag denotes whether the retention policy of this BackupPlan is locked.\nIf set to True, no further update is allowed on this policy, including\nthe locked field itself."]
    pub fn locked(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.locked", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlanTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl GkeBackupBackupPlanTimeoutsEl {
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

impl ToListMappable for GkeBackupBackupPlanTimeoutsEl {
    type O = BlockAssignable<GkeBackupBackupPlanTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlanTimeoutsEl {}

impl BuildGkeBackupBackupPlanTimeoutsEl {
    pub fn build(self) -> GkeBackupBackupPlanTimeoutsEl {
        GkeBackupBackupPlanTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupBackupPlanTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeBackupBackupPlanTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeBackupBackupPlanTimeoutsElRef {
        GkeBackupBackupPlanTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlanTimeoutsElRef {
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
struct GkeBackupBackupPlanDynamic {
    backup_config: Option<DynamicBlock<GkeBackupBackupPlanBackupConfigEl>>,
    backup_schedule: Option<DynamicBlock<GkeBackupBackupPlanBackupScheduleEl>>,
    retention_policy: Option<DynamicBlock<GkeBackupBackupPlanRetentionPolicyEl>>,
}
