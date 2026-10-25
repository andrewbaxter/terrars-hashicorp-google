#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MigrationCenterImportJobData_ {
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
    asset_source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    import_job_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MigrationCenterImportJob_Timeouts>,
}

struct MigrationCenterImportJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MigrationCenterImportJobData_>,
}

#[derive(Clone)]
pub struct MigrationCenterImportJob(std::rc::Rc<MigrationCenterImportJob_>);

impl MigrationCenterImportJob {
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

    #[doc = "Set the field `display_name`.\nUser-friendly display name. Maximum length is 256 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MigrationCenterImportJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `asset_source` after provisioning.\nReference to a source."]
    pub fn asset_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `complete_time` after provisioning.\nThe timestamp when the import job was completed."]
    pub fn complete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.complete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the import job was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly display name. Maximum length is 256 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_report` after provisioning.\nA resource that reports result of the import job execution."]
    pub fn execution_report(&self) -> terrars::ListRef<MigrationCenterImportJob_ExecutionReportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_report", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_job_id` after provisioning.\nID of the import job."]
    pub fn import_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_job_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full name of the import job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the import job.\nPossible values:\nIMPORT_JOB_STATE_PENDING\nIMPORT_JOB_STATE_RUNNING\nIMPORT_JOB_STATE_COMPLETED\nIMPORT_JOB_STATE_FAILED\nIMPORT_JOB_STATE_VALIDATING\nIMPORT_JOB_STATE_FAILED_VALIDATION\nIMPORT_JOB_STATE_READY"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the import job was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_report` after provisioning.\nA resource that aggregates errors across import job files."]
    pub fn validation_report(&self) -> terrars::ListRef<MigrationCenterImportJob_ValidationReportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_report", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MigrationCenterImportJob_TimeoutsRef {
        MigrationCenterImportJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MigrationCenterImportJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MigrationCenterImportJob { }

impl terrars::ToListMappable for MigrationCenterImportJob {
    type O = terrars::ListRef<MigrationCenterImportJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MigrationCenterImportJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_migration_center_import_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMigrationCenterImportJob {
    pub tf_id: std::string::String,
    #[doc = "Reference to a source."]
    pub asset_source: terrars::PrimField<std::string::String>,
    #[doc = "ID of the import job."]
    pub import_job_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildMigrationCenterImportJob {
    pub fn build(self, stack: &mut terrars::Stack) -> MigrationCenterImportJob {
        let out = MigrationCenterImportJob(std::rc::Rc::new(MigrationCenterImportJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MigrationCenterImportJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                asset_source: self.asset_source,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                import_job_id: self.import_job_id,
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MigrationCenterImportJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MigrationCenterImportJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asset_source` after provisioning.\nReference to a source."]
    pub fn asset_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `complete_time` after provisioning.\nThe timestamp when the import job was completed."]
    pub fn complete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.complete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the import job was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly display name. Maximum length is 256 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `execution_report` after provisioning.\nA resource that reports result of the import job execution."]
    pub fn execution_report(&self) -> terrars::ListRef<MigrationCenterImportJob_ExecutionReportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_report", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `import_job_id` after provisioning.\nID of the import job."]
    pub fn import_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.import_job_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full name of the import job."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the import job.\nPossible values:\nIMPORT_JOB_STATE_PENDING\nIMPORT_JOB_STATE_RUNNING\nIMPORT_JOB_STATE_COMPLETED\nIMPORT_JOB_STATE_FAILED\nIMPORT_JOB_STATE_VALIDATING\nIMPORT_JOB_STATE_FAILED_VALIDATION\nIMPORT_JOB_STATE_READY"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the import job was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_report` after provisioning.\nA resource that aggregates errors across import job files."]
    pub fn validation_report(&self) -> terrars::ListRef<MigrationCenterImportJob_ValidationReportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_report", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MigrationCenterImportJob_TimeoutsRef {
        MigrationCenterImportJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_FileErrors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_FileErrors {
    #[doc = "Set the field `error_details`.\n"]
    pub fn set_error_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ValidationReport_FileValidations_FileErrors {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_ValidationReport_FileValidations_FileErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ValidationReport_FileValidations_FileErrors {}

impl BuildMigrationCenterImportJob_ValidationReport_FileValidations_FileErrors {
    pub fn build(self) -> MigrationCenterImportJob_ValidationReport_FileValidations_FileErrors {
        MigrationCenterImportJob_ValidationReport_FileValidations_FileErrors {
            error_details: core::default::Default::default(),
            severity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_FileErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ValidationReport_FileValidations_FileErrorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterImportJob_ValidationReport_FileValidations_FileErrorsRef {
        MigrationCenterImportJob_ValidationReport_FileValidations_FileErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_FileErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_details", self.base))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_number: std::option::Option<terrars::PrimField<f64>>,
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError {
    #[doc = "Set the field `row_number`.\n"]
    pub fn set_row_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.row_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError {}

impl BuildMigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError {
    pub fn build(self) -> MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError {
        MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError {
            row_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvErrorRef {
        MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `row_number` after provisioning.\n"]
    pub fn row_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_number", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveError {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_error: std
    ::option
    ::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveError {
    #[doc = "Set the field `csv_error`.\n"]
    pub fn set_csv_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError>,
                    >,
    ) -> Self {
        self.csv_error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_path`.\n"]
    pub fn set_file_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveError {
    type O =
        terrars::BlockAssignable<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveError>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveError {}

impl BuildMigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveError {
    pub fn build(self) -> MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveError {
        MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveError {
            csv_error: core::default::Default::default(),
            file_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveErrorRef {
        MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv_error` after provisioning.\n"]
    pub fn csv_error(
        &self,
    ) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_error", self.base))
    }

    #[doc = "Get a reference to the value of field `file_path` after provisioning.\n"]
    pub fn file_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_path", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_Errors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_Errors {
    #[doc = "Set the field `error_details`.\n"]
    pub fn set_error_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_Errors {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_Errors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_Errors {}

impl BuildMigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_Errors {
    pub fn build(self) -> MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_Errors {
        MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_Errors {
            error_details: core::default::Default::default(),
            severity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ErrorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ErrorsRef {
        MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_details", self.base))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxError {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sheet: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxError {
    #[doc = "Set the field `row_number`.\n"]
    pub fn set_row_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.row_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sheet`.\n"]
    pub fn set_sheet(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sheet = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxError {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxError>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxError {}

impl BuildMigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxError {
    pub fn build(self) -> MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxError {
        MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxError {
            row_number: core::default::Default::default(),
            sheet: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxErrorRef {
        MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `row_number` after provisioning.\n"]
    pub fn row_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_number", self.base))
    }

    #[doc = "Get a reference to the value of field `sheet` after provisioning.\n"]
    pub fn sheet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sheet", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_error: std
    ::option
    ::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveError>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset_title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_error: std
    ::option
    ::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    errors: std
    ::option
    ::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_Errors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xlsx_error: std
    ::option
    ::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxError>>,
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors {
    #[doc = "Set the field `archive_error`.\n"]
    pub fn set_archive_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveError>,
                    >,
    ) -> Self {
        self.archive_error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `asset_title`.\n"]
    pub fn set_asset_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.asset_title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_error`.\n"]
    pub fn set_csv_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvError>,
                    >,
    ) -> Self {
        self.csv_error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `errors`.\n"]
    pub fn set_errors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_Errors>,
                    >,
    ) -> Self {
        self.errors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `row_number`.\n"]
    pub fn set_row_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.row_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_name`.\n"]
    pub fn set_vm_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vm_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_uuid`.\n"]
    pub fn set_vm_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vm_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xlsx_error`.\n"]
    pub fn set_xlsx_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxError>,
                    >,
    ) -> Self {
        self.xlsx_error = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ValidationReport_FileValidations_RowErrors {}

impl BuildMigrationCenterImportJob_ValidationReport_FileValidations_RowErrors {
    pub fn build(self) -> MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors {
        MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors {
            archive_error: core::default::Default::default(),
            asset_title: core::default::Default::default(),
            csv_error: core::default::Default::default(),
            errors: core::default::Default::default(),
            row_number: core::default::Default::default(),
            vm_name: core::default::Default::default(),
            vm_uuid: core::default::Default::default(),
            xlsx_error: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations_RowErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ValidationReport_FileValidations_RowErrorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterImportJob_ValidationReport_FileValidations_RowErrorsRef {
        MigrationCenterImportJob_ValidationReport_FileValidations_RowErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ValidationReport_FileValidations_RowErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `archive_error` after provisioning.\n"]
    pub fn archive_error(
        &self,
    ) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ArchiveErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_error", self.base))
    }

    #[doc = "Get a reference to the value of field `asset_title` after provisioning.\n"]
    pub fn asset_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_title", self.base))
    }

    #[doc = "Get a reference to the value of field `csv_error` after provisioning.\n"]
    pub fn csv_error(
        &self,
    ) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_CsvErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_error", self.base))
    }

    #[doc = "Get a reference to the value of field `errors` after provisioning.\n"]
    pub fn errors(
        &self,
    ) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.base))
    }

    #[doc = "Get a reference to the value of field `row_number` after provisioning.\n"]
    pub fn row_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_number", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_name` after provisioning.\n"]
    pub fn vm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_name", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_uuid` after provisioning.\n"]
    pub fn vm_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `xlsx_error` after provisioning.\n"]
    pub fn xlsx_error(
        &self,
    ) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors_XlsxErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xlsx_error", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_errors: std
    ::option
    ::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_FileErrors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partial_report: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_errors: std
    ::option
    ::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors>>,
}

impl MigrationCenterImportJob_ValidationReport_FileValidations {
    #[doc = "Set the field `file_errors`.\n"]
    pub fn set_file_errors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_FileErrors>>,
    ) -> Self {
        self.file_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_name`.\n"]
    pub fn set_file_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partial_report`.\n"]
    pub fn set_partial_report(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.partial_report = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `row_errors`.\n"]
    pub fn set_row_errors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrors>>,
    ) -> Self {
        self.row_errors = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ValidationReport_FileValidations {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_ValidationReport_FileValidations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ValidationReport_FileValidations {}

impl BuildMigrationCenterImportJob_ValidationReport_FileValidations {
    pub fn build(self) -> MigrationCenterImportJob_ValidationReport_FileValidations {
        MigrationCenterImportJob_ValidationReport_FileValidations {
            file_errors: core::default::Default::default(),
            file_name: core::default::Default::default(),
            partial_report: core::default::Default::default(),
            row_errors: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ValidationReport_FileValidationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ValidationReport_FileValidationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterImportJob_ValidationReport_FileValidationsRef {
        MigrationCenterImportJob_ValidationReport_FileValidationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ValidationReport_FileValidationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_errors` after provisioning.\n"]
    pub fn file_errors(
        &self,
    ) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_FileValidations_FileErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `file_name` after provisioning.\n"]
    pub fn file_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }

    #[doc = "Get a reference to the value of field `partial_report` after provisioning.\n"]
    pub fn partial_report(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partial_report", self.base))
    }

    #[doc = "Get a reference to the value of field `row_errors` after provisioning.\n"]
    pub fn row_errors(&self) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_FileValidations_RowErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_errors", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ValidationReport_JobErrors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterImportJob_ValidationReport_JobErrors {
    #[doc = "Set the field `error_details`.\n"]
    pub fn set_error_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ValidationReport_JobErrors {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_ValidationReport_JobErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ValidationReport_JobErrors {}

impl BuildMigrationCenterImportJob_ValidationReport_JobErrors {
    pub fn build(self) -> MigrationCenterImportJob_ValidationReport_JobErrors {
        MigrationCenterImportJob_ValidationReport_JobErrors {
            error_details: core::default::Default::default(),
            severity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ValidationReport_JobErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ValidationReport_JobErrorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterImportJob_ValidationReport_JobErrorsRef {
        MigrationCenterImportJob_ValidationReport_JobErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ValidationReport_JobErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_details", self.base))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ExecutionReport_ExecutionErrors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_validations: std::option::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_errors: std::option::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_JobErrors>>,
}

impl MigrationCenterImportJob_ExecutionReport_ExecutionErrors {
    #[doc = "Set the field `file_validations`.\n"]
    pub fn set_file_validations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations>>,
    ) -> Self {
        self.file_validations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_errors`.\n"]
    pub fn set_job_errors(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterImportJob_ValidationReport_JobErrors>>,
    ) -> Self {
        self.job_errors = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ExecutionReport_ExecutionErrors {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_ExecutionReport_ExecutionErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ExecutionReport_ExecutionErrors {}

impl BuildMigrationCenterImportJob_ExecutionReport_ExecutionErrors {
    pub fn build(self) -> MigrationCenterImportJob_ExecutionReport_ExecutionErrors {
        MigrationCenterImportJob_ExecutionReport_ExecutionErrors {
            file_validations: core::default::Default::default(),
            job_errors: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ExecutionReport_ExecutionErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ExecutionReport_ExecutionErrorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterImportJob_ExecutionReport_ExecutionErrorsRef {
        MigrationCenterImportJob_ExecutionReport_ExecutionErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ExecutionReport_ExecutionErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_validations` after provisioning.\n"]
    pub fn file_validations(&self) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_FileValidationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_validations", self.base))
    }

    #[doc = "Get a reference to the value of field `job_errors` after provisioning.\n"]
    pub fn job_errors(&self) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_JobErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_errors", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ExecutionReport {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_errors: std::option::Option<terrars::ListField<MigrationCenterImportJob_ExecutionReport_ExecutionErrors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frames_reported: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_rows_count: std::option::Option<terrars::PrimField<f64>>,
}

impl MigrationCenterImportJob_ExecutionReport {
    #[doc = "Set the field `execution_errors`.\n"]
    pub fn set_execution_errors(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterImportJob_ExecutionReport_ExecutionErrors>>,
    ) -> Self {
        self.execution_errors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `frames_reported`.\n"]
    pub fn set_frames_reported(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.frames_reported = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_rows_count`.\n"]
    pub fn set_total_rows_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_rows_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ExecutionReport {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_ExecutionReport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ExecutionReport {}

impl BuildMigrationCenterImportJob_ExecutionReport {
    pub fn build(self) -> MigrationCenterImportJob_ExecutionReport {
        MigrationCenterImportJob_ExecutionReport {
            execution_errors: core::default::Default::default(),
            frames_reported: core::default::Default::default(),
            total_rows_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ExecutionReportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ExecutionReportRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MigrationCenterImportJob_ExecutionReportRef {
        MigrationCenterImportJob_ExecutionReportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ExecutionReportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_errors` after provisioning.\n"]
    pub fn execution_errors(&self) -> terrars::ListRef<MigrationCenterImportJob_ExecutionReport_ExecutionErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_errors", self.base))
    }

    #[doc = "Get a reference to the value of field `frames_reported` after provisioning.\n"]
    pub fn frames_reported(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frames_reported", self.base))
    }

    #[doc = "Get a reference to the value of field `total_rows_count` after provisioning.\n"]
    pub fn total_rows_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_rows_count", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_ValidationReport {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_validations: std::option::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_errors: std::option::Option<terrars::ListField<MigrationCenterImportJob_ValidationReport_JobErrors>>,
}

impl MigrationCenterImportJob_ValidationReport {
    #[doc = "Set the field `file_validations`.\n"]
    pub fn set_file_validations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterImportJob_ValidationReport_FileValidations>>,
    ) -> Self {
        self.file_validations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `job_errors`.\n"]
    pub fn set_job_errors(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterImportJob_ValidationReport_JobErrors>>,
    ) -> Self {
        self.job_errors = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterImportJob_ValidationReport {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_ValidationReport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_ValidationReport {}

impl BuildMigrationCenterImportJob_ValidationReport {
    pub fn build(self) -> MigrationCenterImportJob_ValidationReport {
        MigrationCenterImportJob_ValidationReport {
            file_validations: core::default::Default::default(),
            job_errors: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_ValidationReportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_ValidationReportRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MigrationCenterImportJob_ValidationReportRef {
        MigrationCenterImportJob_ValidationReportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_ValidationReportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_validations` after provisioning.\n"]
    pub fn file_validations(&self) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_FileValidationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_validations", self.base))
    }

    #[doc = "Get a reference to the value of field `job_errors` after provisioning.\n"]
    pub fn job_errors(&self) -> terrars::ListRef<MigrationCenterImportJob_ValidationReport_JobErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_errors", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterImportJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterImportJob_Timeouts {
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

impl terrars::ToListMappable for MigrationCenterImportJob_Timeouts {
    type O = terrars::BlockAssignable<MigrationCenterImportJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterImportJob_Timeouts {}

impl BuildMigrationCenterImportJob_Timeouts {
    pub fn build(self) -> MigrationCenterImportJob_Timeouts {
        MigrationCenterImportJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterImportJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterImportJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MigrationCenterImportJob_TimeoutsRef {
        MigrationCenterImportJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterImportJob_TimeoutsRef {
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
