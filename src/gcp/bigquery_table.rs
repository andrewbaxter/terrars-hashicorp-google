#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryTableData_ {
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
    clustering: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    dataset_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    friendly_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_auto_generated_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_schema_changes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_staleness: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_partition_filter: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
    table_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_metadata_view: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    biglake_configuration: std::option::Option<std::vec::Vec<BigqueryTable_BiglakeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<BigqueryTable_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_catalog_table_options: std::option::Option<std::vec::Vec<BigqueryTable_ExternalCatalogTableOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_data_configuration: std::option::Option<std::vec::Vec<BigqueryTable_ExternalDataConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    materialized_view: std::option::Option<std::vec::Vec<BigqueryTable_MaterializedView>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_partitioning: std::option::Option<std::vec::Vec<BigqueryTable_RangePartitioning>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_foreign_type_info: std::option::Option<std::vec::Vec<BigqueryTable_SchemaForeignTypeInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_constraints: std::option::Option<std::vec::Vec<BigqueryTable_TableConstraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_replication_info: std::option::Option<std::vec::Vec<BigqueryTable_TableReplicationInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_partitioning: std::option::Option<std::vec::Vec<BigqueryTable_TimePartitioning>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view: std::option::Option<std::vec::Vec<BigqueryTable_View>>,
    dynamic: BigqueryTableDynamic,
}

struct BigqueryTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryTableData_>,
}

#[derive(Clone)]
pub struct BigqueryTable(std::rc::Rc<BigqueryTable_>);

impl BigqueryTable {
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
        "Set the field `clustering`.\nSpecifies column names to use for data clustering. Up to four top-level columns are allowed, and should be specified in descending priority order."]
    pub fn set_clustering(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().clustering = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the instance. When the field is set to true or unset in Terraform state, a terraform apply or terraform destroy that would delete the table will fail. When the field is set to false, deleting the table is allowed."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe field description."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expiration_time`.\nThe time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
    pub fn set_expiration_time(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().expiration_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `friendly_name`.\nA descriptive name for the table."]
    pub fn set_friendly_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().friendly_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_auto_generated_schema`.\nWhether Terraform will prevent implicitly added columns in schema from showing diff."]
    pub fn set_ignore_auto_generated_schema(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_auto_generated_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_schema_changes`.\nMention which fields in schema are to be ignored"]
    pub fn set_ignore_schema_changes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ignore_schema_changes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA mapping of labels to assign to the resource.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_staleness`.\nThe maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of [SQL IntervalValue type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type)."]
    pub fn set_max_staleness(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().max_staleness = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require_partition_filter`.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn set_require_partition_filter(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_partition_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_tags`.\nThe tags attached to this table. Tag keys are globally unique. Tag key is expected to be in the namespaced format, for example \"123456789012/environment\" where 123456789012 is the ID of the parent organization or project resource for this tag key. Tag value is expected to be the short name, for example \"Production\"."]
    pub fn set_resource_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\nA JSON schema for the table."]
    pub fn set_schema(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `table_metadata_view`.\nView sets the optional parameter \"view\": Specifies the view that determines which table information is returned. By default, basic table information and storage statistics (STORAGE_STATS) are returned. Possible values: TABLE_METADATA_VIEW_UNSPECIFIED, BASIC, STORAGE_STATS, FULL"]
    pub fn set_table_metadata_view(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().table_metadata_view = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `biglake_configuration`.\n"]
    pub fn set_biglake_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_BiglakeConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().biglake_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.biglake_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_EncryptionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `external_catalog_table_options`.\n"]
    pub fn set_external_catalog_table_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_ExternalCatalogTableOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_catalog_table_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_catalog_table_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `external_data_configuration`.\n"]
    pub fn set_external_data_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_data_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_data_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `materialized_view`.\n"]
    pub fn set_materialized_view(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_MaterializedView>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().materialized_view = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.materialized_view = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `range_partitioning`.\n"]
    pub fn set_range_partitioning(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_RangePartitioning>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().range_partitioning = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.range_partitioning = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_foreign_type_info`.\n"]
    pub fn set_schema_foreign_type_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_SchemaForeignTypeInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema_foreign_type_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema_foreign_type_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_constraints`.\n"]
    pub fn set_table_constraints(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_TableConstraints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table_constraints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_replication_info`.\n"]
    pub fn set_table_replication_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_TableReplicationInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table_replication_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table_replication_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_partitioning`.\n"]
    pub fn set_time_partitioning(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_TimePartitioning>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().time_partitioning = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.time_partitioning = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `view`.\n"]
    pub fn set_view(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_View>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().view = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.view = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
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
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nDescribes the table type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `biglake_configuration` after provisioning.\n"]
    pub fn biglake_configuration(&self) -> terrars::ListRef<BigqueryTable_BiglakeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.biglake_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<BigqueryTable_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_catalog_table_options` after provisioning.\n"]
    pub fn external_catalog_table_options(&self) -> terrars::ListRef<BigqueryTable_ExternalCatalogTableOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.external_catalog_table_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `external_data_configuration` after provisioning.\n"]
    pub fn external_data_configuration(&self) -> terrars::ListRef<BigqueryTable_ExternalDataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_data_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `materialized_view` after provisioning.\n"]
    pub fn materialized_view(&self) -> terrars::ListRef<BigqueryTable_MaterializedViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.materialized_view", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `range_partitioning` after provisioning.\n"]
    pub fn range_partitioning(&self) -> terrars::ListRef<BigqueryTable_RangePartitioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range_partitioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_foreign_type_info` after provisioning.\n"]
    pub fn schema_foreign_type_info(&self) -> terrars::ListRef<BigqueryTable_SchemaForeignTypeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_foreign_type_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_constraints` after provisioning.\n"]
    pub fn table_constraints(&self) -> terrars::ListRef<BigqueryTable_TableConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_replication_info` after provisioning.\n"]
    pub fn table_replication_info(&self) -> terrars::ListRef<BigqueryTable_TableReplicationInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_replication_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_partitioning` after provisioning.\n"]
    pub fn time_partitioning(&self) -> terrars::ListRef<BigqueryTable_TimePartitioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_partitioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> terrars::ListRef<BigqueryTable_ViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.view", self.extract_ref()))
    }
}

impl terrars::Referable for BigqueryTable {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryTable { }

impl terrars::ToListMappable for BigqueryTable {
    type O = terrars::ListRef<BigqueryTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryTable_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryTable {
    pub tf_id: std::string::String,
    #[doc = "The dataset ID to create the table in. Changing this forces a new resource to be created."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "A unique ID for the resource. Changing this forces a new resource to be created."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryTable {
        let out = BigqueryTable(std::rc::Rc::new(BigqueryTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                clustering: core::default::Default::default(),
                dataset_id: self.dataset_id,
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                expiration_time: core::default::Default::default(),
                friendly_name: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_auto_generated_schema: core::default::Default::default(),
                ignore_schema_changes: core::default::Default::default(),
                labels: core::default::Default::default(),
                max_staleness: core::default::Default::default(),
                project: core::default::Default::default(),
                require_partition_filter: core::default::Default::default(),
                resource_tags: core::default::Default::default(),
                schema: core::default::Default::default(),
                table_id: self.table_id,
                table_metadata_view: core::default::Default::default(),
                biglake_configuration: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                external_catalog_table_options: core::default::Default::default(),
                external_data_configuration: core::default::Default::default(),
                materialized_view: core::default::Default::default(),
                range_partitioning: core::default::Default::default(),
                schema_foreign_type_info: core::default::Default::default(),
                table_constraints: core::default::Default::default(),
                table_replication_info: core::default::Default::default(),
                time_partitioning: core::default::Default::default(),
                view: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryTableRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
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
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nDescribes the table type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `biglake_configuration` after provisioning.\n"]
    pub fn biglake_configuration(&self) -> terrars::ListRef<BigqueryTable_BiglakeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.biglake_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<BigqueryTable_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_catalog_table_options` after provisioning.\n"]
    pub fn external_catalog_table_options(&self) -> terrars::ListRef<BigqueryTable_ExternalCatalogTableOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.external_catalog_table_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `external_data_configuration` after provisioning.\n"]
    pub fn external_data_configuration(&self) -> terrars::ListRef<BigqueryTable_ExternalDataConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_data_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `materialized_view` after provisioning.\n"]
    pub fn materialized_view(&self) -> terrars::ListRef<BigqueryTable_MaterializedViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.materialized_view", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `range_partitioning` after provisioning.\n"]
    pub fn range_partitioning(&self) -> terrars::ListRef<BigqueryTable_RangePartitioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range_partitioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_foreign_type_info` after provisioning.\n"]
    pub fn schema_foreign_type_info(&self) -> terrars::ListRef<BigqueryTable_SchemaForeignTypeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_foreign_type_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_constraints` after provisioning.\n"]
    pub fn table_constraints(&self) -> terrars::ListRef<BigqueryTable_TableConstraintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_replication_info` after provisioning.\n"]
    pub fn table_replication_info(&self) -> terrars::ListRef<BigqueryTable_TableReplicationInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_replication_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_partitioning` after provisioning.\n"]
    pub fn time_partitioning(&self) -> terrars::ListRef<BigqueryTable_TimePartitioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_partitioning", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> terrars::ListRef<BigqueryTable_ViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.view", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTableDynamic {
    biglake_configuration: std::option::Option<terrars::DynamicBlock<BigqueryTable_BiglakeConfiguration>>,
    encryption_configuration: std::option::Option<terrars::DynamicBlock<BigqueryTable_EncryptionConfiguration>>,
    external_catalog_table_options: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_ExternalCatalogTableOptions>>,
    external_data_configuration: std::option::Option<terrars::DynamicBlock<BigqueryTable_ExternalDataConfiguration>>,
    materialized_view: std::option::Option<terrars::DynamicBlock<BigqueryTable_MaterializedView>>,
    range_partitioning: std::option::Option<terrars::DynamicBlock<BigqueryTable_RangePartitioning>>,
    schema_foreign_type_info: std::option::Option<terrars::DynamicBlock<BigqueryTable_SchemaForeignTypeInfo>>,
    table_constraints: std::option::Option<terrars::DynamicBlock<BigqueryTable_TableConstraints>>,
    table_replication_info: std::option::Option<terrars::DynamicBlock<BigqueryTable_TableReplicationInfo>>,
    time_partitioning: std::option::Option<terrars::DynamicBlock<BigqueryTable_TimePartitioning>>,
    view: std::option::Option<terrars::DynamicBlock<BigqueryTable_View>>,
}

#[derive(Serialize)]
pub struct BigqueryTable_BiglakeConfiguration {
    connection_id: terrars::PrimField<std::string::String>,
    file_format: terrars::PrimField<std::string::String>,
    storage_uri: terrars::PrimField<std::string::String>,
    table_format: terrars::PrimField<std::string::String>,
}

impl BigqueryTable_BiglakeConfiguration { }

impl terrars::ToListMappable for BigqueryTable_BiglakeConfiguration {
    type O = terrars::BlockAssignable<BigqueryTable_BiglakeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_BiglakeConfiguration {
    #[doc =
        "The connection specifying the credentials to be used to read and write to external storage, such as Cloud Storage. The connection_id can have the form \"&lt;project\\_id&gt;.&lt;location\\_id&gt;.&lt;connection\\_id&gt;\" or \"projects/&lt;project\\_id&gt;/locations/&lt;location\\_id&gt;/connections/&lt;connection\\_id&gt;\"."]
    pub connection_id: terrars::PrimField<std::string::String>,
    #[doc = "The file format the data is stored in."]
    pub file_format: terrars::PrimField<std::string::String>,
    #[doc =
        "The fully qualified location prefix of the external folder where table data is stored. The '*' wildcard character is not allowed. The URI should be in the format \"gs://bucket/path_to_table/\""]
    pub storage_uri: terrars::PrimField<std::string::String>,
    #[doc = "The table format the metadata only snapshots are stored in."]
    pub table_format: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_BiglakeConfiguration {
    pub fn build(self) -> BigqueryTable_BiglakeConfiguration {
        BigqueryTable_BiglakeConfiguration {
            connection_id: self.connection_id,
            file_format: self.file_format,
            storage_uri: self.storage_uri,
            table_format: self.table_format,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_BiglakeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_BiglakeConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_BiglakeConfigurationRef {
        BigqueryTable_BiglakeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_BiglakeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_id` after provisioning.\nThe connection specifying the credentials to be used to read and write to external storage, such as Cloud Storage. The connection_id can have the form \"&lt;project\\_id&gt;.&lt;location\\_id&gt;.&lt;connection\\_id&gt;\" or \"projects/&lt;project\\_id&gt;/locations/&lt;location\\_id&gt;/connections/&lt;connection\\_id&gt;\"."]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_format` after provisioning.\nThe file format the data is stored in."]
    pub fn file_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_uri` after provisioning.\nThe fully qualified location prefix of the external folder where table data is stored. The '*' wildcard character is not allowed. The URI should be in the format \"gs://bucket/path_to_table/\""]
    pub fn storage_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_format` after provisioning.\nThe table format the metadata only snapshots are stored in."]
    pub fn table_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_format", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_EncryptionConfiguration {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl BigqueryTable_EncryptionConfiguration { }

impl terrars::ToListMappable for BigqueryTable_EncryptionConfiguration {
    type O = terrars::BlockAssignable<BigqueryTable_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_EncryptionConfiguration {
    #[doc =
        "The self link or full name of a key which should be used to encrypt this table. Note that the default bigquery service account will need to have encrypt/decrypt permissions on this key - you may want to see the google_bigquery_default_service_account datasource and the google_kms_crypto_key_iam_binding resource."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_EncryptionConfiguration {
    pub fn build(self) -> BigqueryTable_EncryptionConfiguration {
        BigqueryTable_EncryptionConfiguration { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_EncryptionConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_EncryptionConfigurationRef {
        BigqueryTable_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe self link or full name of a key which should be used to encrypt this table. Note that the default bigquery service account will need to have encrypt/decrypt permissions on this key - you may want to see the google_bigquery_default_service_account datasource and the google_kms_crypto_key_iam_binding resource."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_version` after provisioning.\nThe self link or full name of the kms key version used to encrypt this table."]
    pub fn kms_key_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_version", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    serialization_library: terrars::PrimField<std::string::String>,
}

impl BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
    #[doc = "Set the field `name`.\nName of the SerDe. The maximum length is 256 characters."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parameters`.\nKey-value pairs that define the initialization parameters for the serialization library. Maximum size 10 Kib."]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
    #[doc =
        "Specifies a fully-qualified class name of the serialization library that is responsible for the translation of data between table representation and the underlying low-level input and output format structures. The maximum length is 256 characters."]
    pub serialization_library: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
    pub fn build(self) -> BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
        BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo {
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            serialization_library: self.serialization_library,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef {
        BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the SerDe. The maximum length is 256 characters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nKey-value pairs that define the initialization parameters for the serialization library. Maximum size 10 Kib."]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serialization_library` after provisioning.\nSpecifies a fully-qualified class name of the serialization library that is responsible for the translation of data between table representation and the underlying low-level input and output format structures. The maximum length is 256 characters."]
    pub fn serialization_library(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serialization_library", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTable_ExternalCatalogTableOptions_StorageDescriptorDynamic {
    serde_info: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo>>,
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serde_info: std::option::Option<std::vec::Vec<BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo>>,
    dynamic: BigqueryTable_ExternalCatalogTableOptions_StorageDescriptorDynamic,
}

impl BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
    #[doc =
        "Set the field `input_format`.\nSpecifies the fully qualified class name of the InputFormat (e.g. \"org.apache.hadoop.hive.ql.io.orc.OrcInputFormat\"). The maximum length is 128 characters."]
    pub fn set_input_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_format = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location_uri`.\nThe physical location of the table (e.g. 'gs://spark-dataproc-data/pangea-data/case_sensitive/' or 'gs://spark-dataproc-data/pangea-data/*'). The maximum length is 2056 bytes."]
    pub fn set_location_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_format`.\nSpecifies the fully qualified class name of the OutputFormat (e.g. \"org.apache.hadoop.hive.ql.io.orc.OrcOutputFormat\"). The maximum length is 128 characters."]
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
                        terrars
                        ::BlockAssignable<BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfo>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.serde_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.serde_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {}

impl BuildBigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
    pub fn build(self) -> BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
        BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor {
            input_format: core::default::Default::default(),
            location_uri: core::default::Default::default(),
            output_format: core::default::Default::default(),
            serde_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef {
        BigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `input_format` after provisioning.\nSpecifies the fully qualified class name of the InputFormat (e.g. \"org.apache.hadoop.hive.ql.io.orc.OrcInputFormat\"). The maximum length is 128 characters."]
    pub fn input_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location_uri` after provisioning.\nThe physical location of the table (e.g. 'gs://spark-dataproc-data/pangea-data/case_sensitive/' or 'gs://spark-dataproc-data/pangea-data/*'). The maximum length is 2056 bytes."]
    pub fn location_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_format` after provisioning.\nSpecifies the fully qualified class name of the OutputFormat (e.g. \"org.apache.hadoop.hive.ql.io.orc.OrcOutputFormat\"). The maximum length is 128 characters."]
    pub fn output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.base))
    }

    #[doc = "Get a reference to the value of field `serde_info` after provisioning.\n"]
    pub fn serde_info(
        &self,
    ) -> terrars::ListRef<BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor_SerdeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serde_info", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTable_ExternalCatalogTableOptionsDynamic {
    storage_descriptor: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor>>,
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalCatalogTableOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_descriptor: std::option::Option<std::vec::Vec<BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor>>,
    dynamic: BigqueryTable_ExternalCatalogTableOptionsDynamic,
}

impl BigqueryTable_ExternalCatalogTableOptions {
    #[doc =
        "Set the field `connection_id`.\nThe connection specifying the credentials to be used to read external storage, such as Azure Blob, Cloud Storage, or S3. The connection is needed to read the open source table from BigQuery Engine. The connection_id can have the form <project_id>.<location_id>.<connection_id> or projects/<project_id>/locations/<location_id>/connections/<connection_id>."]
    pub fn set_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parameters`.\nA map of key value pairs defining the parameters and properties of the open source table. Corresponds with hive meta store table parameters. Maximum size of 4Mib."]
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
                    ::Into<terrars::BlockAssignable<BigqueryTable_ExternalCatalogTableOptions_StorageDescriptor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_descriptor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_descriptor = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalCatalogTableOptions {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalCatalogTableOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalCatalogTableOptions {}

impl BuildBigqueryTable_ExternalCatalogTableOptions {
    pub fn build(self) -> BigqueryTable_ExternalCatalogTableOptions {
        BigqueryTable_ExternalCatalogTableOptions {
            connection_id: core::default::Default::default(),
            parameters: core::default::Default::default(),
            storage_descriptor: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalCatalogTableOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalCatalogTableOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_ExternalCatalogTableOptionsRef {
        BigqueryTable_ExternalCatalogTableOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalCatalogTableOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_id` after provisioning.\nThe connection specifying the credentials to be used to read external storage, such as Azure Blob, Cloud Storage, or S3. The connection is needed to read the open source table from BigQuery Engine. The connection_id can have the form <project_id>.<location_id>.<connection_id> or projects/<project_id>/locations/<location_id>/connections/<connection_id>."]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nA map of key value pairs defining the parameters and properties of the open source table. Corresponds with hive meta store table parameters. Maximum size of 4Mib."]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<BigqueryTable_ExternalCatalogTableOptions_StorageDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalDataConfiguration_AvroOptions {
    use_avro_logical_types: terrars::PrimField<bool>,
}

impl BigqueryTable_ExternalDataConfiguration_AvroOptions { }

impl terrars::ToListMappable for BigqueryTable_ExternalDataConfiguration_AvroOptions {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_AvroOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalDataConfiguration_AvroOptions {
    #[doc =
        "If sourceFormat is set to \"AVRO\", indicates whether to interpret logical types as the corresponding BigQuery data type (for example, TIMESTAMP), instead of using the raw type (for example, INTEGER)."]
    pub use_avro_logical_types: terrars::PrimField<bool>,
}

impl BuildBigqueryTable_ExternalDataConfiguration_AvroOptions {
    pub fn build(self) -> BigqueryTable_ExternalDataConfiguration_AvroOptions {
        BigqueryTable_ExternalDataConfiguration_AvroOptions { use_avro_logical_types: self.use_avro_logical_types }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalDataConfiguration_AvroOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalDataConfiguration_AvroOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalDataConfiguration_AvroOptionsRef {
        BigqueryTable_ExternalDataConfiguration_AvroOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalDataConfiguration_AvroOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `use_avro_logical_types` after provisioning.\nIf sourceFormat is set to \"AVRO\", indicates whether to interpret logical types as the corresponding BigQuery data type (for example, TIMESTAMP), instead of using the raw type (for example, INTEGER)."]
    pub fn use_avro_logical_types(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_avro_logical_types", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
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

impl BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
    #[doc =
        "Set the field `encoding`.\nThe encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. 'encoding' can also be set at the column family level. However, the setting at this level takes precedence if 'encoding' is set at both levels."]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `field_name`.\nIf the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as the column field name and is used as field name in queries."]
    pub fn set_field_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `only_read_latest`.\nIf this is set, only the latest version of value in this column are exposed. 'onlyReadLatest' can also be set at the column family level. However, the setting at this level takes precedence if 'onlyReadLatest' is set at both levels."]
    pub fn set_only_read_latest(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.only_read_latest = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `qualifier_encoded`.\nQualifier of the column. Columns in the parent column family that has this exact qualifier are exposed as . field. If the qualifier is valid UTF-8 string, it can be specified in the qualifierString field. Otherwise, a base-64 encoded value must be set to qualifierEncoded. The column field name is the same as the column qualifier. However, if the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as fieldName."]
    pub fn set_qualifier_encoded(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.qualifier_encoded = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `qualifier_string`.\nQualifier string."]
    pub fn set_qualifier_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.qualifier_string = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type to convert the value in cells of this column. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive): \"BYTES\", \"STRING\", \"INTEGER\", \"FLOAT\", \"BOOLEAN\", \"JSON\", Default type is \"BYTES\". 'type' can also be set at the column family level. However, the setting at this level takes precedence if 'type' is set at both levels."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {}

impl BuildBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
    pub fn build(self) -> BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
        BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column {
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
pub struct BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef {
        BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encoding` after provisioning.\nThe encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. 'encoding' can also be set at the column family level. However, the setting at this level takes precedence if 'encoding' is set at both levels."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field_name` after provisioning.\nIf the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as the column field name and is used as field name in queries."]
    pub fn field_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `only_read_latest` after provisioning.\nIf this is set, only the latest version of value in this column are exposed. 'onlyReadLatest' can also be set at the column family level. However, the setting at this level takes precedence if 'onlyReadLatest' is set at both levels."]
    pub fn only_read_latest(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.only_read_latest", self.base))
    }

    #[doc =
        "Get a reference to the value of field `qualifier_encoded` after provisioning.\nQualifier of the column. Columns in the parent column family that has this exact qualifier are exposed as . field. If the qualifier is valid UTF-8 string, it can be specified in the qualifierString field. Otherwise, a base-64 encoded value must be set to qualifierEncoded. The column field name is the same as the column qualifier. However, if the qualifier is not a valid BigQuery field identifier i.e. does not match [a-zA-Z][a-zA-Z0-9_]*, a valid identifier must be provided as fieldName."]
    pub fn qualifier_encoded(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier_encoded", self.base))
    }

    #[doc = "Get a reference to the value of field `qualifier_string` after provisioning.\nQualifier string."]
    pub fn qualifier_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier_string", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type to convert the value in cells of this column. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive): \"BYTES\", \"STRING\", \"INTEGER\", \"FLOAT\", \"BOOLEAN\", \"JSON\", Default type is \"BYTES\". 'type' can also be set at the column family level. However, the setting at this level takes precedence if 'type' is set at both levels."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyDynamic {
    column: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column>>,
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    family_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    only_read_latest: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std
    ::option
    ::Option<std::vec::Vec<BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column>>,
    dynamic: BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyDynamic,
}

impl BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
    #[doc =
        "Set the field `encoding`.\nThe encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. This can be overridden for a specific column by listing that column in 'columns' and specifying an encoding for it."]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `family_id`.\nIdentifier of the column family."]
    pub fn set_family_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.family_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `only_read_latest`.\nIf this is set only the latest version of value are exposed for all columns in this column family. This can be overridden for a specific column by listing that column in 'columns' and specifying a different setting for that column."]
    pub fn set_only_read_latest(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.only_read_latest = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type to convert the value in cells of this column family. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive): \"BYTES\", \"STRING\", \"INTEGER\", \"FLOAT\", \"BOOLEAN\", \"JSON\". Default type is BYTES. This can be overridden for a specific column by listing that column in 'columns' and specifying a type for it."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column`.\n"]
    pub fn set_column(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_Column>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {}

impl BuildBigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
    pub fn build(self) -> BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
        BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily {
            encoding: core::default::Default::default(),
            family_id: core::default::Default::default(),
            only_read_latest: core::default::Default::default(),
            type_: core::default::Default::default(),
            column: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef {
        BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encoding` after provisioning.\nThe encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. This can be overridden for a specific column by listing that column in 'columns' and specifying an encoding for it."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `family_id` after provisioning.\nIdentifier of the column family."]
    pub fn family_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `only_read_latest` after provisioning.\nIf this is set only the latest version of value are exposed for all columns in this column family. This can be overridden for a specific column by listing that column in 'columns' and specifying a different setting for that column."]
    pub fn only_read_latest(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.only_read_latest", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type to convert the value in cells of this column family. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive): \"BYTES\", \"STRING\", \"INTEGER\", \"FLOAT\", \"BOOLEAN\", \"JSON\". Default type is BYTES. This can be overridden for a specific column by listing that column in 'columns' and specifying a type for it."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\n"]
    pub fn column(
        &self,
    ) -> terrars::ListRef<BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily_ColumnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTable_ExternalDataConfiguration_BigtableOptionsDynamic {
    column_family: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily>>,
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalDataConfiguration_BigtableOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_unspecified_column_families: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_column_families_as_json: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_rowkey_as_string: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_family: std
    ::option
    ::Option<std::vec::Vec<BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily>>,
    dynamic: BigqueryTable_ExternalDataConfiguration_BigtableOptionsDynamic,
}

impl BigqueryTable_ExternalDataConfiguration_BigtableOptions {
    #[doc =
        "Set the field `ignore_unspecified_column_families`.\nIf field is true, then the column families that are not specified in columnFamilies list are not exposed in the table schema. Otherwise, they are read with BYTES type values. The default value is false."]
    pub fn set_ignore_unspecified_column_families(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.ignore_unspecified_column_families = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_column_families_as_json`.\nIf field is true, then each column family will be read as a single JSON column. Otherwise they are read as a repeated cell structure containing timestamp/value tuples. The default value is false."]
    pub fn set_output_column_families_as_json(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.output_column_families_as_json = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `read_rowkey_as_string`.\nIf field is true, then the rowkey column families will be read and converted to string. Otherwise they are read with BYTES type values and users need to manually cast them with CAST if necessary. The default value is false."]
    pub fn set_read_rowkey_as_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_rowkey_as_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_family`.\n"]
    pub fn set_column_family(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamily>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column_family = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column_family = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalDataConfiguration_BigtableOptions {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_BigtableOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalDataConfiguration_BigtableOptions {}

impl BuildBigqueryTable_ExternalDataConfiguration_BigtableOptions {
    pub fn build(self) -> BigqueryTable_ExternalDataConfiguration_BigtableOptions {
        BigqueryTable_ExternalDataConfiguration_BigtableOptions {
            ignore_unspecified_column_families: core::default::Default::default(),
            output_column_families_as_json: core::default::Default::default(),
            read_rowkey_as_string: core::default::Default::default(),
            column_family: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalDataConfiguration_BigtableOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalDataConfiguration_BigtableOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalDataConfiguration_BigtableOptionsRef {
        BigqueryTable_ExternalDataConfiguration_BigtableOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalDataConfiguration_BigtableOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ignore_unspecified_column_families` after provisioning.\nIf field is true, then the column families that are not specified in columnFamilies list are not exposed in the table schema. Otherwise, they are read with BYTES type values. The default value is false."]
    pub fn ignore_unspecified_column_families(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_unspecified_column_families", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_column_families_as_json` after provisioning.\nIf field is true, then each column family will be read as a single JSON column. Otherwise they are read as a repeated cell structure containing timestamp/value tuples. The default value is false."]
    pub fn output_column_families_as_json(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_column_families_as_json", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_rowkey_as_string` after provisioning.\nIf field is true, then the rowkey column families will be read and converted to string. Otherwise they are read with BYTES type values and users need to manually cast them with CAST if necessary. The default value is false."]
    pub fn read_rowkey_as_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_rowkey_as_string", self.base))
    }

    #[doc = "Get a reference to the value of field `column_family` after provisioning.\n"]
    pub fn column_family(
        &self,
    ) -> terrars::ListRef<BigqueryTable_ExternalDataConfiguration_BigtableOptions_ColumnFamilyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_family", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalDataConfiguration_CsvOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_jagged_rows: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_quoted_newlines: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    quote: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_leading_rows: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_column_match: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryTable_ExternalDataConfiguration_CsvOptions {
    #[doc =
        "Set the field `allow_jagged_rows`.\nIndicates if BigQuery should accept rows that are missing trailing optional columns."]
    pub fn set_allow_jagged_rows(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_jagged_rows = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_quoted_newlines`.\nIndicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false."]
    pub fn set_allow_quoted_newlines(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_quoted_newlines = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `encoding`.\nThe character encoding of the data. The supported values are UTF-8 or ISO-8859-1."]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_delimiter`.\nThe separator for fields in a CSV file."]
    pub fn set_field_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_leading_rows`.\nThe number of rows at the top of a CSV file that BigQuery will skip when reading the data."]
    pub fn set_skip_leading_rows(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.skip_leading_rows = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_column_match`.\nSpecifies how source columns are matched to the table schema. Valid values are POSITION (columns matched by position, assuming same ordering) or NAME (columns matched by name, reads header row and reorders columns to align with schema field names)."]
    pub fn set_source_column_match(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_column_match = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalDataConfiguration_CsvOptions {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_CsvOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalDataConfiguration_CsvOptions {
    #[doc =
        "The value that is used to quote data sections in a CSV file. If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allow_quoted_newlines property to true. The API-side default is \", specified in Terraform escaped as \\\". Due to limitations with Terraform default values, this value is required to be explicitly set."]
    pub quote: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_ExternalDataConfiguration_CsvOptions {
    pub fn build(self) -> BigqueryTable_ExternalDataConfiguration_CsvOptions {
        BigqueryTable_ExternalDataConfiguration_CsvOptions {
            allow_jagged_rows: core::default::Default::default(),
            allow_quoted_newlines: core::default::Default::default(),
            encoding: core::default::Default::default(),
            field_delimiter: core::default::Default::default(),
            quote: self.quote,
            skip_leading_rows: core::default::Default::default(),
            source_column_match: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalDataConfiguration_CsvOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalDataConfiguration_CsvOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalDataConfiguration_CsvOptionsRef {
        BigqueryTable_ExternalDataConfiguration_CsvOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalDataConfiguration_CsvOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_jagged_rows` after provisioning.\nIndicates if BigQuery should accept rows that are missing trailing optional columns."]
    pub fn allow_jagged_rows(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_jagged_rows", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_quoted_newlines` after provisioning.\nIndicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false."]
    pub fn allow_quoted_newlines(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_quoted_newlines", self.base))
    }

    #[doc =
        "Get a reference to the value of field `encoding` after provisioning.\nThe character encoding of the data. The supported values are UTF-8 or ISO-8859-1."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field_delimiter` after provisioning.\nThe separator for fields in a CSV file."]
    pub fn field_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_delimiter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `quote` after provisioning.\nThe value that is used to quote data sections in a CSV file. If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allow_quoted_newlines property to true. The API-side default is \", specified in Terraform escaped as \\\". Due to limitations with Terraform default values, this value is required to be explicitly set."]
    pub fn quote(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quote", self.base))
    }

    #[doc =
        "Get a reference to the value of field `skip_leading_rows` after provisioning.\nThe number of rows at the top of a CSV file that BigQuery will skip when reading the data."]
    pub fn skip_leading_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_leading_rows", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_column_match` after provisioning.\nSpecifies how source columns are matched to the table schema. Valid values are POSITION (columns matched by position, assuming same ordering) or NAME (columns matched by name, reads header row and reorders columns to align with schema field names)."]
    pub fn source_column_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_column_match", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_leading_rows: std::option::Option<terrars::PrimField<f64>>,
}

impl BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
    #[doc =
        "Set the field `range`.\nRange of a sheet to query from. Only used when non-empty. At least one of range or skip_leading_rows must be set. Typical format: \"sheet_name!top_left_cell_id:bottom_right_cell_id\" For example: \"sheet1!A1:B20"]
    pub fn set_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.range = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_leading_rows`.\nThe number of rows at the top of the sheet that BigQuery will skip when reading the data. At least one of range or skip_leading_rows must be set."]
    pub fn set_skip_leading_rows(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.skip_leading_rows = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {}

impl BuildBigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
    pub fn build(self) -> BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
        BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions {
            range: core::default::Default::default(),
            skip_leading_rows: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef {
        BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `range` after provisioning.\nRange of a sheet to query from. Only used when non-empty. At least one of range or skip_leading_rows must be set. Typical format: \"sheet_name!top_left_cell_id:bottom_right_cell_id\" For example: \"sheet1!A1:B20"]
    pub fn range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `skip_leading_rows` after provisioning.\nThe number of rows at the top of the sheet that BigQuery will skip when reading the data. At least one of range or skip_leading_rows must be set."]
    pub fn skip_leading_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_leading_rows", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_partition_filter: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_uri_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
    #[doc = "Set the field `mode`.\nWhen set, what mode of hive partitioning to use when reading data."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require_partition_filter`.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn set_require_partition_filter(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_partition_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_uri_prefix`.\nWhen hive partition detection is requested, a common for all source uris must be required. The prefix must end immediately before the partition key encoding begins."]
    pub fn set_source_uri_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_uri_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_HivePartitioningOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {}

impl BuildBigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
    pub fn build(self) -> BigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
        BigqueryTable_ExternalDataConfiguration_HivePartitioningOptions {
            mode: core::default::Default::default(),
            require_partition_filter: core::default::Default::default(),
            source_uri_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef {
        BigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nWhen set, what mode of hive partitioning to use when reading data."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn require_partition_filter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_uri_prefix` after provisioning.\nWhen hive partition detection is requested, a common for all source uris must be required. The prefix must end immediately before the partition key encoding begins."]
    pub fn source_uri_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_uri_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalDataConfiguration_JsonOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryTable_ExternalDataConfiguration_JsonOptions {
    #[doc =
        "Set the field `encoding`.\nThe character encoding of the data. The supported values are UTF-8, UTF-16BE, UTF-16LE, UTF-32BE, and UTF-32LE. The default value is UTF-8."]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalDataConfiguration_JsonOptions {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_JsonOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalDataConfiguration_JsonOptions {}

impl BuildBigqueryTable_ExternalDataConfiguration_JsonOptions {
    pub fn build(self) -> BigqueryTable_ExternalDataConfiguration_JsonOptions {
        BigqueryTable_ExternalDataConfiguration_JsonOptions { encoding: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalDataConfiguration_JsonOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalDataConfiguration_JsonOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalDataConfiguration_JsonOptionsRef {
        BigqueryTable_ExternalDataConfiguration_JsonOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalDataConfiguration_JsonOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encoding` after provisioning.\nThe character encoding of the data. The supported values are UTF-8, UTF-16BE, UTF-16LE, UTF-32BE, and UTF-32LE. The default value is UTF-8."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalDataConfiguration_ParquetOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_list_inference: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_as_string: std::option::Option<terrars::PrimField<bool>>,
}

impl BigqueryTable_ExternalDataConfiguration_ParquetOptions {
    #[doc =
        "Set the field `enable_list_inference`.\nIndicates whether to use schema inference specifically for Parquet LIST logical type."]
    pub fn set_enable_list_inference(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_list_inference = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enum_as_string`.\nIndicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default."]
    pub fn set_enum_as_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enum_as_string = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalDataConfiguration_ParquetOptions {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_ParquetOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalDataConfiguration_ParquetOptions {}

impl BuildBigqueryTable_ExternalDataConfiguration_ParquetOptions {
    pub fn build(self) -> BigqueryTable_ExternalDataConfiguration_ParquetOptions {
        BigqueryTable_ExternalDataConfiguration_ParquetOptions {
            enable_list_inference: core::default::Default::default(),
            enum_as_string: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalDataConfiguration_ParquetOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalDataConfiguration_ParquetOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_ExternalDataConfiguration_ParquetOptionsRef {
        BigqueryTable_ExternalDataConfiguration_ParquetOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalDataConfiguration_ParquetOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_list_inference` after provisioning.\nIndicates whether to use schema inference specifically for Parquet LIST logical type."]
    pub fn enable_list_inference(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_list_inference", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enum_as_string` after provisioning.\nIndicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default."]
    pub fn enum_as_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enum_as_string", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTable_ExternalDataConfigurationDynamic {
    avro_options: std::option::Option<terrars::DynamicBlock<BigqueryTable_ExternalDataConfiguration_AvroOptions>>,
    bigtable_options: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_ExternalDataConfiguration_BigtableOptions>>,
    csv_options: std::option::Option<terrars::DynamicBlock<BigqueryTable_ExternalDataConfiguration_CsvOptions>>,
    google_sheets_options: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions>>,
    hive_partitioning_options: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_ExternalDataConfiguration_HivePartitioningOptions>>,
    json_options: std::option::Option<terrars::DynamicBlock<BigqueryTable_ExternalDataConfiguration_JsonOptions>>,
    parquet_options: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_ExternalDataConfiguration_ParquetOptions>>,
}

#[derive(Serialize)]
pub struct BigqueryTable_ExternalDataConfiguration {
    autodetect: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    decimal_target_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_set_spec_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_unknown_values: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_extension: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_bad_records: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_cache_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_metadata: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_file_schema_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_format: std::option::Option<terrars::PrimField<std::string::String>>,
    source_uris: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro_options: std::option::Option<std::vec::Vec<BigqueryTable_ExternalDataConfiguration_AvroOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigtable_options: std::option::Option<std::vec::Vec<BigqueryTable_ExternalDataConfiguration_BigtableOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csv_options: std::option::Option<std::vec::Vec<BigqueryTable_ExternalDataConfiguration_CsvOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_sheets_options: std
    ::option
    ::Option<std::vec::Vec<BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_partitioning_options: std
    ::option
    ::Option<std::vec::Vec<BigqueryTable_ExternalDataConfiguration_HivePartitioningOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_options: std::option::Option<std::vec::Vec<BigqueryTable_ExternalDataConfiguration_JsonOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parquet_options: std::option::Option<std::vec::Vec<BigqueryTable_ExternalDataConfiguration_ParquetOptions>>,
    dynamic: BigqueryTable_ExternalDataConfigurationDynamic,
}

impl BigqueryTable_ExternalDataConfiguration {
    #[doc =
        "Set the field `compression`.\nThe compression type of the data source. Valid values are \"NONE\" or \"GZIP\"."]
    pub fn set_compression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection_id`.\nThe connection specifying the credentials to be used to read external storage, such as Azure Blob, Cloud Storage, or S3. The connectionId can have the form \"<project>.<location>.<connection_id>\" or \"projects/<project>/locations/<location>/connections/<connection_id>\"."]
    pub fn set_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `decimal_target_types`.\nThe data types that could be used as a target type when converting decimal values."]
    pub fn set_decimal_target_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.decimal_target_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_set_spec_type`.\nSpecifies how source URIs are interpreted for constructing the file set to load.  By default source URIs are expanded against the underlying storage.  Other options include specifying manifest files. Only applicable to object storage systems."]
    pub fn set_file_set_spec_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.file_set_spec_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_unknown_values`.\nIndicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false."]
    pub fn set_ignore_unknown_values(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_unknown_values = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `json_extension`.\nLoad option to be used together with sourceFormat newline-delimited JSON to indicate that a variant of JSON is being loaded. To load newline-delimited GeoJSON, specify GEOJSON (and sourceFormat must be set to NEWLINE_DELIMITED_JSON)."]
    pub fn set_json_extension(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json_extension = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_bad_records`.\nThe maximum number of bad records that BigQuery can ignore when reading data."]
    pub fn set_max_bad_records(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_bad_records = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metadata_cache_mode`.\nMetadata Cache Mode for the table. Set this to enable caching of metadata from external data source."]
    pub fn set_metadata_cache_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metadata_cache_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `object_metadata`.\nObject Metadata is used to create Object Tables. Object Tables contain a listing of objects (with their metadata) found at the sourceUris. If ObjectMetadata is set, sourceFormat should be omitted."]
    pub fn set_object_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reference_file_schema_uri`.\nWhen creating an external table, the user can provide a reference file with the table schema. This is enabled for the following formats: AVRO, PARQUET, ORC."]
    pub fn set_reference_file_schema_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.reference_file_schema_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schema`.\nA JSON schema for the external table. Schema is required for CSV and JSON formats and is disallowed for Google Cloud Bigtable, Cloud Datastore backups, and Avro formats when using external tables."]
    pub fn set_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_format`.\nPlease see sourceFormat under ExternalDataConfiguration in Bigquery's public API documentation (https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#externaldataconfiguration) for supported formats. To use \"GOOGLE_SHEETS\" the scopes must include \"googleapis.com/auth/drive.readonly\"."]
    pub fn set_source_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `avro_options`.\n"]
    pub fn set_avro_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_AvroOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.avro_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.avro_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bigtable_options`.\n"]
    pub fn set_bigtable_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_BigtableOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigtable_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigtable_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_CsvOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csv_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csv_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_sheets_options`.\n"]
    pub fn set_google_sheets_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_sheets_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_sheets_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hive_partitioning_options`.\n"]
    pub fn set_hive_partitioning_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_HivePartitioningOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hive_partitioning_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hive_partitioning_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json_options`.\n"]
    pub fn set_json_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_JsonOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parquet_options`.\n"]
    pub fn set_parquet_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration_ParquetOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parquet_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parquet_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_ExternalDataConfiguration {
    type O = terrars::BlockAssignable<BigqueryTable_ExternalDataConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_ExternalDataConfiguration {
    #[doc = "Let BigQuery try to autodetect the schema and format of the table."]
    pub autodetect: terrars::PrimField<bool>,
    #[doc = "A list of the fully-qualified URIs that point to your data in Google Cloud."]
    pub source_uris: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBigqueryTable_ExternalDataConfiguration {
    pub fn build(self) -> BigqueryTable_ExternalDataConfiguration {
        BigqueryTable_ExternalDataConfiguration {
            autodetect: self.autodetect,
            compression: core::default::Default::default(),
            connection_id: core::default::Default::default(),
            decimal_target_types: core::default::Default::default(),
            file_set_spec_type: core::default::Default::default(),
            ignore_unknown_values: core::default::Default::default(),
            json_extension: core::default::Default::default(),
            max_bad_records: core::default::Default::default(),
            metadata_cache_mode: core::default::Default::default(),
            object_metadata: core::default::Default::default(),
            reference_file_schema_uri: core::default::Default::default(),
            schema: core::default::Default::default(),
            source_format: core::default::Default::default(),
            source_uris: self.source_uris,
            avro_options: core::default::Default::default(),
            bigtable_options: core::default::Default::default(),
            csv_options: core::default::Default::default(),
            google_sheets_options: core::default::Default::default(),
            hive_partitioning_options: core::default::Default::default(),
            json_options: core::default::Default::default(),
            parquet_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ExternalDataConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ExternalDataConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_ExternalDataConfigurationRef {
        BigqueryTable_ExternalDataConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ExternalDataConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `autodetect` after provisioning.\nLet BigQuery try to autodetect the schema and format of the table."]
    pub fn autodetect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autodetect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compression` after provisioning.\nThe compression type of the data source. Valid values are \"NONE\" or \"GZIP\"."]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_id` after provisioning.\nThe connection specifying the credentials to be used to read external storage, such as Azure Blob, Cloud Storage, or S3. The connectionId can have the form \"<project>.<location>.<connection_id>\" or \"projects/<project>/locations/<location>/connections/<connection_id>\"."]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `decimal_target_types` after provisioning.\nThe data types that could be used as a target type when converting decimal values."]
    pub fn decimal_target_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.decimal_target_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_set_spec_type` after provisioning.\nSpecifies how source URIs are interpreted for constructing the file set to load.  By default source URIs are expanded against the underlying storage.  Other options include specifying manifest files. Only applicable to object storage systems."]
    pub fn file_set_spec_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_set_spec_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_unknown_values` after provisioning.\nIndicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false."]
    pub fn ignore_unknown_values(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_unknown_values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `json_extension` after provisioning.\nLoad option to be used together with sourceFormat newline-delimited JSON to indicate that a variant of JSON is being loaded. To load newline-delimited GeoJSON, specify GEOJSON (and sourceFormat must be set to NEWLINE_DELIMITED_JSON)."]
    pub fn json_extension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_extension", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_bad_records` after provisioning.\nThe maximum number of bad records that BigQuery can ignore when reading data."]
    pub fn max_bad_records(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_bad_records", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata_cache_mode` after provisioning.\nMetadata Cache Mode for the table. Set this to enable caching of metadata from external data source."]
    pub fn metadata_cache_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_cache_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `object_metadata` after provisioning.\nObject Metadata is used to create Object Tables. Object Tables contain a listing of objects (with their metadata) found at the sourceUris. If ObjectMetadata is set, sourceFormat should be omitted."]
    pub fn object_metadata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reference_file_schema_uri` after provisioning.\nWhen creating an external table, the user can provide a reference file with the table schema. This is enabled for the following formats: AVRO, PARQUET, ORC."]
    pub fn reference_file_schema_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_file_schema_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nA JSON schema for the external table. Schema is required for CSV and JSON formats and is disallowed for Google Cloud Bigtable, Cloud Datastore backups, and Avro formats when using external tables."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_format` after provisioning.\nPlease see sourceFormat under ExternalDataConfiguration in Bigquery's public API documentation (https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#externaldataconfiguration) for supported formats. To use \"GOOGLE_SHEETS\" the scopes must include \"googleapis.com/auth/drive.readonly\"."]
    pub fn source_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_uris` after provisioning.\nA list of the fully-qualified URIs that point to your data in Google Cloud."]
    pub fn source_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_uris", self.base))
    }

    #[doc = "Get a reference to the value of field `avro_options` after provisioning.\n"]
    pub fn avro_options(&self) -> terrars::ListRef<BigqueryTable_ExternalDataConfiguration_AvroOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro_options", self.base))
    }

    #[doc = "Get a reference to the value of field `bigtable_options` after provisioning.\n"]
    pub fn bigtable_options(&self) -> terrars::ListRef<BigqueryTable_ExternalDataConfiguration_BigtableOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigtable_options", self.base))
    }

    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(&self) -> terrars::ListRef<BigqueryTable_ExternalDataConfiguration_CsvOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csv_options", self.base))
    }

    #[doc = "Get a reference to the value of field `google_sheets_options` after provisioning.\n"]
    pub fn google_sheets_options(
        &self,
    ) -> terrars::ListRef<BigqueryTable_ExternalDataConfiguration_GoogleSheetsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_sheets_options", self.base))
    }

    #[doc = "Get a reference to the value of field `hive_partitioning_options` after provisioning.\n"]
    pub fn hive_partitioning_options(
        &self,
    ) -> terrars::ListRef<BigqueryTable_ExternalDataConfiguration_HivePartitioningOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_partitioning_options", self.base))
    }

    #[doc = "Get a reference to the value of field `json_options` after provisioning.\n"]
    pub fn json_options(&self) -> terrars::ListRef<BigqueryTable_ExternalDataConfiguration_JsonOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_options", self.base))
    }

    #[doc = "Get a reference to the value of field `parquet_options` after provisioning.\n"]
    pub fn parquet_options(&self) -> terrars::ListRef<BigqueryTable_ExternalDataConfiguration_ParquetOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parquet_options", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_MaterializedView {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_non_incremental_definition: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_refresh: std::option::Option<terrars::PrimField<bool>>,
    query: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_interval_ms: std::option::Option<terrars::PrimField<f64>>,
}

impl BigqueryTable_MaterializedView {
    #[doc =
        "Set the field `allow_non_incremental_definition`.\nAllow non incremental materialized view definition. The default value is false."]
    pub fn set_allow_non_incremental_definition(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_non_incremental_definition = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_refresh`.\nSpecifies if BigQuery should automatically refresh materialized view when the base table is updated. The default is true."]
    pub fn set_enable_refresh(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_refresh = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `refresh_interval_ms`.\nSpecifies maximum frequency at which this materialized view will be refreshed. The default is 1800000."]
    pub fn set_refresh_interval_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.refresh_interval_ms = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_MaterializedView {
    type O = terrars::BlockAssignable<BigqueryTable_MaterializedView>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_MaterializedView {
    #[doc = "A query whose result is persisted."]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_MaterializedView {
    pub fn build(self) -> BigqueryTable_MaterializedView {
        BigqueryTable_MaterializedView {
            allow_non_incremental_definition: core::default::Default::default(),
            enable_refresh: core::default::Default::default(),
            query: self.query,
            refresh_interval_ms: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_MaterializedViewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_MaterializedViewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_MaterializedViewRef {
        BigqueryTable_MaterializedViewRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_MaterializedViewRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_non_incremental_definition` after provisioning.\nAllow non incremental materialized view definition. The default value is false."]
    pub fn allow_non_incremental_definition(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_non_incremental_definition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_refresh` after provisioning.\nSpecifies if BigQuery should automatically refresh materialized view when the base table is updated. The default is true."]
    pub fn enable_refresh(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_refresh", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\nA query whose result is persisted."]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc =
        "Get a reference to the value of field `refresh_interval_ms` after provisioning.\nSpecifies maximum frequency at which this materialized view will be refreshed. The default is 1800000."]
    pub fn refresh_interval_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_interval_ms", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_RangePartitioning_Range {
    end: terrars::PrimField<f64>,
    interval: terrars::PrimField<f64>,
    start: terrars::PrimField<f64>,
}

impl BigqueryTable_RangePartitioning_Range { }

impl terrars::ToListMappable for BigqueryTable_RangePartitioning_Range {
    type O = terrars::BlockAssignable<BigqueryTable_RangePartitioning_Range>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_RangePartitioning_Range {
    #[doc = "End of the range partitioning, exclusive."]
    pub end: terrars::PrimField<f64>,
    #[doc = "The width of each range within the partition."]
    pub interval: terrars::PrimField<f64>,
    #[doc = "Start of the range partitioning, inclusive."]
    pub start: terrars::PrimField<f64>,
}

impl BuildBigqueryTable_RangePartitioning_Range {
    pub fn build(self) -> BigqueryTable_RangePartitioning_Range {
        BigqueryTable_RangePartitioning_Range {
            end: self.end,
            interval: self.interval,
            start: self.start,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_RangePartitioning_RangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_RangePartitioning_RangeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_RangePartitioning_RangeRef {
        BigqueryTable_RangePartitioning_RangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_RangePartitioning_RangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end` after provisioning.\nEnd of the range partitioning, exclusive."]
    pub fn end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nThe width of each range within the partition."]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start` after provisioning.\nStart of the range partitioning, inclusive."]
    pub fn start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTable_RangePartitioningDynamic {
    range: std::option::Option<terrars::DynamicBlock<BigqueryTable_RangePartitioning_Range>>,
}

#[derive(Serialize)]
pub struct BigqueryTable_RangePartitioning {
    field: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<std::vec::Vec<BigqueryTable_RangePartitioning_Range>>,
    dynamic: BigqueryTable_RangePartitioningDynamic,
}

impl BigqueryTable_RangePartitioning {
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_RangePartitioning_Range>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_RangePartitioning {
    type O = terrars::BlockAssignable<BigqueryTable_RangePartitioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_RangePartitioning {
    #[doc = "The field used to determine how to create a range-based partition."]
    pub field: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_RangePartitioning {
    pub fn build(self) -> BigqueryTable_RangePartitioning {
        BigqueryTable_RangePartitioning {
            field: self.field,
            range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_RangePartitioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_RangePartitioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_RangePartitioningRef {
        BigqueryTable_RangePartitioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_RangePartitioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `field` after provisioning.\nThe field used to determine how to create a range-based partition."]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::ListRef<BigqueryTable_RangePartitioning_RangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_SchemaForeignTypeInfo {
    type_system: terrars::PrimField<std::string::String>,
}

impl BigqueryTable_SchemaForeignTypeInfo { }

impl terrars::ToListMappable for BigqueryTable_SchemaForeignTypeInfo {
    type O = terrars::BlockAssignable<BigqueryTable_SchemaForeignTypeInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_SchemaForeignTypeInfo {
    #[doc = "Specifies the system which defines the foreign data type."]
    pub type_system: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_SchemaForeignTypeInfo {
    pub fn build(self) -> BigqueryTable_SchemaForeignTypeInfo {
        BigqueryTable_SchemaForeignTypeInfo { type_system: self.type_system }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_SchemaForeignTypeInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_SchemaForeignTypeInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_SchemaForeignTypeInfoRef {
        BigqueryTable_SchemaForeignTypeInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_SchemaForeignTypeInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_system` after provisioning.\nSpecifies the system which defines the foreign data type."]
    pub fn type_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_system", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
    referenced_column: terrars::PrimField<std::string::String>,
    referencing_column: terrars::PrimField<std::string::String>,
}

impl BigqueryTable_TableConstraints_ForeignKeys_ColumnReferences { }

impl terrars::ToListMappable for BigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
    type O = terrars::BlockAssignable<BigqueryTable_TableConstraints_ForeignKeys_ColumnReferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
    #[doc = "The column in the primary key that are referenced by the referencingColumn."]
    pub referenced_column: terrars::PrimField<std::string::String>,
    #[doc = "The column that composes the foreign key."]
    pub referencing_column: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
    pub fn build(self) -> BigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
        BigqueryTable_TableConstraints_ForeignKeys_ColumnReferences {
            referenced_column: self.referenced_column,
            referencing_column: self.referencing_column,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef {
        BigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `referenced_column` after provisioning.\nThe column in the primary key that are referenced by the referencingColumn."]
    pub fn referenced_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.referenced_column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `referencing_column` after provisioning.\nThe column that composes the foreign key."]
    pub fn referencing_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.referencing_column", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    table_id: terrars::PrimField<std::string::String>,
}

impl BigqueryTable_TableConstraints_ForeignKeys_ReferencedTable { }

impl terrars::ToListMappable for BigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
    type O = terrars::BlockAssignable<BigqueryTable_TableConstraints_ForeignKeys_ReferencedTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters. Certain operations allow suffixing of the table ID with a partition decorator, such as sample_table$20190123."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
    pub fn build(self) -> BigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
        BigqueryTable_TableConstraints_ForeignKeys_ReferencedTable {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef {
        BigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nThe ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters. Certain operations allow suffixing of the table ID with a partition decorator, such as sample_table$20190123."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTable_TableConstraints_ForeignKeysDynamic {
    column_references: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_TableConstraints_ForeignKeys_ColumnReferences>>,
    referenced_table: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryTable_TableConstraints_ForeignKeys_ReferencedTable>>,
}

#[derive(Serialize)]
pub struct BigqueryTable_TableConstraints_ForeignKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_references: std::option::Option<std::vec::Vec<BigqueryTable_TableConstraints_ForeignKeys_ColumnReferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referenced_table: std::option::Option<std::vec::Vec<BigqueryTable_TableConstraints_ForeignKeys_ReferencedTable>>,
    dynamic: BigqueryTable_TableConstraints_ForeignKeysDynamic,
}

impl BigqueryTable_TableConstraints_ForeignKeys {
    #[doc = "Set the field `name`.\nSet only if the foreign key constraint is named."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_references`.\n"]
    pub fn set_column_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BigqueryTable_TableConstraints_ForeignKeys_ColumnReferences>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column_references = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column_references = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `referenced_table`.\n"]
    pub fn set_referenced_table(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BigqueryTable_TableConstraints_ForeignKeys_ReferencedTable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.referenced_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.referenced_table = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_TableConstraints_ForeignKeys {
    type O = terrars::BlockAssignable<BigqueryTable_TableConstraints_ForeignKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_TableConstraints_ForeignKeys {}

impl BuildBigqueryTable_TableConstraints_ForeignKeys {
    pub fn build(self) -> BigqueryTable_TableConstraints_ForeignKeys {
        BigqueryTable_TableConstraints_ForeignKeys {
            name: core::default::Default::default(),
            column_references: core::default::Default::default(),
            referenced_table: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_TableConstraints_ForeignKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_TableConstraints_ForeignKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_TableConstraints_ForeignKeysRef {
        BigqueryTable_TableConstraints_ForeignKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_TableConstraints_ForeignKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSet only if the foreign key constraint is named."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `column_references` after provisioning.\n"]
    pub fn column_references(&self) -> terrars::ListRef<BigqueryTable_TableConstraints_ForeignKeys_ColumnReferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_references", self.base))
    }

    #[doc = "Get a reference to the value of field `referenced_table` after provisioning.\n"]
    pub fn referenced_table(&self) -> terrars::ListRef<BigqueryTable_TableConstraints_ForeignKeys_ReferencedTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.referenced_table", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_TableConstraints_PrimaryKey {
    columns: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BigqueryTable_TableConstraints_PrimaryKey { }

impl terrars::ToListMappable for BigqueryTable_TableConstraints_PrimaryKey {
    type O = terrars::BlockAssignable<BigqueryTable_TableConstraints_PrimaryKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_TableConstraints_PrimaryKey {
    #[doc = "The columns that are composed of the primary key constraint."]
    pub columns: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBigqueryTable_TableConstraints_PrimaryKey {
    pub fn build(self) -> BigqueryTable_TableConstraints_PrimaryKey {
        BigqueryTable_TableConstraints_PrimaryKey { columns: self.columns }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_TableConstraints_PrimaryKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_TableConstraints_PrimaryKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_TableConstraints_PrimaryKeyRef {
        BigqueryTable_TableConstraints_PrimaryKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_TableConstraints_PrimaryKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `columns` after provisioning.\nThe columns that are composed of the primary key constraint."]
    pub fn columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTable_TableConstraintsDynamic {
    foreign_keys: std::option::Option<terrars::DynamicBlock<BigqueryTable_TableConstraints_ForeignKeys>>,
    primary_key: std::option::Option<terrars::DynamicBlock<BigqueryTable_TableConstraints_PrimaryKey>>,
}

#[derive(Serialize)]
pub struct BigqueryTable_TableConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    foreign_keys: std::option::Option<std::vec::Vec<BigqueryTable_TableConstraints_ForeignKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_key: std::option::Option<std::vec::Vec<BigqueryTable_TableConstraints_PrimaryKey>>,
    dynamic: BigqueryTable_TableConstraintsDynamic,
}

impl BigqueryTable_TableConstraints {
    #[doc = "Set the field `foreign_keys`.\n"]
    pub fn set_foreign_keys(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_TableConstraints_ForeignKeys>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.foreign_keys = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.foreign_keys = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `primary_key`.\n"]
    pub fn set_primary_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryTable_TableConstraints_PrimaryKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.primary_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.primary_key = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_TableConstraints {
    type O = terrars::BlockAssignable<BigqueryTable_TableConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_TableConstraints {}

impl BuildBigqueryTable_TableConstraints {
    pub fn build(self) -> BigqueryTable_TableConstraints {
        BigqueryTable_TableConstraints {
            foreign_keys: core::default::Default::default(),
            primary_key: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_TableConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_TableConstraintsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_TableConstraintsRef {
        BigqueryTable_TableConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_TableConstraintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `foreign_keys` after provisioning.\n"]
    pub fn foreign_keys(&self) -> terrars::ListRef<BigqueryTable_TableConstraints_ForeignKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.foreign_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_key` after provisioning.\n"]
    pub fn primary_key(&self) -> terrars::ListRef<BigqueryTable_TableConstraints_PrimaryKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_TableReplicationInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_interval_ms: std::option::Option<terrars::PrimField<f64>>,
    source_dataset_id: terrars::PrimField<std::string::String>,
    source_project_id: terrars::PrimField<std::string::String>,
    source_table_id: terrars::PrimField<std::string::String>,
}

impl BigqueryTable_TableReplicationInfo {
    #[doc =
        "Set the field `replication_interval_ms`.\nThe interval at which the source materialized view is polled for updates. The default is 300000."]
    pub fn set_replication_interval_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replication_interval_ms = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_TableReplicationInfo {
    type O = terrars::BlockAssignable<BigqueryTable_TableReplicationInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_TableReplicationInfo {
    #[doc = "The ID of the source dataset."]
    pub source_dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the source project."]
    pub source_project_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the source materialized view."]
    pub source_table_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_TableReplicationInfo {
    pub fn build(self) -> BigqueryTable_TableReplicationInfo {
        BigqueryTable_TableReplicationInfo {
            replication_interval_ms: core::default::Default::default(),
            source_dataset_id: self.source_dataset_id,
            source_project_id: self.source_project_id,
            source_table_id: self.source_table_id,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_TableReplicationInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_TableReplicationInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_TableReplicationInfoRef {
        BigqueryTable_TableReplicationInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_TableReplicationInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `replication_interval_ms` after provisioning.\nThe interval at which the source materialized view is polled for updates. The default is 300000."]
    pub fn replication_interval_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_interval_ms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_dataset_id` after provisioning.\nThe ID of the source dataset."]
    pub fn source_dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_project_id` after provisioning.\nThe ID of the source project."]
    pub fn source_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_table_id` after provisioning.\nThe ID of the source materialized view."]
    pub fn source_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_table_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_TimePartitioning {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_ms: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_partition_filter: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BigqueryTable_TimePartitioning {
    #[doc = "Set the field `expiration_ms`.\nNumber of milliseconds for which to keep the storage for a partition."]
    pub fn set_expiration_ms(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.expiration_ms = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `field`.\nThe field used to determine how to create a time-based partition. If time-based partitioning is enabled without this value, the table is partitioned based on the load time."]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require_partition_filter`.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn set_require_partition_filter(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_partition_filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_TimePartitioning {
    type O = terrars::BlockAssignable<BigqueryTable_TimePartitioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_TimePartitioning {
    #[doc =
        "The supported types are DAY, HOUR, MONTH, and YEAR, which will generate one partition per day, hour, month, and year, respectively."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_TimePartitioning {
    pub fn build(self) -> BigqueryTable_TimePartitioning {
        BigqueryTable_TimePartitioning {
            expiration_ms: core::default::Default::default(),
            field: core::default::Default::default(),
            require_partition_filter: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_TimePartitioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_TimePartitioningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_TimePartitioningRef {
        BigqueryTable_TimePartitioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_TimePartitioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expiration_ms` after provisioning.\nNumber of milliseconds for which to keep the storage for a partition."]
    pub fn expiration_ms(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_ms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field` after provisioning.\nThe field used to determine how to create a time-based partition. If time-based partitioning is enabled without this value, the table is partitioned based on the load time."]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn require_partition_filter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe supported types are DAY, HOUR, MONTH, and YEAR, which will generate one partition per day, hour, month, and year, respectively."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTable_View {
    query: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_legacy_sql: std::option::Option<terrars::PrimField<bool>>,
}

impl BigqueryTable_View {
    #[doc =
        "Set the field `use_legacy_sql`.\nSpecifies whether to use BigQuery's legacy SQL for this view. If set to false, the view will use BigQuery's standard SQL"]
    pub fn set_use_legacy_sql(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_legacy_sql = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryTable_View {
    type O = terrars::BlockAssignable<BigqueryTable_View>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTable_View {
    #[doc = "A query that BigQuery executes when the view is referenced."]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryTable_View {
    pub fn build(self) -> BigqueryTable_View {
        BigqueryTable_View {
            query: self.query,
            use_legacy_sql: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryTable_ViewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryTable_ViewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryTable_ViewRef {
        BigqueryTable_ViewRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTable_ViewRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `query` after provisioning.\nA query that BigQuery executes when the view is referenced."]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_legacy_sql` after provisioning.\nSpecifies whether to use BigQuery's legacy SQL for this view. If set to false, the view will use BigQuery's standard SQL"]
    pub fn use_legacy_sql(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_legacy_sql", self.base))
    }
}
