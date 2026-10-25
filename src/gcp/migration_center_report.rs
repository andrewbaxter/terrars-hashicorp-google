#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MigrationCenterReportData_ {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    report_config: terrars::PrimField<std::string::String>,
    report_id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MigrationCenterReport_Timeouts>,
}

struct MigrationCenterReport_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MigrationCenterReportData_>,
}

#[derive(Clone)]
pub struct MigrationCenterReport(std::rc::Rc<MigrationCenterReport_>);

impl MigrationCenterReport {
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

    #[doc = "Set the field `description`.\nFree-text description."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser-friendly display name. Maximum length is 63 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nReport type.\nPossible values:\nTOTAL_COST_OF_OWNERSHIP"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MigrationCenterReport_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nFree-text description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly display name. Maximum length is 63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `report_config` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn report_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `report_id` after provisioning.\nUser specified id for the report. It will become the last component\nof the report name. The id must be unique within the project, must\nconform with RFC-1034, is restricted to lower-cased letters, and has a\nmaximum length of 63 characters. The id must match the regular expression:\n[a-z]([a-z0-9-]{0,61}[a-z0-9])?."]
    pub fn report_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nReport creation state.\nPossible values:\nPENDING\nSUCCEEDED\nFAILED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `summary` after provisioning.\nDescribes the Summary view of a Report, which contains aggregated values\nfor all the groups and preference sets included in this Report."]
    pub fn summary(&self) -> terrars::ListRef<MigrationCenterReport_SummaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summary", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nReport type.\nPossible values:\nTOTAL_COST_OF_OWNERSHIP"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast update timestamp."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MigrationCenterReport_TimeoutsRef {
        MigrationCenterReport_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MigrationCenterReport {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MigrationCenterReport { }

impl terrars::ToListMappable for MigrationCenterReport {
    type O = terrars::ListRef<MigrationCenterReportRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MigrationCenterReport_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_migration_center_report".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMigrationCenterReport {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub report_config: terrars::PrimField<std::string::String>,
    #[doc =
        "User specified id for the report. It will become the last component\nof the report name. The id must be unique within the project, must\nconform with RFC-1034, is restricted to lower-cased letters, and has a\nmaximum length of 63 characters. The id must match the regular expression:\n[a-z]([a-z0-9-]{0,61}[a-z0-9])?."]
    pub report_id: terrars::PrimField<std::string::String>,
}

impl BuildMigrationCenterReport {
    pub fn build(self, stack: &mut terrars::Stack) -> MigrationCenterReport {
        let out = MigrationCenterReport(std::rc::Rc::new(MigrationCenterReport_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MigrationCenterReportData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                report_config: self.report_config,
                report_id: self.report_id,
                type_: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MigrationCenterReportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReportRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MigrationCenterReportRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation timestamp."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nFree-text description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly display name. Maximum length is 63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `report_config` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn report_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `report_id` after provisioning.\nUser specified id for the report. It will become the last component\nof the report name. The id must be unique within the project, must\nconform with RFC-1034, is restricted to lower-cased letters, and has a\nmaximum length of 63 characters. The id must match the regular expression:\n[a-z]([a-z0-9-]{0,61}[a-z0-9])?."]
    pub fn report_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nReport creation state.\nPossible values:\nPENDING\nSUCCEEDED\nFAILED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `summary` after provisioning.\nDescribes the Summary view of a Report, which contains aggregated values\nfor all the groups and preference sets included in this Report."]
    pub fn summary(&self) -> terrars::ListRef<MigrationCenterReport_SummaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summary", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nReport type.\nPossible values:\nTOTAL_COST_OF_OWNERSHIP"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast update timestamp."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MigrationCenterReport_TimeoutsRef {
        MigrationCenterReport_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lower_bound: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upper_bound: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lower_bound`.\n"]
    pub fn set_lower_bound(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lower_bound = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upper_bound`.\n"]
    pub fn set_upper_bound(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.upper_bound = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets {}

impl BuildMigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets {
    pub fn build(self) -> MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets {
        MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets {
            count: core::default::Default::default(),
            lower_bound: core::default::Default::default(),
            upper_bound: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_BucketsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_BucketsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_BucketsRef {
        MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_BucketsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_BucketsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `lower_bound` after provisioning.\n"]
    pub fn lower_bound(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower_bound", self.base))
    }

    #[doc = "Get a reference to the value of field `upper_bound` after provisioning.\n"]
    pub fn upper_bound(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper_bound", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buckets: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets>>,
}

impl MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram {
    #[doc = "Set the field `buckets`.\n"]
    pub fn set_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets>,
                    >,
    ) -> Self {
        self.buckets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram {}

impl BuildMigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram {
    pub fn build(self) -> MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram {
        MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram {
            buckets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogramRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogramRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogramRef {
        MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogramRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogramRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_BucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buckets: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets>>,
}

impl MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram {
    #[doc = "Set the field `buckets`.\n"]
    pub fn set_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets>,
                    >,
    ) -> Self {
        self.buckets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram {}

impl BuildMigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram {
    pub fn build(self) -> MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram {
        MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram {
            buckets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogramRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogramRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogramRef {
        MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogramRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogramRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_BucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    free: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    used: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart {
    #[doc = "Set the field `free`.\n"]
    pub fn set_free(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `used`.\n"]
    pub fn set_used(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.used = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart {}

impl BuildMigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart {
    pub fn build(self) -> MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart {
        MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart {
            free: core::default::Default::default(),
            used: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChartRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChartRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChartRef {
        MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChartRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChartRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `free` after provisioning.\n"]
    pub fn free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.free", self.base))
    }

    #[doc = "Get a reference to the value of field `used` after provisioning.\n"]
    pub fn used(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.used", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints {
    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints {}

impl BuildMigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints {
    pub fn build(self) -> MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints {
        MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints {
            label: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPointsRef {
        MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_points: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints>>,
}

impl MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem {
    #[doc = "Set the field `data_points`.\n"]
    pub fn set_data_points(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints>,
                    >,
    ) -> Self {
        self.data_points = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_AllAssetsStats_OperatingSystem {}

impl BuildMigrationCenterReport_Summary_AllAssetsStats_OperatingSystem {
    pub fn build(self) -> MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem {
        MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem {
            data_points: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_OperatingSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_AllAssetsStats_OperatingSystemRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_AllAssetsStats_OperatingSystemRef {
        MigrationCenterReport_Summary_AllAssetsStats_OperatingSystemRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_AllAssetsStats_OperatingSystemRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_points` after provisioning.\n"]
    pub fn data_points(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_points", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_points: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints>>,
}

impl MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances {
    #[doc = "Set the field `data_points`.\n"]
    pub fn set_data_points(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPoints>,
                    >,
    ) -> Self {
        self.data_points = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances {}

impl BuildMigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances {
    pub fn build(self) -> MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances {
        MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances {
            data_points: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstancesRef {
        MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstancesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_points` after provisioning.\n"]
    pub fn data_points(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem_DataPointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_points", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buckets: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets>>,
}

impl MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram {
    #[doc = "Set the field `buckets`.\n"]
    pub fn set_buckets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_Buckets>,
                    >,
    ) -> Self {
        self.buckets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram {}

impl BuildMigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram {
    pub fn build(self) -> MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram {
        MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram {
            buckets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogramRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogramRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogramRef {
        MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogramRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogramRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram_BucketsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    free: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    used: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart {
    #[doc = "Set the field `free`.\n"]
    pub fn set_free(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.free = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `used`.\n"]
    pub fn set_used(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.used = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart {}

impl BuildMigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart {
    pub fn build(self) -> MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart {
        MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart {
            free: core::default::Default::default(),
            used: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChartRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChartRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChartRef {
        MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChartRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChartRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `free` after provisioning.\n"]
    pub fn free(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.free", self.base))
    }

    #[doc = "Get a reference to the value of field `used` after provisioning.\n"]
    pub fn used(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.used", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_AllAssetsStats {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_count_histogram: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_bytes_histogram: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_utilization_chart: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_instances: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_bytes_histogram: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_utilization_chart: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_assets: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_cores: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_memory_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_storage_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_AllAssetsStats {
    #[doc = "Set the field `core_count_histogram`.\n"]
    pub fn set_core_count_histogram(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram>>,
    ) -> Self {
        self.core_count_histogram = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_bytes_histogram`.\n"]
    pub fn set_memory_bytes_histogram(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram>>,
    ) -> Self {
        self.memory_bytes_histogram = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_utilization_chart`.\n"]
    pub fn set_memory_utilization_chart(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart>>,
    ) -> Self {
        self.memory_utilization_chart = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system`.\n"]
    pub fn set_operating_system(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem>>,
    ) -> Self {
        self.operating_system = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `software_instances`.\n"]
    pub fn set_software_instances(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances>>,
    ) -> Self {
        self.software_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_bytes_histogram`.\n"]
    pub fn set_storage_bytes_histogram(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram>>,
    ) -> Self {
        self.storage_bytes_histogram = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_utilization_chart`.\n"]
    pub fn set_storage_utilization_chart(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart>>,
    ) -> Self {
        self.storage_utilization_chart = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_assets`.\n"]
    pub fn set_total_assets(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.total_assets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_cores`.\n"]
    pub fn set_total_cores(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.total_cores = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_memory_bytes`.\n"]
    pub fn set_total_memory_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_memory_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_storage_bytes`.\n"]
    pub fn set_total_storage_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_storage_bytes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_AllAssetsStats {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_AllAssetsStats>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_AllAssetsStats {}

impl BuildMigrationCenterReport_Summary_AllAssetsStats {
    pub fn build(self) -> MigrationCenterReport_Summary_AllAssetsStats {
        MigrationCenterReport_Summary_AllAssetsStats {
            core_count_histogram: core::default::Default::default(),
            memory_bytes_histogram: core::default::Default::default(),
            memory_utilization_chart: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            software_instances: core::default::Default::default(),
            storage_bytes_histogram: core::default::Default::default(),
            storage_utilization_chart: core::default::Default::default(),
            total_assets: core::default::Default::default(),
            total_cores: core::default::Default::default(),
            total_memory_bytes: core::default::Default::default(),
            total_storage_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_AllAssetsStatsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_AllAssetsStatsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_AllAssetsStatsRef {
        MigrationCenterReport_Summary_AllAssetsStatsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_AllAssetsStatsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `core_count_histogram` after provisioning.\n"]
    pub fn core_count_histogram(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogramRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.core_count_histogram", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_bytes_histogram` after provisioning.\n"]
    pub fn memory_bytes_histogram(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogramRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_bytes_histogram", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_utilization_chart` after provisioning.\n"]
    pub fn memory_utilization_chart(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChartRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_utilization_chart", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystemRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operating_system", self.base))
    }

    #[doc = "Get a reference to the value of field `software_instances` after provisioning.\n"]
    pub fn software_instances(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_bytes_histogram` after provisioning.\n"]
    pub fn storage_bytes_histogram(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogramRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_bytes_histogram", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_utilization_chart` after provisioning.\n"]
    pub fn storage_utilization_chart(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChartRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_utilization_chart", self.base))
    }

    #[doc = "Get a reference to the value of field `total_assets` after provisioning.\n"]
    pub fn total_assets(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_assets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_cores` after provisioning.\n"]
    pub fn total_cores(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_cores", self.base))
    }

    #[doc = "Get a reference to the value of field `total_memory_bytes` after provisioning.\n"]
    pub fn total_memory_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_memory_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `total_storage_bytes` after provisioning.\n"]
    pub fn total_storage_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_AssetAggregateStats {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_count_histogram: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_bytes_histogram: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_utilization_chart: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_instances: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_bytes_histogram: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_utilization_chart: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_assets: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_cores: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_memory_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_storage_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_AssetAggregateStats {
    #[doc = "Set the field `core_count_histogram`.\n"]
    pub fn set_core_count_histogram(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogram>>,
    ) -> Self {
        self.core_count_histogram = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_bytes_histogram`.\n"]
    pub fn set_memory_bytes_histogram(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogram>>,
    ) -> Self {
        self.memory_bytes_histogram = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_utilization_chart`.\n"]
    pub fn set_memory_utilization_chart(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChart>>,
    ) -> Self {
        self.memory_utilization_chart = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system`.\n"]
    pub fn set_operating_system(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystem>>,
    ) -> Self {
        self.operating_system = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `software_instances`.\n"]
    pub fn set_software_instances(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstances>>,
    ) -> Self {
        self.software_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_bytes_histogram`.\n"]
    pub fn set_storage_bytes_histogram(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogram>>,
    ) -> Self {
        self.storage_bytes_histogram = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_utilization_chart`.\n"]
    pub fn set_storage_utilization_chart(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChart>>,
    ) -> Self {
        self.storage_utilization_chart = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_assets`.\n"]
    pub fn set_total_assets(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.total_assets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_cores`.\n"]
    pub fn set_total_cores(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.total_cores = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_memory_bytes`.\n"]
    pub fn set_total_memory_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_memory_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_storage_bytes`.\n"]
    pub fn set_total_storage_bytes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_storage_bytes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_GroupFindings_AssetAggregateStats {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_AssetAggregateStats>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_AssetAggregateStats {}

impl BuildMigrationCenterReport_Summary_GroupFindings_AssetAggregateStats {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_AssetAggregateStats {
        MigrationCenterReport_Summary_GroupFindings_AssetAggregateStats {
            core_count_histogram: core::default::Default::default(),
            memory_bytes_histogram: core::default::Default::default(),
            memory_utilization_chart: core::default::Default::default(),
            operating_system: core::default::Default::default(),
            software_instances: core::default::Default::default(),
            storage_bytes_histogram: core::default::Default::default(),
            storage_utilization_chart: core::default::Default::default(),
            total_assets: core::default::Default::default(),
            total_cores: core::default::Default::default(),
            total_memory_bytes: core::default::Default::default(),
            total_storage_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_AssetAggregateStatsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_AssetAggregateStatsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_AssetAggregateStatsRef {
        MigrationCenterReport_Summary_GroupFindings_AssetAggregateStatsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_AssetAggregateStatsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `core_count_histogram` after provisioning.\n"]
    pub fn core_count_histogram(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_CoreCountHistogramRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.core_count_histogram", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_bytes_histogram` after provisioning.\n"]
    pub fn memory_bytes_histogram(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_MemoryBytesHistogramRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_bytes_histogram", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_utilization_chart` after provisioning.\n"]
    pub fn memory_utilization_chart(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_MemoryUtilizationChartRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_utilization_chart", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_OperatingSystemRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operating_system", self.base))
    }

    #[doc = "Get a reference to the value of field `software_instances` after provisioning.\n"]
    pub fn software_instances(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_SoftwareInstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_bytes_histogram` after provisioning.\n"]
    pub fn storage_bytes_histogram(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_StorageBytesHistogramRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_bytes_histogram", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_utilization_chart` after provisioning.\n"]
    pub fn storage_utilization_chart(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStats_StorageUtilizationChartRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_utilization_chart", self.base))
    }

    #[doc = "Get a reference to the value of field `total_assets` after provisioning.\n"]
    pub fn total_assets(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_assets", self.base))
    }

    #[doc = "Get a reference to the value of field `total_cores` after provisioning.\n"]
    pub fn total_cores(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_cores", self.base))
    }

    #[doc = "Get a reference to the value of field `total_memory_bytes` after provisioning.\n"]
    pub fn total_memory_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_memory_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `total_storage_bytes` after provisioning.\n"]
    pub fn total_storage_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeries {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeries {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeries,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeries {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeries {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeries {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeries {
            code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeriesRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_asset_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_series: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeries,
        >,
    >,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations {
    #[doc = "Set the field `allocated_asset_count`.\n"]
    pub fn set_allocated_asset_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_asset_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_series`.\n"]
    pub fn set_machine_series(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeries,
                        >,
                    >,
    ) -> Self {
        self.machine_series = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations {
            allocated_asset_count: core::default::Default::default(),
            machine_series: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocationsRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_asset_count` after provisioning.\n"]
    pub fn allocated_asset_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_asset_count", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_series` after provisioning.\n"]
    pub fn machine_series(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations_MachineSeriesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_series", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_asset_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_disk_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_series_allocations: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations,
        >,
    >,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding {
    #[doc = "Set the field `allocated_asset_count`.\n"]
    pub fn set_allocated_asset_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_asset_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allocated_disk_types`.\n"]
    pub fn set_allocated_disk_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allocated_disk_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allocated_regions`.\n"]
    pub fn set_allocated_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allocated_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_series_allocations`.\n"]
    pub fn set_machine_series_allocations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocations,
                        >,
                    >,
    ) -> Self {
        self.machine_series_allocations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding {
            allocated_asset_count: core::default::Default::default(),
            allocated_disk_types: core::default::Default::default(),
            allocated_regions: core::default::Default::default(),
            machine_series_allocations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFindingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFindingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFindingRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFindingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFindingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_asset_count` after provisioning.\n"]
    pub fn allocated_asset_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_asset_count", self.base))
    }

    #[doc = "Get a reference to the value of field `allocated_disk_types` after provisioning.\n"]
    pub fn allocated_disk_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allocated_disk_types", self.base))
    }

    #[doc = "Get a reference to the value of field `allocated_regions` after provisioning.\n"]
    pub fn allocated_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allocated_regions", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_series_allocations` after provisioning.\n"]
    pub fn machine_series_allocations(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding_MachineSeriesAllocationsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_series_allocations", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
            code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_machine_series: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries,
        >,
    >,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences {
    #[doc = "Set the field `allowed_machine_series`.\n"]
    pub fn set_allowed_machine_series(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries,
                        >,
                    >,
    ) -> Self {
        self.allowed_machine_series = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences {
            allowed_machine_series: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferencesRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_machine_series` after provisioning.\n"]
    pub fn allowed_machine_series(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_machine_series", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_preferences: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistent_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences {
    #[doc = "Set the field `license_type`.\n"]
    pub fn set_license_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.license_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_preferences`.\n"]
    pub fn set_machine_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferences,
                        >,
                    >,
    ) -> Self {
        self.machine_preferences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `persistent_disk_type`.\n"]
    pub fn set_persistent_disk_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.persistent_disk_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences {
            license_type: core::default::Default::default(),
            machine_preferences: core::default::Default::default(),
            persistent_disk_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferencesRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `license_type` after provisioning.\n"]
    pub fn license_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_type", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_preferences` after provisioning.\n"]
    pub fn machine_preferences(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences_MachinePreferencesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_preferences", self.base))
    }

    #[doc = "Get a reference to the value of field `persistent_disk_type` after provisioning.\n"]
    pub fn persistent_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persistent_disk_type", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferences {
    #[doc = "Set the field `preferred_regions`.\n"]
    pub fn set_preferred_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.preferred_regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferences {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferences,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferences {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferences {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferences {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferences {
            preferred_regions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferencesRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `preferred_regions` after provisioning.\n"]
    pub fn preferred_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_regions", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypes {
    #[doc = "Set the field `node_name`.\n"]
    pub fn set_node_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypes {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypes {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypes {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypes {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypes {
            node_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypesRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_name` after provisioning.\n"]
    pub fn node_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_name", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commitment_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_overcommit_ratio: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_maintenance_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_types: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypes,
        >,
    >,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences {
    #[doc = "Set the field `commitment_plan`.\n"]
    pub fn set_commitment_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commitment_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_overcommit_ratio`.\n"]
    pub fn set_cpu_overcommit_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_overcommit_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_maintenance_policy`.\n"]
    pub fn set_host_maintenance_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.host_maintenance_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_types`.\n"]
    pub fn set_node_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypes,
                        >,
                    >,
    ) -> Self {
        self.node_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences {
            commitment_plan: core::default::Default::default(),
            cpu_overcommit_ratio: core::default::Default::default(),
            host_maintenance_policy: core::default::Default::default(),
            node_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferencesRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commitment_plan` after provisioning.\n"]
    pub fn commitment_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment_plan", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_overcommit_ratio` after provisioning.\n"]
    pub fn cpu_overcommit_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_overcommit_ratio", self.base))
    }

    #[doc = "Get a reference to the value of field `host_maintenance_policy` after provisioning.\n"]
    pub fn host_maintenance_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_maintenance_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `node_types` after provisioning.\n"]
    pub fn node_types(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences_NodeTypesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_types", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commitment_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_overcommit_ratio: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_overcommit_ratio: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_deduplication_compression_ratio: std::option::Option<terrars::PrimField<f64>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferences {
    #[doc = "Set the field `commitment_plan`.\n"]
    pub fn set_commitment_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commitment_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_overcommit_ratio`.\n"]
    pub fn set_cpu_overcommit_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_overcommit_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_overcommit_ratio`.\n"]
    pub fn set_memory_overcommit_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_overcommit_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_deduplication_compression_ratio`.\n"]
    pub fn set_storage_deduplication_compression_ratio(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.storage_deduplication_compression_ratio = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferences {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferences,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferences {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferences {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferences {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferences {
            commitment_plan: core::default::Default::default(),
            cpu_overcommit_ratio: core::default::Default::default(),
            memory_overcommit_ratio: core::default::Default::default(),
            storage_deduplication_compression_ratio: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferencesRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commitment_plan` after provisioning.\n"]
    pub fn commitment_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment_plan", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_overcommit_ratio` after provisioning.\n"]
    pub fn cpu_overcommit_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_overcommit_ratio", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_overcommit_ratio` after provisioning.\n"]
    pub fn memory_overcommit_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_overcommit_ratio", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_deduplication_compression_ratio` after provisioning.\n"]
    pub fn storage_deduplication_compression_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_deduplication_compression_ratio", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commitment_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_engine_preferences: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_preferences: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferences,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sizing_optimization_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sole_tenancy_preferences: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_product: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vmware_engine_preferences: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferences,
        >,
    >,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences {
    #[doc = "Set the field `commitment_plan`.\n"]
    pub fn set_commitment_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commitment_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_engine_preferences`.\n"]
    pub fn set_compute_engine_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferences,
                        >,
                    >,
    ) -> Self {
        self.compute_engine_preferences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_preferences`.\n"]
    pub fn set_region_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferences,
                        >,
                    >,
    ) -> Self {
        self.region_preferences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sizing_optimization_strategy`.\n"]
    pub fn set_sizing_optimization_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sizing_optimization_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sole_tenancy_preferences`.\n"]
    pub fn set_sole_tenancy_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferences,
                        >,
                    >,
    ) -> Self {
        self.sole_tenancy_preferences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_product`.\n"]
    pub fn set_target_product(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_product = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vmware_engine_preferences`.\n"]
    pub fn set_vmware_engine_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferences,
                        >,
                    >,
    ) -> Self {
        self.vmware_engine_preferences = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences {
            commitment_plan: core::default::Default::default(),
            compute_engine_preferences: core::default::Default::default(),
            region_preferences: core::default::Default::default(),
            sizing_optimization_strategy: core::default::Default::default(),
            sole_tenancy_preferences: core::default::Default::default(),
            target_product: core::default::Default::default(),
            vmware_engine_preferences: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferencesRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commitment_plan` after provisioning.\n"]
    pub fn commitment_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment_plan", self.base))
    }

    #[doc = "Get a reference to the value of field `compute_engine_preferences` after provisioning.\n"]
    pub fn compute_engine_preferences(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_ComputeEnginePreferencesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_engine_preferences", self.base))
    }

    #[doc = "Get a reference to the value of field `region_preferences` after provisioning.\n"]
    pub fn region_preferences(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_RegionPreferencesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.region_preferences", self.base))
    }

    #[doc = "Get a reference to the value of field `sizing_optimization_strategy` after provisioning.\n"]
    pub fn sizing_optimization_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sizing_optimization_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `sole_tenancy_preferences` after provisioning.\n"]
    pub fn sole_tenancy_preferences(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_SoleTenancyPreferencesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sole_tenancy_preferences", self.base))
    }

    #[doc = "Get a reference to the value of field `target_product` after provisioning.\n"]
    pub fn target_product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_product", self.base))
    }

    #[doc = "Get a reference to the value of field `vmware_engine_preferences` after provisioning.\n"]
    pub fn vmware_engine_preferences(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences_VmwareEnginePreferencesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vmware_engine_preferences", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostCompute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    units: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostCompute {
    #[doc = "Set the field `currency_code`.\n"]
    pub fn set_currency_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `units`.\n"]
    pub fn set_units(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostCompute {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostCompute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostCompute {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostCompute {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostCompute {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostCompute {
            currency_code: core::default::Default::default(),
            nanos: core::default::Default::default(),
            units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostComputeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostComputeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostComputeRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostComputeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostComputeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\n"]
    pub fn currency_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency_code", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `units` after provisioning.\n"]
    pub fn units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    units: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgress {
    #[doc = "Set the field `currency_code`.\n"]
    pub fn set_currency_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `units`.\n"]
    pub fn set_units(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgress {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgress {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgress {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgress {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgress {
            currency_code: core::default::Default::default(),
            nanos: core::default::Default::default(),
            units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgressRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgressRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\n"]
    pub fn currency_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency_code", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `units` after provisioning.\n"]
    pub fn units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicense {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    units: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicense {
    #[doc = "Set the field `currency_code`.\n"]
    pub fn set_currency_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `units`.\n"]
    pub fn set_units(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicense {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicense>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicense {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicense {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicense {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicense {
            currency_code: core::default::Default::default(),
            nanos: core::default::Default::default(),
            units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicenseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicenseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicenseRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicenseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicenseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\n"]
    pub fn currency_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency_code", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `units` after provisioning.\n"]
    pub fn units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOther {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    units: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOther {
    #[doc = "Set the field `currency_code`.\n"]
    pub fn set_currency_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `units`.\n"]
    pub fn set_units(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOther {
    type O =
        terrars::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOther>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOther {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOther {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOther {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOther {
            currency_code: core::default::Default::default(),
            nanos: core::default::Default::default(),
            units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOtherRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOtherRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOtherRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOtherRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOtherRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\n"]
    pub fn currency_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency_code", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `units` after provisioning.\n"]
    pub fn units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    units: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorage {
    #[doc = "Set the field `currency_code`.\n"]
    pub fn set_currency_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `units`.\n"]
    pub fn set_units(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorage {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorage {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorage {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorage {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorage {
            currency_code: core::default::Default::default(),
            nanos: core::default::Default::default(),
            units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorageRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\n"]
    pub fn currency_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency_code", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `units` after provisioning.\n"]
    pub fn units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    units: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotal {
    #[doc = "Set the field `currency_code`.\n"]
    pub fn set_currency_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `units`.\n"]
    pub fn set_units(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotal {
    type O =
        terrars::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotal {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotal {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotal {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotal {
            currency_code: core::default::Default::default(),
            nanos: core::default::Default::default(),
            units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotalRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\n"]
    pub fn currency_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency_code", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `units` after provisioning.\n"]
    pub fn units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_Node {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_Node {
    #[doc = "Set the field `node_name`.\n"]
    pub fn set_node_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_Node {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_Node,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_Node {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_Node {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_Node {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_Node {
            node_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_NodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_NodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_NodeRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_NodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_NodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_name` after provisioning.\n"]
    pub fn node_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_name", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_asset_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_Node,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations {
    #[doc = "Set the field `allocated_asset_count`.\n"]
    pub fn set_allocated_asset_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_asset_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node`.\n"]
    pub fn set_node(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_Node,
                        >,
                    >,
    ) -> Self {
        self.node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations {
            allocated_asset_count: core::default::Default::default(),
            node: core::default::Default::default(),
            node_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocationsRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_asset_count` after provisioning.\n"]
    pub fn allocated_asset_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_asset_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node` after provisioning.\n"]
    pub fn node(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations_NodeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_asset_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_allocations: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations,
        >,
    >,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding {
    #[doc = "Set the field `allocated_asset_count`.\n"]
    pub fn set_allocated_asset_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_asset_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allocated_regions`.\n"]
    pub fn set_allocated_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allocated_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_allocations`.\n"]
    pub fn set_node_allocations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocations,
                        >,
                    >,
    ) -> Self {
        self.node_allocations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding {
            allocated_asset_count: core::default::Default::default(),
            allocated_regions: core::default::Default::default(),
            node_allocations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFindingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFindingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFindingRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFindingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFindingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_asset_count` after provisioning.\n"]
    pub fn allocated_asset_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_asset_count", self.base))
    }

    #[doc = "Get a reference to the value of field `allocated_regions` after provisioning.\n"]
    pub fn allocated_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allocated_regions", self.base))
    }

    #[doc = "Get a reference to the value of field `node_allocations` after provisioning.\n"]
    pub fn node_allocations(
        &self,
    ) -> terrars
    ::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding_NodeAllocationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_allocations", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNode {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNode {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNode,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNode {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNode {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNode {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNode {
            code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNodeRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_asset_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vmware_node: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNode,
        >,
    >,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations {
    #[doc = "Set the field `allocated_asset_count`.\n"]
    pub fn set_allocated_asset_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_asset_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vmware_node`.\n"]
    pub fn set_vmware_node(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNode,
                        >,
                    >,
    ) -> Self {
        self.vmware_node = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations {
    pub fn build(
        self,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations {
            allocated_asset_count: core::default::Default::default(),
            node_count: core::default::Default::default(),
            vmware_node: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocationsRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_asset_count` after provisioning.\n"]
    pub fn allocated_asset_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_asset_count", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `vmware_node` after provisioning.\n"]
    pub fn vmware_node(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations_VmwareNodeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vmware_node", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_asset_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_allocations: std
    ::option
    ::Option<
        terrars
        ::ListField<
            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations,
        >,
    >,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding {
    #[doc = "Set the field `allocated_asset_count`.\n"]
    pub fn set_allocated_asset_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_asset_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allocated_regions`.\n"]
    pub fn set_allocated_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allocated_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_allocations`.\n"]
    pub fn set_node_allocations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocations,
                        >,
                    >,
    ) -> Self {
        self.node_allocations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding {
            allocated_asset_count: core::default::Default::default(),
            allocated_regions: core::default::Default::default(),
            node_allocations: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFindingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFindingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFindingRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFindingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFindingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_asset_count` after provisioning.\n"]
    pub fn allocated_asset_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_asset_count", self.base))
    }

    #[doc = "Get a reference to the value of field `allocated_regions` after provisioning.\n"]
    pub fn allocated_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allocated_regions", self.base))
    }

    #[doc = "Get a reference to the value of field `node_allocations` after provisioning.\n"]
    pub fn node_allocations(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding_NodeAllocationsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_allocations", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_engine_finding: std
    ::option
    ::Option<
        terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_preferences: std
    ::option
    ::Option<
        terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_cost_compute: std
    ::option
    ::Option<
        terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostCompute>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_cost_network_egress: std
    ::option
    ::Option<
        terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgress>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_cost_os_license: std
    ::option
    ::Option<
        terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicense>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_cost_other: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOther>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_cost_storage: std
    ::option
    ::Option<
        terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorage>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_cost_total: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotal>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sole_tenant_finding: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vmware_engine_finding: std
    ::option
    ::Option<
        terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding>,
    >,
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings {
    #[doc = "Set the field `compute_engine_finding`.\n"]
    pub fn set_compute_engine_finding(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFinding,
                        >,
                    >,
    ) -> Self {
        self.compute_engine_finding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_preferences`.\n"]
    pub fn set_machine_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferences,
                        >,
                    >,
    ) -> Self {
        self.machine_preferences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_cost_compute`.\n"]
    pub fn set_monthly_cost_compute(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostCompute,
                        >,
                    >,
    ) -> Self {
        self.monthly_cost_compute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_cost_network_egress`.\n"]
    pub fn set_monthly_cost_network_egress(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgress,
                        >,
                    >,
    ) -> Self {
        self.monthly_cost_network_egress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_cost_os_license`.\n"]
    pub fn set_monthly_cost_os_license(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicense,
                        >,
                    >,
    ) -> Self {
        self.monthly_cost_os_license = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_cost_other`.\n"]
    pub fn set_monthly_cost_other(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOther,
                        >,
                    >,
    ) -> Self {
        self.monthly_cost_other = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_cost_storage`.\n"]
    pub fn set_monthly_cost_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorage,
                        >,
                    >,
    ) -> Self {
        self.monthly_cost_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_cost_total`.\n"]
    pub fn set_monthly_cost_total(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotal,
                        >,
                    >,
    ) -> Self {
        self.monthly_cost_total = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sole_tenant_finding`.\n"]
    pub fn set_sole_tenant_finding(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFinding,
                        >,
                    >,
    ) -> Self {
        self.sole_tenant_finding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vmware_engine_finding`.\n"]
    pub fn set_vmware_engine_finding(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFinding,
                        >,
                    >,
    ) -> Self {
        self.vmware_engine_finding = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings {}

impl BuildMigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings {
            compute_engine_finding: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            machine_preferences: core::default::Default::default(),
            monthly_cost_compute: core::default::Default::default(),
            monthly_cost_network_egress: core::default::Default::default(),
            monthly_cost_os_license: core::default::Default::default(),
            monthly_cost_other: core::default::Default::default(),
            monthly_cost_storage: core::default::Default::default(),
            monthly_cost_total: core::default::Default::default(),
            sole_tenant_finding: core::default::Default::default(),
            vmware_engine_finding: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindingsRef {
        MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compute_engine_finding` after provisioning.\n"]
    pub fn compute_engine_finding(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_ComputeEngineFindingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_engine_finding", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_preferences` after provisioning.\n"]
    pub fn machine_preferences(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MachinePreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_preferences", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_cost_compute` after provisioning.\n"]
    pub fn monthly_cost_compute(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostComputeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly_cost_compute", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_cost_network_egress` after provisioning.\n"]
    pub fn monthly_cost_network_egress(
        &self,
    ) -> terrars
    ::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostNetworkEgressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly_cost_network_egress", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_cost_os_license` after provisioning.\n"]
    pub fn monthly_cost_os_license(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOsLicenseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly_cost_os_license", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_cost_other` after provisioning.\n"]
    pub fn monthly_cost_other(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostOtherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly_cost_other", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_cost_storage` after provisioning.\n"]
    pub fn monthly_cost_storage(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly_cost_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_cost_total` after provisioning.\n"]
    pub fn monthly_cost_total(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_MonthlyCostTotalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly_cost_total", self.base))
    }

    #[doc = "Get a reference to the value of field `sole_tenant_finding` after provisioning.\n"]
    pub fn sole_tenant_finding(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_SoleTenantFindingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sole_tenant_finding", self.base))
    }

    #[doc = "Get a reference to the value of field `vmware_engine_finding` after provisioning.\n"]
    pub fn vmware_engine_finding(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings_VmwareEngineFindingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vmware_engine_finding", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary_GroupFindings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset_aggregate_stats: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_GroupFindings_AssetAggregateStats>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overlapping_asset_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preference_set_findings: std
    ::option
    ::Option<terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings>>,
}

impl MigrationCenterReport_Summary_GroupFindings {
    #[doc = "Set the field `asset_aggregate_stats`.\n"]
    pub fn set_asset_aggregate_stats(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_GroupFindings_AssetAggregateStats>>,
    ) -> Self {
        self.asset_aggregate_stats = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overlapping_asset_count`.\n"]
    pub fn set_overlapping_asset_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.overlapping_asset_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preference_set_findings`.\n"]
    pub fn set_preference_set_findings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindings>>,
    ) -> Self {
        self.preference_set_findings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary_GroupFindings {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary_GroupFindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary_GroupFindings {}

impl BuildMigrationCenterReport_Summary_GroupFindings {
    pub fn build(self) -> MigrationCenterReport_Summary_GroupFindings {
        MigrationCenterReport_Summary_GroupFindings {
            asset_aggregate_stats: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            overlapping_asset_count: core::default::Default::default(),
            preference_set_findings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_Summary_GroupFindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_Summary_GroupFindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterReport_Summary_GroupFindingsRef {
        MigrationCenterReport_Summary_GroupFindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_Summary_GroupFindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asset_aggregate_stats` after provisioning.\n"]
    pub fn asset_aggregate_stats(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_AssetAggregateStatsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asset_aggregate_stats", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `overlapping_asset_count` after provisioning.\n"]
    pub fn overlapping_asset_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overlapping_asset_count", self.base))
    }

    #[doc = "Get a reference to the value of field `preference_set_findings` after provisioning.\n"]
    pub fn preference_set_findings(
        &self,
    ) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindings_PreferenceSetFindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preference_set_findings", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Summary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_assets_stats: std::option::Option<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_findings: std::option::Option<terrars::ListField<MigrationCenterReport_Summary_GroupFindings>>,
}

impl MigrationCenterReport_Summary {
    #[doc = "Set the field `all_assets_stats`.\n"]
    pub fn set_all_assets_stats(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterReport_Summary_AllAssetsStats>>,
    ) -> Self {
        self.all_assets_stats = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_findings`.\n"]
    pub fn set_group_findings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<MigrationCenterReport_Summary_GroupFindings>>,
    ) -> Self {
        self.group_findings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterReport_Summary {
    type O = terrars::BlockAssignable<MigrationCenterReport_Summary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Summary {}

impl BuildMigrationCenterReport_Summary {
    pub fn build(self) -> MigrationCenterReport_Summary {
        MigrationCenterReport_Summary {
            all_assets_stats: core::default::Default::default(),
            group_findings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_SummaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_SummaryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MigrationCenterReport_SummaryRef {
        MigrationCenterReport_SummaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_SummaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_assets_stats` after provisioning.\n"]
    pub fn all_assets_stats(&self) -> terrars::ListRef<MigrationCenterReport_Summary_AllAssetsStatsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_assets_stats", self.base))
    }

    #[doc = "Get a reference to the value of field `group_findings` after provisioning.\n"]
    pub fn group_findings(&self) -> terrars::ListRef<MigrationCenterReport_Summary_GroupFindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_findings", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterReport_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterReport_Timeouts {
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
}

impl terrars::ToListMappable for MigrationCenterReport_Timeouts {
    type O = terrars::BlockAssignable<MigrationCenterReport_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterReport_Timeouts {}

impl BuildMigrationCenterReport_Timeouts {
    pub fn build(self) -> MigrationCenterReport_Timeouts {
        MigrationCenterReport_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterReport_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterReport_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MigrationCenterReport_TimeoutsRef {
        MigrationCenterReport_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterReport_TimeoutsRef {
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
}
