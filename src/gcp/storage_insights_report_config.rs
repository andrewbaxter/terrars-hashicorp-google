#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageInsightsReportConfigData_ {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_options: std::option::Option<std::vec::Vec<StorageInsightsReportConfig_CsvOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frequency_options: std::option::Option<std::vec::Vec<StorageInsightsReportConfig_FrequencyOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_metadata_report_options: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsReportConfig_ObjectMetadataReportOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parquet_options: std::option::Option<std::vec::Vec<StorageInsightsReportConfig_ParquetOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StorageInsightsReportConfig_Timeouts>,
    dynamic: StorageInsightsReportConfigDynamic,
}

struct StorageInsightsReportConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StorageInsightsReportConfigData_>,
}

#[derive(Clone)]
pub struct StorageInsightsReportConfig(std::rc::Rc<StorageInsightsReportConfig_>);

impl StorageInsightsReportConfig {
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
        "Set the field `display_name`.\nThe editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_destroy`.\nIf set, all the inventory report details associated with this report configuration are deleted."]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsReportConfig_CsvOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().csv_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.csv_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `frequency_options`.\n"]
    pub fn set_frequency_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsReportConfig_FrequencyOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().frequency_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.frequency_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `object_metadata_report_options`.\n"]
    pub fn set_object_metadata_report_options(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageInsightsReportConfig_ObjectMetadataReportOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().object_metadata_report_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.object_metadata_report_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parquet_options`.\n"]
    pub fn set_parquet_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsReportConfig_ParquetOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parquet_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parquet_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StorageInsightsReportConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nIf set, all the inventory report details associated with this report configuration are deleted."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the ReportConfig. The source and destination buckets specified in the ReportConfig\nmust be in the same location."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe UUID of the inventory report configuration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(&self) -> terrars::ListRef<StorageInsightsReportConfig_CsvOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `frequency_options` after provisioning.\n"]
    pub fn frequency_options(&self) -> terrars::ListRef<StorageInsightsReportConfig_FrequencyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frequency_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_metadata_report_options` after provisioning.\n"]
    pub fn object_metadata_report_options(
        &self,
    ) -> terrars::ListRef<StorageInsightsReportConfig_ObjectMetadataReportOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.object_metadata_report_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parquet_options` after provisioning.\n"]
    pub fn parquet_options(&self) -> terrars::ListRef<StorageInsightsReportConfig_ParquetOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parquet_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageInsightsReportConfig_TimeoutsRef {
        StorageInsightsReportConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for StorageInsightsReportConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StorageInsightsReportConfig { }

impl terrars::ToListMappable for StorageInsightsReportConfig {
    type O = terrars::ListRef<StorageInsightsReportConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StorageInsightsReportConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_storage_insights_report_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStorageInsightsReportConfig {
    pub tf_id: std::string::String,
    #[doc =
        "The location of the ReportConfig. The source and destination buckets specified in the ReportConfig\nmust be in the same location."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildStorageInsightsReportConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> StorageInsightsReportConfig {
        let out = StorageInsightsReportConfig(std::rc::Rc::new(StorageInsightsReportConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StorageInsightsReportConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                csv_options: core::default::Default::default(),
                frequency_options: core::default::Default::default(),
                object_metadata_report_options: core::default::Default::default(),
                parquet_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StorageInsightsReportConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StorageInsightsReportConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nIf set, all the inventory report details associated with this report configuration are deleted."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the ReportConfig. The source and destination buckets specified in the ReportConfig\nmust be in the same location."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe UUID of the inventory report configuration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(&self) -> terrars::ListRef<StorageInsightsReportConfig_CsvOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `frequency_options` after provisioning.\n"]
    pub fn frequency_options(&self) -> terrars::ListRef<StorageInsightsReportConfig_FrequencyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frequency_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_metadata_report_options` after provisioning.\n"]
    pub fn object_metadata_report_options(
        &self,
    ) -> terrars::ListRef<StorageInsightsReportConfig_ObjectMetadataReportOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.object_metadata_report_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parquet_options` after provisioning.\n"]
    pub fn parquet_options(&self) -> terrars::ListRef<StorageInsightsReportConfig_ParquetOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parquet_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageInsightsReportConfig_TimeoutsRef {
        StorageInsightsReportConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct StorageInsightsReportConfigDynamic {
    csv_options: std::option::Option<terrars::DynamicBlock<StorageInsightsReportConfig_CsvOptions>>,
    frequency_options: std::option::Option<terrars::DynamicBlock<StorageInsightsReportConfig_FrequencyOptions>>,
    object_metadata_report_options: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsReportConfig_ObjectMetadataReportOptions>>,
    parquet_options: std::option::Option<terrars::DynamicBlock<StorageInsightsReportConfig_ParquetOptions>>,
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfig_CsvOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_separator: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsReportConfig_CsvOptions {
    #[doc = "Set the field `delimiter`.\nThe delimiter used to separate the fields in the inventory report CSV file."]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `header_required`.\nThe boolean that indicates whether or not headers are included in the inventory report CSV file."]
    pub fn set_header_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.header_required = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `record_separator`.\nThe character used to separate the records in the inventory report CSV file."]
    pub fn set_record_separator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.record_separator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageInsightsReportConfig_CsvOptions {
    type O = terrars::BlockAssignable<StorageInsightsReportConfig_CsvOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfig_CsvOptions {}

impl BuildStorageInsightsReportConfig_CsvOptions {
    pub fn build(self) -> StorageInsightsReportConfig_CsvOptions {
        StorageInsightsReportConfig_CsvOptions {
            delimiter: core::default::Default::default(),
            header_required: core::default::Default::default(),
            record_separator: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsReportConfig_CsvOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfig_CsvOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageInsightsReportConfig_CsvOptionsRef {
        StorageInsightsReportConfig_CsvOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfig_CsvOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delimiter` after provisioning.\nThe delimiter used to separate the fields in the inventory report CSV file."]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_required` after provisioning.\nThe boolean that indicates whether or not headers are included in the inventory report CSV file."]
    pub fn header_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `record_separator` after provisioning.\nThe character used to separate the records in the inventory report CSV file."]
    pub fn record_separator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_separator", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfig_FrequencyOptions_EndDate {
    day: terrars::PrimField<f64>,
    month: terrars::PrimField<f64>,
    year: terrars::PrimField<f64>,
}

impl StorageInsightsReportConfig_FrequencyOptions_EndDate { }

impl terrars::ToListMappable for StorageInsightsReportConfig_FrequencyOptions_EndDate {
    type O = terrars::BlockAssignable<StorageInsightsReportConfig_FrequencyOptions_EndDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfig_FrequencyOptions_EndDate {
    #[doc = "The day of the month to stop generating inventory reports."]
    pub day: terrars::PrimField<f64>,
    #[doc = "The month to stop generating inventory reports."]
    pub month: terrars::PrimField<f64>,
    #[doc = "The year to stop generating inventory reports"]
    pub year: terrars::PrimField<f64>,
}

impl BuildStorageInsightsReportConfig_FrequencyOptions_EndDate {
    pub fn build(self) -> StorageInsightsReportConfig_FrequencyOptions_EndDate {
        StorageInsightsReportConfig_FrequencyOptions_EndDate {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsReportConfig_FrequencyOptions_EndDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfig_FrequencyOptions_EndDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfig_FrequencyOptions_EndDateRef {
        StorageInsightsReportConfig_FrequencyOptions_EndDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfig_FrequencyOptions_EndDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nThe day of the month to stop generating inventory reports."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nThe month to stop generating inventory reports."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nThe year to stop generating inventory reports"]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfig_FrequencyOptions_StartDate {
    day: terrars::PrimField<f64>,
    month: terrars::PrimField<f64>,
    year: terrars::PrimField<f64>,
}

impl StorageInsightsReportConfig_FrequencyOptions_StartDate { }

impl terrars::ToListMappable for StorageInsightsReportConfig_FrequencyOptions_StartDate {
    type O = terrars::BlockAssignable<StorageInsightsReportConfig_FrequencyOptions_StartDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfig_FrequencyOptions_StartDate {
    #[doc = "The day of the month to start generating inventory reports."]
    pub day: terrars::PrimField<f64>,
    #[doc = "The month to start generating inventory reports."]
    pub month: terrars::PrimField<f64>,
    #[doc = "The year to start generating inventory reports"]
    pub year: terrars::PrimField<f64>,
}

impl BuildStorageInsightsReportConfig_FrequencyOptions_StartDate {
    pub fn build(self) -> StorageInsightsReportConfig_FrequencyOptions_StartDate {
        StorageInsightsReportConfig_FrequencyOptions_StartDate {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsReportConfig_FrequencyOptions_StartDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfig_FrequencyOptions_StartDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfig_FrequencyOptions_StartDateRef {
        StorageInsightsReportConfig_FrequencyOptions_StartDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfig_FrequencyOptions_StartDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nThe day of the month to start generating inventory reports."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nThe month to start generating inventory reports."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nThe year to start generating inventory reports"]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageInsightsReportConfig_FrequencyOptionsDynamic {
    end_date: std::option::Option<terrars::DynamicBlock<StorageInsightsReportConfig_FrequencyOptions_EndDate>>,
    start_date: std::option::Option<terrars::DynamicBlock<StorageInsightsReportConfig_FrequencyOptions_StartDate>>,
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfig_FrequencyOptions {
    frequency: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std::option::Option<std::vec::Vec<StorageInsightsReportConfig_FrequencyOptions_EndDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<std::vec::Vec<StorageInsightsReportConfig_FrequencyOptions_StartDate>>,
    dynamic: StorageInsightsReportConfig_FrequencyOptionsDynamic,
}

impl StorageInsightsReportConfig_FrequencyOptions {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsReportConfig_FrequencyOptions_EndDate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsReportConfig_FrequencyOptions_StartDate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageInsightsReportConfig_FrequencyOptions {
    type O = terrars::BlockAssignable<StorageInsightsReportConfig_FrequencyOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfig_FrequencyOptions {
    #[doc =
        "The frequency in which inventory reports are generated. Values are DAILY or WEEKLY. Possible values: [\"DAILY\", \"WEEKLY\"]"]
    pub frequency: terrars::PrimField<std::string::String>,
}

impl BuildStorageInsightsReportConfig_FrequencyOptions {
    pub fn build(self) -> StorageInsightsReportConfig_FrequencyOptions {
        StorageInsightsReportConfig_FrequencyOptions {
            frequency: self.frequency,
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsReportConfig_FrequencyOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfig_FrequencyOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfig_FrequencyOptionsRef {
        StorageInsightsReportConfig_FrequencyOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfig_FrequencyOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nThe frequency in which inventory reports are generated. Values are DAILY or WEEKLY. Possible values: [\"DAILY\", \"WEEKLY\"]"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::ListRef<StorageInsightsReportConfig_FrequencyOptions_EndDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::ListRef<StorageInsightsReportConfig_FrequencyOptions_StartDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions {
    #[doc =
        "Set the field `destination_path`.\nThe path within the destination bucket to store generated inventory reports."]
    pub fn set_destination_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions {
    type O =
        terrars::BlockAssignable<StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions {
    #[doc = "The destination bucket that stores the generated inventory reports."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildStorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions {
    pub fn build(self) -> StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions {
        StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions {
            bucket: self.bucket,
            destination_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptionsRef {
        StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nThe destination bucket that stores the generated inventory reports."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `destination_path` after provisioning.\nThe path within the destination bucket to store generated inventory reports."]
    pub fn destination_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_path", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters {
    #[doc =
        "Set the field `bucket`.\nThe filter to use when specifying which bucket to generate inventory reports for."]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters {
    type O = terrars::BlockAssignable<StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters {}

impl BuildStorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters {
    pub fn build(self) -> StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters {
        StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters {
            bucket: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFiltersRef {
        StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nThe filter to use when specifying which bucket to generate inventory reports for."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}

#[derive(Serialize, Default)]
struct StorageInsightsReportConfig_ObjectMetadataReportOptionsDynamic {
    storage_destination_options: std
    ::option
    ::Option<
        terrars::DynamicBlock<StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions>,
    >,
    storage_filters: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters>>,
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfig_ObjectMetadataReportOptions {
    metadata_fields: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_destination_options: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_filters: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters>>,
    dynamic: StorageInsightsReportConfig_ObjectMetadataReportOptionsDynamic,
}

impl StorageInsightsReportConfig_ObjectMetadataReportOptions {
    #[doc = "Set the field `storage_destination_options`.\n"]
    pub fn set_storage_destination_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_destination_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_destination_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_filters`.\n"]
    pub fn set_storage_filters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFilters>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_filters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for StorageInsightsReportConfig_ObjectMetadataReportOptions {
    type O = terrars::BlockAssignable<StorageInsightsReportConfig_ObjectMetadataReportOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfig_ObjectMetadataReportOptions {
    #[doc = "The metadata fields included in an inventory report."]
    pub metadata_fields: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageInsightsReportConfig_ObjectMetadataReportOptions {
    pub fn build(self) -> StorageInsightsReportConfig_ObjectMetadataReportOptions {
        StorageInsightsReportConfig_ObjectMetadataReportOptions {
            metadata_fields: self.metadata_fields,
            storage_destination_options: core::default::Default::default(),
            storage_filters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsReportConfig_ObjectMetadataReportOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfig_ObjectMetadataReportOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfig_ObjectMetadataReportOptionsRef {
        StorageInsightsReportConfig_ObjectMetadataReportOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfig_ObjectMetadataReportOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metadata_fields` after provisioning.\nThe metadata fields included in an inventory report."]
    pub fn metadata_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_destination_options` after provisioning.\n"]
    pub fn storage_destination_options(
        &self,
    ) -> terrars::ListRef<StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageDestinationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_destination_options", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_filters` after provisioning.\n"]
    pub fn storage_filters(
        &self,
    ) -> terrars::ListRef<StorageInsightsReportConfig_ObjectMetadataReportOptions_StorageFiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_filters", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfig_ParquetOptions {}

impl StorageInsightsReportConfig_ParquetOptions { }

impl terrars::ToListMappable for StorageInsightsReportConfig_ParquetOptions {
    type O = terrars::BlockAssignable<StorageInsightsReportConfig_ParquetOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfig_ParquetOptions {}

impl BuildStorageInsightsReportConfig_ParquetOptions {
    pub fn build(self) -> StorageInsightsReportConfig_ParquetOptions {
        StorageInsightsReportConfig_ParquetOptions {}
    }
}

#[allow(dead_code)]
pub struct StorageInsightsReportConfig_ParquetOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfig_ParquetOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageInsightsReportConfig_ParquetOptionsRef {
        StorageInsightsReportConfig_ParquetOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfig_ParquetOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsReportConfig_Timeouts {
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

impl terrars::ToListMappable for StorageInsightsReportConfig_Timeouts {
    type O = terrars::BlockAssignable<StorageInsightsReportConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfig_Timeouts {}

impl BuildStorageInsightsReportConfig_Timeouts {
    pub fn build(self) -> StorageInsightsReportConfig_Timeouts {
        StorageInsightsReportConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StorageInsightsReportConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageInsightsReportConfig_TimeoutsRef {
        StorageInsightsReportConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfig_TimeoutsRef {
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
