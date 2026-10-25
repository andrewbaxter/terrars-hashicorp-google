#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeBackupBackupPlanData_ {
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
    deactivated: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_config: std::option::Option<std::vec::Vec<GkeBackupBackupPlan_BackupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_schedule: std::option::Option<std::vec::Vec<GkeBackupBackupPlan_BackupSchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_policy: std::option::Option<std::vec::Vec<GkeBackupBackupPlan_RetentionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeBackupBackupPlan_Timeouts>,
    dynamic: GkeBackupBackupPlanDynamic,
}

struct GkeBackupBackupPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeBackupBackupPlanData_>,
}

#[derive(Clone)]
pub struct GkeBackupBackupPlan(std::rc::Rc<GkeBackupBackupPlan_>);

impl GkeBackupBackupPlan {
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
        "Set the field `deactivated`.\nThis flag indicates whether this BackupPlan has been deactivated.\nSetting this field to True locks the BackupPlan such that no further updates will be allowed\n(except deletes), including the deactivated field itself. It also prevents any new Backups\nfrom being created via this BackupPlan (including scheduled Backups)."]
    pub fn set_deactivated(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deactivated = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nUser specified descriptive string for this BackupPlan."]
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
        "Set the field `labels`.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `backup_config`.\n"]
    pub fn set_backup_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `backup_schedule`.\n"]
    pub fn set_backup_schedule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeBackupBackupPlan_BackupSchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retention_policy`.\n"]
    pub fn set_retention_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeBackupBackupPlan_RetentionPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeBackupBackupPlan_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe source cluster from which Backups will be created via this BackupPlan."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deactivated` after provisioning.\nThis flag indicates whether this BackupPlan has been deactivated.\nSetting this field to True locks the BackupPlan such that no further updates will be allowed\n(except deletes), including the deactivated field itself. It also prevents any new Backups\nfrom being created via this BackupPlan (including scheduled Backups)."]
    pub fn deactivated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deactivated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser specified descriptive string for this BackupPlan."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\netag is used for optimistic concurrency control as a way to help prevent simultaneous\nupdates of a backup plan from overwriting each other. It is strongly suggested that\nsystems make use of the 'etag' in the read-modify-write cycle to perform BackupPlan updates\nin order to avoid race conditions: An etag is returned in the response to backupPlans.get,\nand systems are expected to put that etag in the request to backupPlans.patch or\nbackupPlans.delete to ensure that their change will be applied to the same version of the resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe region of the Backup Plan."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full name of the BackupPlan Resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protected_namespace_count` after provisioning.\nThe number of Kubernetes Namespaces backed up in the last successful Backup created via this BackupPlan."]
    pub fn protected_namespace_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protected_namespace_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protected_pod_count` after provisioning.\nThe number of Kubernetes Pods backed up in the last successful Backup created via this BackupPlan."]
    pub fn protected_pod_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protected_pod_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe State of the BackupPlan."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_reason` after provisioning.\nDetailed description of why BackupPlan is in its current state."]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer generated, unique identifier of UUID format."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_config` after provisioning.\n"]
    pub fn backup_config(&self) -> terrars::ListRef<GkeBackupBackupPlan_BackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_schedule` after provisioning.\n"]
    pub fn backup_schedule(&self) -> terrars::ListRef<GkeBackupBackupPlan_BackupScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<GkeBackupBackupPlan_RetentionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeBackupBackupPlan_TimeoutsRef {
        GkeBackupBackupPlan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GkeBackupBackupPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeBackupBackupPlan { }

impl terrars::ToListMappable for GkeBackupBackupPlan {
    type O = terrars::ListRef<GkeBackupBackupPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeBackupBackupPlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gke_backup_backup_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeBackupBackupPlan {
    pub tf_id: std::string::String,
    #[doc = "The source cluster from which Backups will be created via this BackupPlan."]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = "The region of the Backup Plan."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The full name of the BackupPlan Resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGkeBackupBackupPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeBackupBackupPlan {
        let out = GkeBackupBackupPlan(std::rc::Rc::new(GkeBackupBackupPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeBackupBackupPlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster: self.cluster,
                deactivated: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeBackupBackupPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeBackupBackupPlanRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe source cluster from which Backups will be created via this BackupPlan."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deactivated` after provisioning.\nThis flag indicates whether this BackupPlan has been deactivated.\nSetting this field to True locks the BackupPlan such that no further updates will be allowed\n(except deletes), including the deactivated field itself. It also prevents any new Backups\nfrom being created via this BackupPlan (including scheduled Backups)."]
    pub fn deactivated(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deactivated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser specified descriptive string for this BackupPlan."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\netag is used for optimistic concurrency control as a way to help prevent simultaneous\nupdates of a backup plan from overwriting each other. It is strongly suggested that\nsystems make use of the 'etag' in the read-modify-write cycle to perform BackupPlan updates\nin order to avoid race conditions: An etag is returned in the response to backupPlans.get,\nand systems are expected to put that etag in the request to backupPlans.patch or\nbackupPlans.delete to ensure that their change will be applied to the same version of the resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe region of the Backup Plan."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full name of the BackupPlan Resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protected_namespace_count` after provisioning.\nThe number of Kubernetes Namespaces backed up in the last successful Backup created via this BackupPlan."]
    pub fn protected_namespace_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protected_namespace_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protected_pod_count` after provisioning.\nThe number of Kubernetes Pods backed up in the last successful Backup created via this BackupPlan."]
    pub fn protected_pod_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protected_pod_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe State of the BackupPlan."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_reason` after provisioning.\nDetailed description of why BackupPlan is in its current state."]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer generated, unique identifier of UUID format."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_config` after provisioning.\n"]
    pub fn backup_config(&self) -> terrars::ListRef<GkeBackupBackupPlan_BackupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_schedule` after provisioning.\n"]
    pub fn backup_schedule(&self) -> terrars::ListRef<GkeBackupBackupPlan_BackupScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_policy` after provisioning.\n"]
    pub fn retention_policy(&self) -> terrars::ListRef<GkeBackupBackupPlan_RetentionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeBackupBackupPlan_TimeoutsRef {
        GkeBackupBackupPlan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupBackupPlanDynamic {
    backup_config: std::option::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupConfig>>,
    backup_schedule: std::option::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupSchedule>>,
    retention_policy: std::option::Option<terrars::DynamicBlock<GkeBackupBackupPlan_RetentionPolicy>>,
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupConfig_EncryptionKey {
    gcp_kms_encryption_key: terrars::PrimField<std::string::String>,
}

impl GkeBackupBackupPlan_BackupConfig_EncryptionKey { }

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupConfig_EncryptionKey {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig_EncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupConfig_EncryptionKey {
    #[doc = "Google Cloud KMS encryption key. Format: projects/*/locations/*/keyRings/*/cryptoKeys/*"]
    pub gcp_kms_encryption_key: terrars::PrimField<std::string::String>,
}

impl BuildGkeBackupBackupPlan_BackupConfig_EncryptionKey {
    pub fn build(self) -> GkeBackupBackupPlan_BackupConfig_EncryptionKey {
        GkeBackupBackupPlan_BackupConfig_EncryptionKey { gcp_kms_encryption_key: self.gcp_kms_encryption_key }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupConfig_EncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupConfig_EncryptionKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupConfig_EncryptionKeyRef {
        GkeBackupBackupPlan_BackupConfig_EncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupConfig_EncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_kms_encryption_key` after provisioning.\nGoogle Cloud KMS encryption key. Format: projects/*/locations/*/keyRings/*/cryptoKeys/*"]
    pub fn gcp_kms_encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_kms_encryption_key", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames {
    name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
}

impl GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames { }

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames {
    #[doc = "The name of a Kubernetes Resource."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The namespace of a Kubernetes Resource."]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildGkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames {
    pub fn build(self) -> GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames {
        GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames {
            name: self.name,
            namespace: self.namespace,
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNamesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNamesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNamesRef {
        GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNamesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNamesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of a Kubernetes Resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nThe namespace of a Kubernetes Resource."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupBackupPlan_BackupConfig_SelectedApplicationsDynamic {
    namespaced_names: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames>>,
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupConfig_SelectedApplications {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespaced_names: std
    ::option
    ::Option<std::vec::Vec<GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames>>,
    dynamic: GkeBackupBackupPlan_BackupConfig_SelectedApplicationsDynamic,
}

impl GkeBackupBackupPlan_BackupConfig_SelectedApplications {
    #[doc = "Set the field `namespaced_names`.\n"]
    pub fn set_namespaced_names(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNames>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.namespaced_names = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.namespaced_names = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupConfig_SelectedApplications {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig_SelectedApplications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupConfig_SelectedApplications {}

impl BuildGkeBackupBackupPlan_BackupConfig_SelectedApplications {
    pub fn build(self) -> GkeBackupBackupPlan_BackupConfig_SelectedApplications {
        GkeBackupBackupPlan_BackupConfig_SelectedApplications {
            namespaced_names: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupConfig_SelectedApplicationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupConfig_SelectedApplicationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupConfig_SelectedApplicationsRef {
        GkeBackupBackupPlan_BackupConfig_SelectedApplicationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupConfig_SelectedApplicationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespaced_names` after provisioning.\n"]
    pub fn namespaced_names(
        &self,
    ) -> terrars::ListRef<GkeBackupBackupPlan_BackupConfig_SelectedApplications_NamespacedNamesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.namespaced_names", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels { }

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels {
    #[doc = "The key of the kubernetes label."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "The value of the Label."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildGkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels {
    pub fn build(self) -> GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels {
        GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabelsRef {
        GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe key of the kubernetes label."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the Label."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabelsDynamic {
    resource_labels: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels>>,
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_labels: std
    ::option
    ::Option<std::vec::Vec<GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels>>,
    dynamic: GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabelsDynamic,
}

impl GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels {
    #[doc = "Set the field `resource_labels`.\n"]
    pub fn set_resource_labels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabels>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_labels = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels {}

impl BuildGkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels {
    pub fn build(self) -> GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels {
        GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels {
            resource_labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabelsRef {
        GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabelsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_labels` after provisioning.\n"]
    pub fn resource_labels(
        &self,
    ) -> terrars::ListRef<GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels_ResourceLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_labels", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupConfig_SelectedNamespaces {
    namespaces: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeBackupBackupPlan_BackupConfig_SelectedNamespaces { }

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupConfig_SelectedNamespaces {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig_SelectedNamespaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupConfig_SelectedNamespaces {
    #[doc = "A list of Kubernetes Namespaces."]
    pub namespaces: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeBackupBackupPlan_BackupConfig_SelectedNamespaces {
    pub fn build(self) -> GkeBackupBackupPlan_BackupConfig_SelectedNamespaces {
        GkeBackupBackupPlan_BackupConfig_SelectedNamespaces { namespaces: self.namespaces }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupConfig_SelectedNamespacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupConfig_SelectedNamespacesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupConfig_SelectedNamespacesRef {
        GkeBackupBackupPlan_BackupConfig_SelectedNamespacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupConfig_SelectedNamespacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\nA list of Kubernetes Namespaces."]
    pub fn namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.namespaces", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupBackupPlan_BackupConfigDynamic {
    encryption_key: std::option::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupConfig_EncryptionKey>>,
    selected_applications: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupConfig_SelectedApplications>>,
    selected_namespace_labels: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels>>,
    selected_namespaces: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupConfig_SelectedNamespaces>>,
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_namespaces: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_secrets: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_volume_data: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissive_mode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key: std::option::Option<std::vec::Vec<GkeBackupBackupPlan_BackupConfig_EncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_applications: std::option::Option<std::vec::Vec<GkeBackupBackupPlan_BackupConfig_SelectedApplications>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_namespace_labels: std
    ::option
    ::Option<std::vec::Vec<GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_namespaces: std::option::Option<std::vec::Vec<GkeBackupBackupPlan_BackupConfig_SelectedNamespaces>>,
    dynamic: GkeBackupBackupPlan_BackupConfigDynamic,
}

impl GkeBackupBackupPlan_BackupConfig {
    #[doc = "Set the field `all_namespaces`.\nIf True, include all namespaced resources."]
    pub fn set_all_namespaces(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_secrets`.\nThis flag specifies whether Kubernetes Secret resources should be included\nwhen they fall into the scope of Backups."]
    pub fn set_include_secrets(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_secrets = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_volume_data`.\nThis flag specifies whether volume data should be backed up when PVCs are\nincluded in the scope of a Backup."]
    pub fn set_include_volume_data(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_volume_data = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permissive_mode`.\nThis flag specifies whether Backups will not fail when\nBackup for GKE detects Kubernetes configuration that is\nnon-standard or requires additional setup to restore."]
    pub fn set_permissive_mode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.permissive_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig_EncryptionKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selected_applications`.\n"]
    pub fn set_selected_applications(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig_SelectedApplications>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selected_applications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_applications = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selected_namespace_labels`.\n"]
    pub fn set_selected_namespace_labels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabels>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selected_namespace_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_namespace_labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selected_namespaces`.\n"]
    pub fn set_selected_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig_SelectedNamespaces>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selected_namespaces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_namespaces = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupConfig {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupConfig {}

impl BuildGkeBackupBackupPlan_BackupConfig {
    pub fn build(self) -> GkeBackupBackupPlan_BackupConfig {
        GkeBackupBackupPlan_BackupConfig {
            all_namespaces: core::default::Default::default(),
            include_secrets: core::default::Default::default(),
            include_volume_data: core::default::Default::default(),
            permissive_mode: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
            selected_applications: core::default::Default::default(),
            selected_namespace_labels: core::default::Default::default(),
            selected_namespaces: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeBackupBackupPlan_BackupConfigRef {
        GkeBackupBackupPlan_BackupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all_namespaces` after provisioning.\nIf True, include all namespaced resources."]
    pub fn all_namespaces(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_secrets` after provisioning.\nThis flag specifies whether Kubernetes Secret resources should be included\nwhen they fall into the scope of Backups."]
    pub fn include_secrets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_secrets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_volume_data` after provisioning.\nThis flag specifies whether volume data should be backed up when PVCs are\nincluded in the scope of a Backup."]
    pub fn include_volume_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_volume_data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permissive_mode` after provisioning.\nThis flag specifies whether Backups will not fail when\nBackup for GKE detects Kubernetes configuration that is\nnon-standard or requires additional setup to restore."]
    pub fn permissive_mode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permissive_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> terrars::ListRef<GkeBackupBackupPlan_BackupConfig_EncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `selected_applications` after provisioning.\n"]
    pub fn selected_applications(&self) -> terrars::ListRef<GkeBackupBackupPlan_BackupConfig_SelectedApplicationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selected_applications", self.base))
    }

    #[doc = "Get a reference to the value of field `selected_namespace_labels` after provisioning.\n"]
    pub fn selected_namespace_labels(
        &self,
    ) -> terrars::ListRef<GkeBackupBackupPlan_BackupConfig_SelectedNamespaceLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selected_namespace_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `selected_namespaces` after provisioning.\n"]
    pub fn selected_namespaces(&self) -> terrars::ListRef<GkeBackupBackupPlan_BackupConfig_SelectedNamespacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selected_namespaces", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_week: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek {
    #[doc =
        "Set the field `days_of_week`.\nA list of days of week. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn set_days_of_week(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.days_of_week = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek {}

impl BuildGkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek {
    pub fn build(self) -> GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek {
        GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek {
            days_of_week: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeekRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeekRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeekRef {
        GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeekRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeekRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days_of_week` after provisioning.\nA list of days of week. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn days_of_week(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate {
    #[doc = "Set the field `day`.\nDay of a month."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `month`.\nMonth of a year."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate {
    type O =
        terrars::BlockAssignable<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate {}

impl BuildGkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate {
    pub fn build(self) -> GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate {
        GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDateRef {
        GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\nDay of a month."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `month` after provisioning.\nMonth of a year."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc = "Get a reference to the value of field `year` after provisioning.\nYear of the date."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime {
    #[doc = "Set the field `hours`.\nHours of day in 24 hour format."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\nMinutes of hour of day."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\nSeconds of minutes of the time."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime {}

impl BuildGkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime {
    pub fn build(self) -> GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime {
        GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTimeRef {
        GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindowsDynamic {
    days_of_week: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek>>,
    single_occurrence_date: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate>,
    >,
    start_time: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime>>,
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    daily: std::option::Option<terrars::PrimField<bool>>,
    duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_week: std
    ::option
    ::Option<std::vec::Vec<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_occurrence_date: std
    ::option
    ::Option<std::vec::Vec<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<std::vec::Vec<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime>>,
    dynamic: GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindowsDynamic,
}

impl GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows {
    #[doc =
        "Set the field `daily`.\nThe exclusion window occurs every day if set to \"True\".\nSpecifying this field to \"False\" is an error.\nOnly one of singleOccurrenceDate, daily and daysOfWeek may be set."]
    pub fn set_daily(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.daily = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `days_of_week`.\n"]
    pub fn set_days_of_week(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeek>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.days_of_week = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.days_of_week = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `single_occurrence_date`.\n"]
    pub fn set_single_occurrence_date(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDate,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.single_occurrence_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.single_occurrence_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTime>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows {
    #[doc =
        "Specifies duration of the window in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\". Restrictions for duration based on the\nrecurrence type to allow some time for backup to happen:\n  - single_occurrence_date:  no restriction\n  - daily window: duration < 24 hours\n  - weekly window:\n    - days of week includes all seven days of a week: duration < 24 hours\n    - all other weekly window: duration < 168 hours (i.e., 24 * 7 hours)"]
    pub duration: terrars::PrimField<std::string::String>,
}

impl BuildGkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows {
    pub fn build(self) -> GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows {
        GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows {
            daily: core::default::Default::default(),
            duration: self.duration,
            days_of_week: core::default::Default::default(),
            single_occurrence_date: core::default::Default::default(),
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindowsRef {
        GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `daily` after provisioning.\nThe exclusion window occurs every day if set to \"True\".\nSpecifying this field to \"False\" is an error.\nOnly one of singleOccurrenceDate, daily and daysOfWeek may be set."]
    pub fn daily(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.daily", self.base))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nSpecifies duration of the window in seconds with up to nine fractional digits,\nterminated by 's'. Example: \"3.5s\". Restrictions for duration based on the\nrecurrence type to allow some time for backup to happen:\n  - single_occurrence_date:  no restriction\n  - daily window: duration < 24 hours\n  - weekly window:\n    - days of week includes all seven days of a week: duration < 24 hours\n    - all other weekly window: duration < 168 hours (i.e., 24 * 7 hours)"]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `days_of_week` after provisioning.\n"]
    pub fn days_of_week(
        &self,
    ) -> terrars::ListRef<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_DaysOfWeekRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `single_occurrence_date` after provisioning.\n"]
    pub fn single_occurrence_date(
        &self,
    ) -> terrars::ListRef<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_SingleOccurrenceDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_occurrence_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupBackupPlan_BackupSchedule_RpoConfigDynamic {
    exclusion_windows: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows>>,
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupSchedule_RpoConfig {
    target_rpo_minutes: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_windows: std::option::Option<std::vec::Vec<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows>>,
    dynamic: GkeBackupBackupPlan_BackupSchedule_RpoConfigDynamic,
}

impl GkeBackupBackupPlan_BackupSchedule_RpoConfig {
    #[doc = "Set the field `exclusion_windows`.\n"]
    pub fn set_exclusion_windows(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindows>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion_windows = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_windows = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupSchedule_RpoConfig {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupSchedule_RpoConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupSchedule_RpoConfig {
    #[doc =
        "Defines the target RPO for the BackupPlan in minutes, which means the target\nmaximum data loss in time that is acceptable for this BackupPlan. This must be\nat least 60, i.e., 1 hour, and at most 86400, i.e., 60 days."]
    pub target_rpo_minutes: terrars::PrimField<f64>,
}

impl BuildGkeBackupBackupPlan_BackupSchedule_RpoConfig {
    pub fn build(self) -> GkeBackupBackupPlan_BackupSchedule_RpoConfig {
        GkeBackupBackupPlan_BackupSchedule_RpoConfig {
            target_rpo_minutes: self.target_rpo_minutes,
            exclusion_windows: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupSchedule_RpoConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupSchedule_RpoConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupBackupPlan_BackupSchedule_RpoConfigRef {
        GkeBackupBackupPlan_BackupSchedule_RpoConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupSchedule_RpoConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_rpo_minutes` after provisioning.\nDefines the target RPO for the BackupPlan in minutes, which means the target\nmaximum data loss in time that is acceptable for this BackupPlan. This must be\nat least 60, i.e., 1 hour, and at most 86400, i.e., 60 days."]
    pub fn target_rpo_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_rpo_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusion_windows` after provisioning.\n"]
    pub fn exclusion_windows(
        &self,
    ) -> terrars::ListRef<GkeBackupBackupPlan_BackupSchedule_RpoConfig_ExclusionWindowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_windows", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupBackupPlan_BackupScheduleDynamic {
    rpo_config: std::option::Option<terrars::DynamicBlock<GkeBackupBackupPlan_BackupSchedule_RpoConfig>>,
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_BackupSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cron_schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    paused: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rpo_config: std::option::Option<std::vec::Vec<GkeBackupBackupPlan_BackupSchedule_RpoConfig>>,
    dynamic: GkeBackupBackupPlan_BackupScheduleDynamic,
}

impl GkeBackupBackupPlan_BackupSchedule {
    #[doc =
        "Set the field `cron_schedule`.\nA standard cron string that defines a repeating schedule for\ncreating Backups via this BackupPlan.\nThis is mutually exclusive with the rpoConfig field since at most one\nschedule can be defined for a BackupPlan.\nIf this is defined, then backupRetainDays must also be defined."]
    pub fn set_cron_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cron_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `paused`.\nThis flag denotes whether automatic Backup creation is paused for this BackupPlan."]
    pub fn set_paused(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.paused = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rpo_config`.\n"]
    pub fn set_rpo_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeBackupBackupPlan_BackupSchedule_RpoConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rpo_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rpo_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupBackupPlan_BackupSchedule {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_BackupSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_BackupSchedule {}

impl BuildGkeBackupBackupPlan_BackupSchedule {
    pub fn build(self) -> GkeBackupBackupPlan_BackupSchedule {
        GkeBackupBackupPlan_BackupSchedule {
            cron_schedule: core::default::Default::default(),
            paused: core::default::Default::default(),
            rpo_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_BackupScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_BackupScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeBackupBackupPlan_BackupScheduleRef {
        GkeBackupBackupPlan_BackupScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_BackupScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cron_schedule` after provisioning.\nA standard cron string that defines a repeating schedule for\ncreating Backups via this BackupPlan.\nThis is mutually exclusive with the rpoConfig field since at most one\nschedule can be defined for a BackupPlan.\nIf this is defined, then backupRetainDays must also be defined."]
    pub fn cron_schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron_schedule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `paused` after provisioning.\nThis flag denotes whether automatic Backup creation is paused for this BackupPlan."]
    pub fn paused(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paused", self.base))
    }

    #[doc = "Get a reference to the value of field `rpo_config` after provisioning.\n"]
    pub fn rpo_config(&self) -> terrars::ListRef<GkeBackupBackupPlan_BackupSchedule_RpoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rpo_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_RetentionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_delete_lock_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_retain_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locked: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeBackupBackupPlan_RetentionPolicy {
    #[doc =
        "Set the field `backup_delete_lock_days`.\nMinimum age for a Backup created via this BackupPlan (in days).\nMust be an integer value between 0-90 (inclusive).\nA Backup created under this BackupPlan will not be deletable\nuntil it reaches Backup's (create time + backup_delete_lock_days).\nUpdating this field of a BackupPlan does not affect existing Backups.\nBackups created after a successful update will inherit this new value."]
    pub fn set_backup_delete_lock_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.backup_delete_lock_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `backup_retain_days`.\nThe default maximum age of a Backup created via this BackupPlan.\nThis field MUST be an integer value >= 0 and <= 365. If specified,\na Backup created under this BackupPlan will be automatically deleted\nafter its age reaches (createTime + backupRetainDays).\nIf not specified, Backups created under this BackupPlan will NOT be\nsubject to automatic deletion. Updating this field does NOT affect\nexisting Backups under it. Backups created AFTER a successful update\nwill automatically pick up the new value.\nNOTE: backupRetainDays must be >= backupDeleteLockDays.\nIf cronSchedule is defined, then this must be <= 360 * the creation interval.\nIf rpo_config is defined, then this must be\n<= 360 * targetRpoMinutes/(1440minutes/day)"]
    pub fn set_backup_retain_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.backup_retain_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `locked`.\nThis flag denotes whether the retention policy of this BackupPlan is locked.\nIf set to True, no further update is allowed on this policy, including\nthe locked field itself."]
    pub fn set_locked(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.locked = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeBackupBackupPlan_RetentionPolicy {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_RetentionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_RetentionPolicy {}

impl BuildGkeBackupBackupPlan_RetentionPolicy {
    pub fn build(self) -> GkeBackupBackupPlan_RetentionPolicy {
        GkeBackupBackupPlan_RetentionPolicy {
            backup_delete_lock_days: core::default::Default::default(),
            backup_retain_days: core::default::Default::default(),
            locked: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_RetentionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_RetentionPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeBackupBackupPlan_RetentionPolicyRef {
        GkeBackupBackupPlan_RetentionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_RetentionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_delete_lock_days` after provisioning.\nMinimum age for a Backup created via this BackupPlan (in days).\nMust be an integer value between 0-90 (inclusive).\nA Backup created under this BackupPlan will not be deletable\nuntil it reaches Backup's (create time + backup_delete_lock_days).\nUpdating this field of a BackupPlan does not affect existing Backups.\nBackups created after a successful update will inherit this new value."]
    pub fn backup_delete_lock_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_delete_lock_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `backup_retain_days` after provisioning.\nThe default maximum age of a Backup created via this BackupPlan.\nThis field MUST be an integer value >= 0 and <= 365. If specified,\na Backup created under this BackupPlan will be automatically deleted\nafter its age reaches (createTime + backupRetainDays).\nIf not specified, Backups created under this BackupPlan will NOT be\nsubject to automatic deletion. Updating this field does NOT affect\nexisting Backups under it. Backups created AFTER a successful update\nwill automatically pick up the new value.\nNOTE: backupRetainDays must be >= backupDeleteLockDays.\nIf cronSchedule is defined, then this must be <= 360 * the creation interval.\nIf rpo_config is defined, then this must be\n<= 360 * targetRpoMinutes/(1440minutes/day)"]
    pub fn backup_retain_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retain_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `locked` after provisioning.\nThis flag denotes whether the retention policy of this BackupPlan is locked.\nIf set to True, no further update is allowed on this policy, including\nthe locked field itself."]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupBackupPlan_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeBackupBackupPlan_Timeouts {
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

impl terrars::ToListMappable for GkeBackupBackupPlan_Timeouts {
    type O = terrars::BlockAssignable<GkeBackupBackupPlan_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupBackupPlan_Timeouts {}

impl BuildGkeBackupBackupPlan_Timeouts {
    pub fn build(self) -> GkeBackupBackupPlan_Timeouts {
        GkeBackupBackupPlan_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeBackupBackupPlan_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupBackupPlan_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeBackupBackupPlan_TimeoutsRef {
        GkeBackupBackupPlan_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupBackupPlan_TimeoutsRef {
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
