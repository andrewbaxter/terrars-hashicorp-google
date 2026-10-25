#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BackupDrBackupPlanData_ {
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
    backup_plan_id: terrars::PrimField<std::string::String>,
    backup_vault: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_retention_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_custom_on_demand_retention_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_rules: std::option::Option<std::vec::Vec<BackupDrBackupPlan_BackupRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_instance_backup_plan_properties: std
    ::option
    ::Option<std::vec::Vec<BackupDrBackupPlan_ComputeInstanceBackupPlanProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_backup_plan_properties: std::option::Option<std::vec::Vec<BackupDrBackupPlan_DiskBackupPlanProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BackupDrBackupPlan_Timeouts>,
    dynamic: BackupDrBackupPlanDynamic,
}

struct BackupDrBackupPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BackupDrBackupPlanData_>,
}

#[derive(Clone)]
pub struct BackupDrBackupPlan(std::rc::Rc<BackupDrBackupPlan_>);

impl BackupDrBackupPlan {
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
        "Set the field `description`.\nThe description allows for additional details about 'BackupPlan' and its use cases to be provided."]
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
        "Set the field `log_retention_days`.\nThis is only applicable for CloudSql resource. Days for which logs will be stored. This value should be greater than or equal to minimum enforced log retention duration of the backup vault."]
    pub fn set_log_retention_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().log_retention_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_custom_on_demand_retention_days`.\nThe maximum number of days for which an on-demand backup taken with custom retention can be retained."]
    pub fn set_max_custom_on_demand_retention_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_custom_on_demand_retention_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_rules`.\n"]
    pub fn set_backup_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupDrBackupPlan_BackupRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compute_instance_backup_plan_properties`.\n"]
    pub fn set_compute_instance_backup_plan_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupDrBackupPlan_ComputeInstanceBackupPlanProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_instance_backup_plan_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_instance_backup_plan_properties =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_backup_plan_properties`.\n"]
    pub fn set_disk_backup_plan_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupDrBackupPlan_DiskBackupPlanProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk_backup_plan_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk_backup_plan_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BackupDrBackupPlan_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `backup_plan_id` after provisioning.\nThe ID of the backup plan"]
    pub fn backup_plan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_vault` after provisioning.\nBackup vault where the backups gets stored using this Backup plan."]
    pub fn backup_vault(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_vault_service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the BackupVault for taking backups."]
    pub fn backup_vault_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nWhen the 'BackupPlan' was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description allows for additional details about 'BackupPlan' and its use cases to be provided."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the backup plan"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_retention_days` after provisioning.\nThis is only applicable for CloudSql resource. Days for which logs will be stored. This value should be greater than or equal to minimum enforced log retention duration of the backup vault."]
    pub fn log_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_retention_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_custom_on_demand_retention_days` after provisioning.\nThe maximum number of days for which an on-demand backup taken with custom retention can be retained."]
    pub fn max_custom_on_demand_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_custom_on_demand_retention_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of backup plan resource created"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type to which the 'BackupPlan' will be applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"sqladmin.googleapis.com/Instance\", \"alloydb.googleapis.com/Cluster\", \"file.googleapis.com/Instance\" and \"storage.googleapis.com/Bucket\"."]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_resource_types` after provisioning.\nThe list of all resource types to which the 'BackupPlan' can be applied."]
    pub fn supported_resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_resource_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nWhen the 'BackupPlan' was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_rules` after provisioning.\n"]
    pub fn backup_rules(&self) -> terrars::ListRef<BackupDrBackupPlan_BackupRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_instance_backup_plan_properties` after provisioning.\n"]
    pub fn compute_instance_backup_plan_properties(
        &self,
    ) -> terrars::ListRef<BackupDrBackupPlan_ComputeInstanceBackupPlanPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_backup_plan_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_backup_plan_properties` after provisioning.\n"]
    pub fn disk_backup_plan_properties(&self) -> terrars::ListRef<BackupDrBackupPlan_DiskBackupPlanPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_backup_plan_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrBackupPlan_TimeoutsRef {
        BackupDrBackupPlan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BackupDrBackupPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BackupDrBackupPlan { }

impl terrars::ToListMappable for BackupDrBackupPlan {
    type O = terrars::ListRef<BackupDrBackupPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BackupDrBackupPlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_backup_dr_backup_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupDrBackupPlan {
    pub tf_id: std::string::String,
    #[doc = "The ID of the backup plan"]
    pub backup_plan_id: terrars::PrimField<std::string::String>,
    #[doc = "Backup vault where the backups gets stored using this Backup plan."]
    pub backup_vault: terrars::PrimField<std::string::String>,
    #[doc = "The location for the backup plan"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource type to which the 'BackupPlan' will be applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"sqladmin.googleapis.com/Instance\", \"alloydb.googleapis.com/Cluster\", \"file.googleapis.com/Instance\" and \"storage.googleapis.com/Bucket\"."]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrBackupPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> BackupDrBackupPlan {
        let out = BackupDrBackupPlan(std::rc::Rc::new(BackupDrBackupPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BackupDrBackupPlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                backup_plan_id: self.backup_plan_id,
                backup_vault: self.backup_vault,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                log_retention_days: core::default::Default::default(),
                max_custom_on_demand_retention_days: core::default::Default::default(),
                project: core::default::Default::default(),
                resource_type: self.resource_type,
                backup_rules: core::default::Default::default(),
                compute_instance_backup_plan_properties: core::default::Default::default(),
                disk_backup_plan_properties: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupDrBackupPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BackupDrBackupPlanRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_plan_id` after provisioning.\nThe ID of the backup plan"]
    pub fn backup_plan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_vault` after provisioning.\nBackup vault where the backups gets stored using this Backup plan."]
    pub fn backup_vault(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_vault_service_account` after provisioning.\nThe Google Cloud Platform Service Account to be used by the BackupVault for taking backups."]
    pub fn backup_vault_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_vault_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nWhen the 'BackupPlan' was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description allows for additional details about 'BackupPlan' and its use cases to be provided."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the backup plan"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_retention_days` after provisioning.\nThis is only applicable for CloudSql resource. Days for which logs will be stored. This value should be greater than or equal to minimum enforced log retention duration of the backup vault."]
    pub fn log_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_retention_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_custom_on_demand_retention_days` after provisioning.\nThe maximum number of days for which an on-demand backup taken with custom retention can be retained."]
    pub fn max_custom_on_demand_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_custom_on_demand_retention_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of backup plan resource created"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type to which the 'BackupPlan' will be applied.\nExamples include, \"compute.googleapis.com/Instance\", \"compute.googleapis.com/Disk\", \"sqladmin.googleapis.com/Instance\", \"alloydb.googleapis.com/Cluster\", \"file.googleapis.com/Instance\" and \"storage.googleapis.com/Bucket\"."]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_resource_types` after provisioning.\nThe list of all resource types to which the 'BackupPlan' can be applied."]
    pub fn supported_resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_resource_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nWhen the 'BackupPlan' was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_rules` after provisioning.\n"]
    pub fn backup_rules(&self) -> terrars::ListRef<BackupDrBackupPlan_BackupRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_instance_backup_plan_properties` after provisioning.\n"]
    pub fn compute_instance_backup_plan_properties(
        &self,
    ) -> terrars::ListRef<BackupDrBackupPlan_ComputeInstanceBackupPlanPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.compute_instance_backup_plan_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_backup_plan_properties` after provisioning.\n"]
    pub fn disk_backup_plan_properties(&self) -> terrars::ListRef<BackupDrBackupPlan_DiskBackupPlanPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_backup_plan_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BackupDrBackupPlan_TimeoutsRef {
        BackupDrBackupPlan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BackupDrBackupPlanDynamic {
    backup_rules: std::option::Option<terrars::DynamicBlock<BackupDrBackupPlan_BackupRules>>,
    compute_instance_backup_plan_properties: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrBackupPlan_ComputeInstanceBackupPlanProperties>>,
    disk_backup_plan_properties: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrBackupPlan_DiskBackupPlanProperties>>,
}

#[derive(Serialize)]
pub struct BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_hour_of_day: std::option::Option<terrars::PrimField<f64>>,
    start_hour_of_day: terrars::PrimField<f64>,
}

impl BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow {
    #[doc =
        "Set the field `end_hour_of_day`.\nThe hour of the day (1-24) when the window ends, for example, if the value of end hour of the day is 10, that means the backup window end time is 10:00.\nThe end hour of the day should be greater than the start"]
    pub fn set_end_hour_of_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.end_hour_of_day = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow {
    type O = terrars::BlockAssignable<BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow {
    #[doc =
        "The hour of the day (0-23) when the window starts, for example, if the value of the start hour of the day is 6, that means the backup window starts at 6:00."]
    pub start_hour_of_day: terrars::PrimField<f64>,
}

impl BuildBackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow {
    pub fn build(self) -> BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow {
        BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow {
            end_hour_of_day: core::default::Default::default(),
            start_hour_of_day: self.start_hour_of_day,
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindowRef {
        BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_hour_of_day` after provisioning.\nThe hour of the day (1-24) when the window ends, for example, if the value of end hour of the day is 10, that means the backup window end time is 10:00.\nThe end hour of the day should be greater than the start"]
    pub fn end_hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_hour_of_day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_hour_of_day` after provisioning.\nThe hour of the day (0-23) when the window starts, for example, if the value of the start hour of the day is 6, that means the backup window starts at 6:00."]
    pub fn start_hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_hour_of_day", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth {
    day_of_week: terrars::PrimField<std::string::String>,
    week_of_month: terrars::PrimField<std::string::String>,
}

impl BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth { }

impl terrars::ToListMappable for BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth {
    type O = terrars::BlockAssignable<BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth {
    #[doc =
        "Specifies the day of the week. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: terrars::PrimField<std::string::String>,
    #[doc =
        "WeekOfMonth enumerates possible weeks in the month, e.g. the first, third, or last week of the month. Possible values: [\"WEEK_OF_MONTH_UNSPECIFIED\", \"FIRST\", \"SECOND\", \"THIRD\", \"FOURTH\", \"LAST\"]"]
    pub week_of_month: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth {
    pub fn build(self) -> BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth {
        BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth {
            day_of_week: self.day_of_week,
            week_of_month: self.week_of_month,
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonthRef {
        BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day_of_week` after provisioning.\nSpecifies the day of the week. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc =
        "Get a reference to the value of field `week_of_month` after provisioning.\nWeekOfMonth enumerates possible weeks in the month, e.g. the first, third, or last week of the month. Possible values: [\"WEEK_OF_MONTH_UNSPECIFIED\", \"FIRST\", \"SECOND\", \"THIRD\", \"FOURTH\", \"LAST\"]"]
    pub fn week_of_month(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.week_of_month", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupDrBackupPlan_BackupRules_StandardScheduleDynamic {
    backup_window: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow>>,
    week_day_of_month: std
    ::option
    ::Option<terrars::DynamicBlock<BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth>>,
}

#[derive(Serialize)]
pub struct BackupDrBackupPlan_BackupRules_StandardSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_month: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_week: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hourly_frequency: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    months: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    recurrence_type: terrars::PrimField<std::string::String>,
    time_zone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_window: std::option::Option<std::vec::Vec<BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    week_day_of_month: std::option::Option<std::vec::Vec<BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth>>,
    dynamic: BackupDrBackupPlan_BackupRules_StandardScheduleDynamic,
}

impl BackupDrBackupPlan_BackupRules_StandardSchedule {
    #[doc = "Set the field `days_of_month`.\nSpecifies days of months like 1, 5, or 14 on which jobs will run."]
    pub fn set_days_of_month(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.days_of_month = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `days_of_week`.\nSpecifies days of week like MONDAY or TUESDAY, on which jobs will run. This is required for 'recurrence_type', 'WEEKLY' and is not applicable otherwise. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn set_days_of_week(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.days_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hourly_frequency`.\nSpecifies frequency for hourly backups. An hourly frequency of 2 means jobs will run every 2 hours from start time till end time defined.\nThis is required for 'recurrence_type', 'HOURLY' and is not applicable otherwise."]
    pub fn set_hourly_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hourly_frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `months`.\nSpecifies values of months Possible values: [\"MONTH_UNSPECIFIED\", \"JANUARY\", \"FEBRUARY\", \"MARCH\", \"APRIL\", \"MAY\", \"JUNE\", \"JULY\", \"AUGUST\", \"SEPTEMBER\", \"OCTOBER\", \"NOVEMBER\", \"DECEMBER\"]"]
    pub fn set_months(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.months = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_window`.\n"]
    pub fn set_backup_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.backup_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `week_day_of_month`.\n"]
    pub fn set_week_day_of_month(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonth>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.week_day_of_month = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.week_day_of_month = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupDrBackupPlan_BackupRules_StandardSchedule {
    type O = terrars::BlockAssignable<BackupDrBackupPlan_BackupRules_StandardSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrBackupPlan_BackupRules_StandardSchedule {
    #[doc =
        "RecurrenceType enumerates the applicable periodicity for the schedule. Possible values: [\"HOURLY\", \"DAILY\", \"WEEKLY\", \"MONTHLY\", \"YEARLY\"]"]
    pub recurrence_type: terrars::PrimField<std::string::String>,
    #[doc = "The time zone to be used when interpreting the schedule."]
    pub time_zone: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrBackupPlan_BackupRules_StandardSchedule {
    pub fn build(self) -> BackupDrBackupPlan_BackupRules_StandardSchedule {
        BackupDrBackupPlan_BackupRules_StandardSchedule {
            days_of_month: core::default::Default::default(),
            days_of_week: core::default::Default::default(),
            hourly_frequency: core::default::Default::default(),
            months: core::default::Default::default(),
            recurrence_type: self.recurrence_type,
            time_zone: self.time_zone,
            backup_window: core::default::Default::default(),
            week_day_of_month: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrBackupPlan_BackupRules_StandardScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupPlan_BackupRules_StandardScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrBackupPlan_BackupRules_StandardScheduleRef {
        BackupDrBackupPlan_BackupRules_StandardScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrBackupPlan_BackupRules_StandardScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days_of_month` after provisioning.\nSpecifies days of months like 1, 5, or 14 on which jobs will run."]
    pub fn days_of_month(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.days_of_month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `days_of_week` after provisioning.\nSpecifies days of week like MONDAY or TUESDAY, on which jobs will run. This is required for 'recurrence_type', 'WEEKLY' and is not applicable otherwise. Possible values: [\"DAY_OF_WEEK_UNSPECIFIED\", \"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn days_of_week(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hourly_frequency` after provisioning.\nSpecifies frequency for hourly backups. An hourly frequency of 2 means jobs will run every 2 hours from start time till end time defined.\nThis is required for 'recurrence_type', 'HOURLY' and is not applicable otherwise."]
    pub fn hourly_frequency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hourly_frequency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `months` after provisioning.\nSpecifies values of months Possible values: [\"MONTH_UNSPECIFIED\", \"JANUARY\", \"FEBRUARY\", \"MARCH\", \"APRIL\", \"MAY\", \"JUNE\", \"JULY\", \"AUGUST\", \"SEPTEMBER\", \"OCTOBER\", \"NOVEMBER\", \"DECEMBER\"]"]
    pub fn months(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.months", self.base))
    }

    #[doc =
        "Get a reference to the value of field `recurrence_type` after provisioning.\nRecurrenceType enumerates the applicable periodicity for the schedule. Possible values: [\"HOURLY\", \"DAILY\", \"WEEKLY\", \"MONTHLY\", \"YEARLY\"]"]
    pub fn recurrence_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone to be used when interpreting the schedule."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_window` after provisioning.\n"]
    pub fn backup_window(&self) -> terrars::ListRef<BackupDrBackupPlan_BackupRules_StandardSchedule_BackupWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_window", self.base))
    }

    #[doc = "Get a reference to the value of field `week_day_of_month` after provisioning.\n"]
    pub fn week_day_of_month(
        &self,
    ) -> terrars::ListRef<BackupDrBackupPlan_BackupRules_StandardSchedule_WeekDayOfMonthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.week_day_of_month", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupDrBackupPlan_BackupRulesDynamic {
    standard_schedule: std::option::Option<terrars::DynamicBlock<BackupDrBackupPlan_BackupRules_StandardSchedule>>,
}

#[derive(Serialize)]
pub struct BackupDrBackupPlan_BackupRules {
    backup_retention_days: terrars::PrimField<f64>,
    rule_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_schedule: std::option::Option<std::vec::Vec<BackupDrBackupPlan_BackupRules_StandardSchedule>>,
    dynamic: BackupDrBackupPlan_BackupRulesDynamic,
}

impl BackupDrBackupPlan_BackupRules {
    #[doc = "Set the field `standard_schedule`.\n"]
    pub fn set_standard_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupDrBackupPlan_BackupRules_StandardSchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.standard_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.standard_schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupDrBackupPlan_BackupRules {
    type O = terrars::BlockAssignable<BackupDrBackupPlan_BackupRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrBackupPlan_BackupRules {
    #[doc =
        "Configures the duration for which backup data will be kept. The value should be greater than or equal to minimum enforced retention of the backup vault."]
    pub backup_retention_days: terrars::PrimField<f64>,
    #[doc = "The unique ID of this 'BackupRule'. The 'rule_id' is unique per 'BackupPlan'."]
    pub rule_id: terrars::PrimField<std::string::String>,
}

impl BuildBackupDrBackupPlan_BackupRules {
    pub fn build(self) -> BackupDrBackupPlan_BackupRules {
        BackupDrBackupPlan_BackupRules {
            backup_retention_days: self.backup_retention_days,
            rule_id: self.rule_id,
            standard_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrBackupPlan_BackupRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupPlan_BackupRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupDrBackupPlan_BackupRulesRef {
        BackupDrBackupPlan_BackupRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrBackupPlan_BackupRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_retention_days` after provisioning.\nConfigures the duration for which backup data will be kept. The value should be greater than or equal to minimum enforced retention of the backup vault."]
    pub fn backup_retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rule_id` after provisioning.\nThe unique ID of this 'BackupRule'. The 'rule_id' is unique per 'BackupPlan'."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }

    #[doc = "Get a reference to the value of field `standard_schedule` after provisioning.\n"]
    pub fn standard_schedule(&self) -> terrars::ListRef<BackupDrBackupPlan_BackupRules_StandardScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrBackupPlan_ComputeInstanceBackupPlanProperties {
    guest_flush: terrars::PrimField<bool>,
}

impl BackupDrBackupPlan_ComputeInstanceBackupPlanProperties { }

impl terrars::ToListMappable for BackupDrBackupPlan_ComputeInstanceBackupPlanProperties {
    type O = terrars::BlockAssignable<BackupDrBackupPlan_ComputeInstanceBackupPlanProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrBackupPlan_ComputeInstanceBackupPlanProperties {
    #[doc =
        "Indicates whether to perform a guest flush operation before taking a\ncompute instance backup. When set to true, the system will attempt\nto ensure application-consistent backups."]
    pub guest_flush: terrars::PrimField<bool>,
}

impl BuildBackupDrBackupPlan_ComputeInstanceBackupPlanProperties {
    pub fn build(self) -> BackupDrBackupPlan_ComputeInstanceBackupPlanProperties {
        BackupDrBackupPlan_ComputeInstanceBackupPlanProperties { guest_flush: self.guest_flush }
    }
}

#[allow(dead_code)]
pub struct BackupDrBackupPlan_ComputeInstanceBackupPlanPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupPlan_ComputeInstanceBackupPlanPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrBackupPlan_ComputeInstanceBackupPlanPropertiesRef {
        BackupDrBackupPlan_ComputeInstanceBackupPlanPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrBackupPlan_ComputeInstanceBackupPlanPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `guest_flush` after provisioning.\nIndicates whether to perform a guest flush operation before taking a\ncompute instance backup. When set to true, the system will attempt\nto ensure application-consistent backups."]
    pub fn guest_flush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrBackupPlan_DiskBackupPlanProperties {
    guest_flush: terrars::PrimField<bool>,
}

impl BackupDrBackupPlan_DiskBackupPlanProperties { }

impl terrars::ToListMappable for BackupDrBackupPlan_DiskBackupPlanProperties {
    type O = terrars::BlockAssignable<BackupDrBackupPlan_DiskBackupPlanProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrBackupPlan_DiskBackupPlanProperties {
    #[doc =
        "Indicates whether to perform a guest flush operation before taking a disk\nbackup. When set to true, the system will attempt to ensure\napplication-consistent backups. When set to false, the system will\ncreate crash-consistent backups."]
    pub guest_flush: terrars::PrimField<bool>,
}

impl BuildBackupDrBackupPlan_DiskBackupPlanProperties {
    pub fn build(self) -> BackupDrBackupPlan_DiskBackupPlanProperties {
        BackupDrBackupPlan_DiskBackupPlanProperties { guest_flush: self.guest_flush }
    }
}

#[allow(dead_code)]
pub struct BackupDrBackupPlan_DiskBackupPlanPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupPlan_DiskBackupPlanPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupDrBackupPlan_DiskBackupPlanPropertiesRef {
        BackupDrBackupPlan_DiskBackupPlanPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrBackupPlan_DiskBackupPlanPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `guest_flush` after provisioning.\nIndicates whether to perform a guest flush operation before taking a disk\nbackup. When set to true, the system will attempt to ensure\napplication-consistent backups. When set to false, the system will\ncreate crash-consistent backups."]
    pub fn guest_flush(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guest_flush", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupDrBackupPlan_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupDrBackupPlan_Timeouts {
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

impl terrars::ToListMappable for BackupDrBackupPlan_Timeouts {
    type O = terrars::BlockAssignable<BackupDrBackupPlan_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupDrBackupPlan_Timeouts {}

impl BuildBackupDrBackupPlan_Timeouts {
    pub fn build(self) -> BackupDrBackupPlan_Timeouts {
        BackupDrBackupPlan_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupDrBackupPlan_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupDrBackupPlan_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupDrBackupPlan_TimeoutsRef {
        BackupDrBackupPlan_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupDrBackupPlan_TimeoutsRef {
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
