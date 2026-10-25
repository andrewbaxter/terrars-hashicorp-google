#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCatalogEntryData_ {
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
    entry_group: terrars::PrimField<std::string::String>,
    entry_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_specified_system: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_specified_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_fileset_spec: std::option::Option<std::vec::Vec<DataCatalogEntry_GcsFilesetSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataCatalogEntry_Timeouts>,
    dynamic: DataCatalogEntryDynamic,
}

struct DataCatalogEntry_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCatalogEntryData_>,
}

#[derive(Clone)]
pub struct DataCatalogEntry(std::rc::Rc<DataCatalogEntry_>);

impl DataCatalogEntry {
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
        "Set the field `description`.\nEntry description, which can consist of several sentences or paragraphs that describe entry contents."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nDisplay information such as title and description. A short name to identify the entry,\nfor example, \"Analytics Data - Jan 2011\"."]
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
        "Set the field `linked_resource`.\nThe resource this metadata entry refers to.\nFor Google Cloud Platform resources, linkedResource is the full name of the resource.\nFor example, the linkedResource for a table resource from BigQuery is:\n//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId\nOutput only when Entry is of type in the EntryType enum. For entries with userSpecifiedType,\nthis field is optional and defaults to an empty string."]
    pub fn set_linked_resource(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().linked_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schema`.\nSchema of the entry (e.g. BigQuery, GoogleSQL, Avro schema), as a json string. An entry might not have any schema\nattached to it. See\nhttps://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries#schema\nfor what fields this schema can contain."]
    pub fn set_schema(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type of the entry. Only used for Entries with types in the EntryType enum.\nCurrently, only FILESET enum value is allowed. All other entries created through Data Catalog must use userSpecifiedType. Possible values: [\"FILESET\"]"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_specified_system`.\nThis field indicates the entry's source system that Data Catalog does not integrate with.\nuserSpecifiedSystem strings must begin with a letter or underscore and can only contain letters, numbers,\nand underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn set_user_specified_system(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_specified_system = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_specified_type`.\nEntry type if it does not fit any of the input-allowed values listed in EntryType enum above.\nWhen creating an entry, users should check the enum values first, if nothing matches the entry\nto be created, then provide a custom value, for example \"my_special_type\".\nuserSpecifiedType strings must begin with a letter or underscore and can only contain letters,\nnumbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn set_user_specified_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_specified_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_fileset_spec`.\n"]
    pub fn set_gcs_fileset_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCatalogEntry_GcsFilesetSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gcs_fileset_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gcs_fileset_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataCatalogEntry_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `bigquery_date_sharded_spec` after provisioning.\nSpecification for a group of BigQuery tables with name pattern [prefix]YYYYMMDD.\nContext: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding."]
    pub fn bigquery_date_sharded_spec(&self) -> terrars::ListRef<DataCatalogEntry_BigqueryDateShardedSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_date_sharded_spec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bigquery_table_spec` after provisioning.\nSpecification that applies to a BigQuery table. This is only valid on entries of type TABLE."]
    pub fn bigquery_table_spec(&self) -> terrars::ListRef<DataCatalogEntry_BigqueryTableSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_table_spec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nEntry description, which can consist of several sentences or paragraphs that describe entry contents."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay information such as title and description. A short name to identify the entry,\nfor example, \"Analytics Data - Jan 2011\"."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_group` after provisioning.\nThe name of the entry group this entry is in."]
    pub fn entry_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\nThe id of the entry to create."]
    pub fn entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integrated_system` after provisioning.\nThis field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub."]
    pub fn integrated_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integrated_system", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `linked_resource` after provisioning.\nThe resource this metadata entry refers to.\nFor Google Cloud Platform resources, linkedResource is the full name of the resource.\nFor example, the linkedResource for a table resource from BigQuery is:\n//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId\nOutput only when Entry is of type in the EntryType enum. For entries with userSpecifiedType,\nthis field is optional and defaults to an empty string."]
    pub fn linked_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.linked_resource", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe Data Catalog resource name of the entry in URL format.\nExample: projects/{project_id}/locations/{location}/entryGroups/{entryGroupId}/entries/{entryId}.\nNote that this Entry and its child resources may not actually be stored in the location in this name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nSchema of the entry (e.g. BigQuery, GoogleSQL, Avro schema), as a json string. An entry might not have any schema\nattached to it. See\nhttps://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries#schema\nfor what fields this schema can contain."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the entry. Only used for Entries with types in the EntryType enum.\nCurrently, only FILESET enum value is allowed. All other entries created through Data Catalog must use userSpecifiedType. Possible values: [\"FILESET\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_specified_system` after provisioning.\nThis field indicates the entry's source system that Data Catalog does not integrate with.\nuserSpecifiedSystem strings must begin with a letter or underscore and can only contain letters, numbers,\nand underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn user_specified_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_specified_system", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_specified_type` after provisioning.\nEntry type if it does not fit any of the input-allowed values listed in EntryType enum above.\nWhen creating an entry, users should check the enum values first, if nothing matches the entry\nto be created, then provide a custom value, for example \"my_special_type\".\nuserSpecifiedType strings must begin with a letter or underscore and can only contain letters,\nnumbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn user_specified_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_specified_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcs_fileset_spec` after provisioning.\n"]
    pub fn gcs_fileset_spec(&self) -> terrars::ListRef<DataCatalogEntry_GcsFilesetSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_fileset_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataCatalogEntry_TimeoutsRef {
        DataCatalogEntry_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataCatalogEntry {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataCatalogEntry { }

impl terrars::ToListMappable for DataCatalogEntry {
    type O = terrars::ListRef<DataCatalogEntryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataCatalogEntry_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_data_catalog_entry".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCatalogEntry {
    pub tf_id: std::string::String,
    #[doc = "The name of the entry group this entry is in."]
    pub entry_group: terrars::PrimField<std::string::String>,
    #[doc = "The id of the entry to create."]
    pub entry_id: terrars::PrimField<std::string::String>,
}

impl BuildDataCatalogEntry {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCatalogEntry {
        let out = DataCatalogEntry(std::rc::Rc::new(DataCatalogEntry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCatalogEntryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                entry_group: self.entry_group,
                entry_id: self.entry_id,
                id: core::default::Default::default(),
                linked_resource: core::default::Default::default(),
                schema: core::default::Default::default(),
                type_: core::default::Default::default(),
                user_specified_system: core::default::Default::default(),
                user_specified_type: core::default::Default::default(),
                gcs_fileset_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataCatalogEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogEntryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCatalogEntryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bigquery_date_sharded_spec` after provisioning.\nSpecification for a group of BigQuery tables with name pattern [prefix]YYYYMMDD.\nContext: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding."]
    pub fn bigquery_date_sharded_spec(&self) -> terrars::ListRef<DataCatalogEntry_BigqueryDateShardedSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_date_sharded_spec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bigquery_table_spec` after provisioning.\nSpecification that applies to a BigQuery table. This is only valid on entries of type TABLE."]
    pub fn bigquery_table_spec(&self) -> terrars::ListRef<DataCatalogEntry_BigqueryTableSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_table_spec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nEntry description, which can consist of several sentences or paragraphs that describe entry contents."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nDisplay information such as title and description. A short name to identify the entry,\nfor example, \"Analytics Data - Jan 2011\"."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_group` after provisioning.\nThe name of the entry group this entry is in."]
    pub fn entry_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\nThe id of the entry to create."]
    pub fn entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `integrated_system` after provisioning.\nThis field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub."]
    pub fn integrated_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integrated_system", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `linked_resource` after provisioning.\nThe resource this metadata entry refers to.\nFor Google Cloud Platform resources, linkedResource is the full name of the resource.\nFor example, the linkedResource for a table resource from BigQuery is:\n//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId\nOutput only when Entry is of type in the EntryType enum. For entries with userSpecifiedType,\nthis field is optional and defaults to an empty string."]
    pub fn linked_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.linked_resource", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe Data Catalog resource name of the entry in URL format.\nExample: projects/{project_id}/locations/{location}/entryGroups/{entryGroupId}/entries/{entryId}.\nNote that this Entry and its child resources may not actually be stored in the location in this name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nSchema of the entry (e.g. BigQuery, GoogleSQL, Avro schema), as a json string. An entry might not have any schema\nattached to it. See\nhttps://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries#schema\nfor what fields this schema can contain."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the entry. Only used for Entries with types in the EntryType enum.\nCurrently, only FILESET enum value is allowed. All other entries created through Data Catalog must use userSpecifiedType. Possible values: [\"FILESET\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_specified_system` after provisioning.\nThis field indicates the entry's source system that Data Catalog does not integrate with.\nuserSpecifiedSystem strings must begin with a letter or underscore and can only contain letters, numbers,\nand underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn user_specified_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_specified_system", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_specified_type` after provisioning.\nEntry type if it does not fit any of the input-allowed values listed in EntryType enum above.\nWhen creating an entry, users should check the enum values first, if nothing matches the entry\nto be created, then provide a custom value, for example \"my_special_type\".\nuserSpecifiedType strings must begin with a letter or underscore and can only contain letters,\nnumbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn user_specified_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_specified_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcs_fileset_spec` after provisioning.\n"]
    pub fn gcs_fileset_spec(&self) -> terrars::ListRef<DataCatalogEntry_GcsFilesetSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_fileset_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataCatalogEntry_TimeoutsRef {
        DataCatalogEntry_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataCatalogEntryDynamic {
    gcs_fileset_spec: std::option::Option<terrars::DynamicBlock<DataCatalogEntry_GcsFilesetSpec>>,
}

#[derive(Serialize)]
pub struct DataCatalogEntry_BigqueryDateShardedSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shard_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCatalogEntry_BigqueryDateShardedSpec {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shard_count`.\n"]
    pub fn set_shard_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shard_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_prefix`.\n"]
    pub fn set_table_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCatalogEntry_BigqueryDateShardedSpec {
    type O = terrars::BlockAssignable<DataCatalogEntry_BigqueryDateShardedSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogEntry_BigqueryDateShardedSpec {}

impl BuildDataCatalogEntry_BigqueryDateShardedSpec {
    pub fn build(self) -> DataCatalogEntry_BigqueryDateShardedSpec {
        DataCatalogEntry_BigqueryDateShardedSpec {
            dataset: core::default::Default::default(),
            shard_count: core::default::Default::default(),
            table_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCatalogEntry_BigqueryDateShardedSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogEntry_BigqueryDateShardedSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCatalogEntry_BigqueryDateShardedSpecRef {
        DataCatalogEntry_BigqueryDateShardedSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogEntry_BigqueryDateShardedSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `shard_count` after provisioning.\n"]
    pub fn shard_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shard_count", self.base))
    }

    #[doc = "Get a reference to the value of field `table_prefix` after provisioning.\n"]
    pub fn table_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCatalogEntry_BigqueryTableSpec_TableSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grouped_entry: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCatalogEntry_BigqueryTableSpec_TableSpec {
    #[doc = "Set the field `grouped_entry`.\n"]
    pub fn set_grouped_entry(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grouped_entry = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCatalogEntry_BigqueryTableSpec_TableSpec {
    type O = terrars::BlockAssignable<DataCatalogEntry_BigqueryTableSpec_TableSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogEntry_BigqueryTableSpec_TableSpec {}

impl BuildDataCatalogEntry_BigqueryTableSpec_TableSpec {
    pub fn build(self) -> DataCatalogEntry_BigqueryTableSpec_TableSpec {
        DataCatalogEntry_BigqueryTableSpec_TableSpec { grouped_entry: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCatalogEntry_BigqueryTableSpec_TableSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogEntry_BigqueryTableSpec_TableSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCatalogEntry_BigqueryTableSpec_TableSpecRef {
        DataCatalogEntry_BigqueryTableSpec_TableSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogEntry_BigqueryTableSpec_TableSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grouped_entry` after provisioning.\n"]
    pub fn grouped_entry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grouped_entry", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCatalogEntry_BigqueryTableSpec_ViewSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_query: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCatalogEntry_BigqueryTableSpec_ViewSpec {
    #[doc = "Set the field `view_query`.\n"]
    pub fn set_view_query(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.view_query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCatalogEntry_BigqueryTableSpec_ViewSpec {
    type O = terrars::BlockAssignable<DataCatalogEntry_BigqueryTableSpec_ViewSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogEntry_BigqueryTableSpec_ViewSpec {}

impl BuildDataCatalogEntry_BigqueryTableSpec_ViewSpec {
    pub fn build(self) -> DataCatalogEntry_BigqueryTableSpec_ViewSpec {
        DataCatalogEntry_BigqueryTableSpec_ViewSpec { view_query: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataCatalogEntry_BigqueryTableSpec_ViewSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogEntry_BigqueryTableSpec_ViewSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCatalogEntry_BigqueryTableSpec_ViewSpecRef {
        DataCatalogEntry_BigqueryTableSpec_ViewSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogEntry_BigqueryTableSpec_ViewSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `view_query` after provisioning.\n"]
    pub fn view_query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_query", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCatalogEntry_BigqueryTableSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_source_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_spec: std::option::Option<terrars::ListField<DataCatalogEntry_BigqueryTableSpec_TableSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_spec: std::option::Option<terrars::ListField<DataCatalogEntry_BigqueryTableSpec_ViewSpec>>,
}

impl DataCatalogEntry_BigqueryTableSpec {
    #[doc = "Set the field `table_source_type`.\n"]
    pub fn set_table_source_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_source_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_spec`.\n"]
    pub fn set_table_spec(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCatalogEntry_BigqueryTableSpec_TableSpec>>,
    ) -> Self {
        self.table_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_spec`.\n"]
    pub fn set_view_spec(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCatalogEntry_BigqueryTableSpec_ViewSpec>>,
    ) -> Self {
        self.view_spec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCatalogEntry_BigqueryTableSpec {
    type O = terrars::BlockAssignable<DataCatalogEntry_BigqueryTableSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogEntry_BigqueryTableSpec {}

impl BuildDataCatalogEntry_BigqueryTableSpec {
    pub fn build(self) -> DataCatalogEntry_BigqueryTableSpec {
        DataCatalogEntry_BigqueryTableSpec {
            table_source_type: core::default::Default::default(),
            table_spec: core::default::Default::default(),
            view_spec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCatalogEntry_BigqueryTableSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogEntry_BigqueryTableSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCatalogEntry_BigqueryTableSpecRef {
        DataCatalogEntry_BigqueryTableSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogEntry_BigqueryTableSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_source_type` after provisioning.\n"]
    pub fn table_source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `table_spec` after provisioning.\n"]
    pub fn table_spec(&self) -> terrars::ListRef<DataCatalogEntry_BigqueryTableSpec_TableSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `view_spec` after provisioning.\n"]
    pub fn view_spec(&self) -> terrars::ListRef<DataCatalogEntry_BigqueryTableSpec_ViewSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.view_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_bytes: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecs {
    #[doc = "Set the field `file_path`.\n"]
    pub fn set_file_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_bytes`.\n"]
    pub fn set_size_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_bytes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecs {
    type O = terrars::BlockAssignable<DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecs {}

impl BuildDataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecs {
    pub fn build(self) -> DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecs {
        DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecs {
            file_path: core::default::Default::default(),
            size_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecsRef {
        DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_path` after provisioning.\n"]
    pub fn file_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_path", self.base))
    }

    #[doc = "Get a reference to the value of field `size_bytes` after provisioning.\n"]
    pub fn size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_bytes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCatalogEntry_GcsFilesetSpec {
    file_patterns: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataCatalogEntry_GcsFilesetSpec { }

impl terrars::ToListMappable for DataCatalogEntry_GcsFilesetSpec {
    type O = terrars::BlockAssignable<DataCatalogEntry_GcsFilesetSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogEntry_GcsFilesetSpec {
    #[doc =
        "Patterns to identify a set of files in Google Cloud Storage.\nSee [Cloud Storage documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames)\nfor more information. Note that bucket wildcards are currently not supported. Examples of valid filePatterns:\n\n* gs://bucket_name/dir/*: matches all files within bucket_name/dir directory.\n* gs://bucket_name/dir/**: matches all files in bucket_name/dir spanning all subdirectories.\n* gs://bucket_name/file*: matches files prefixed by file in bucket_name\n* gs://bucket_name/??.txt: matches files with two characters followed by .txt in bucket_name\n* gs://bucket_name/[aeiou].txt: matches files that contain a single vowel character followed by .txt in bucket_name\n* gs://bucket_name/[a-m].txt: matches files that contain a, b, ... or m followed by .txt in bucket_name\n* gs://bucket_name/a/*/b: matches all files in bucket_name that match a/*/b pattern, such as a/c/b, a/d/b\n* gs://another_bucket/a.txt: matches gs://another_bucket/a.txt"]
    pub file_patterns: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataCatalogEntry_GcsFilesetSpec {
    pub fn build(self) -> DataCatalogEntry_GcsFilesetSpec {
        DataCatalogEntry_GcsFilesetSpec { file_patterns: self.file_patterns }
    }
}

#[allow(dead_code)]
pub struct DataCatalogEntry_GcsFilesetSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogEntry_GcsFilesetSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCatalogEntry_GcsFilesetSpecRef {
        DataCatalogEntry_GcsFilesetSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogEntry_GcsFilesetSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_patterns` after provisioning.\nPatterns to identify a set of files in Google Cloud Storage.\nSee [Cloud Storage documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames)\nfor more information. Note that bucket wildcards are currently not supported. Examples of valid filePatterns:\n\n* gs://bucket_name/dir/*: matches all files within bucket_name/dir directory.\n* gs://bucket_name/dir/**: matches all files in bucket_name/dir spanning all subdirectories.\n* gs://bucket_name/file*: matches files prefixed by file in bucket_name\n* gs://bucket_name/??.txt: matches files with two characters followed by .txt in bucket_name\n* gs://bucket_name/[aeiou].txt: matches files that contain a single vowel character followed by .txt in bucket_name\n* gs://bucket_name/[a-m].txt: matches files that contain a, b, ... or m followed by .txt in bucket_name\n* gs://bucket_name/a/*/b: matches all files in bucket_name that match a/*/b pattern, such as a/c/b, a/d/b\n* gs://another_bucket/a.txt: matches gs://another_bucket/a.txt"]
    pub fn file_patterns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_patterns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sample_gcs_file_specs` after provisioning.\nSample files contained in this fileset, not all files contained in this fileset are represented here."]
    pub fn sample_gcs_file_specs(&self) -> terrars::ListRef<DataCatalogEntry_GcsFilesetSpec_SampleGcsFileSpecsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_gcs_file_specs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCatalogEntry_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCatalogEntry_Timeouts {
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

impl terrars::ToListMappable for DataCatalogEntry_Timeouts {
    type O = terrars::BlockAssignable<DataCatalogEntry_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogEntry_Timeouts {}

impl BuildDataCatalogEntry_Timeouts {
    pub fn build(self) -> DataCatalogEntry_Timeouts {
        DataCatalogEntry_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCatalogEntry_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogEntry_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCatalogEntry_TimeoutsRef {
        DataCatalogEntry_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogEntry_TimeoutsRef {
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
