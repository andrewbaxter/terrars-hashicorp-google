use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clustering: Option<ListField<PrimField<String>>>,
    dataset_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_time: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    friendly_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_staleness: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_partition_filter: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
    table_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<Vec<BigqueryTableEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_data_configuration: Option<Vec<BigqueryTableExternalDataConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    materialized_view: Option<Vec<BigqueryTableMaterializedViewEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_partitioning: Option<Vec<BigqueryTableRangePartitioningEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_constraints: Option<Vec<BigqueryTableTableConstraintsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_partitioning: Option<Vec<BigqueryTableTimePartitioningEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view: Option<Vec<BigqueryTableViewEl>>,
    dynamic: BigqueryTableDynamic,
}

struct BigqueryTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryTableData>,
}

#[derive(Clone)]
pub struct BigqueryTable(Rc<BigqueryTable_>);

impl BigqueryTable {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `clustering`.\nSpecifies column names to use for data clustering. Up to four top-level columns are allowed, and should be specified in descending priority order."]
    pub fn set_clustering(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().clustering = Some(v.into());
        self
    }

    #[doc= "Set the field `deletion_protection`.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nThe field description."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `expiration_time`.\nThe time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
    pub fn set_expiration_time(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().expiration_time = Some(v.into());
        self
    }

    #[doc= "Set the field `friendly_name`.\nA descriptive name for the table."]
    pub fn set_friendly_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().friendly_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nA mapping of labels to assign to the resource.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `max_staleness`.\nThe maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of [SQL IntervalValue type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type)."]
    pub fn set_max_staleness(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().max_staleness = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nThe ID of the project in which the resource belongs."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `require_partition_filter`.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn set_require_partition_filter(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().require_partition_filter = Some(v.into());
        self
    }

    #[doc= "Set the field `schema`.\nA JSON schema for the table."]
    pub fn set_schema(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schema = Some(v.into());
        self
    }

    #[doc= "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<BigqueryTableEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `external_data_configuration`.\n"]
    pub fn set_external_data_configuration(
        self,
        v: impl Into<BlockAssignable<BigqueryTableExternalDataConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_data_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_data_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `materialized_view`.\n"]
    pub fn set_materialized_view(self, v: impl Into<BlockAssignable<BigqueryTableMaterializedViewEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().materialized_view = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.materialized_view = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `range_partitioning`.\n"]
    pub fn set_range_partitioning(self, v: impl Into<BlockAssignable<BigqueryTableRangePartitioningEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().range_partitioning = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.range_partitioning = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `table_constraints`.\n"]
    pub fn set_table_constraints(self, v: impl Into<BlockAssignable<BigqueryTableTableConstraintsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table_constraints = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table_constraints = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `time_partitioning`.\n"]
    pub fn set_time_partitioning(self, v: impl Into<BlockAssignable<BigqueryTableTimePartitioningEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().time_partitioning = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.time_partitioning = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `view`.\n"]
    pub fn set_view(self, v: impl Into<BlockAssignable<BigqueryTableViewEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().view = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.view = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `clustering` after provisioning.\nSpecifies column names to use for data clustering. Up to four top-level columns are allowed, and should be specified in descending priority order."]
    pub fn clustering(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.clustering", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this table was created, in milliseconds since the epoch."]
    pub fn creation_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset ID to create the table in. Changing this forces a new resource to be created."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe field description."]
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

    #[doc= "Get a reference to the value of field `expiration_time` after provisioning.\nThe time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
    pub fn expiration_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the table."]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA mapping of labels to assign to the resource.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this table was last modified, in milliseconds since the epoch."]
    pub fn last_modified_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_staleness` after provisioning.\nThe maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of [SQL IntervalValue type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type)."]
    pub fn max_staleness(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_staleness", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `num_bytes` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn num_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `num_long_term_bytes` after provisioning.\nThe number of bytes in the table that are considered \"long-term storage\"."]
    pub fn num_long_term_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_long_term_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `num_rows` after provisioning.\nThe number of rows of data in this table, excluding any data in the streaming buffer."]
    pub fn num_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_rows", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn require_partition_filter(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schema` after provisioning.\nA JSON schema for the table."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `table_id` after provisioning.\nA unique ID for the resource. Changing this forces a new resource to be created."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nDescribes the table type."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<BigqueryTableEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_data_configuration` after provisioning.\n"]
    pub fn external_data_configuration(&self) -> ListRef<BigqueryTableExternalDataConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_data_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `materialized_view` after provisioning.\n"]
    pub fn materialized_view(&self) -> ListRef<BigqueryTableMaterializedViewElRef> {
        ListRef::new(self.shared().clone(), format!("{}.materialized_view", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `range_partitioning` after provisioning.\n"]
    pub fn range_partitioning(&self) -> ListRef<BigqueryTableRangePartitioningElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range_partitioning", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `table_constraints` after provisioning.\n"]
    pub fn table_constraints(&self) -> ListRef<BigqueryTableTableConstraintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.table_constraints", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `time_partitioning` after provisioning.\n"]
    pub fn time_partitioning(&self) -> ListRef<BigqueryTableTimePartitioningElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_partitioning", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> ListRef<BigqueryTableViewElRef> {
        ListRef::new(self.shared().clone(), format!("{}.view", self.extract_ref()))
    }
}

impl Referable for BigqueryTable {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BigqueryTable { }

impl ToListMappable for BigqueryTable {
    type O = ListRef<BigqueryTableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BigqueryTable_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_table".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryTable {
    pub tf_id: String,
    #[doc= "The dataset ID to create the table in. Changing this forces a new resource to be created."]
    pub dataset_id: PrimField<String>,
    #[doc= "A unique ID for the resource. Changing this forces a new resource to be created."]
    pub table_id: PrimField<String>,
}

impl BuildBigqueryTable {
    pub fn build(self, stack: &mut Stack) -> BigqueryTable {
        let out = BigqueryTable(Rc::new(BigqueryTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                clustering: core::default::Default::default(),
                dataset_id: self.dataset_id,
                deletion_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                expiration_time: core::default::Default::default(),
                friendly_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                max_staleness: core::default::Default::default(),
                project: core::default::Default::default(),
                require_partition_filter: core::default::Default::default(),
                schema: core::default::Default::default(),
                table_id: self.table_id,
                encryption_configuration: core::default::Default::default(),
                external_data_configuration: core::default::Default::default(),
                materialized_view: core::default::Default::default(),
                range_partitioning: core::default::Default::default(),
                table_constraints: core::default::Default::default(),
                time_partitioning: core::default::Default::default(),
                view: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryTableRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryTableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `clustering` after provisioning.\nSpecifies column names to use for data clustering. Up to four top-level columns are allowed, and should be specified in descending priority order."]
    pub fn clustering(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.clustering", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this table was created, in milliseconds since the epoch."]
    pub fn creation_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset ID to create the table in. Changing this forces a new resource to be created."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nThe field description."]
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

    #[doc= "Get a reference to the value of field `expiration_time` after provisioning.\nThe time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
    pub fn expiration_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the table."]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA mapping of labels to assign to the resource.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this table was last modified, in milliseconds since the epoch."]
    pub fn last_modified_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_staleness` after provisioning.\nThe maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of [SQL IntervalValue type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type)."]
    pub fn max_staleness(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_staleness", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `num_bytes` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn num_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `num_long_term_bytes` after provisioning.\nThe number of bytes in the table that are considered \"long-term storage\"."]
    pub fn num_long_term_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_long_term_bytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `num_rows` after provisioning.\nThe number of rows of data in this table, excluding any data in the streaming buffer."]
    pub fn num_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_rows", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn require_partition_filter(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schema` after provisioning.\nA JSON schema for the table."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `table_id` after provisioning.\nA unique ID for the resource. Changing this forces a new resource to be created."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nDescribes the table type."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<BigqueryTableEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `external_data_configuration` after provisioning.\n"]
    pub fn external_data_configuration(&self) -> ListRef<BigqueryTableExternalDataConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external_data_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `materialized_view` after provisioning.\n"]
    pub fn materialized_view(&self) -> ListRef<BigqueryTableMaterializedViewElRef> {
        ListRef::new(self.shared().clone(), format!("{}.materialized_view", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `range_partitioning` after provisioning.\n"]
    pub fn range_partitioning(&self) -> ListRef<BigqueryTableRangePartitioningElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range_partitioning", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `table_constraints` after provisioning.\n"]
    pub fn table_constraints(&self) -> ListRef<BigqueryTableTableConstraintsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.table_constraints", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `time_partitioning` after provisioning.\n"]
    pub fn time_partitioning(&self) -> ListRef<BigqueryTableTimePartitioningElRef> {
        ListRef::new(self.shared().clone(), format!("{}.time_partitioning", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> ListRef<BigqueryTableViewElRef> {
        ListRef::new(self.shared().clone(), format!("{}.view", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableEncryptionConfigurationEl {
    kms_key_name: PrimField<String>,
}

impl BigqueryTableEncryptionConfigurationEl { }

impl ToListMappable for BigqueryTableEncryptionConfigurationEl {
    type O = BlockAssignable<BigqueryTableEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableEncryptionConfigurationEl {
    #[doc= "The self link or full name of a key which should be used to encrypt this table. Note that the default bigquery service account will need to have encrypt/decrypt permissions on this key - you may want to see the google_bigquery_default_service_account datasource and the google_kms_crypto_key_iam_binding resource."]
    pub kms_key_name: PrimField<String>,
}

impl BuildBigqueryTableEncryptionConfigurationEl {
    pub fn build(self) -> BigqueryTableEncryptionConfigurationEl {
        BigqueryTableEncryptionConfigurationEl { kms_key_name: self.kms_key_name }
    }
}

pub struct BigqueryTableEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableEncryptionConfigurationElRef {
        BigqueryTableEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nThe self link or full name of a key which should be used to encrypt this table. Note that the default bigquery service account will need to have encrypt/decrypt permissions on this key - you may want to see the google_bigquery_default_service_account datasource and the google_kms_crypto_key_iam_binding resource."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_version` after provisioning.\nThe self link or full name of the kms key version used to encrypt this table."]
    pub fn kms_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_version", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableExternalDataConfigurationElAvroOptionsEl {
    use_avro_logical_types: PrimField<bool>,
}

impl BigqueryTableExternalDataConfigurationElAvroOptionsEl { }

impl ToListMappable for BigqueryTableExternalDataConfigurationElAvroOptionsEl {
    type O = BlockAssignable<BigqueryTableExternalDataConfigurationElAvroOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableExternalDataConfigurationElAvroOptionsEl {
    #[doc= "If sourceFormat is set to \"AVRO\", indicates whether to interpret logical types as the corresponding BigQuery data type (for example, TIMESTAMP), instead of using the raw type (for example, INTEGER)."]
    pub use_avro_logical_types: PrimField<bool>,
}

impl BuildBigqueryTableExternalDataConfigurationElAvroOptionsEl {
    pub fn build(self) -> BigqueryTableExternalDataConfigurationElAvroOptionsEl {
        BigqueryTableExternalDataConfigurationElAvroOptionsEl { use_avro_logical_types: self.use_avro_logical_types }
    }
}

pub struct BigqueryTableExternalDataConfigurationElAvroOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableExternalDataConfigurationElAvroOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableExternalDataConfigurationElAvroOptionsElRef {
        BigqueryTableExternalDataConfigurationElAvroOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableExternalDataConfigurationElAvroOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `use_avro_logical_types` after provisioning.\nIf sourceFormat is set to \"AVRO\", indicates whether to interpret logical types as the corresponding BigQuery data type (for example, TIMESTAMP), instead of using the raw type (for example, INTEGER)."]
    pub fn use_avro_logical_types(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_avro_logical_types", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableExternalDataConfigurationElCsvOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_jagged_rows: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_quoted_newlines: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_delimiter: Option<PrimField<String>>,
    quote: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_leading_rows: Option<PrimField<f64>>,
}

impl BigqueryTableExternalDataConfigurationElCsvOptionsEl {
    #[doc= "Set the field `allow_jagged_rows`.\nIndicates if BigQuery should accept rows that are missing trailing optional columns."]
    pub fn set_allow_jagged_rows(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_jagged_rows = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_quoted_newlines`.\nIndicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false."]
    pub fn set_allow_quoted_newlines(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_quoted_newlines = Some(v.into());
        self
    }

    #[doc= "Set the field `encoding`.\nThe character encoding of the data. The supported values are UTF-8 or ISO-8859-1."]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }

    #[doc= "Set the field `field_delimiter`.\nThe separator for fields in a CSV file."]
    pub fn set_field_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_delimiter = Some(v.into());
        self
    }

    #[doc= "Set the field `skip_leading_rows`.\nThe number of rows at the top of a CSV file that BigQuery will skip when reading the data."]
    pub fn set_skip_leading_rows(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.skip_leading_rows = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryTableExternalDataConfigurationElCsvOptionsEl {
    type O = BlockAssignable<BigqueryTableExternalDataConfigurationElCsvOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableExternalDataConfigurationElCsvOptionsEl {
    #[doc= "The value that is used to quote data sections in a CSV file. If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allow_quoted_newlines property to true. The API-side default is \", specified in Terraform escaped as \\\". Due to limitations with Terraform default values, this value is required to be explicitly set."]
    pub quote: PrimField<String>,
}

impl BuildBigqueryTableExternalDataConfigurationElCsvOptionsEl {
    pub fn build(self) -> BigqueryTableExternalDataConfigurationElCsvOptionsEl {
        BigqueryTableExternalDataConfigurationElCsvOptionsEl {
            allow_jagged_rows: core::default::Default::default(),
            allow_quoted_newlines: core::default::Default::default(),
            encoding: core::default::Default::default(),
            field_delimiter: core::default::Default::default(),
            quote: self.quote,
            skip_leading_rows: core::default::Default::default(),
        }
    }
}

pub struct BigqueryTableExternalDataConfigurationElCsvOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableExternalDataConfigurationElCsvOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableExternalDataConfigurationElCsvOptionsElRef {
        BigqueryTableExternalDataConfigurationElCsvOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableExternalDataConfigurationElCsvOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_jagged_rows` after provisioning.\nIndicates if BigQuery should accept rows that are missing trailing optional columns."]
    pub fn allow_jagged_rows(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_jagged_rows", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_quoted_newlines` after provisioning.\nIndicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false."]
    pub fn allow_quoted_newlines(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_quoted_newlines", self.base))
    }

    #[doc= "Get a reference to the value of field `encoding` after provisioning.\nThe character encoding of the data. The supported values are UTF-8 or ISO-8859-1."]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc= "Get a reference to the value of field `field_delimiter` after provisioning.\nThe separator for fields in a CSV file."]
    pub fn field_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_delimiter", self.base))
    }

    #[doc= "Get a reference to the value of field `quote` after provisioning.\nThe value that is used to quote data sections in a CSV file. If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allow_quoted_newlines property to true. The API-side default is \", specified in Terraform escaped as \\\". Due to limitations with Terraform default values, this value is required to be explicitly set."]
    pub fn quote(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quote", self.base))
    }

    #[doc= "Get a reference to the value of field `skip_leading_rows` after provisioning.\nThe number of rows at the top of a CSV file that BigQuery will skip when reading the data."]
    pub fn skip_leading_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_leading_rows", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_leading_rows: Option<PrimField<f64>>,
}

impl BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
    #[doc= "Set the field `range`.\nRange of a sheet to query from. Only used when non-empty. At least one of range or skip_leading_rows must be set. Typical format: \"sheet_name!top_left_cell_id:bottom_right_cell_id\" For example: \"sheet1!A1:B20\""]
    pub fn set_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range = Some(v.into());
        self
    }

    #[doc= "Set the field `skip_leading_rows`.\nThe number of rows at the top of the sheet that BigQuery will skip when reading the data. At least one of range or skip_leading_rows must be set."]
    pub fn set_skip_leading_rows(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.skip_leading_rows = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
    type O = BlockAssignable<BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {}

impl BuildBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
    pub fn build(self) -> BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
        BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
            range: core::default::Default::default(),
            skip_leading_rows: core::default::Default::default(),
        }
    }
}

pub struct BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef {
        BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `range` after provisioning.\nRange of a sheet to query from. Only used when non-empty. At least one of range or skip_leading_rows must be set. Typical format: \"sheet_name!top_left_cell_id:bottom_right_cell_id\" For example: \"sheet1!A1:B20\""]
    pub fn range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }

    #[doc= "Get a reference to the value of field `skip_leading_rows` after provisioning.\nThe number of rows at the top of the sheet that BigQuery will skip when reading the data. At least one of range or skip_leading_rows must be set."]
    pub fn skip_leading_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_leading_rows", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_partition_filter: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_uri_prefix: Option<PrimField<String>>,
}

impl BigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
    #[doc= "Set the field `mode`.\nWhen set, what mode of hive partitioning to use when reading data."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `require_partition_filter`.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn set_require_partition_filter(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_partition_filter = Some(v.into());
        self
    }

    #[doc= "Set the field `source_uri_prefix`.\nWhen hive partition detection is requested, a common for all source uris must be required. The prefix must end immediately before the partition key encoding begins."]
    pub fn set_source_uri_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_uri_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
    type O = BlockAssignable<BigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {}

impl BuildBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
    pub fn build(self) -> BigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
        BigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
            mode: core::default::Default::default(),
            require_partition_filter: core::default::Default::default(),
            source_uri_prefix: core::default::Default::default(),
        }
    }
}

pub struct BigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef {
        BigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nWhen set, what mode of hive partitioning to use when reading data."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn require_partition_filter(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.base))
    }

    #[doc= "Get a reference to the value of field `source_uri_prefix` after provisioning.\nWhen hive partition detection is requested, a common for all source uris must be required. The prefix must end immediately before the partition key encoding begins."]
    pub fn source_uri_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_uri_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableExternalDataConfigurationElJsonOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
}

impl BigqueryTableExternalDataConfigurationElJsonOptionsEl {
    #[doc= "Set the field `encoding`.\nThe character encoding of the data. The supported values are UTF-8, UTF-16BE, UTF-16LE, UTF-32BE, and UTF-32LE. The default value is UTF-8."]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryTableExternalDataConfigurationElJsonOptionsEl {
    type O = BlockAssignable<BigqueryTableExternalDataConfigurationElJsonOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableExternalDataConfigurationElJsonOptionsEl {}

impl BuildBigqueryTableExternalDataConfigurationElJsonOptionsEl {
    pub fn build(self) -> BigqueryTableExternalDataConfigurationElJsonOptionsEl {
        BigqueryTableExternalDataConfigurationElJsonOptionsEl { encoding: core::default::Default::default() }
    }
}

pub struct BigqueryTableExternalDataConfigurationElJsonOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableExternalDataConfigurationElJsonOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableExternalDataConfigurationElJsonOptionsElRef {
        BigqueryTableExternalDataConfigurationElJsonOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableExternalDataConfigurationElJsonOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `encoding` after provisioning.\nThe character encoding of the data. The supported values are UTF-8, UTF-16BE, UTF-16LE, UTF-32BE, and UTF-32LE. The default value is UTF-8."]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableExternalDataConfigurationElParquetOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_list_inference: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_as_string: Option<PrimField<bool>>,
}

impl BigqueryTableExternalDataConfigurationElParquetOptionsEl {
    #[doc= "Set the field `enable_list_inference`.\nIndicates whether to use schema inference specifically for Parquet LIST logical type."]
    pub fn set_enable_list_inference(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_list_inference = Some(v.into());
        self
    }

    #[doc= "Set the field `enum_as_string`.\nIndicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default."]
    pub fn set_enum_as_string(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enum_as_string = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryTableExternalDataConfigurationElParquetOptionsEl {
    type O = BlockAssignable<BigqueryTableExternalDataConfigurationElParquetOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableExternalDataConfigurationElParquetOptionsEl {}

impl BuildBigqueryTableExternalDataConfigurationElParquetOptionsEl {
    pub fn build(self) -> BigqueryTableExternalDataConfigurationElParquetOptionsEl {
        BigqueryTableExternalDataConfigurationElParquetOptionsEl {
            enable_list_inference: core::default::Default::default(),
            enum_as_string: core::default::Default::default(),
        }
    }
}

pub struct BigqueryTableExternalDataConfigurationElParquetOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableExternalDataConfigurationElParquetOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableExternalDataConfigurationElParquetOptionsElRef {
        BigqueryTableExternalDataConfigurationElParquetOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableExternalDataConfigurationElParquetOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_list_inference` after provisioning.\nIndicates whether to use schema inference specifically for Parquet LIST logical type."]
    pub fn enable_list_inference(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_list_inference", self.base))
    }

    #[doc= "Get a reference to the value of field `enum_as_string` after provisioning.\nIndicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default."]
    pub fn enum_as_string(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enum_as_string", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTableExternalDataConfigurationElDynamic {
    avro_options: Option<DynamicBlock<BigqueryTableExternalDataConfigurationElAvroOptionsEl>>,
    csv_options: Option<DynamicBlock<BigqueryTableExternalDataConfigurationElCsvOptionsEl>>,
    google_sheets_options: Option<DynamicBlock<BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl>>,
    hive_partitioning_options: Option<
        DynamicBlock<BigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl>,
    >,
    json_options: Option<DynamicBlock<BigqueryTableExternalDataConfigurationElJsonOptionsEl>>,
    parquet_options: Option<DynamicBlock<BigqueryTableExternalDataConfigurationElParquetOptionsEl>>,
}

#[derive(Serialize)]
pub struct BigqueryTableExternalDataConfigurationEl {
    autodetect: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_set_spec_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_unknown_values: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_bad_records: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_cache_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_metadata: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_file_schema_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_format: Option<PrimField<String>>,
    source_uris: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avro_options: Option<Vec<BigqueryTableExternalDataConfigurationElAvroOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_options: Option<Vec<BigqueryTableExternalDataConfigurationElCsvOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_sheets_options: Option<Vec<BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hive_partitioning_options: Option<Vec<BigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_options: Option<Vec<BigqueryTableExternalDataConfigurationElJsonOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parquet_options: Option<Vec<BigqueryTableExternalDataConfigurationElParquetOptionsEl>>,
    dynamic: BigqueryTableExternalDataConfigurationElDynamic,
}

impl BigqueryTableExternalDataConfigurationEl {
    #[doc= "Set the field `compression`.\nThe compression type of the data source. Valid values are \"NONE\" or \"GZIP\"."]
    pub fn set_compression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression = Some(v.into());
        self
    }

    #[doc= "Set the field `connection_id`.\nThe connection specifying the credentials to be used to read external storage, such as Azure Blob, Cloud Storage, or S3. The connectionId can have the form \"{{project}}.{{location}}.{{connection_id}}\" or \"projects/{{project}}/locations/{{location}}/connections/{{connection_id}}\"."]
    pub fn set_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_id = Some(v.into());
        self
    }

    #[doc= "Set the field `file_set_spec_type`.\nSpecifies how source URIs are interpreted for constructing the file set to load.  By default source URIs are expanded against the underlying storage.  Other options include specifying manifest files. Only applicable to object storage systems."]
    pub fn set_file_set_spec_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_set_spec_type = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore_unknown_values`.\nIndicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false."]
    pub fn set_ignore_unknown_values(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_unknown_values = Some(v.into());
        self
    }

    #[doc= "Set the field `max_bad_records`.\nThe maximum number of bad records that BigQuery can ignore when reading data."]
    pub fn set_max_bad_records(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_bad_records = Some(v.into());
        self
    }

    #[doc= "Set the field `metadata_cache_mode`.\nMetadata Cache Mode for the table. Set this to enable caching of metadata from external data source."]
    pub fn set_metadata_cache_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata_cache_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `object_metadata`.\nObject Metadata is used to create Object Tables. Object Tables contain a listing of objects (with their metadata) found at the sourceUris. If ObjectMetadata is set, sourceFormat should be omitted."]
    pub fn set_object_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_metadata = Some(v.into());
        self
    }

    #[doc= "Set the field `reference_file_schema_uri`.\nWhen creating an external table, the user can provide a reference file with the table schema. This is enabled for the following formats: AVRO, PARQUET, ORC."]
    pub fn set_reference_file_schema_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reference_file_schema_uri = Some(v.into());
        self
    }

    #[doc= "Set the field `schema`.\nA JSON schema for the external table. Schema is required for CSV and JSON formats and is disallowed for Google Cloud Bigtable, Cloud Datastore backups, and Avro formats when using external tables."]
    pub fn set_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema = Some(v.into());
        self
    }

    #[doc= "Set the field `source_format`.\n Please see sourceFormat under ExternalDataConfiguration in Bigquery's public API documentation (https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#externaldataconfiguration) for supported formats. To use \"GOOGLE_SHEETS\" the scopes must include \"googleapis.com/auth/drive.readonly\"."]
    pub fn set_source_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_format = Some(v.into());
        self
    }

    #[doc= "Set the field `avro_options`.\n"]
    pub fn set_avro_options(
        mut self,
        v: impl Into<BlockAssignable<BigqueryTableExternalDataConfigurationElAvroOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.avro_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.avro_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        mut self,
        v: impl Into<BlockAssignable<BigqueryTableExternalDataConfigurationElCsvOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.csv_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.csv_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `google_sheets_options`.\n"]
    pub fn set_google_sheets_options(
        mut self,
        v: impl Into<BlockAssignable<BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.google_sheets_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.google_sheets_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `hive_partitioning_options`.\n"]
    pub fn set_hive_partitioning_options(
        mut self,
        v: impl Into<BlockAssignable<BigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hive_partitioning_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hive_partitioning_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `json_options`.\n"]
    pub fn set_json_options(
        mut self,
        v: impl Into<BlockAssignable<BigqueryTableExternalDataConfigurationElJsonOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `parquet_options`.\n"]
    pub fn set_parquet_options(
        mut self,
        v: impl Into<BlockAssignable<BigqueryTableExternalDataConfigurationElParquetOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parquet_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parquet_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BigqueryTableExternalDataConfigurationEl {
    type O = BlockAssignable<BigqueryTableExternalDataConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableExternalDataConfigurationEl {
    #[doc= "Let BigQuery try to autodetect the schema and format of the table."]
    pub autodetect: PrimField<bool>,
    #[doc= "A list of the fully-qualified URIs that point to your data in Google Cloud."]
    pub source_uris: ListField<PrimField<String>>,
}

impl BuildBigqueryTableExternalDataConfigurationEl {
    pub fn build(self) -> BigqueryTableExternalDataConfigurationEl {
        BigqueryTableExternalDataConfigurationEl {
            autodetect: self.autodetect,
            compression: core::default::Default::default(),
            connection_id: core::default::Default::default(),
            file_set_spec_type: core::default::Default::default(),
            ignore_unknown_values: core::default::Default::default(),
            max_bad_records: core::default::Default::default(),
            metadata_cache_mode: core::default::Default::default(),
            object_metadata: core::default::Default::default(),
            reference_file_schema_uri: core::default::Default::default(),
            schema: core::default::Default::default(),
            source_format: core::default::Default::default(),
            source_uris: self.source_uris,
            avro_options: core::default::Default::default(),
            csv_options: core::default::Default::default(),
            google_sheets_options: core::default::Default::default(),
            hive_partitioning_options: core::default::Default::default(),
            json_options: core::default::Default::default(),
            parquet_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BigqueryTableExternalDataConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableExternalDataConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableExternalDataConfigurationElRef {
        BigqueryTableExternalDataConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableExternalDataConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `autodetect` after provisioning.\nLet BigQuery try to autodetect the schema and format of the table."]
    pub fn autodetect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.autodetect", self.base))
    }

    #[doc= "Get a reference to the value of field `compression` after provisioning.\nThe compression type of the data source. Valid values are \"NONE\" or \"GZIP\"."]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc= "Get a reference to the value of field `connection_id` after provisioning.\nThe connection specifying the credentials to be used to read external storage, such as Azure Blob, Cloud Storage, or S3. The connectionId can have the form \"{{project}}.{{location}}.{{connection_id}}\" or \"projects/{{project}}/locations/{{location}}/connections/{{connection_id}}\"."]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.base))
    }

    #[doc= "Get a reference to the value of field `file_set_spec_type` after provisioning.\nSpecifies how source URIs are interpreted for constructing the file set to load.  By default source URIs are expanded against the underlying storage.  Other options include specifying manifest files. Only applicable to object storage systems."]
    pub fn file_set_spec_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_set_spec_type", self.base))
    }

    #[doc= "Get a reference to the value of field `ignore_unknown_values` after provisioning.\nIndicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false."]
    pub fn ignore_unknown_values(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_unknown_values", self.base))
    }

    #[doc= "Get a reference to the value of field `max_bad_records` after provisioning.\nThe maximum number of bad records that BigQuery can ignore when reading data."]
    pub fn max_bad_records(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_bad_records", self.base))
    }

    #[doc= "Get a reference to the value of field `metadata_cache_mode` after provisioning.\nMetadata Cache Mode for the table. Set this to enable caching of metadata from external data source."]
    pub fn metadata_cache_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_cache_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `object_metadata` after provisioning.\nObject Metadata is used to create Object Tables. Object Tables contain a listing of objects (with their metadata) found at the sourceUris. If ObjectMetadata is set, sourceFormat should be omitted."]
    pub fn object_metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_metadata", self.base))
    }

    #[doc= "Get a reference to the value of field `reference_file_schema_uri` after provisioning.\nWhen creating an external table, the user can provide a reference file with the table schema. This is enabled for the following formats: AVRO, PARQUET, ORC."]
    pub fn reference_file_schema_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reference_file_schema_uri", self.base))
    }

    #[doc= "Get a reference to the value of field `schema` after provisioning.\nA JSON schema for the external table. Schema is required for CSV and JSON formats and is disallowed for Google Cloud Bigtable, Cloud Datastore backups, and Avro formats when using external tables."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc= "Get a reference to the value of field `source_format` after provisioning.\n Please see sourceFormat under ExternalDataConfiguration in Bigquery's public API documentation (https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#externaldataconfiguration) for supported formats. To use \"GOOGLE_SHEETS\" the scopes must include \"googleapis.com/auth/drive.readonly\"."]
    pub fn source_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_format", self.base))
    }

    #[doc= "Get a reference to the value of field `source_uris` after provisioning.\nA list of the fully-qualified URIs that point to your data in Google Cloud."]
    pub fn source_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.source_uris", self.base))
    }

    #[doc= "Get a reference to the value of field `avro_options` after provisioning.\n"]
    pub fn avro_options(&self) -> ListRef<BigqueryTableExternalDataConfigurationElAvroOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.avro_options", self.base))
    }

    #[doc= "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(&self) -> ListRef<BigqueryTableExternalDataConfigurationElCsvOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.csv_options", self.base))
    }

    #[doc= "Get a reference to the value of field `google_sheets_options` after provisioning.\n"]
    pub fn google_sheets_options(&self) -> ListRef<BigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.google_sheets_options", self.base))
    }

    #[doc= "Get a reference to the value of field `hive_partitioning_options` after provisioning.\n"]
    pub fn hive_partitioning_options(
        &self,
    ) -> ListRef<BigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hive_partitioning_options", self.base))
    }

    #[doc= "Get a reference to the value of field `json_options` after provisioning.\n"]
    pub fn json_options(&self) -> ListRef<BigqueryTableExternalDataConfigurationElJsonOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json_options", self.base))
    }

    #[doc= "Get a reference to the value of field `parquet_options` after provisioning.\n"]
    pub fn parquet_options(&self) -> ListRef<BigqueryTableExternalDataConfigurationElParquetOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.parquet_options", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableMaterializedViewEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_non_incremental_definition: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_refresh: Option<PrimField<bool>>,
    query: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_interval_ms: Option<PrimField<f64>>,
}

impl BigqueryTableMaterializedViewEl {
    #[doc= "Set the field `allow_non_incremental_definition`.\nAllow non incremental materialized view definition. The default value is false."]
    pub fn set_allow_non_incremental_definition(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_non_incremental_definition = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_refresh`.\nSpecifies if BigQuery should automatically refresh materialized view when the base table is updated. The default is true."]
    pub fn set_enable_refresh(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_refresh = Some(v.into());
        self
    }

    #[doc= "Set the field `refresh_interval_ms`.\nSpecifies maximum frequency at which this materialized view will be refreshed. The default is 1800000"]
    pub fn set_refresh_interval_ms(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.refresh_interval_ms = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryTableMaterializedViewEl {
    type O = BlockAssignable<BigqueryTableMaterializedViewEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableMaterializedViewEl {
    #[doc= "A query whose result is persisted."]
    pub query: PrimField<String>,
}

impl BuildBigqueryTableMaterializedViewEl {
    pub fn build(self) -> BigqueryTableMaterializedViewEl {
        BigqueryTableMaterializedViewEl {
            allow_non_incremental_definition: core::default::Default::default(),
            enable_refresh: core::default::Default::default(),
            query: self.query,
            refresh_interval_ms: core::default::Default::default(),
        }
    }
}

pub struct BigqueryTableMaterializedViewElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableMaterializedViewElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableMaterializedViewElRef {
        BigqueryTableMaterializedViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableMaterializedViewElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_non_incremental_definition` after provisioning.\nAllow non incremental materialized view definition. The default value is false."]
    pub fn allow_non_incremental_definition(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_non_incremental_definition", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_refresh` after provisioning.\nSpecifies if BigQuery should automatically refresh materialized view when the base table is updated. The default is true."]
    pub fn enable_refresh(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_refresh", self.base))
    }

    #[doc= "Get a reference to the value of field `query` after provisioning.\nA query whose result is persisted."]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc= "Get a reference to the value of field `refresh_interval_ms` after provisioning.\nSpecifies maximum frequency at which this materialized view will be refreshed. The default is 1800000"]
    pub fn refresh_interval_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.refresh_interval_ms", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableRangePartitioningElRangeEl {
    end: PrimField<f64>,
    interval: PrimField<f64>,
    start: PrimField<f64>,
}

impl BigqueryTableRangePartitioningElRangeEl { }

impl ToListMappable for BigqueryTableRangePartitioningElRangeEl {
    type O = BlockAssignable<BigqueryTableRangePartitioningElRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableRangePartitioningElRangeEl {
    #[doc= "End of the range partitioning, exclusive."]
    pub end: PrimField<f64>,
    #[doc= "The width of each range within the partition."]
    pub interval: PrimField<f64>,
    #[doc= "Start of the range partitioning, inclusive."]
    pub start: PrimField<f64>,
}

impl BuildBigqueryTableRangePartitioningElRangeEl {
    pub fn build(self) -> BigqueryTableRangePartitioningElRangeEl {
        BigqueryTableRangePartitioningElRangeEl {
            end: self.end,
            interval: self.interval,
            start: self.start,
        }
    }
}

pub struct BigqueryTableRangePartitioningElRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableRangePartitioningElRangeElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableRangePartitioningElRangeElRef {
        BigqueryTableRangePartitioningElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableRangePartitioningElRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `end` after provisioning.\nEnd of the range partitioning, exclusive."]
    pub fn end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nThe width of each range within the partition."]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc= "Get a reference to the value of field `start` after provisioning.\nStart of the range partitioning, inclusive."]
    pub fn start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTableRangePartitioningElDynamic {
    range: Option<DynamicBlock<BigqueryTableRangePartitioningElRangeEl>>,
}

#[derive(Serialize)]
pub struct BigqueryTableRangePartitioningEl {
    field: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<BigqueryTableRangePartitioningElRangeEl>>,
    dynamic: BigqueryTableRangePartitioningElDynamic,
}

impl BigqueryTableRangePartitioningEl {
    #[doc= "Set the field `range`.\n"]
    pub fn set_range(mut self, v: impl Into<BlockAssignable<BigqueryTableRangePartitioningElRangeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BigqueryTableRangePartitioningEl {
    type O = BlockAssignable<BigqueryTableRangePartitioningEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableRangePartitioningEl {
    #[doc= "The field used to determine how to create a range-based partition."]
    pub field: PrimField<String>,
}

impl BuildBigqueryTableRangePartitioningEl {
    pub fn build(self) -> BigqueryTableRangePartitioningEl {
        BigqueryTableRangePartitioningEl {
            field: self.field,
            range: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BigqueryTableRangePartitioningElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableRangePartitioningElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableRangePartitioningElRef {
        BigqueryTableRangePartitioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableRangePartitioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `field` after provisioning.\nThe field used to determine how to create a range-based partition."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc= "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> ListRef<BigqueryTableRangePartitioningElRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
    referenced_column: PrimField<String>,
    referencing_column: PrimField<String>,
}

impl BigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl { }

impl ToListMappable for BigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
    type O = BlockAssignable<BigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
    #[doc= "The column in the primary key that are referenced by the referencingColumn."]
    pub referenced_column: PrimField<String>,
    #[doc= "The column that composes the foreign key."]
    pub referencing_column: PrimField<String>,
}

impl BuildBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
    pub fn build(self) -> BigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
        BigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
            referenced_column: self.referenced_column,
            referencing_column: self.referencing_column,
        }
    }
}

pub struct BigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef {
        BigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `referenced_column` after provisioning.\nThe column in the primary key that are referenced by the referencingColumn."]
    pub fn referenced_column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.referenced_column", self.base))
    }

    #[doc= "Get a reference to the value of field `referencing_column` after provisioning.\nThe column that composes the foreign key."]
    pub fn referencing_column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.referencing_column", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
    dataset_id: PrimField<String>,
    project_id: PrimField<String>,
    table_id: PrimField<String>,
}

impl BigqueryTableTableConstraintsElForeignKeysElReferencedTableEl { }

impl ToListMappable for BigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
    type O = BlockAssignable<BigqueryTableTableConstraintsElForeignKeysElReferencedTableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
    #[doc= "The ID of the dataset containing this table."]
    pub dataset_id: PrimField<String>,
    #[doc= "The ID of the project containing this table."]
    pub project_id: PrimField<String>,
    #[doc= "The ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters. Certain operations allow suffixing of the table ID with a partition decorator, such as sample_table$20190123."]
    pub table_id: PrimField<String>,
}

impl BuildBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
    pub fn build(self) -> BigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
        BigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}

pub struct BigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef {
        BigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc= "Get a reference to the value of field `table_id` after provisioning.\nThe ID of the table. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters. Certain operations allow suffixing of the table ID with a partition decorator, such as sample_table$20190123."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTableTableConstraintsElForeignKeysElDynamic {
    column_references: Option<DynamicBlock<BigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl>>,
    referenced_table: Option<DynamicBlock<BigqueryTableTableConstraintsElForeignKeysElReferencedTableEl>>,
}

#[derive(Serialize)]
pub struct BigqueryTableTableConstraintsElForeignKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_references: Option<Vec<BigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referenced_table: Option<Vec<BigqueryTableTableConstraintsElForeignKeysElReferencedTableEl>>,
    dynamic: BigqueryTableTableConstraintsElForeignKeysElDynamic,
}

impl BigqueryTableTableConstraintsElForeignKeysEl {
    #[doc= "Set the field `name`.\nSet only if the foreign key constraint is named."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `column_references`.\n"]
    pub fn set_column_references(
        mut self,
        v: impl Into<BlockAssignable<BigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.column_references = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.column_references = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `referenced_table`.\n"]
    pub fn set_referenced_table(
        mut self,
        v: impl Into<BlockAssignable<BigqueryTableTableConstraintsElForeignKeysElReferencedTableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.referenced_table = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.referenced_table = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BigqueryTableTableConstraintsElForeignKeysEl {
    type O = BlockAssignable<BigqueryTableTableConstraintsElForeignKeysEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableTableConstraintsElForeignKeysEl {}

impl BuildBigqueryTableTableConstraintsElForeignKeysEl {
    pub fn build(self) -> BigqueryTableTableConstraintsElForeignKeysEl {
        BigqueryTableTableConstraintsElForeignKeysEl {
            name: core::default::Default::default(),
            column_references: core::default::Default::default(),
            referenced_table: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BigqueryTableTableConstraintsElForeignKeysElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableTableConstraintsElForeignKeysElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableTableConstraintsElForeignKeysElRef {
        BigqueryTableTableConstraintsElForeignKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableTableConstraintsElForeignKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nSet only if the foreign key constraint is named."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `column_references` after provisioning.\n"]
    pub fn column_references(&self) -> ListRef<BigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.column_references", self.base))
    }

    #[doc= "Get a reference to the value of field `referenced_table` after provisioning.\n"]
    pub fn referenced_table(&self) -> ListRef<BigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.referenced_table", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableTableConstraintsElPrimaryKeyEl {
    columns: ListField<PrimField<String>>,
}

impl BigqueryTableTableConstraintsElPrimaryKeyEl { }

impl ToListMappable for BigqueryTableTableConstraintsElPrimaryKeyEl {
    type O = BlockAssignable<BigqueryTableTableConstraintsElPrimaryKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableTableConstraintsElPrimaryKeyEl {
    #[doc= "The columns that are composed of the primary key constraint."]
    pub columns: ListField<PrimField<String>>,
}

impl BuildBigqueryTableTableConstraintsElPrimaryKeyEl {
    pub fn build(self) -> BigqueryTableTableConstraintsElPrimaryKeyEl {
        BigqueryTableTableConstraintsElPrimaryKeyEl { columns: self.columns }
    }
}

pub struct BigqueryTableTableConstraintsElPrimaryKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableTableConstraintsElPrimaryKeyElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableTableConstraintsElPrimaryKeyElRef {
        BigqueryTableTableConstraintsElPrimaryKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableTableConstraintsElPrimaryKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `columns` after provisioning.\nThe columns that are composed of the primary key constraint."]
    pub fn columns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTableTableConstraintsElDynamic {
    foreign_keys: Option<DynamicBlock<BigqueryTableTableConstraintsElForeignKeysEl>>,
    primary_key: Option<DynamicBlock<BigqueryTableTableConstraintsElPrimaryKeyEl>>,
}

#[derive(Serialize)]
pub struct BigqueryTableTableConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    foreign_keys: Option<Vec<BigqueryTableTableConstraintsElForeignKeysEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_key: Option<Vec<BigqueryTableTableConstraintsElPrimaryKeyEl>>,
    dynamic: BigqueryTableTableConstraintsElDynamic,
}

impl BigqueryTableTableConstraintsEl {
    #[doc= "Set the field `foreign_keys`.\n"]
    pub fn set_foreign_keys(
        mut self,
        v: impl Into<BlockAssignable<BigqueryTableTableConstraintsElForeignKeysEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.foreign_keys = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.foreign_keys = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `primary_key`.\n"]
    pub fn set_primary_key(
        mut self,
        v: impl Into<BlockAssignable<BigqueryTableTableConstraintsElPrimaryKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.primary_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.primary_key = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BigqueryTableTableConstraintsEl {
    type O = BlockAssignable<BigqueryTableTableConstraintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableTableConstraintsEl {}

impl BuildBigqueryTableTableConstraintsEl {
    pub fn build(self) -> BigqueryTableTableConstraintsEl {
        BigqueryTableTableConstraintsEl {
            foreign_keys: core::default::Default::default(),
            primary_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BigqueryTableTableConstraintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableTableConstraintsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableTableConstraintsElRef {
        BigqueryTableTableConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableTableConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `foreign_keys` after provisioning.\n"]
    pub fn foreign_keys(&self) -> ListRef<BigqueryTableTableConstraintsElForeignKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.foreign_keys", self.base))
    }

    #[doc= "Get a reference to the value of field `primary_key` after provisioning.\n"]
    pub fn primary_key(&self) -> ListRef<BigqueryTableTableConstraintsElPrimaryKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableTimePartitioningEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_ms: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_partition_filter: Option<PrimField<bool>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl BigqueryTableTimePartitioningEl {
    #[doc= "Set the field `expiration_ms`.\nNumber of milliseconds for which to keep the storage for a partition."]
    pub fn set_expiration_ms(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.expiration_ms = Some(v.into());
        self
    }

    #[doc= "Set the field `field`.\nThe field used to determine how to create a time-based partition. If time-based partitioning is enabled without this value, the table is partitioned based on the load time."]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }

    #[doc= "Set the field `require_partition_filter`.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn set_require_partition_filter(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_partition_filter = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryTableTimePartitioningEl {
    type O = BlockAssignable<BigqueryTableTimePartitioningEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableTimePartitioningEl {
    #[doc= "The supported types are DAY, HOUR, MONTH, and YEAR, which will generate one partition per day, hour, month, and year, respectively."]
    pub type_: PrimField<String>,
}

impl BuildBigqueryTableTimePartitioningEl {
    pub fn build(self) -> BigqueryTableTimePartitioningEl {
        BigqueryTableTimePartitioningEl {
            expiration_ms: core::default::Default::default(),
            field: core::default::Default::default(),
            require_partition_filter: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct BigqueryTableTimePartitioningElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableTimePartitioningElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableTimePartitioningElRef {
        BigqueryTableTimePartitioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableTimePartitioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expiration_ms` after provisioning.\nNumber of milliseconds for which to keep the storage for a partition."]
    pub fn expiration_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expiration_ms", self.base))
    }

    #[doc= "Get a reference to the value of field `field` after provisioning.\nThe field used to determine how to create a time-based partition. If time-based partitioning is enabled without this value, the table is partitioned based on the load time."]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc= "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn require_partition_filter(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe supported types are DAY, HOUR, MONTH, and YEAR, which will generate one partition per day, hour, month, and year, respectively."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryTableViewEl {
    query: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_legacy_sql: Option<PrimField<bool>>,
}

impl BigqueryTableViewEl {
    #[doc= "Set the field `use_legacy_sql`.\nSpecifies whether to use BigQuery's legacy SQL for this view. The default value is true. If set to false, the view will use BigQuery's standard SQL"]
    pub fn set_use_legacy_sql(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_legacy_sql = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryTableViewEl {
    type O = BlockAssignable<BigqueryTableViewEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryTableViewEl {
    #[doc= "A query that BigQuery executes when the view is referenced."]
    pub query: PrimField<String>,
}

impl BuildBigqueryTableViewEl {
    pub fn build(self) -> BigqueryTableViewEl {
        BigqueryTableViewEl {
            query: self.query,
            use_legacy_sql: core::default::Default::default(),
        }
    }
}

pub struct BigqueryTableViewElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryTableViewElRef {
    fn new(shared: StackShared, base: String) -> BigqueryTableViewElRef {
        BigqueryTableViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryTableViewElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `query` after provisioning.\nA query that BigQuery executes when the view is referenced."]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }

    #[doc= "Get a reference to the value of field `use_legacy_sql` after provisioning.\nSpecifies whether to use BigQuery's legacy SQL for this view. The default value is true. If set to false, the view will use BigQuery's standard SQL"]
    pub fn use_legacy_sql(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_legacy_sql", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryTableDynamic {
    encryption_configuration: Option<DynamicBlock<BigqueryTableEncryptionConfigurationEl>>,
    external_data_configuration: Option<DynamicBlock<BigqueryTableExternalDataConfigurationEl>>,
    materialized_view: Option<DynamicBlock<BigqueryTableMaterializedViewEl>>,
    range_partitioning: Option<DynamicBlock<BigqueryTableRangePartitioningEl>>,
    table_constraints: Option<DynamicBlock<BigqueryTableTableConstraintsEl>>,
    time_partitioning: Option<DynamicBlock<BigqueryTableTimePartitioningEl>>,
    view: Option<DynamicBlock<BigqueryTableViewEl>>,
}
