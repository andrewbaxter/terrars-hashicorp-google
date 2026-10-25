#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MigrationCenterAssetsExportJobData_ {
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
    assets_export_job_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show_hidden: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<MigrationCenterAssetsExportJob_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_data: std::option::Option<std::vec::Vec<MigrationCenterAssetsExportJob_PerformanceData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_uri_destination: std::option::Option<std::vec::Vec<MigrationCenterAssetsExportJob_SignedUriDestination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MigrationCenterAssetsExportJob_Timeouts>,
    dynamic: MigrationCenterAssetsExportJobDynamic,
}

struct MigrationCenterAssetsExportJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MigrationCenterAssetsExportJobData_>,
}

#[derive(Clone)]
pub struct MigrationCenterAssetsExportJob(std::rc::Rc<MigrationCenterAssetsExportJob_>);

impl MigrationCenterAssetsExportJob {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels as key value pairs.\nLabels must meet the following constraints:\n\n* Keys and values can contain only lowercase letters, numeric characters,\nunderscores, and dashes.\n* All characters must use UTF-8 encoding, and international characters are\nallowed.\n* Keys must start with a lowercase letter or international character.\n* Each resource is limited to a maximum of 64 labels.\n\nBoth keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc =
        "Set the field `show_hidden`.\nWhen this value is set to 'true' the response will include all assets,\nincluding those that are hidden."]
    pub fn set_show_hidden(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().show_hidden = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MigrationCenterAssetsExportJob_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `performance_data`.\n"]
    pub fn set_performance_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MigrationCenterAssetsExportJob_PerformanceData>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().performance_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.performance_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `signed_uri_destination`.\n"]
    pub fn set_signed_uri_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MigrationCenterAssetsExportJob_SignedUriDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().signed_uri_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.signed_uri_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MigrationCenterAssetsExportJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `assets_export_job_id` after provisioning.\nThe ID to use for the asset export job."]
    pub fn assets_export_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assets_export_job_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nResource creation time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inventory` after provisioning.\nConfiguration for asset inventory details exports."]
    pub fn inventory(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_InventoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inventory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\nLabels must meet the following constraints:\n\n* Keys and values can contain only lowercase letters, numeric characters,\nunderscores, and dashes.\n* All characters must use UTF-8 encoding, and international characters are\nallowed.\n* Keys must start with a lowercase letter or international character.\n* Each resource is limited to a maximum of 64 labels.\n\nBoth keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_dependencies` after provisioning.\nConfiguration for network dependencies exports."]
    pub fn network_dependencies(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_NetworkDependenciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_dependencies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recent_executions` after provisioning.\nRecent non expired executions of the job."]
    pub fn recent_executions(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_RecentExecutionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recent_executions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `show_hidden` after provisioning.\nWhen this value is set to 'true' the response will include all assets,\nincluding those that are hidden."]
    pub fn show_hidden(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_hidden", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nResource update time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_data` after provisioning.\n"]
    pub fn performance_data(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_PerformanceDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.performance_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_uri_destination` after provisioning.\n"]
    pub fn signed_uri_destination(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_SignedUriDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_uri_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MigrationCenterAssetsExportJob_TimeoutsRef {
        MigrationCenterAssetsExportJob_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for MigrationCenterAssetsExportJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MigrationCenterAssetsExportJob { }

impl terrars::ToListMappable for MigrationCenterAssetsExportJob {
    type O = terrars::ListRef<MigrationCenterAssetsExportJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MigrationCenterAssetsExportJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_migration_center_assets_export_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMigrationCenterAssetsExportJob {
    pub tf_id: std::string::String,
    #[doc = "The ID to use for the asset export job."]
    pub assets_export_job_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildMigrationCenterAssetsExportJob {
    pub fn build(self, stack: &mut terrars::Stack) -> MigrationCenterAssetsExportJob {
        let out = MigrationCenterAssetsExportJob(std::rc::Rc::new(MigrationCenterAssetsExportJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MigrationCenterAssetsExportJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                assets_export_job_id: self.assets_export_job_id,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                show_hidden: core::default::Default::default(),
                condition: core::default::Default::default(),
                performance_data: core::default::Default::default(),
                signed_uri_destination: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MigrationCenterAssetsExportJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MigrationCenterAssetsExportJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `assets_export_job_id` after provisioning.\nThe ID to use for the asset export job."]
    pub fn assets_export_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assets_export_job_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nResource creation time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inventory` after provisioning.\nConfiguration for asset inventory details exports."]
    pub fn inventory(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_InventoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inventory", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\nLabels must meet the following constraints:\n\n* Keys and values can contain only lowercase letters, numeric characters,\nunderscores, and dashes.\n* All characters must use UTF-8 encoding, and international characters are\nallowed.\n* Keys must start with a lowercase letter or international character.\n* Each resource is limited to a maximum of 64 labels.\n\nBoth keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Resource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_dependencies` after provisioning.\nConfiguration for network dependencies exports."]
    pub fn network_dependencies(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_NetworkDependenciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_dependencies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recent_executions` after provisioning.\nRecent non expired executions of the job."]
    pub fn recent_executions(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_RecentExecutionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recent_executions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `show_hidden` after provisioning.\nWhen this value is set to 'true' the response will include all assets,\nincluding those that are hidden."]
    pub fn show_hidden(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_hidden", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nResource update time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_data` after provisioning.\n"]
    pub fn performance_data(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_PerformanceDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.performance_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signed_uri_destination` after provisioning.\n"]
    pub fn signed_uri_destination(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_SignedUriDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_uri_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MigrationCenterAssetsExportJob_TimeoutsRef {
        MigrationCenterAssetsExportJob_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct MigrationCenterAssetsExportJobDynamic {
    condition: std::option::Option<terrars::DynamicBlock<MigrationCenterAssetsExportJob_Condition>>,
    performance_data: std::option::Option<terrars::DynamicBlock<MigrationCenterAssetsExportJob_PerformanceData>>,
    signed_uri_destination: std
    ::option
    ::Option<terrars::DynamicBlock<MigrationCenterAssetsExportJob_SignedUriDestination>>,
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_Inventory {}

impl MigrationCenterAssetsExportJob_Inventory { }

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_Inventory {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_Inventory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_Inventory {}

impl BuildMigrationCenterAssetsExportJob_Inventory {
    pub fn build(self) -> MigrationCenterAssetsExportJob_Inventory {
        MigrationCenterAssetsExportJob_Inventory {}
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_InventoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_InventoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MigrationCenterAssetsExportJob_InventoryRef {
        MigrationCenterAssetsExportJob_InventoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_InventoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_NetworkDependencies {}

impl MigrationCenterAssetsExportJob_NetworkDependencies { }

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_NetworkDependencies {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_NetworkDependencies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_NetworkDependencies {}

impl BuildMigrationCenterAssetsExportJob_NetworkDependencies {
    pub fn build(self) -> MigrationCenterAssetsExportJob_NetworkDependencies {
        MigrationCenterAssetsExportJob_NetworkDependencies {}
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_NetworkDependenciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_NetworkDependenciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_NetworkDependenciesRef {
        MigrationCenterAssetsExportJob_NetworkDependenciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_NetworkDependenciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_Details {}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_Details { }

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_Details {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_Details>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_Error_Details {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_Error_Details {
    pub fn build(self) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_Details {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_Details {}
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_DetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_DetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_DetailsRef {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_DetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_DetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std
    ::option
    ::Option<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_Details>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_Error {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_Details>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions_Result_Error {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_RecentExecutions_Result_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_Error {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_Error {
    pub fn build(self) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_Error {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_Error {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_RecentExecutions_Result_ErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_ErrorRef {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_Result_Error_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri {
    pub fn build(
        self,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri {
            file: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUriRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUriRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUriRef {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUriRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUriRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_uri: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri,
        >,
    >,
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile {
    #[doc = "Set the field `columns_count`.\n"]
    pub fn set_columns_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.columns_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `row_count`.\n"]
    pub fn set_row_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.row_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signed_uri`.\n"]
    pub fn set_signed_uri(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri,
                        >,
                    >,
    ) -> Self {
        self.signed_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile {
    pub fn build(self) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile {
            columns_count: core::default::Default::default(),
            row_count: core::default::Default::default(),
            signed_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFileRef {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `columns_count` after provisioning.\n"]
    pub fn columns_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.columns_count", self.base))
    }

    #[doc = "Get a reference to the value of field `row_count` after provisioning.\n"]
    pub fn row_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_count", self.base))
    }

    #[doc = "Get a reference to the value of field `signed_uri` after provisioning.\n"]
    pub fn signed_uri(
        &self,
    ) -> terrars
    ::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUriRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_uri: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri,
        >,
    >,
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFile {
    #[doc = "Set the field `signed_uri`.\n"]
    pub fn set_signed_uri(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUri,
                        >,
                    >,
    ) -> Self {
        self.signed_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFile {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFile {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFile {
    pub fn build(self) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFile {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFile {
            signed_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFileRef {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `signed_uri` after provisioning.\n"]
    pub fn signed_uri(
        &self,
    ) -> terrars
    ::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile_SignedUriRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_output_file: std
    ::option
    ::Option<
        terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_size_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xlsx_output_file: std
    ::option
    ::Option<
        terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFile>,
    >,
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries {
    #[doc = "Set the field `csv_output_file`.\n"]
    pub fn set_csv_output_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFile,
                        >,
                    >,
    ) -> Self {
        self.csv_output_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_size_bytes`.\n"]
    pub fn set_file_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_size_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xlsx_output_file`.\n"]
    pub fn set_xlsx_output_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFile,
                        >,
                    >,
    ) -> Self {
        self.xlsx_output_file = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries {
    pub fn build(self) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries {
            csv_output_file: core::default::Default::default(),
            file_size_bytes: core::default::Default::default(),
            xlsx_output_file: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_EntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_EntriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_EntriesRef {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_EntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_EntriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `csv_output_file` after provisioning.\n"]
    pub fn csv_output_file(
        &self,
    ) -> terrars
    ::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_CsvOutputFileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_output_file", self.base))
    }

    #[doc = "Get a reference to the value of field `file_size_bytes` after provisioning.\n"]
    pub fn file_size_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_size_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `xlsx_output_file` after provisioning.\n"]
    pub fn xlsx_output_file(
        &self,
    ) -> terrars
    ::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries_XlsxOutputFileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xlsx_output_file", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entries: std
    ::option
    ::Option<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries>>,
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles {
    #[doc = "Set the field `entries`.\n"]
    pub fn set_entries(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_Entries>,
                    >,
    ) -> Self {
        self.entries = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles {
    pub fn build(self) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles {
            entries: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFilesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFilesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFilesRef {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFilesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFilesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(
        &self,
    ) -> terrars::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles_EntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entries", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUris {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUris {
    #[doc = "Set the field `file`.\n"]
    pub fn set_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUris {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUris>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUris {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUris {
    pub fn build(self) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUris {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUris {
            file: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUrisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUrisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUrisRef {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUrisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUrisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_uris: std
    ::option
    ::Option<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUris>>,
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris {
    #[doc = "Set the field `signed_uris`.\n"]
    pub fn set_signed_uris(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUris>,
                    >,
    ) -> Self {
        self.signed_uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris {
    pub fn build(self) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris {
            signed_uris: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUrisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUrisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUrisRef {
        MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUrisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUrisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `signed_uris` after provisioning.\n"]
    pub fn signed_uris(
        &self,
    ) -> terrars::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris_SignedUrisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_uris", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_Result {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std::option::Option<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_Error>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_files: std
    ::option
    ::Option<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_uris: std
    ::option
    ::Option<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris>>,
}

impl MigrationCenterAssetsExportJob_RecentExecutions_Result {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_Error>>,
    ) -> Self {
        self.error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_files`.\n"]
    pub fn set_output_files(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFiles>>,
    ) -> Self {
        self.output_files = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signed_uris`.\n"]
    pub fn set_signed_uris(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUris>>,
    ) -> Self {
        self.signed_uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions_Result {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_RecentExecutions_Result>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions_Result {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions_Result {
    pub fn build(self) -> MigrationCenterAssetsExportJob_RecentExecutions_Result {
        MigrationCenterAssetsExportJob_RecentExecutions_Result {
            error: core::default::Default::default(),
            output_files: core::default::Default::default(),
            signed_uris: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions_ResultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_RecentExecutions_ResultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutions_ResultRef {
        MigrationCenterAssetsExportJob_RecentExecutions_ResultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutions_ResultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_Result_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `output_files` after provisioning.\n"]
    pub fn output_files(
        &self,
    ) -> terrars::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_Result_OutputFilesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_files", self.base))
    }

    #[doc = "Get a reference to the value of field `signed_uris` after provisioning.\n"]
    pub fn signed_uris(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_Result_SignedUrisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signed_uris", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_RecentExecutions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requested_asset_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: std::option::Option<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterAssetsExportJob_RecentExecutions {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_id`.\n"]
    pub fn set_execution_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expire_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requested_asset_count`.\n"]
    pub fn set_requested_asset_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.requested_asset_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `result`.\n"]
    pub fn set_result(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterAssetsExportJob_RecentExecutions_Result>>,
    ) -> Self {
        self.result = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_RecentExecutions {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_RecentExecutions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_RecentExecutions {}

impl BuildMigrationCenterAssetsExportJob_RecentExecutions {
    pub fn build(self) -> MigrationCenterAssetsExportJob_RecentExecutions {
        MigrationCenterAssetsExportJob_RecentExecutions {
            end_time: core::default::Default::default(),
            execution_id: core::default::Default::default(),
            expire_time: core::default::Default::default(),
            requested_asset_count: core::default::Default::default(),
            result: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_RecentExecutionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_RecentExecutionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_RecentExecutionsRef {
        MigrationCenterAssetsExportJob_RecentExecutionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_RecentExecutionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_id` after provisioning.\n"]
    pub fn execution_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_id", self.base))
    }

    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }

    #[doc = "Get a reference to the value of field `requested_asset_count` after provisioning.\n"]
    pub fn requested_asset_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requested_asset_count", self.base))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> terrars::ListRef<MigrationCenterAssetsExportJob_RecentExecutions_ResultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.result", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterAssetsExportJob_Condition {
    #[doc = "Set the field `filter`.\nAssets filter, supports the same syntax as asset listing."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_Condition {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_Condition {}

impl BuildMigrationCenterAssetsExportJob_Condition {
    pub fn build(self) -> MigrationCenterAssetsExportJob_Condition {
        MigrationCenterAssetsExportJob_Condition { filter: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MigrationCenterAssetsExportJob_ConditionRef {
        MigrationCenterAssetsExportJob_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nAssets filter, supports the same syntax as asset listing."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_PerformanceData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_days: std::option::Option<terrars::PrimField<f64>>,
}

impl MigrationCenterAssetsExportJob_PerformanceData {
    #[doc =
        "Set the field `max_days`.\nWhen this value is set to a positive integer,\nperformance data will be returned for the most recent days\nfor which data is available. When this value is unset (or set to zero),\nall available data is returned.\nThe maximum value is 420; values above 420 will be coerced to 420.\nIf unset (0 value) a default value of 40 will be used."]
    pub fn set_max_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_PerformanceData {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_PerformanceData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_PerformanceData {}

impl BuildMigrationCenterAssetsExportJob_PerformanceData {
    pub fn build(self) -> MigrationCenterAssetsExportJob_PerformanceData {
        MigrationCenterAssetsExportJob_PerformanceData { max_days: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_PerformanceDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_PerformanceDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_PerformanceDataRef {
        MigrationCenterAssetsExportJob_PerformanceDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_PerformanceDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_days` after provisioning.\nWhen this value is set to a positive integer,\nperformance data will be returned for the most recent days\nfor which data is available. When this value is unset (or set to zero),\nall available data is returned.\nThe maximum value is 420; values above 420 will be coerced to 420.\nIf unset (0 value) a default value of 40 will be used."]
    pub fn max_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_days", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_SignedUriDestination {
    file_format: terrars::PrimField<std::string::String>,
}

impl MigrationCenterAssetsExportJob_SignedUriDestination { }

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_SignedUriDestination {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_SignedUriDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_SignedUriDestination {
    #[doc = "The file format to export.\nPossible values:\nCSV\nXLSX"]
    pub file_format: terrars::PrimField<std::string::String>,
}

impl BuildMigrationCenterAssetsExportJob_SignedUriDestination {
    pub fn build(self) -> MigrationCenterAssetsExportJob_SignedUriDestination {
        MigrationCenterAssetsExportJob_SignedUriDestination { file_format: self.file_format }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_SignedUriDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_SignedUriDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterAssetsExportJob_SignedUriDestinationRef {
        MigrationCenterAssetsExportJob_SignedUriDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_SignedUriDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_format` after provisioning.\nThe file format to export.\nPossible values:\nCSV\nXLSX"]
    pub fn file_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_format", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterAssetsExportJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterAssetsExportJob_Timeouts {
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

impl terrars::ToListMappable for MigrationCenterAssetsExportJob_Timeouts {
    type O = terrars::BlockAssignable<MigrationCenterAssetsExportJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterAssetsExportJob_Timeouts {}

impl BuildMigrationCenterAssetsExportJob_Timeouts {
    pub fn build(self) -> MigrationCenterAssetsExportJob_Timeouts {
        MigrationCenterAssetsExportJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterAssetsExportJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterAssetsExportJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MigrationCenterAssetsExportJob_TimeoutsRef {
        MigrationCenterAssetsExportJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterAssetsExportJob_TimeoutsRef {
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
