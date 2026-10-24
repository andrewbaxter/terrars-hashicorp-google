use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBigqueryDatasetData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    dataset_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBigqueryDataset_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBigqueryDatasetData>,
}

#[derive(Clone)]
pub struct DataBigqueryDataset(std::rc::Rc<DataBigqueryDataset_>);

impl DataBigqueryDataset {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc =
        "Get a reference to the value of field `access` after provisioning.\nAn array of objects that define dataset access for one or more entities."]
    pub fn access(&self) -> terrars::SetRef<DataBigqueryDatasetAccessElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this dataset was created, in milliseconds since the\nepoch."]
    pub fn creation_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_collation` after provisioning.\nDefines the default collation specification of future tables created\nin the dataset. If a table is created in this dataset without table-level\ndefault collation, then the table inherits the dataset default collation,\nwhich is applied to the string fields that do not have explicit collation\nspecified. A change to this field affects only tables created afterwards,\nand does not alter the existing tables.\n\nThe following values are supported:\n- 'und:ci': undetermined locale, case insensitive.\n- '': empty string. Default to case-sensitive behavior."]
    pub fn default_collation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_collation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_encryption_configuration` after provisioning.\nThe default encryption key for all tables in the dataset. Once this property is set,\nall newly-created partitioned tables in the dataset will have encryption key set to\nthis value, unless table creation request (or query) overrides the key."]
    pub fn default_encryption_configuration(
        &self,
    ) -> terrars::ListRef<DataBigqueryDatasetDefaultEncryptionConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.default_encryption_configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `default_partition_expiration_ms` after provisioning.\nThe default partition expiration for all partitioned tables in\nthe dataset, in milliseconds.\n\n\nOnce this property is set, all newly-created partitioned tables in\nthe dataset will have an 'expirationMs' property in the 'timePartitioning'\nsettings set to this value, and changing the value will only\naffect new tables, not existing ones. The storage in a partition will\nhave an expiration time of its partition time plus this value.\nSetting this property overrides the use of 'defaultTableExpirationMs'\nfor partitioned tables: only one of 'defaultTableExpirationMs' and\n'defaultPartitionExpirationMs' will be used for any new partitioned\ntable. If you provide an explicit 'timePartitioning.expirationMs' when\ncreating or updating a partitioned table, that value takes precedence\nover the default partition expiration time indicated by this property."]
    pub fn default_partition_expiration_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_partition_expiration_ms", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `default_table_expiration_ms` after provisioning.\nThe default lifetime of all tables in the dataset, in milliseconds.\nThe minimum value is 3600000 milliseconds (one hour).\n\n\nOnce this property is set, all newly-created tables in the dataset\nwill have an 'expirationTime' property set to the creation time plus\nthe value in this property, and changing the value will only affect\nnew tables, not existing ones. When the 'expirationTime' for a given\ntable is reached, that table will be deleted automatically.\nIf a table's 'expirationTime' is modified or removed before the\ntable expires, or if you provide an explicit 'expirationTime' when\ncreating a table, that value takes precedence over the default\nexpiration time indicated by this property."]
    pub fn default_table_expiration_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_table_expiration_ms", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_contents_on_destroy` after provisioning.\nIf set to 'true', delete all the tables in the\ndataset when destroying the resource; otherwise,\ndestroying the resource will fail if tables are present."]
    pub fn delete_contents_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_contents_on_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-friendly description of the dataset"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA hash of the resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the dataset"]
    pub fn friendly_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_case_insensitive` after provisioning.\nTRUE if the dataset and its table names are case-insensitive, otherwise FALSE.\nBy default, this is FALSE, which means the dataset and its table names are\ncase-sensitive. This field does not affect routine references."]
    pub fn is_case_insensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_case_insensitive", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_time` after provisioning.\nThe date when this dataset or any of its tables was last modified, in\nmilliseconds since the epoch."]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the dataset should reside.\nSee [official docs](https://cloud.google.com/bigquery/docs/dataset-locations).\n\n\nThere are two types of locations, regional or multi-regional. A regional\nlocation is a specific geographic place, such as Tokyo, and a multi-regional\nlocation is a large geographic area, such as the United States, that\ncontains at least two geographic places.\n\n\nThe default value is multi-regional location 'US'.\nChanging this forces a new resource to be created."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_time_travel_hours` after provisioning.\nDefines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days)."]
    pub fn max_time_travel_hours(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_time_travel_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_billing_model` after provisioning.\nSpecifies the storage billing model for the dataset.\nSet this flag value to LOGICAL to use logical bytes for storage billing,\nor to PHYSICAL to use physical bytes instead.\n\nLOGICAL is the default if this flag isn't specified."]
    pub fn storage_billing_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_billing_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

impl terrars::Referable for DataBigqueryDataset {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBigqueryDataset { }

impl terrars::ToListMappable for DataBigqueryDataset {
    type O = terrars::ListRef<DataBigqueryDatasetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBigqueryDataset_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_bigquery_dataset".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBigqueryDataset {
    pub tf_id: std::string::String,
    #[doc =
        "A unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub dataset_id: terrars::PrimField<std::string::String>,
}

impl BuildDataBigqueryDataset {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBigqueryDataset {
        let out = DataBigqueryDataset(std::rc::Rc::new(DataBigqueryDataset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBigqueryDatasetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                dataset_id: self.dataset_id,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBigqueryDatasetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryDatasetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBigqueryDatasetRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `access` after provisioning.\nAn array of objects that define dataset access for one or more entities."]
    pub fn access(&self) -> terrars::SetRef<DataBigqueryDatasetAccessElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this dataset was created, in milliseconds since the\nepoch."]
    pub fn creation_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_collation` after provisioning.\nDefines the default collation specification of future tables created\nin the dataset. If a table is created in this dataset without table-level\ndefault collation, then the table inherits the dataset default collation,\nwhich is applied to the string fields that do not have explicit collation\nspecified. A change to this field affects only tables created afterwards,\nand does not alter the existing tables.\n\nThe following values are supported:\n- 'und:ci': undetermined locale, case insensitive.\n- '': empty string. Default to case-sensitive behavior."]
    pub fn default_collation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_collation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_encryption_configuration` after provisioning.\nThe default encryption key for all tables in the dataset. Once this property is set,\nall newly-created partitioned tables in the dataset will have encryption key set to\nthis value, unless table creation request (or query) overrides the key."]
    pub fn default_encryption_configuration(
        &self,
    ) -> terrars::ListRef<DataBigqueryDatasetDefaultEncryptionConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.default_encryption_configuration", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `default_partition_expiration_ms` after provisioning.\nThe default partition expiration for all partitioned tables in\nthe dataset, in milliseconds.\n\n\nOnce this property is set, all newly-created partitioned tables in\nthe dataset will have an 'expirationMs' property in the 'timePartitioning'\nsettings set to this value, and changing the value will only\naffect new tables, not existing ones. The storage in a partition will\nhave an expiration time of its partition time plus this value.\nSetting this property overrides the use of 'defaultTableExpirationMs'\nfor partitioned tables: only one of 'defaultTableExpirationMs' and\n'defaultPartitionExpirationMs' will be used for any new partitioned\ntable. If you provide an explicit 'timePartitioning.expirationMs' when\ncreating or updating a partitioned table, that value takes precedence\nover the default partition expiration time indicated by this property."]
    pub fn default_partition_expiration_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_partition_expiration_ms", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `default_table_expiration_ms` after provisioning.\nThe default lifetime of all tables in the dataset, in milliseconds.\nThe minimum value is 3600000 milliseconds (one hour).\n\n\nOnce this property is set, all newly-created tables in the dataset\nwill have an 'expirationTime' property set to the creation time plus\nthe value in this property, and changing the value will only affect\nnew tables, not existing ones. When the 'expirationTime' for a given\ntable is reached, that table will be deleted automatically.\nIf a table's 'expirationTime' is modified or removed before the\ntable expires, or if you provide an explicit 'expirationTime' when\ncreating a table, that value takes precedence over the default\nexpiration time indicated by this property."]
    pub fn default_table_expiration_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_table_expiration_ms", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_contents_on_destroy` after provisioning.\nIf set to 'true', delete all the tables in the\ndataset when destroying the resource; otherwise,\ndestroying the resource will fail if tables are present."]
    pub fn delete_contents_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_contents_on_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-friendly description of the dataset"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA hash of the resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the dataset"]
    pub fn friendly_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_case_insensitive` after provisioning.\nTRUE if the dataset and its table names are case-insensitive, otherwise FALSE.\nBy default, this is FALSE, which means the dataset and its table names are\ncase-sensitive. This field does not affect routine references."]
    pub fn is_case_insensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_case_insensitive", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_time` after provisioning.\nThe date when this dataset or any of its tables was last modified, in\nmilliseconds since the epoch."]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the dataset should reside.\nSee [official docs](https://cloud.google.com/bigquery/docs/dataset-locations).\n\n\nThere are two types of locations, regional or multi-regional. A regional\nlocation is a specific geographic place, such as Tokyo, and a multi-regional\nlocation is a large geographic area, such as the United States, that\ncontains at least two geographic places.\n\n\nThe default value is multi-regional location 'US'.\nChanging this forces a new resource to be created."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_time_travel_hours` after provisioning.\nDefines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days)."]
    pub fn max_time_travel_hours(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_time_travel_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_billing_model` after provisioning.\nSpecifies the storage billing model for the dataset.\nSet this flag value to LOGICAL to use logical bytes for storage billing,\nor to PHYSICAL to use physical bytes instead.\n\nLOGICAL is the default if this flag isn't specified."]
    pub fn storage_billing_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_billing_model", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetAccessElDatasetElDatasetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryDatasetAccessElDatasetElDatasetEl {
    #[doc = "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryDatasetAccessElDatasetElDatasetEl {
    type O = terrars::BlockAssignable<DataBigqueryDatasetAccessElDatasetElDatasetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryDatasetAccessElDatasetElDatasetEl {}

impl BuildDataBigqueryDatasetAccessElDatasetElDatasetEl {
    pub fn build(self) -> DataBigqueryDatasetAccessElDatasetElDatasetEl {
        DataBigqueryDatasetAccessElDatasetElDatasetEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

pub struct DataBigqueryDatasetAccessElDatasetElDatasetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryDatasetAccessElDatasetElDatasetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryDatasetAccessElDatasetElDatasetElRef {
        DataBigqueryDatasetAccessElDatasetElDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetAccessElDatasetElDatasetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetAccessElDatasetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<terrars::ListField<DataBigqueryDatasetAccessElDatasetElDatasetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataBigqueryDatasetAccessElDatasetEl {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryDatasetAccessElDatasetElDatasetEl>>,
    ) -> Self {
        self.dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_types`.\n"]
    pub fn set_target_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryDatasetAccessElDatasetEl {
    type O = terrars::BlockAssignable<DataBigqueryDatasetAccessElDatasetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryDatasetAccessElDatasetEl {}

impl BuildDataBigqueryDatasetAccessElDatasetEl {
    pub fn build(self) -> DataBigqueryDatasetAccessElDatasetEl {
        DataBigqueryDatasetAccessElDatasetEl {
            dataset: core::default::Default::default(),
            target_types: core::default::Default::default(),
        }
    }
}

pub struct DataBigqueryDatasetAccessElDatasetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryDatasetAccessElDatasetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryDatasetAccessElDatasetElRef {
        DataBigqueryDatasetAccessElDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetAccessElDatasetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> terrars::ListRef<DataBigqueryDatasetAccessElDatasetElDatasetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `target_types` after provisioning.\n"]
    pub fn target_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetAccessElRoutineEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routine_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryDatasetAccessElRoutineEl {
    #[doc = "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routine_id`.\n"]
    pub fn set_routine_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routine_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryDatasetAccessElRoutineEl {
    type O = terrars::BlockAssignable<DataBigqueryDatasetAccessElRoutineEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryDatasetAccessElRoutineEl {}

impl BuildDataBigqueryDatasetAccessElRoutineEl {
    pub fn build(self) -> DataBigqueryDatasetAccessElRoutineEl {
        DataBigqueryDatasetAccessElRoutineEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            routine_id: core::default::Default::default(),
        }
    }
}

pub struct DataBigqueryDatasetAccessElRoutineElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryDatasetAccessElRoutineElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryDatasetAccessElRoutineElRef {
        DataBigqueryDatasetAccessElRoutineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetAccessElRoutineElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `routine_id` after provisioning.\n"]
    pub fn routine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routine_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetAccessElViewEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryDatasetAccessElViewEl {
    #[doc = "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_id`.\n"]
    pub fn set_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryDatasetAccessElViewEl {
    type O = terrars::BlockAssignable<DataBigqueryDatasetAccessElViewEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryDatasetAccessElViewEl {}

impl BuildDataBigqueryDatasetAccessElViewEl {
    pub fn build(self) -> DataBigqueryDatasetAccessElViewEl {
        DataBigqueryDatasetAccessElViewEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: core::default::Default::default(),
        }
    }
}

pub struct DataBigqueryDatasetAccessElViewElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryDatasetAccessElViewElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryDatasetAccessElViewElRef {
        DataBigqueryDatasetAccessElViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetAccessElViewElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_id` after provisioning.\n"]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetAccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<terrars::ListField<DataBigqueryDatasetAccessElDatasetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_by_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_member: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routine: std::option::Option<terrars::ListField<DataBigqueryDatasetAccessElRoutineEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    special_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_by_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view: std::option::Option<terrars::ListField<DataBigqueryDatasetAccessElViewEl>>,
}

impl DataBigqueryDatasetAccessEl {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryDatasetAccessElDatasetEl>>,
    ) -> Self {
        self.dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_by_email`.\n"]
    pub fn set_group_by_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_by_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_member`.\n"]
    pub fn set_iam_member(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.iam_member = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `routine`.\n"]
    pub fn set_routine(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryDatasetAccessElRoutineEl>>,
    ) -> Self {
        self.routine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `special_group`.\n"]
    pub fn set_special_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.special_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_by_email`.\n"]
    pub fn set_user_by_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_by_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view`.\n"]
    pub fn set_view(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryDatasetAccessElViewEl>>,
    ) -> Self {
        self.view = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryDatasetAccessEl {
    type O = terrars::BlockAssignable<DataBigqueryDatasetAccessEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryDatasetAccessEl {}

impl BuildDataBigqueryDatasetAccessEl {
    pub fn build(self) -> DataBigqueryDatasetAccessEl {
        DataBigqueryDatasetAccessEl {
            dataset: core::default::Default::default(),
            domain: core::default::Default::default(),
            group_by_email: core::default::Default::default(),
            iam_member: core::default::Default::default(),
            role: core::default::Default::default(),
            routine: core::default::Default::default(),
            special_group: core::default::Default::default(),
            user_by_email: core::default::Default::default(),
            view: core::default::Default::default(),
        }
    }
}

pub struct DataBigqueryDatasetAccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryDatasetAccessElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryDatasetAccessElRef {
        DataBigqueryDatasetAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetAccessElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> terrars::ListRef<DataBigqueryDatasetAccessElDatasetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `group_by_email` after provisioning.\n"]
    pub fn group_by_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_by_email", self.base))
    }

    #[doc = "Get a reference to the value of field `iam_member` after provisioning.\n"]
    pub fn iam_member(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_member", self.base))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc = "Get a reference to the value of field `routine` after provisioning.\n"]
    pub fn routine(&self) -> terrars::ListRef<DataBigqueryDatasetAccessElRoutineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routine", self.base))
    }

    #[doc = "Get a reference to the value of field `special_group` after provisioning.\n"]
    pub fn special_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.special_group", self.base))
    }

    #[doc = "Get a reference to the value of field `user_by_email` after provisioning.\n"]
    pub fn user_by_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_by_email", self.base))
    }

    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> terrars::ListRef<DataBigqueryDatasetAccessElViewElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.view", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetDefaultEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryDatasetDefaultEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryDatasetDefaultEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<DataBigqueryDatasetDefaultEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryDatasetDefaultEncryptionConfigurationEl {}

impl BuildDataBigqueryDatasetDefaultEncryptionConfigurationEl {
    pub fn build(self) -> DataBigqueryDatasetDefaultEncryptionConfigurationEl {
        DataBigqueryDatasetDefaultEncryptionConfigurationEl { kms_key_name: core::default::Default::default() }
    }
}

pub struct DataBigqueryDatasetDefaultEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryDatasetDefaultEncryptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryDatasetDefaultEncryptionConfigurationElRef {
        DataBigqueryDatasetDefaultEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetDefaultEncryptionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
