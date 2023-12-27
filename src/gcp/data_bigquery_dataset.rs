use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBigqueryDatasetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    dataset_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}

struct DataBigqueryDataset_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBigqueryDatasetData>,
}

#[derive(Clone)]
pub struct DataBigqueryDataset(Rc<DataBigqueryDataset_>);

impl DataBigqueryDataset {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access` after provisioning.\nAn array of objects that define dataset access for one or more entities."]
    pub fn access(&self) -> SetRef<DataBigqueryDatasetAccessElRef> {
        SetRef::new(self.shared().clone(), format!("{}.access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this dataset was created, in milliseconds since the\nepoch."]
    pub fn creation_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_collation` after provisioning.\nDefines the default collation specification of future tables created\nin the dataset. If a table is created in this dataset without table-level\ndefault collation, then the table inherits the dataset default collation,\nwhich is applied to the string fields that do not have explicit collation\nspecified. A change to this field affects only tables created afterwards,\nand does not alter the existing tables.\n\nThe following values are supported:\n- 'und:ci': undetermined locale, case insensitive.\n- '': empty string. Default to case-sensitive behavior."]
    pub fn default_collation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_collation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_encryption_configuration` after provisioning.\nThe default encryption key for all tables in the dataset. Once this property is set,\nall newly-created partitioned tables in the dataset will have encryption key set to\nthis value, unless table creation request (or query) overrides the key."]
    pub fn default_encryption_configuration(&self) -> ListRef<DataBigqueryDatasetDefaultEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_encryption_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_partition_expiration_ms` after provisioning.\nThe default partition expiration for all partitioned tables in\nthe dataset, in milliseconds.\n\n\nOnce this property is set, all newly-created partitioned tables in\nthe dataset will have an 'expirationMs' property in the 'timePartitioning'\nsettings set to this value, and changing the value will only\naffect new tables, not existing ones. The storage in a partition will\nhave an expiration time of its partition time plus this value.\nSetting this property overrides the use of 'defaultTableExpirationMs'\nfor partitioned tables: only one of 'defaultTableExpirationMs' and\n'defaultPartitionExpirationMs' will be used for any new partitioned\ntable. If you provide an explicit 'timePartitioning.expirationMs' when\ncreating or updating a partitioned table, that value takes precedence\nover the default partition expiration time indicated by this property."]
    pub fn default_partition_expiration_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_partition_expiration_ms", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_table_expiration_ms` after provisioning.\nThe default lifetime of all tables in the dataset, in milliseconds.\nThe minimum value is 3600000 milliseconds (one hour).\n\n\nOnce this property is set, all newly-created tables in the dataset\nwill have an 'expirationTime' property set to the creation time plus\nthe value in this property, and changing the value will only affect\nnew tables, not existing ones. When the 'expirationTime' for a given\ntable is reached, that table will be deleted automatically.\nIf a table's 'expirationTime' is modified or removed before the\ntable expires, or if you provide an explicit 'expirationTime' when\ncreating a table, that value takes precedence over the default\nexpiration time indicated by this property."]
    pub fn default_table_expiration_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_table_expiration_ms", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_contents_on_destroy` after provisioning.\nIf set to 'true', delete all the tables in the\ndataset when destroying the resource; otherwise,\ndestroying the resource will fail if tables are present."]
    pub fn delete_contents_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_contents_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA user-friendly description of the dataset"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nA hash of the resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the dataset"]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_case_insensitive` after provisioning.\nTRUE if the dataset and its table names are case-insensitive, otherwise FALSE.\nBy default, this is FALSE, which means the dataset and its table names are\ncase-sensitive. This field does not affect routine references."]
    pub fn is_case_insensitive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_case_insensitive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_modified_time` after provisioning.\nThe date when this dataset or any of its tables was last modified, in\nmilliseconds since the epoch."]
    pub fn last_modified_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the dataset should reside.\nSee [official docs](https://cloud.google.com/bigquery/docs/dataset-locations).\n\n\nThere are two types of locations, regional or multi-regional. A regional\nlocation is a specific geographic place, such as Tokyo, and a multi-regional\nlocation is a large geographic area, such as the United States, that\ncontains at least two geographic places.\n\n\nThe default value is multi-regional location 'US'.\nChanging this forces a new resource to be created."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_time_travel_hours` after provisioning.\nDefines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days)."]
    pub fn max_time_travel_hours(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_time_travel_hours", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_billing_model` after provisioning.\nSpecifies the storage billing model for the dataset.\nSet this flag value to LOGICAL to use logical bytes for storage billing,\nor to PHYSICAL to use physical bytes instead.\n\nLOGICAL is the default if this flag isn't specified."]
    pub fn storage_billing_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_billing_model", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

impl Referable for DataBigqueryDataset {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataBigqueryDataset { }

impl ToListMappable for DataBigqueryDataset {
    type O = ListRef<DataBigqueryDatasetRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBigqueryDataset_ {
    fn extract_datasource_type(&self) -> String {
        "google_bigquery_dataset".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBigqueryDataset {
    pub tf_id: String,
    #[doc= "A unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub dataset_id: PrimField<String>,
}

impl BuildDataBigqueryDataset {
    pub fn build(self, stack: &mut Stack) -> DataBigqueryDataset {
        let out = DataBigqueryDataset(Rc::new(DataBigqueryDataset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBigqueryDatasetData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for DataBigqueryDatasetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBigqueryDatasetRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `access` after provisioning.\nAn array of objects that define dataset access for one or more entities."]
    pub fn access(&self) -> SetRef<DataBigqueryDatasetAccessElRef> {
        SetRef::new(self.shared().clone(), format!("{}.access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this dataset was created, in milliseconds since the\nepoch."]
    pub fn creation_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_collation` after provisioning.\nDefines the default collation specification of future tables created\nin the dataset. If a table is created in this dataset without table-level\ndefault collation, then the table inherits the dataset default collation,\nwhich is applied to the string fields that do not have explicit collation\nspecified. A change to this field affects only tables created afterwards,\nand does not alter the existing tables.\n\nThe following values are supported:\n- 'und:ci': undetermined locale, case insensitive.\n- '': empty string. Default to case-sensitive behavior."]
    pub fn default_collation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_collation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_encryption_configuration` after provisioning.\nThe default encryption key for all tables in the dataset. Once this property is set,\nall newly-created partitioned tables in the dataset will have encryption key set to\nthis value, unless table creation request (or query) overrides the key."]
    pub fn default_encryption_configuration(&self) -> ListRef<DataBigqueryDatasetDefaultEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_encryption_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_partition_expiration_ms` after provisioning.\nThe default partition expiration for all partitioned tables in\nthe dataset, in milliseconds.\n\n\nOnce this property is set, all newly-created partitioned tables in\nthe dataset will have an 'expirationMs' property in the 'timePartitioning'\nsettings set to this value, and changing the value will only\naffect new tables, not existing ones. The storage in a partition will\nhave an expiration time of its partition time plus this value.\nSetting this property overrides the use of 'defaultTableExpirationMs'\nfor partitioned tables: only one of 'defaultTableExpirationMs' and\n'defaultPartitionExpirationMs' will be used for any new partitioned\ntable. If you provide an explicit 'timePartitioning.expirationMs' when\ncreating or updating a partitioned table, that value takes precedence\nover the default partition expiration time indicated by this property."]
    pub fn default_partition_expiration_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_partition_expiration_ms", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_table_expiration_ms` after provisioning.\nThe default lifetime of all tables in the dataset, in milliseconds.\nThe minimum value is 3600000 milliseconds (one hour).\n\n\nOnce this property is set, all newly-created tables in the dataset\nwill have an 'expirationTime' property set to the creation time plus\nthe value in this property, and changing the value will only affect\nnew tables, not existing ones. When the 'expirationTime' for a given\ntable is reached, that table will be deleted automatically.\nIf a table's 'expirationTime' is modified or removed before the\ntable expires, or if you provide an explicit 'expirationTime' when\ncreating a table, that value takes precedence over the default\nexpiration time indicated by this property."]
    pub fn default_table_expiration_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_table_expiration_ms", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_contents_on_destroy` after provisioning.\nIf set to 'true', delete all the tables in the\ndataset when destroying the resource; otherwise,\ndestroying the resource will fail if tables are present."]
    pub fn delete_contents_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_contents_on_destroy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA user-friendly description of the dataset"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nA hash of the resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the dataset"]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_case_insensitive` after provisioning.\nTRUE if the dataset and its table names are case-insensitive, otherwise FALSE.\nBy default, this is FALSE, which means the dataset and its table names are\ncase-sensitive. This field does not affect routine references."]
    pub fn is_case_insensitive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_case_insensitive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_modified_time` after provisioning.\nThe date when this dataset or any of its tables was last modified, in\nmilliseconds since the epoch."]
    pub fn last_modified_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the dataset should reside.\nSee [official docs](https://cloud.google.com/bigquery/docs/dataset-locations).\n\n\nThere are two types of locations, regional or multi-regional. A regional\nlocation is a specific geographic place, such as Tokyo, and a multi-regional\nlocation is a large geographic area, such as the United States, that\ncontains at least two geographic places.\n\n\nThe default value is multi-regional location 'US'.\nChanging this forces a new resource to be created."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_time_travel_hours` after provisioning.\nDefines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days)."]
    pub fn max_time_travel_hours(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_time_travel_hours", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_billing_model` after provisioning.\nSpecifies the storage billing model for the dataset.\nSet this flag value to LOGICAL to use logical bytes for storage billing,\nor to PHYSICAL to use physical bytes instead.\n\nLOGICAL is the default if this flag isn't specified."]
    pub fn storage_billing_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_billing_model", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetAccessElDatasetElDatasetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}

impl DataBigqueryDatasetAccessElDatasetElDatasetEl {
    #[doc= "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataBigqueryDatasetAccessElDatasetElDatasetEl {
    type O = BlockAssignable<DataBigqueryDatasetAccessElDatasetElDatasetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataBigqueryDatasetAccessElDatasetElDatasetElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryDatasetAccessElDatasetElDatasetElRef {
        DataBigqueryDatasetAccessElDatasetElDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetAccessElDatasetElDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetAccessElDatasetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<ListField<DataBigqueryDatasetAccessElDatasetElDatasetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_types: Option<ListField<PrimField<String>>>,
}

impl DataBigqueryDatasetAccessElDatasetEl {
    #[doc= "Set the field `dataset`.\n"]
    pub fn set_dataset(mut self, v: impl Into<ListField<DataBigqueryDatasetAccessElDatasetElDatasetEl>>) -> Self {
        self.dataset = Some(v.into());
        self
    }

    #[doc= "Set the field `target_types`.\n"]
    pub fn set_target_types(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.target_types = Some(v.into());
        self
    }
}

impl ToListMappable for DataBigqueryDatasetAccessElDatasetEl {
    type O = BlockAssignable<DataBigqueryDatasetAccessElDatasetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataBigqueryDatasetAccessElDatasetElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryDatasetAccessElDatasetElRef {
        DataBigqueryDatasetAccessElDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetAccessElDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> ListRef<DataBigqueryDatasetAccessElDatasetElDatasetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc= "Get a reference to the value of field `target_types` after provisioning.\n"]
    pub fn target_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.target_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetAccessElRoutineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routine_id: Option<PrimField<String>>,
}

impl DataBigqueryDatasetAccessElRoutineEl {
    #[doc= "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `routine_id`.\n"]
    pub fn set_routine_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.routine_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataBigqueryDatasetAccessElRoutineEl {
    type O = BlockAssignable<DataBigqueryDatasetAccessElRoutineEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataBigqueryDatasetAccessElRoutineElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryDatasetAccessElRoutineElRef {
        DataBigqueryDatasetAccessElRoutineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetAccessElRoutineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc= "Get a reference to the value of field `routine_id` after provisioning.\n"]
    pub fn routine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routine_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetAccessElViewEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_id: Option<PrimField<String>>,
}

impl DataBigqueryDatasetAccessElViewEl {
    #[doc= "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `table_id`.\n"]
    pub fn set_table_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataBigqueryDatasetAccessElViewEl {
    type O = BlockAssignable<DataBigqueryDatasetAccessElViewEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataBigqueryDatasetAccessElViewElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryDatasetAccessElViewElRef {
        DataBigqueryDatasetAccessElViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetAccessElViewElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc= "Get a reference to the value of field `table_id` after provisioning.\n"]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<ListField<DataBigqueryDatasetAccessElDatasetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_by_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_member: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routine: Option<ListField<DataBigqueryDatasetAccessElRoutineEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    special_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_by_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view: Option<ListField<DataBigqueryDatasetAccessElViewEl>>,
}

impl DataBigqueryDatasetAccessEl {
    #[doc= "Set the field `dataset`.\n"]
    pub fn set_dataset(mut self, v: impl Into<ListField<DataBigqueryDatasetAccessElDatasetEl>>) -> Self {
        self.dataset = Some(v.into());
        self
    }

    #[doc= "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }

    #[doc= "Set the field `group_by_email`.\n"]
    pub fn set_group_by_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_by_email = Some(v.into());
        self
    }

    #[doc= "Set the field `iam_member`.\n"]
    pub fn set_iam_member(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam_member = Some(v.into());
        self
    }

    #[doc= "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }

    #[doc= "Set the field `routine`.\n"]
    pub fn set_routine(mut self, v: impl Into<ListField<DataBigqueryDatasetAccessElRoutineEl>>) -> Self {
        self.routine = Some(v.into());
        self
    }

    #[doc= "Set the field `special_group`.\n"]
    pub fn set_special_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.special_group = Some(v.into());
        self
    }

    #[doc= "Set the field `user_by_email`.\n"]
    pub fn set_user_by_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_by_email = Some(v.into());
        self
    }

    #[doc= "Set the field `view`.\n"]
    pub fn set_view(mut self, v: impl Into<ListField<DataBigqueryDatasetAccessElViewEl>>) -> Self {
        self.view = Some(v.into());
        self
    }
}

impl ToListMappable for DataBigqueryDatasetAccessEl {
    type O = BlockAssignable<DataBigqueryDatasetAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataBigqueryDatasetAccessElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryDatasetAccessElRef {
        DataBigqueryDatasetAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> ListRef<DataBigqueryDatasetAccessElDatasetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc= "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc= "Get a reference to the value of field `group_by_email` after provisioning.\n"]
    pub fn group_by_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_by_email", self.base))
    }

    #[doc= "Get a reference to the value of field `iam_member` after provisioning.\n"]
    pub fn iam_member(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_member", self.base))
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc= "Get a reference to the value of field `routine` after provisioning.\n"]
    pub fn routine(&self) -> ListRef<DataBigqueryDatasetAccessElRoutineElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routine", self.base))
    }

    #[doc= "Get a reference to the value of field `special_group` after provisioning.\n"]
    pub fn special_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.special_group", self.base))
    }

    #[doc= "Get a reference to the value of field `user_by_email` after provisioning.\n"]
    pub fn user_by_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_by_email", self.base))
    }

    #[doc= "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> ListRef<DataBigqueryDatasetAccessElViewElRef> {
        ListRef::new(self.shared().clone(), format!("{}.view", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryDatasetDefaultEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}

impl DataBigqueryDatasetDefaultEncryptionConfigurationEl {
    #[doc= "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataBigqueryDatasetDefaultEncryptionConfigurationEl {
    type O = BlockAssignable<DataBigqueryDatasetDefaultEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataBigqueryDatasetDefaultEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryDatasetDefaultEncryptionConfigurationElRef {
        DataBigqueryDatasetDefaultEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryDatasetDefaultEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
