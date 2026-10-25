#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBigqueryTableData_ {
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
    table_id: terrars::PrimField<std::string::String>,
}

struct DataBigqueryTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBigqueryTableData_>,
}

#[derive(Clone)]
pub struct DataBigqueryTable(std::rc::Rc<DataBigqueryTable_>);

impl DataBigqueryTable {
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

    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `biglake_configuration` after provisioning.\nSpecifies the configuration of a BigLake managed table."]
    pub fn biglake_configuration(&self) -> terrars::ListRef<DataBigqueryTable_BiglakeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.biglake_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `clustering` after provisioning.\nSpecifies column names to use for data clustering. Up to four top-level columns are allowed, and should be specified in descending priority order."]
    pub fn clustering(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clustering", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this table was created, in milliseconds since the epoch."]
    pub fn creation_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset ID to create the table in. Changing this forces a new resource to be created."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the instance. When the field is set to true or unset in Terraform state, a terraform apply or terraform destroy that would delete the table will fail. When the field is set to false, deleting the table is allowed."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe field description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_configuration` after provisioning.\nSpecifies how the table should be encrypted. If left blank, the table will be encrypted with a Google-managed key; that process is transparent to the user."]
    pub fn encryption_configuration(&self) -> terrars::ListRef<DataBigqueryTable_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA hash of the resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiration_time` after provisioning.\nThe time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
    pub fn expiration_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_catalog_table_options` after provisioning.\nOptions defining open source compatible table."]
    pub fn external_catalog_table_options(&self) -> terrars::ListRef<DataBigqueryTable_ExternalCatalogTableOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.external_catalog_table_options", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `external_data_configuration` after provisioning.\nDescribes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table."]
    pub fn external_data_configuration(&self) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_data_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the table."]
    pub fn friendly_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated_schema_columns` after provisioning.\n(Output-only) A list of autogenerated schema fields."]
    pub fn generated_schema_columns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_schema_columns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_auto_generated_schema` after provisioning.\nWhether Terraform will prevent implicitly added columns in schema from showing diff."]
    pub fn ignore_auto_generated_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_auto_generated_schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_schema_changes` after provisioning.\nMention which fields in schema are to be ignored"]
    pub fn ignore_schema_changes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ignore_schema_changes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA mapping of labels to assign to the resource.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this table was last modified, in milliseconds since the epoch."]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `materialized_view` after provisioning.\nIf specified, configures this table as a materialized view."]
    pub fn materialized_view(&self) -> terrars::ListRef<DataBigqueryTable_MaterializedViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.materialized_view", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_staleness` after provisioning.\nThe maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of [SQL IntervalValue type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type)."]
    pub fn max_staleness(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_staleness", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_bytes` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn num_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_long_term_bytes` after provisioning.\nThe number of bytes in the table that are considered \"long-term storage\"."]
    pub fn num_long_term_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_long_term_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_rows` after provisioning.\nThe number of rows of data in this table, excluding any data in the streaming buffer."]
    pub fn num_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_rows", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `range_partitioning` after provisioning.\nIf specified, configures range-based partitioning for this table."]
    pub fn range_partitioning(&self) -> terrars::ListRef<DataBigqueryTable_RangePartitioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range_partitioning", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn require_partition_filter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_tags` after provisioning.\nThe tags attached to this table. Tag keys are globally unique. Tag key is expected to be in the namespaced format, for example \"123456789012/environment\" where 123456789012 is the ID of the parent organization or project resource for this tag key. Tag value is expected to be the short name, for example \"Production\"."]
    pub fn resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nA JSON schema for the table."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema_foreign_type_info` after provisioning.\nSpecifies metadata of the foreign data type definition in field schema."]
    pub fn schema_foreign_type_info(&self) -> terrars::ListRef<DataBigqueryTable_SchemaForeignTypeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_foreign_type_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_constraints` after provisioning.\nDefines the primary key and foreign keys."]
    pub fn table_constraints(&self) -> terrars::ListRef<DataBigqueryTable_TableConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_constraints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nA unique ID for the resource. Changing this forces a new resource to be created."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_metadata_view` after provisioning.\nView sets the optional parameter \"view\": Specifies the view that determines which table information is returned. By default, basic table information and storage statistics (STORAGE_STATS) are returned. Possible values: TABLE_METADATA_VIEW_UNSPECIFIED, BASIC, STORAGE_STATS, FULL"]
    pub fn table_metadata_view(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_metadata_view", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_replication_info` after provisioning.\nReplication info of a table created using \"AS REPLICA\" DDL like: \"CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv\"."]
    pub fn table_replication_info(&self) -> terrars::ListRef<DataBigqueryTable_TableReplicationInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_replication_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_partitioning` after provisioning.\nIf specified, configures time-based partitioning for this table."]
    pub fn time_partitioning(&self) -> terrars::ListRef<DataBigqueryTable_TimePartitioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_partitioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nDescribes the table type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `view` after provisioning.\nIf specified, configures this table as a view."]
    pub fn view(&self) -> terrars::ListRef<DataBigqueryTable_ViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.view", self.extract_ref()))
    }
}

impl terrars::Referable for DataBigqueryTable {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBigqueryTable { }

impl terrars::ToListMappable for DataBigqueryTable {
    type O = terrars::ListRef<DataBigqueryTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBigqueryTable_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_bigquery_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBigqueryTable {
    pub tf_id: std::string::String,
    #[doc = "The dataset ID to create the table in. Changing this forces a new resource to be created."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "A unique ID for the resource. Changing this forces a new resource to be created."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildDataBigqueryTable {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBigqueryTable {
        let out = DataBigqueryTable(std::rc::Rc::new(DataBigqueryTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBigqueryTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                dataset_id: self.dataset_id,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                table_id: self.table_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBigqueryTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBigqueryTableRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `biglake_configuration` after provisioning.\nSpecifies the configuration of a BigLake managed table."]
    pub fn biglake_configuration(&self) -> terrars::ListRef<DataBigqueryTable_BiglakeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.biglake_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `clustering` after provisioning.\nSpecifies column names to use for data clustering. Up to four top-level columns are allowed, and should be specified in descending priority order."]
    pub fn clustering(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clustering", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this table was created, in milliseconds since the epoch."]
    pub fn creation_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset ID to create the table in. Changing this forces a new resource to be created."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the instance. When the field is set to true or unset in Terraform state, a terraform apply or terraform destroy that would delete the table will fail. When the field is set to false, deleting the table is allowed."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe field description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_configuration` after provisioning.\nSpecifies how the table should be encrypted. If left blank, the table will be encrypted with a Google-managed key; that process is transparent to the user."]
    pub fn encryption_configuration(&self) -> terrars::ListRef<DataBigqueryTable_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA hash of the resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expiration_time` after provisioning.\nThe time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
    pub fn expiration_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_catalog_table_options` after provisioning.\nOptions defining open source compatible table."]
    pub fn external_catalog_table_options(&self) -> terrars::ListRef<DataBigqueryTable_ExternalCatalogTableOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.external_catalog_table_options", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `external_data_configuration` after provisioning.\nDescribes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table."]
    pub fn external_data_configuration(&self) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_data_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the table."]
    pub fn friendly_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `generated_schema_columns` after provisioning.\n(Output-only) A list of autogenerated schema fields."]
    pub fn generated_schema_columns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_schema_columns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_auto_generated_schema` after provisioning.\nWhether Terraform will prevent implicitly added columns in schema from showing diff."]
    pub fn ignore_auto_generated_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_auto_generated_schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_schema_changes` after provisioning.\nMention which fields in schema are to be ignored"]
    pub fn ignore_schema_changes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ignore_schema_changes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA mapping of labels to assign to the resource.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this table was last modified, in milliseconds since the epoch."]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `materialized_view` after provisioning.\nIf specified, configures this table as a materialized view."]
    pub fn materialized_view(&self) -> terrars::ListRef<DataBigqueryTable_MaterializedViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.materialized_view", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_staleness` after provisioning.\nThe maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of [SQL IntervalValue type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type)."]
    pub fn max_staleness(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_staleness", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_bytes` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn num_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_long_term_bytes` after provisioning.\nThe number of bytes in the table that are considered \"long-term storage\"."]
    pub fn num_long_term_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_long_term_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `num_rows` after provisioning.\nThe number of rows of data in this table, excluding any data in the streaming buffer."]
    pub fn num_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_rows", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `range_partitioning` after provisioning.\nIf specified, configures range-based partitioning for this table."]
    pub fn range_partitioning(&self) -> terrars::ListRef<DataBigqueryTable_RangePartitioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range_partitioning", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn require_partition_filter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_tags` after provisioning.\nThe tags attached to this table. Tag keys are globally unique. Tag key is expected to be in the namespaced format, for example \"123456789012/environment\" where 123456789012 is the ID of the parent organization or project resource for this tag key. Tag value is expected to be the short name, for example \"Production\"."]
    pub fn resource_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nA JSON schema for the table."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema_foreign_type_info` after provisioning.\nSpecifies metadata of the foreign data type definition in field schema."]
    pub fn schema_foreign_type_info(&self) -> terrars::ListRef<DataBigqueryTable_SchemaForeignTypeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_foreign_type_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_constraints` after provisioning.\nDefines the primary key and foreign keys."]
    pub fn table_constraints(&self) -> terrars::ListRef<DataBigqueryTable_TableConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_constraints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nA unique ID for the resource. Changing this forces a new resource to be created."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_metadata_view` after provisioning.\nView sets the optional parameter \"view\": Specifies the view that determines which table information is returned. By default, basic table information and storage statistics (STORAGE_STATS) are returned. Possible values: TABLE_METADATA_VIEW_UNSPECIFIED, BASIC, STORAGE_STATS, FULL"]
    pub fn table_metadata_view(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_metadata_view", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_replication_info` after provisioning.\nReplication info of a table created using \"AS REPLICA\" DDL like: \"CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv\"."]
    pub fn table_replication_info(&self) -> terrars::ListRef<DataBigqueryTable_TableReplicationInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_replication_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `time_partitioning` after provisioning.\nIf specified, configures time-based partitioning for this table."]
    pub fn time_partitioning(&self) -> terrars::ListRef<DataBigqueryTable_TimePartitioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_partitioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nDescribes the table type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `view` after provisioning.\nIf specified, configures this table as a view."]
    pub fn view(&self) -> terrars::ListRef<DataBigqueryTable_ViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.view", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_BiglakeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_BiglakeConfiguration {
    #[doc = "Set the field `connection_id`.\n"]
    pub fn set_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_format`.\n"]
    pub fn set_file_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_uri`.\n"]
    pub fn set_storage_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_format`.\n"]
    pub fn set_table_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_BiglakeConfiguration {
    type O = terrars::BlockAssignable<DataBigqueryTable_BiglakeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_BiglakeConfiguration {}

impl BuildDataBigqueryTable_BiglakeConfiguration {
    pub fn build(self) -> DataBigqueryTable_BiglakeConfiguration {
        DataBigqueryTable_BiglakeConfiguration {
            connection_id: core::default::Default::default(),
            file_format: core::default::Default::default(),
            storage_uri: core::default::Default::default(),
            table_format: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_BiglakeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_BiglakeConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryTable_BiglakeConfigurationRef {
        DataBigqueryTable_BiglakeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_BiglakeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_format` after provisioning.\n"]
    pub fn file_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_format", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_uri` after provisioning.\n"]
    pub fn storage_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `table_format` after provisioning.\n"]
    pub fn table_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_format", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_EncryptionConfiguration {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_version`.\n"]
    pub fn set_kms_key_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_EncryptionConfiguration {
    type O = terrars::BlockAssignable<DataBigqueryTable_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_EncryptionConfiguration {}

impl BuildDataBigqueryTable_EncryptionConfiguration {
    pub fn build(self) -> DataBigqueryTable_EncryptionConfiguration {
        DataBigqueryTable_EncryptionConfiguration {
            kms_key_name: core::default::Default::default(),
            kms_key_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_EncryptionConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryTable_EncryptionConfigurationRef {
        DataBigqueryTable_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_version` after provisioning.\n"]
    pub fn kms_key_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serialization_library: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serialization_library`.\n"]
    pub fn set_serialization_library(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.serialization_library = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {}

impl BuildDataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
    pub fn build(self) -> DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
        DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            serialization_library: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef {
        DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `serialization_library` after provisioning.\n"]
    pub fn serialization_library(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serialization_library", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serde_info: std
    ::option
    ::Option<terrars::ListField<DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo>>,
}

impl DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
    #[doc = "Set the field `input_format`.\n"]
    pub fn set_input_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location_uri`.\n"]
    pub fn set_location_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_format`.\n"]
    pub fn set_output_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serde_info`.\n"]
    pub fn set_serde_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo>,
                    >,
    ) -> Self {
        self.serde_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {}

impl BuildDataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
    pub fn build(self) -> DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
        DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
            input_format: core::default::Default::default(),
            location_uri: core::default::Default::default(),
            output_format: core::default::Default::default(),
            serde_info: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef {
        DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `input_format` after provisioning.\n"]
    pub fn input_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_format", self.base))
    }

    #[doc = "Get a reference to the value of field `location_uri` after provisioning.\n"]
    pub fn location_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.base))
    }

    #[doc = "Get a reference to the value of field `serde_info` after provisioning.\n"]
    pub fn serde_info(
        &self,
    ) -> terrars::ListRef<DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serde_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalCatalogTableOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_descriptor: std
    ::option
    ::Option<terrars::ListField<DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor>>,
}

impl DataBigqueryTable_ExternalCatalogTableOptions {
    #[doc = "Set the field `connection_id`.\n"]
    pub fn set_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_descriptor`.\n"]
    pub fn set_storage_descriptor(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor>>,
    ) -> Self {
        self.storage_descriptor = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalCatalogTableOptions {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalCatalogTableOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalCatalogTableOptions {}

impl BuildDataBigqueryTable_ExternalCatalogTableOptions {
    pub fn build(self) -> DataBigqueryTable_ExternalCatalogTableOptions {
        DataBigqueryTable_ExternalCatalogTableOptions {
            connection_id: core::default::Default::default(),
            parameters: core::default::Default::default(),
            storage_descriptor: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalCatalogTableOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalCatalogTableOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalCatalogTableOptionsRef {
        DataBigqueryTable_ExternalCatalogTableOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalCatalogTableOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(
        &self,
    ) -> terrars::ListRef<DataBigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalDataConfiguration_AvroOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_avro_logical_types: std::option::Option<terrars::PrimField<bool>>,
}

impl DataBigqueryTable_ExternalDataConfiguration_AvroOptions {
    #[doc = "Set the field `use_avro_logical_types`.\n"]
    pub fn set_use_avro_logical_types(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_avro_logical_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalDataConfiguration_AvroOptions {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalDataConfiguration_AvroOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalDataConfiguration_AvroOptions {}

impl BuildDataBigqueryTable_ExternalDataConfiguration_AvroOptions {
    pub fn build(self) -> DataBigqueryTable_ExternalDataConfiguration_AvroOptions {
        DataBigqueryTable_ExternalDataConfiguration_AvroOptions {
            use_avro_logical_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalDataConfiguration_AvroOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalDataConfiguration_AvroOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalDataConfiguration_AvroOptionsRef {
        DataBigqueryTable_ExternalDataConfiguration_AvroOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalDataConfiguration_AvroOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `use_avro_logical_types` after provisioning.\n"]
    pub fn use_avro_logical_types(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_avro_logical_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    only_read_latest: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifier_encoded: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualifier_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_name`.\n"]
    pub fn set_field_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `only_read_latest`.\n"]
    pub fn set_only_read_latest(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.only_read_latest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qualifier_encoded`.\n"]
    pub fn set_qualifier_encoded(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.qualifier_encoded = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qualifier_string`.\n"]
    pub fn set_qualifier_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.qualifier_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
    type O =
        terrars::BlockAssignable<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {}

impl BuildDataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
    pub fn build(self) -> DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
        DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
            encoding: core::default::Default::default(),
            field_name: core::default::Default::default(),
            only_read_latest: core::default::Default::default(),
            qualifier_encoded: core::default::Default::default(),
            qualifier_string: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef {
        DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `field_name` after provisioning.\n"]
    pub fn field_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }

    #[doc = "Get a reference to the value of field `only_read_latest` after provisioning.\n"]
    pub fn only_read_latest(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.only_read_latest", self.base))
    }

    #[doc = "Get a reference to the value of field `qualifier_encoded` after provisioning.\n"]
    pub fn qualifier_encoded(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier_encoded", self.base))
    }

    #[doc = "Get a reference to the value of field `qualifier_string` after provisioning.\n"]
    pub fn qualifier_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier_string", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std
    ::option
    ::Option<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    family_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    only_read_latest: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
    #[doc = "Set the field `column`.\n"]
    pub fn set_column(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column>,
                    >,
    ) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `family_id`.\n"]
    pub fn set_family_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.family_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `only_read_latest`.\n"]
    pub fn set_only_read_latest(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.only_read_latest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {}

impl BuildDataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
    pub fn build(self) -> DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
        DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
            column: core::default::Default::default(),
            encoding: core::default::Default::default(),
            family_id: core::default::Default::default(),
            only_read_latest: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef {
        DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\n"]
    pub fn column(
        &self,
    ) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `family_id` after provisioning.\n"]
    pub fn family_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family_id", self.base))
    }

    #[doc = "Get a reference to the value of field `only_read_latest` after provisioning.\n"]
    pub fn only_read_latest(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.only_read_latest", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalDataConfiguration_BigtableOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_family: std
    ::option
    ::Option<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_unspecified_column_families: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_column_families_as_json: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_rowkey_as_string: std::option::Option<terrars::PrimField<bool>>,
}

impl DataBigqueryTable_ExternalDataConfiguration_BigtableOptions {
    #[doc = "Set the field `column_family`.\n"]
    pub fn set_column_family(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily>,
                    >,
    ) -> Self {
        self.column_family = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_unspecified_column_families`.\n"]
    pub fn set_ignore_unspecified_column_families(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.ignore_unspecified_column_families = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_column_families_as_json`.\n"]
    pub fn set_output_column_families_as_json(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.output_column_families_as_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_rowkey_as_string`.\n"]
    pub fn set_read_rowkey_as_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_rowkey_as_string = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalDataConfiguration_BigtableOptions {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalDataConfiguration_BigtableOptions {}

impl BuildDataBigqueryTable_ExternalDataConfiguration_BigtableOptions {
    pub fn build(self) -> DataBigqueryTable_ExternalDataConfiguration_BigtableOptions {
        DataBigqueryTable_ExternalDataConfiguration_BigtableOptions {
            column_family: core::default::Default::default(),
            ignore_unspecified_column_families: core::default::Default::default(),
            output_column_families_as_json: core::default::Default::default(),
            read_rowkey_as_string: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalDataConfiguration_BigtableOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalDataConfiguration_BigtableOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalDataConfiguration_BigtableOptionsRef {
        DataBigqueryTable_ExternalDataConfiguration_BigtableOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalDataConfiguration_BigtableOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_family` after provisioning.\n"]
    pub fn column_family(
        &self,
    ) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_family", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_unspecified_column_families` after provisioning.\n"]
    pub fn ignore_unspecified_column_families(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_unspecified_column_families", self.base))
    }

    #[doc = "Get a reference to the value of field `output_column_families_as_json` after provisioning.\n"]
    pub fn output_column_families_as_json(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_column_families_as_json", self.base))
    }

    #[doc = "Get a reference to the value of field `read_rowkey_as_string` after provisioning.\n"]
    pub fn read_rowkey_as_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_rowkey_as_string", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalDataConfiguration_CsvOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_jagged_rows: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_quoted_newlines: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quote: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_leading_rows: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_column_match: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_ExternalDataConfiguration_CsvOptions {
    #[doc = "Set the field `allow_jagged_rows`.\n"]
    pub fn set_allow_jagged_rows(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_jagged_rows = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_quoted_newlines`.\n"]
    pub fn set_allow_quoted_newlines(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_quoted_newlines = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_delimiter`.\n"]
    pub fn set_field_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quote`.\n"]
    pub fn set_quote(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.quote = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_leading_rows`.\n"]
    pub fn set_skip_leading_rows(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.skip_leading_rows = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_column_match`.\n"]
    pub fn set_source_column_match(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_column_match = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalDataConfiguration_CsvOptions {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalDataConfiguration_CsvOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalDataConfiguration_CsvOptions {}

impl BuildDataBigqueryTable_ExternalDataConfiguration_CsvOptions {
    pub fn build(self) -> DataBigqueryTable_ExternalDataConfiguration_CsvOptions {
        DataBigqueryTable_ExternalDataConfiguration_CsvOptions {
            allow_jagged_rows: core::default::Default::default(),
            allow_quoted_newlines: core::default::Default::default(),
            encoding: core::default::Default::default(),
            field_delimiter: core::default::Default::default(),
            quote: core::default::Default::default(),
            skip_leading_rows: core::default::Default::default(),
            source_column_match: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalDataConfiguration_CsvOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalDataConfiguration_CsvOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalDataConfiguration_CsvOptionsRef {
        DataBigqueryTable_ExternalDataConfiguration_CsvOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalDataConfiguration_CsvOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_jagged_rows` after provisioning.\n"]
    pub fn allow_jagged_rows(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_jagged_rows", self.base))
    }

    #[doc = "Get a reference to the value of field `allow_quoted_newlines` after provisioning.\n"]
    pub fn allow_quoted_newlines(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_quoted_newlines", self.base))
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `field_delimiter` after provisioning.\n"]
    pub fn field_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `quote` after provisioning.\n"]
    pub fn quote(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quote", self.base))
    }

    #[doc = "Get a reference to the value of field `skip_leading_rows` after provisioning.\n"]
    pub fn skip_leading_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_leading_rows", self.base))
    }

    #[doc = "Get a reference to the value of field `source_column_match` after provisioning.\n"]
    pub fn source_column_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_column_match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_leading_rows: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_leading_rows`.\n"]
    pub fn set_skip_leading_rows(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.skip_leading_rows = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {}

impl BuildDataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
    pub fn build(self) -> DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
        DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
            range: core::default::Default::default(),
            skip_leading_rows: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef {
        DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }

    #[doc = "Get a reference to the value of field `skip_leading_rows` after provisioning.\n"]
    pub fn skip_leading_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_leading_rows", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_partition_filter: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_uri_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_partition_filter`.\n"]
    pub fn set_require_partition_filter(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_partition_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_uri_prefix`.\n"]
    pub fn set_source_uri_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_uri_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {}

impl BuildDataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
    pub fn build(self) -> DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
        DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
            mode: core::default::Default::default(),
            require_partition_filter: core::default::Default::default(),
            source_uri_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef {
        DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `require_partition_filter` after provisioning.\n"]
    pub fn require_partition_filter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `source_uri_prefix` after provisioning.\n"]
    pub fn source_uri_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_uri_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalDataConfiguration_JsonOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_ExternalDataConfiguration_JsonOptions {
    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalDataConfiguration_JsonOptions {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalDataConfiguration_JsonOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalDataConfiguration_JsonOptions {}

impl BuildDataBigqueryTable_ExternalDataConfiguration_JsonOptions {
    pub fn build(self) -> DataBigqueryTable_ExternalDataConfiguration_JsonOptions {
        DataBigqueryTable_ExternalDataConfiguration_JsonOptions { encoding: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalDataConfiguration_JsonOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalDataConfiguration_JsonOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalDataConfiguration_JsonOptionsRef {
        DataBigqueryTable_ExternalDataConfiguration_JsonOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalDataConfiguration_JsonOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalDataConfiguration_ParquetOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_list_inference: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_as_string: std::option::Option<terrars::PrimField<bool>>,
}

impl DataBigqueryTable_ExternalDataConfiguration_ParquetOptions {
    #[doc = "Set the field `enable_list_inference`.\n"]
    pub fn set_enable_list_inference(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_list_inference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enum_as_string`.\n"]
    pub fn set_enum_as_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enum_as_string = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalDataConfiguration_ParquetOptions {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalDataConfiguration_ParquetOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalDataConfiguration_ParquetOptions {}

impl BuildDataBigqueryTable_ExternalDataConfiguration_ParquetOptions {
    pub fn build(self) -> DataBigqueryTable_ExternalDataConfiguration_ParquetOptions {
        DataBigqueryTable_ExternalDataConfiguration_ParquetOptions {
            enable_list_inference: core::default::Default::default(),
            enum_as_string: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalDataConfiguration_ParquetOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalDataConfiguration_ParquetOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalDataConfiguration_ParquetOptionsRef {
        DataBigqueryTable_ExternalDataConfiguration_ParquetOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalDataConfiguration_ParquetOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_list_inference` after provisioning.\n"]
    pub fn enable_list_inference(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_list_inference", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_as_string` after provisioning.\n"]
    pub fn enum_as_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enum_as_string", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_ExternalDataConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autodetect: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro_options: std::option::Option<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_AvroOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigtable_options: std
    ::option
    ::Option<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_options: std::option::Option<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_CsvOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decimal_target_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_set_spec_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_sheets_options: std
    ::option
    ::Option<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_partitioning_options: std
    ::option
    ::Option<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_unknown_values: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_extension: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_options: std::option::Option<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_JsonOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_bad_records: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_cache_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_metadata: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parquet_options: std::option::Option<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_ParquetOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_file_schema_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataBigqueryTable_ExternalDataConfiguration {
    #[doc = "Set the field `autodetect`.\n"]
    pub fn set_autodetect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.autodetect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `avro_options`.\n"]
    pub fn set_avro_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_AvroOptions>>,
    ) -> Self {
        self.avro_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigtable_options`.\n"]
    pub fn set_bigtable_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_BigtableOptions>>,
    ) -> Self {
        self.bigtable_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compression`.\n"]
    pub fn set_compression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_id`.\n"]
    pub fn set_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_CsvOptions>>,
    ) -> Self {
        self.csv_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `decimal_target_types`.\n"]
    pub fn set_decimal_target_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.decimal_target_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_set_spec_type`.\n"]
    pub fn set_file_set_spec_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.file_set_spec_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_sheets_options`.\n"]
    pub fn set_google_sheets_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions>>,
    ) -> Self {
        self.google_sheets_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hive_partitioning_options`.\n"]
    pub fn set_hive_partitioning_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions>>,
    ) -> Self {
        self.hive_partitioning_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_unknown_values`.\n"]
    pub fn set_ignore_unknown_values(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_unknown_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `json_extension`.\n"]
    pub fn set_json_extension(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json_extension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `json_options`.\n"]
    pub fn set_json_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_JsonOptions>>,
    ) -> Self {
        self.json_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_bad_records`.\n"]
    pub fn set_max_bad_records(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_bad_records = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_cache_mode`.\n"]
    pub fn set_metadata_cache_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metadata_cache_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_metadata`.\n"]
    pub fn set_object_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parquet_options`.\n"]
    pub fn set_parquet_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryTable_ExternalDataConfiguration_ParquetOptions>>,
    ) -> Self {
        self.parquet_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reference_file_schema_uri`.\n"]
    pub fn set_reference_file_schema_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.reference_file_schema_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_format`.\n"]
    pub fn set_source_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_uris`.\n"]
    pub fn set_source_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_ExternalDataConfiguration {
    type O = terrars::BlockAssignable<DataBigqueryTable_ExternalDataConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_ExternalDataConfiguration {}

impl BuildDataBigqueryTable_ExternalDataConfiguration {
    pub fn build(self) -> DataBigqueryTable_ExternalDataConfiguration {
        DataBigqueryTable_ExternalDataConfiguration {
            autodetect: core::default::Default::default(),
            avro_options: core::default::Default::default(),
            bigtable_options: core::default::Default::default(),
            compression: core::default::Default::default(),
            connection_id: core::default::Default::default(),
            csv_options: core::default::Default::default(),
            decimal_target_types: core::default::Default::default(),
            file_set_spec_type: core::default::Default::default(),
            google_sheets_options: core::default::Default::default(),
            hive_partitioning_options: core::default::Default::default(),
            ignore_unknown_values: core::default::Default::default(),
            json_extension: core::default::Default::default(),
            json_options: core::default::Default::default(),
            max_bad_records: core::default::Default::default(),
            metadata_cache_mode: core::default::Default::default(),
            object_metadata: core::default::Default::default(),
            parquet_options: core::default::Default::default(),
            reference_file_schema_uri: core::default::Default::default(),
            schema: core::default::Default::default(),
            source_format: core::default::Default::default(),
            source_uris: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ExternalDataConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ExternalDataConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_ExternalDataConfigurationRef {
        DataBigqueryTable_ExternalDataConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ExternalDataConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autodetect` after provisioning.\n"]
    pub fn autodetect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autodetect", self.base))
    }

    #[doc = "Get a reference to the value of field `avro_options` after provisioning.\n"]
    pub fn avro_options(&self) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfiguration_AvroOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro_options", self.base))
    }

    #[doc = "Get a reference to the value of field `bigtable_options` after provisioning.\n"]
    pub fn bigtable_options(&self) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfiguration_BigtableOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigtable_options", self.base))
    }

    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(&self) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfiguration_CsvOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_options", self.base))
    }

    #[doc = "Get a reference to the value of field `decimal_target_types` after provisioning.\n"]
    pub fn decimal_target_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.decimal_target_types", self.base))
    }

    #[doc = "Get a reference to the value of field `file_set_spec_type` after provisioning.\n"]
    pub fn file_set_spec_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_set_spec_type", self.base))
    }

    #[doc = "Get a reference to the value of field `google_sheets_options` after provisioning.\n"]
    pub fn google_sheets_options(
        &self,
    ) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_sheets_options", self.base))
    }

    #[doc = "Get a reference to the value of field `hive_partitioning_options` after provisioning.\n"]
    pub fn hive_partitioning_options(
        &self,
    ) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_partitioning_options", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_unknown_values` after provisioning.\n"]
    pub fn ignore_unknown_values(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_unknown_values", self.base))
    }

    #[doc = "Get a reference to the value of field `json_extension` after provisioning.\n"]
    pub fn json_extension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_extension", self.base))
    }

    #[doc = "Get a reference to the value of field `json_options` after provisioning.\n"]
    pub fn json_options(&self) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfiguration_JsonOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_options", self.base))
    }

    #[doc = "Get a reference to the value of field `max_bad_records` after provisioning.\n"]
    pub fn max_bad_records(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_bad_records", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_cache_mode` after provisioning.\n"]
    pub fn metadata_cache_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_cache_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `object_metadata` after provisioning.\n"]
    pub fn object_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `parquet_options` after provisioning.\n"]
    pub fn parquet_options(&self) -> terrars::ListRef<DataBigqueryTable_ExternalDataConfiguration_ParquetOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parquet_options", self.base))
    }

    #[doc = "Get a reference to the value of field `reference_file_schema_uri` after provisioning.\n"]
    pub fn reference_file_schema_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_file_schema_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `source_format` after provisioning.\n"]
    pub fn source_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_format", self.base))
    }

    #[doc = "Get a reference to the value of field `source_uris` after provisioning.\n"]
    pub fn source_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_uris", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_MaterializedView {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_non_incremental_definition: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_refresh: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_interval_ms: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBigqueryTable_MaterializedView {
    #[doc = "Set the field `allow_non_incremental_definition`.\n"]
    pub fn set_allow_non_incremental_definition(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_non_incremental_definition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_refresh`.\n"]
    pub fn set_enable_refresh(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_refresh = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_interval_ms`.\n"]
    pub fn set_refresh_interval_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.refresh_interval_ms = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_MaterializedView {
    type O = terrars::BlockAssignable<DataBigqueryTable_MaterializedView>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_MaterializedView {}

impl BuildDataBigqueryTable_MaterializedView {
    pub fn build(self) -> DataBigqueryTable_MaterializedView {
        DataBigqueryTable_MaterializedView {
            allow_non_incremental_definition: core::default::Default::default(),
            enable_refresh: core::default::Default::default(),
            query: core::default::Default::default(),
            refresh_interval_ms: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_MaterializedViewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_MaterializedViewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryTable_MaterializedViewRef {
        DataBigqueryTable_MaterializedViewRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_MaterializedViewRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_non_incremental_definition` after provisioning.\n"]
    pub fn allow_non_incremental_definition(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_non_incremental_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_refresh` after provisioning.\n"]
    pub fn enable_refresh(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_refresh", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_interval_ms` after provisioning.\n"]
    pub fn refresh_interval_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_interval_ms", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_RangePartitioning_Range {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBigqueryTable_RangePartitioning_Range {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_RangePartitioning_Range {
    type O = terrars::BlockAssignable<DataBigqueryTable_RangePartitioning_Range>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_RangePartitioning_Range {}

impl BuildDataBigqueryTable_RangePartitioning_Range {
    pub fn build(self) -> DataBigqueryTable_RangePartitioning_Range {
        DataBigqueryTable_RangePartitioning_Range {
            end: core::default::Default::default(),
            interval: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_RangePartitioning_RangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_RangePartitioning_RangeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryTable_RangePartitioning_RangeRef {
        DataBigqueryTable_RangePartitioning_RangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_RangePartitioning_RangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_RangePartitioning {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<terrars::ListField<DataBigqueryTable_RangePartitioning_Range>>,
}

impl DataBigqueryTable_RangePartitioning {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryTable_RangePartitioning_Range>>,
    ) -> Self {
        self.range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_RangePartitioning {
    type O = terrars::BlockAssignable<DataBigqueryTable_RangePartitioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_RangePartitioning {}

impl BuildDataBigqueryTable_RangePartitioning {
    pub fn build(self) -> DataBigqueryTable_RangePartitioning {
        DataBigqueryTable_RangePartitioning {
            field: core::default::Default::default(),
            range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_RangePartitioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_RangePartitioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryTable_RangePartitioningRef {
        DataBigqueryTable_RangePartitioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_RangePartitioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::ListRef<DataBigqueryTable_RangePartitioning_RangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_SchemaForeignTypeInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    type_system: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_SchemaForeignTypeInfo {
    #[doc = "Set the field `type_system`.\n"]
    pub fn set_type_system(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_system = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_SchemaForeignTypeInfo {
    type O = terrars::BlockAssignable<DataBigqueryTable_SchemaForeignTypeInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_SchemaForeignTypeInfo {}

impl BuildDataBigqueryTable_SchemaForeignTypeInfo {
    pub fn build(self) -> DataBigqueryTable_SchemaForeignTypeInfo {
        DataBigqueryTable_SchemaForeignTypeInfo { type_system: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_SchemaForeignTypeInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_SchemaForeignTypeInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryTable_SchemaForeignTypeInfoRef {
        DataBigqueryTable_SchemaForeignTypeInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_SchemaForeignTypeInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_system` after provisioning.\n"]
    pub fn type_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_system", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referenced_column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referencing_column: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
    #[doc = "Set the field `referenced_column`.\n"]
    pub fn set_referenced_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.referenced_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `referencing_column`.\n"]
    pub fn set_referencing_column(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.referencing_column = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
    type O = terrars::BlockAssignable<DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {}

impl BuildDataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
    pub fn build(self) -> DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
        DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
            referenced_column: core::default::Default::default(),
            referencing_column: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef {
        DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `referenced_column` after provisioning.\n"]
    pub fn referenced_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.referenced_column", self.base))
    }

    #[doc = "Get a reference to the value of field `referencing_column` after provisioning.\n"]
    pub fn referencing_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.referencing_column", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
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

impl terrars::ToListMappable for DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
    type O = terrars::BlockAssignable<DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {}

impl BuildDataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
    pub fn build(self) -> DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
        DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef {
        DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef {
    #[allow(dead_code)]
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
pub struct DataBigqueryTable_TableConstraints_ForeignKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_references: std
    ::option
    ::Option<terrars::ListField<DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referenced_table: std
    ::option
    ::Option<terrars::ListField<DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable>>,
}

impl DataBigqueryTable_TableConstraints_ForeignKeys {
    #[doc = "Set the field `column_references`.\n"]
    pub fn set_column_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences>>,
    ) -> Self {
        self.column_references = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `referenced_table`.\n"]
    pub fn set_referenced_table(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable>>,
    ) -> Self {
        self.referenced_table = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_TableConstraints_ForeignKeys {
    type O = terrars::BlockAssignable<DataBigqueryTable_TableConstraints_ForeignKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_TableConstraints_ForeignKeys {}

impl BuildDataBigqueryTable_TableConstraints_ForeignKeys {
    pub fn build(self) -> DataBigqueryTable_TableConstraints_ForeignKeys {
        DataBigqueryTable_TableConstraints_ForeignKeys {
            column_references: core::default::Default::default(),
            name: core::default::Default::default(),
            referenced_table: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_TableConstraints_ForeignKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_TableConstraints_ForeignKeysRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_TableConstraints_ForeignKeysRef {
        DataBigqueryTable_TableConstraints_ForeignKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_TableConstraints_ForeignKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_references` after provisioning.\n"]
    pub fn column_references(
        &self,
    ) -> terrars::ListRef<DataBigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_references", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `referenced_table` after provisioning.\n"]
    pub fn referenced_table(
        &self,
    ) -> terrars::ListRef<DataBigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referenced_table", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_TableConstraints_PrimaryKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataBigqueryTable_TableConstraints_PrimaryKey {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.columns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_TableConstraints_PrimaryKey {
    type O = terrars::BlockAssignable<DataBigqueryTable_TableConstraints_PrimaryKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_TableConstraints_PrimaryKey {}

impl BuildDataBigqueryTable_TableConstraints_PrimaryKey {
    pub fn build(self) -> DataBigqueryTable_TableConstraints_PrimaryKey {
        DataBigqueryTable_TableConstraints_PrimaryKey { columns: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_TableConstraints_PrimaryKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_TableConstraints_PrimaryKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBigqueryTable_TableConstraints_PrimaryKeyRef {
        DataBigqueryTable_TableConstraints_PrimaryKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_TableConstraints_PrimaryKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_TableConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    foreign_keys: std::option::Option<terrars::ListField<DataBigqueryTable_TableConstraints_ForeignKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_key: std::option::Option<terrars::ListField<DataBigqueryTable_TableConstraints_PrimaryKey>>,
}

impl DataBigqueryTable_TableConstraints {
    #[doc = "Set the field `foreign_keys`.\n"]
    pub fn set_foreign_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryTable_TableConstraints_ForeignKeys>>,
    ) -> Self {
        self.foreign_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_key`.\n"]
    pub fn set_primary_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBigqueryTable_TableConstraints_PrimaryKey>>,
    ) -> Self {
        self.primary_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_TableConstraints {
    type O = terrars::BlockAssignable<DataBigqueryTable_TableConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_TableConstraints {}

impl BuildDataBigqueryTable_TableConstraints {
    pub fn build(self) -> DataBigqueryTable_TableConstraints {
        DataBigqueryTable_TableConstraints {
            foreign_keys: core::default::Default::default(),
            primary_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_TableConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_TableConstraintsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryTable_TableConstraintsRef {
        DataBigqueryTable_TableConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_TableConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `foreign_keys` after provisioning.\n"]
    pub fn foreign_keys(&self) -> terrars::ListRef<DataBigqueryTable_TableConstraints_ForeignKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.foreign_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_key` after provisioning.\n"]
    pub fn primary_key(&self) -> terrars::ListRef<DataBigqueryTable_TableConstraints_PrimaryKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_TableReplicationInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_interval_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_TableReplicationInfo {
    #[doc = "Set the field `replication_interval_ms`.\n"]
    pub fn set_replication_interval_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replication_interval_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dataset_id`.\n"]
    pub fn set_source_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_project_id`.\n"]
    pub fn set_source_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_table_id`.\n"]
    pub fn set_source_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_table_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_TableReplicationInfo {
    type O = terrars::BlockAssignable<DataBigqueryTable_TableReplicationInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_TableReplicationInfo {}

impl BuildDataBigqueryTable_TableReplicationInfo {
    pub fn build(self) -> DataBigqueryTable_TableReplicationInfo {
        DataBigqueryTable_TableReplicationInfo {
            replication_interval_ms: core::default::Default::default(),
            source_dataset_id: core::default::Default::default(),
            source_project_id: core::default::Default::default(),
            source_table_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_TableReplicationInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_TableReplicationInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryTable_TableReplicationInfoRef {
        DataBigqueryTable_TableReplicationInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_TableReplicationInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replication_interval_ms` after provisioning.\n"]
    pub fn replication_interval_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_interval_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `source_dataset_id` after provisioning.\n"]
    pub fn source_dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dataset_id", self.base))
    }

    #[doc = "Get a reference to the value of field `source_project_id` after provisioning.\n"]
    pub fn source_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `source_table_id` after provisioning.\n"]
    pub fn source_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_table_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_TimePartitioning {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_partition_filter: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBigqueryTable_TimePartitioning {
    #[doc = "Set the field `expiration_ms`.\n"]
    pub fn set_expiration_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.expiration_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_partition_filter`.\n"]
    pub fn set_require_partition_filter(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_partition_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_TimePartitioning {
    type O = terrars::BlockAssignable<DataBigqueryTable_TimePartitioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_TimePartitioning {}

impl BuildDataBigqueryTable_TimePartitioning {
    pub fn build(self) -> DataBigqueryTable_TimePartitioning {
        DataBigqueryTable_TimePartitioning {
            expiration_ms: core::default::Default::default(),
            field: core::default::Default::default(),
            require_partition_filter: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_TimePartitioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_TimePartitioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryTable_TimePartitioningRef {
        DataBigqueryTable_TimePartitioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_TimePartitioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expiration_ms` after provisioning.\n"]
    pub fn expiration_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_ms", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `require_partition_filter` after provisioning.\n"]
    pub fn require_partition_filter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBigqueryTable_View {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_legacy_sql: std::option::Option<terrars::PrimField<bool>>,
}

impl DataBigqueryTable_View {
    #[doc = "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_legacy_sql`.\n"]
    pub fn set_use_legacy_sql(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_legacy_sql = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBigqueryTable_View {
    type O = terrars::BlockAssignable<DataBigqueryTable_View>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBigqueryTable_View {}

impl BuildDataBigqueryTable_View {
    pub fn build(self) -> DataBigqueryTable_View {
        DataBigqueryTable_View {
            query: core::default::Default::default(),
            use_legacy_sql: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBigqueryTable_ViewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBigqueryTable_ViewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBigqueryTable_ViewRef {
        DataBigqueryTable_ViewRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBigqueryTable_ViewRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc = "Get a reference to the value of field `use_legacy_sql` after provisioning.\n"]
    pub fn use_legacy_sql(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_legacy_sql", self.base))
    }
}
