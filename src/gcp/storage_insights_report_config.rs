use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageInsightsReportConfigData {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_options: std::option::Option<std::vec::Vec<StorageInsightsReportConfigCsvOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frequency_options: std::option::Option<std::vec::Vec<StorageInsightsReportConfigFrequencyOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_metadata_report_options: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsReportConfigObjectMetadataReportOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StorageInsightsReportConfigTimeoutsEl>,
    dynamic: StorageInsightsReportConfigDynamic,
}

struct StorageInsightsReportConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StorageInsightsReportConfigData>,
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
        "Set the field `display_name`.\nThe editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty."]
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

    #[doc = "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsReportConfigCsvOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsReportConfigFrequencyOptionsEl>>,
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
                    ::Into<terrars::BlockAssignable<StorageInsightsReportConfigObjectMetadataReportOptionsEl>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StorageInsightsReportConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
    pub fn csv_options(&self) -> terrars::ListRef<StorageInsightsReportConfigCsvOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `frequency_options` after provisioning.\n"]
    pub fn frequency_options(&self) -> terrars::ListRef<StorageInsightsReportConfigFrequencyOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frequency_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_metadata_report_options` after provisioning.\n"]
    pub fn object_metadata_report_options(
        &self,
    ) -> terrars::ListRef<StorageInsightsReportConfigObjectMetadataReportOptionsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.object_metadata_report_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageInsightsReportConfigTimeoutsElRef {
        StorageInsightsReportConfigTimeoutsElRef::new(
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
            data: std::cell::RefCell::new(StorageInsightsReportConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                csv_options: core::default::Default::default(),
                frequency_options: core::default::Default::default(),
                object_metadata_report_options: core::default::Default::default(),
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
        "Get a reference to the value of field `display_name` after provisioning.\nThe editable display name of the inventory report configuration. Has a limit of 256 characters. Can be empty."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
    pub fn csv_options(&self) -> terrars::ListRef<StorageInsightsReportConfigCsvOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `frequency_options` after provisioning.\n"]
    pub fn frequency_options(&self) -> terrars::ListRef<StorageInsightsReportConfigFrequencyOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.frequency_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_metadata_report_options` after provisioning.\n"]
    pub fn object_metadata_report_options(
        &self,
    ) -> terrars::ListRef<StorageInsightsReportConfigObjectMetadataReportOptionsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.object_metadata_report_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageInsightsReportConfigTimeoutsElRef {
        StorageInsightsReportConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfigCsvOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_separator: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsReportConfigCsvOptionsEl {
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

impl terrars::ToListMappable for StorageInsightsReportConfigCsvOptionsEl {
    type O = terrars::BlockAssignable<StorageInsightsReportConfigCsvOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfigCsvOptionsEl {}

impl BuildStorageInsightsReportConfigCsvOptionsEl {
    pub fn build(self) -> StorageInsightsReportConfigCsvOptionsEl {
        StorageInsightsReportConfigCsvOptionsEl {
            delimiter: core::default::Default::default(),
            header_required: core::default::Default::default(),
            record_separator: core::default::Default::default(),
        }
    }
}

pub struct StorageInsightsReportConfigCsvOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfigCsvOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageInsightsReportConfigCsvOptionsElRef {
        StorageInsightsReportConfigCsvOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfigCsvOptionsElRef {
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
pub struct StorageInsightsReportConfigFrequencyOptionsElEndDateEl {
    day: terrars::PrimField<f64>,
    month: terrars::PrimField<f64>,
    year: terrars::PrimField<f64>,
}

impl StorageInsightsReportConfigFrequencyOptionsElEndDateEl { }

impl terrars::ToListMappable for StorageInsightsReportConfigFrequencyOptionsElEndDateEl {
    type O = terrars::BlockAssignable<StorageInsightsReportConfigFrequencyOptionsElEndDateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfigFrequencyOptionsElEndDateEl {
    #[doc = "The day of the month to stop generating inventory reports."]
    pub day: terrars::PrimField<f64>,
    #[doc = "The month to stop generating inventory reports."]
    pub month: terrars::PrimField<f64>,
    #[doc = "The year to stop generating inventory reports"]
    pub year: terrars::PrimField<f64>,
}

impl BuildStorageInsightsReportConfigFrequencyOptionsElEndDateEl {
    pub fn build(self) -> StorageInsightsReportConfigFrequencyOptionsElEndDateEl {
        StorageInsightsReportConfigFrequencyOptionsElEndDateEl {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

pub struct StorageInsightsReportConfigFrequencyOptionsElEndDateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfigFrequencyOptionsElEndDateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfigFrequencyOptionsElEndDateElRef {
        StorageInsightsReportConfigFrequencyOptionsElEndDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfigFrequencyOptionsElEndDateElRef {
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
pub struct StorageInsightsReportConfigFrequencyOptionsElStartDateEl {
    day: terrars::PrimField<f64>,
    month: terrars::PrimField<f64>,
    year: terrars::PrimField<f64>,
}

impl StorageInsightsReportConfigFrequencyOptionsElStartDateEl { }

impl terrars::ToListMappable for StorageInsightsReportConfigFrequencyOptionsElStartDateEl {
    type O = terrars::BlockAssignable<StorageInsightsReportConfigFrequencyOptionsElStartDateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfigFrequencyOptionsElStartDateEl {
    #[doc = "The day of the month to start generating inventory reports."]
    pub day: terrars::PrimField<f64>,
    #[doc = "The month to start generating inventory reports."]
    pub month: terrars::PrimField<f64>,
    #[doc = "The year to start generating inventory reports"]
    pub year: terrars::PrimField<f64>,
}

impl BuildStorageInsightsReportConfigFrequencyOptionsElStartDateEl {
    pub fn build(self) -> StorageInsightsReportConfigFrequencyOptionsElStartDateEl {
        StorageInsightsReportConfigFrequencyOptionsElStartDateEl {
            day: self.day,
            month: self.month,
            year: self.year,
        }
    }
}

pub struct StorageInsightsReportConfigFrequencyOptionsElStartDateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfigFrequencyOptionsElStartDateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfigFrequencyOptionsElStartDateElRef {
        StorageInsightsReportConfigFrequencyOptionsElStartDateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfigFrequencyOptionsElStartDateElRef {
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
struct StorageInsightsReportConfigFrequencyOptionsElDynamic {
    end_date: std::option::Option<terrars::DynamicBlock<StorageInsightsReportConfigFrequencyOptionsElEndDateEl>>,
    start_date: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsReportConfigFrequencyOptionsElStartDateEl>>,
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfigFrequencyOptionsEl {
    frequency: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std::option::Option<std::vec::Vec<StorageInsightsReportConfigFrequencyOptionsElEndDateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<std::vec::Vec<StorageInsightsReportConfigFrequencyOptionsElStartDateEl>>,
    dynamic: StorageInsightsReportConfigFrequencyOptionsElDynamic,
}

impl StorageInsightsReportConfigFrequencyOptionsEl {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageInsightsReportConfigFrequencyOptionsElEndDateEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<StorageInsightsReportConfigFrequencyOptionsElStartDateEl>>,
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

impl terrars::ToListMappable for StorageInsightsReportConfigFrequencyOptionsEl {
    type O = terrars::BlockAssignable<StorageInsightsReportConfigFrequencyOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfigFrequencyOptionsEl {
    #[doc =
        "The frequency in which inventory reports are generated. Values are DAILY or WEEKLY. Possible values: [\"DAILY\", \"WEEKLY\"]"]
    pub frequency: terrars::PrimField<std::string::String>,
}

impl BuildStorageInsightsReportConfigFrequencyOptionsEl {
    pub fn build(self) -> StorageInsightsReportConfigFrequencyOptionsEl {
        StorageInsightsReportConfigFrequencyOptionsEl {
            frequency: self.frequency,
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct StorageInsightsReportConfigFrequencyOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfigFrequencyOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfigFrequencyOptionsElRef {
        StorageInsightsReportConfigFrequencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfigFrequencyOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nThe frequency in which inventory reports are generated. Values are DAILY or WEEKLY. Possible values: [\"DAILY\", \"WEEKLY\"]"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::ListRef<StorageInsightsReportConfigFrequencyOptionsElEndDateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::ListRef<StorageInsightsReportConfigFrequencyOptionsElStartDateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
    #[doc =
        "Set the field `destination_path`.\nThe path within the destination bucket to store generated inventory reports."]
    pub fn set_destination_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
    type O =
        terrars
        ::BlockAssignable<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
    #[doc = "The destination bucket that stores the generated inventory reports."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildStorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
    pub fn build(self) -> StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
        StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl {
            bucket: self.bucket,
            destination_path: core::default::Default::default(),
        }
    }
}

pub struct StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef {
        StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef {
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
pub struct StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
    #[doc =
        "Set the field `bucket`.\nThe filter to use when specifying which bucket to generate inventory reports for."]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
    type O = terrars::BlockAssignable<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {}

impl BuildStorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
    pub fn build(self) -> StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
        StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl {
            bucket: core::default::Default::default(),
        }
    }
}

pub struct StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef {
        StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef {
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
struct StorageInsightsReportConfigObjectMetadataReportOptionsElDynamic {
    storage_destination_options: std
    ::option
    ::Option<
        terrars::DynamicBlock<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl>,
    >,
    storage_filters: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl>>,
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfigObjectMetadataReportOptionsEl {
    metadata_fields: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_destination_options: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_filters: std
    ::option
    ::Option<std::vec::Vec<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl>>,
    dynamic: StorageInsightsReportConfigObjectMetadataReportOptionsElDynamic,
}

impl StorageInsightsReportConfigObjectMetadataReportOptionsEl {
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
                            StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsEl,
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
                        ::BlockAssignable<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersEl>,
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

impl terrars::ToListMappable for StorageInsightsReportConfigObjectMetadataReportOptionsEl {
    type O = terrars::BlockAssignable<StorageInsightsReportConfigObjectMetadataReportOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfigObjectMetadataReportOptionsEl {
    #[doc = "The metadata fields included in an inventory report."]
    pub metadata_fields: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildStorageInsightsReportConfigObjectMetadataReportOptionsEl {
    pub fn build(self) -> StorageInsightsReportConfigObjectMetadataReportOptionsEl {
        StorageInsightsReportConfigObjectMetadataReportOptionsEl {
            metadata_fields: self.metadata_fields,
            storage_destination_options: core::default::Default::default(),
            storage_filters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct StorageInsightsReportConfigObjectMetadataReportOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfigObjectMetadataReportOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StorageInsightsReportConfigObjectMetadataReportOptionsElRef {
        StorageInsightsReportConfigObjectMetadataReportOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfigObjectMetadataReportOptionsElRef {
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
    ) -> terrars::ListRef<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageDestinationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_destination_options", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_filters` after provisioning.\n"]
    pub fn storage_filters(
        &self,
    ) -> terrars::ListRef<StorageInsightsReportConfigObjectMetadataReportOptionsElStorageFiltersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_filters", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageInsightsReportConfigTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageInsightsReportConfigTimeoutsEl {
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

impl terrars::ToListMappable for StorageInsightsReportConfigTimeoutsEl {
    type O = terrars::BlockAssignable<StorageInsightsReportConfigTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageInsightsReportConfigTimeoutsEl {}

impl BuildStorageInsightsReportConfigTimeoutsEl {
    pub fn build(self) -> StorageInsightsReportConfigTimeoutsEl {
        StorageInsightsReportConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct StorageInsightsReportConfigTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageInsightsReportConfigTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageInsightsReportConfigTimeoutsElRef {
        StorageInsightsReportConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageInsightsReportConfigTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct StorageInsightsReportConfigDynamic {
    csv_options: std::option::Option<terrars::DynamicBlock<StorageInsightsReportConfigCsvOptionsEl>>,
    frequency_options: std::option::Option<terrars::DynamicBlock<StorageInsightsReportConfigFrequencyOptionsEl>>,
    object_metadata_report_options: std
    ::option
    ::Option<terrars::DynamicBlock<StorageInsightsReportConfigObjectMetadataReportOptionsEl>>,
}
