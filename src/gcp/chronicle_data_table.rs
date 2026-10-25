#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleDataTableData_ {
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
    data_table_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_time_to_live: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_info: std::option::Option<std::vec::Vec<ChronicleDataTable_ColumnInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope_info: std::option::Option<std::vec::Vec<ChronicleDataTable_ScopeInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleDataTable_Timeouts>,
    dynamic: ChronicleDataTableDynamic,
}

struct ChronicleDataTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleDataTableData_>,
}

#[derive(Clone)]
pub struct ChronicleDataTable(std::rc::Rc<ChronicleDataTable_>);

impl ChronicleDataTable {
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
        "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_data_table.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `row_time_to_live`.\nUser-provided TTL of the data table."]
    pub fn set_row_time_to_live(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().row_time_to_live = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_info`.\n"]
    pub fn set_column_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDataTable_ColumnInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().column_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.column_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope_info`.\n"]
    pub fn set_scope_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleDataTable_ScopeInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleDataTable_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `approximate_row_count` after provisioning.\nThe count of rows in the data table."]
    pub fn approximate_row_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approximate_row_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTable create time"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_table_id` after provisioning.\nThe ID to use for the data table. This is also the display name for\nthe data table. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Must be unique and has length < 256."]
    pub fn data_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_table_uuid` after provisioning.\nData table unique id"]
    pub fn data_table_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_table_uuid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_data_table.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-provided description of the data table."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe unique display name of the data table."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the data table\nFormat:\n\"{project}/locations/{region}/instances/{instance}/dataTables/{data_table}\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `row_time_to_live` after provisioning.\nUser-provided TTL of the data table."]
    pub fn row_time_to_live(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_time_to_live", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `row_time_to_live_update_time` after provisioning.\nLast update time of the TTL of the data table."]
    pub fn row_time_to_live_update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_time_to_live_update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_associations_count` after provisioning.\nThe count of rules using the data table."]
    pub fn rule_associations_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_associations_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nThe resource names for the associated Rules that use this\ndata table. Format:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}.\n{rule} here refers to the rule id."]
    pub fn rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_source` after provisioning.\nPossible values:\nUSER\nRULE\nSEARCH"]
    pub fn update_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTable update time"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `column_info` after provisioning.\n"]
    pub fn column_info(&self) -> terrars::ListRef<ChronicleDataTable_ColumnInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_info` after provisioning.\n"]
    pub fn scope_info(&self) -> terrars::ListRef<ChronicleDataTable_ScopeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDataTable_TimeoutsRef {
        ChronicleDataTable_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleDataTable {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleDataTable { }

impl terrars::ToListMappable for ChronicleDataTable {
    type O = terrars::ListRef<ChronicleDataTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleDataTable_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_data_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleDataTable {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the data table. This is also the display name for\nthe data table. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Must be unique and has length < 256."]
    pub data_table_id: terrars::PrimField<std::string::String>,
    #[doc = "A user-provided description of the data table."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDataTable {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleDataTable {
        let out = ChronicleDataTable(std::rc::Rc::new(ChronicleDataTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleDataTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_table_id: self.data_table_id,
                deletion_policy: core::default::Default::default(),
                description: self.description,
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                row_time_to_live: core::default::Default::default(),
                column_info: core::default::Default::default(),
                scope_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleDataTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDataTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleDataTableRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `approximate_row_count` after provisioning.\nThe count of rows in the data table."]
    pub fn approximate_row_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approximate_row_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTable create time"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_table_id` after provisioning.\nThe ID to use for the data table. This is also the display name for\nthe data table. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Must be unique and has length < 256."]
    pub fn data_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_table_uuid` after provisioning.\nData table unique id"]
    pub fn data_table_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_table_uuid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_data_table.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-provided description of the data table."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe unique display name of the data table."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the data table\nFormat:\n\"{project}/locations/{region}/instances/{instance}/dataTables/{data_table}\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `row_time_to_live` after provisioning.\nUser-provided TTL of the data table."]
    pub fn row_time_to_live(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_time_to_live", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `row_time_to_live_update_time` after provisioning.\nLast update time of the TTL of the data table."]
    pub fn row_time_to_live_update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_time_to_live_update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_associations_count` after provisioning.\nThe count of rules using the data table."]
    pub fn rule_associations_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_associations_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nThe resource names for the associated Rules that use this\ndata table. Format:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}.\n{rule} here refers to the rule id."]
    pub fn rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_source` after provisioning.\nPossible values:\nUSER\nRULE\nSEARCH"]
    pub fn update_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTable update time"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `column_info` after provisioning.\n"]
    pub fn column_info(&self) -> terrars::ListRef<ChronicleDataTable_ColumnInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_info` after provisioning.\n"]
    pub fn scope_info(&self) -> terrars::ListRef<ChronicleDataTable_ScopeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDataTable_TimeoutsRef {
        ChronicleDataTable_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleDataTableDynamic {
    column_info: std::option::Option<terrars::DynamicBlock<ChronicleDataTable_ColumnInfo>>,
    scope_info: std::option::Option<terrars::DynamicBlock<ChronicleDataTable_ScopeInfo>>,
}

#[derive(Serialize)]
pub struct ChronicleDataTable_ColumnInfo {
    column_index: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_column: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapped_column_path: std::option::Option<terrars::PrimField<std::string::String>>,
    original_column: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repeated_values: std::option::Option<terrars::PrimField<bool>>,
}

impl ChronicleDataTable_ColumnInfo {
    #[doc =
        "Set the field `column_type`.\nColumn type can be STRING, CIDR (Ex- 10.1.1.0/24), REGEX\nPossible values:\nSTRING\nREGEX\nCIDR\nNUMBER Possible values: [\"STRING\", \"REGEX\", \"CIDR\", \"NUMBER\"]"]
    pub fn set_column_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_column`.\nWhether to include this column in the calculation of the row ID.\nIf no columns have key_column = true, all columns will be included in the\ncalculation of the row ID."]
    pub fn set_key_column(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.key_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mapped_column_path`.\nEntity proto field path that the column is mapped to"]
    pub fn set_mapped_column_path(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.mapped_column_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repeated_values`.\nWhether the column is a repeated values column."]
    pub fn set_repeated_values(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.repeated_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleDataTable_ColumnInfo {
    type O = terrars::BlockAssignable<ChronicleDataTable_ColumnInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDataTable_ColumnInfo {
    #[doc = "Column Index. 0,1,2..."]
    pub column_index: terrars::PrimField<f64>,
    #[doc =
        "Original column name of the Data Table (present in the CSV header in case\nof creation of data tables using file uploads). It must satisfy the\nfollowing requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Must be unique and has length < 256"]
    pub original_column: terrars::PrimField<std::string::String>,
}

impl BuildChronicleDataTable_ColumnInfo {
    pub fn build(self) -> ChronicleDataTable_ColumnInfo {
        ChronicleDataTable_ColumnInfo {
            column_index: self.column_index,
            column_type: core::default::Default::default(),
            key_column: core::default::Default::default(),
            mapped_column_path: core::default::Default::default(),
            original_column: self.original_column,
            repeated_values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDataTable_ColumnInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDataTable_ColumnInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleDataTable_ColumnInfoRef {
        ChronicleDataTable_ColumnInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDataTable_ColumnInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_index` after provisioning.\nColumn Index. 0,1,2..."]
    pub fn column_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_index", self.base))
    }

    #[doc =
        "Get a reference to the value of field `column_type` after provisioning.\nColumn type can be STRING, CIDR (Ex- 10.1.1.0/24), REGEX\nPossible values:\nSTRING\nREGEX\nCIDR\nNUMBER Possible values: [\"STRING\", \"REGEX\", \"CIDR\", \"NUMBER\"]"]
    pub fn column_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_column` after provisioning.\nWhether to include this column in the calculation of the row ID.\nIf no columns have key_column = true, all columns will be included in the\ncalculation of the row ID."]
    pub fn key_column(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mapped_column_path` after provisioning.\nEntity proto field path that the column is mapped to"]
    pub fn mapped_column_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mapped_column_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `original_column` after provisioning.\nOriginal column name of the Data Table (present in the CSV header in case\nof creation of data tables using file uploads). It must satisfy the\nfollowing requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Must be unique and has length < 256"]
    pub fn original_column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.original_column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repeated_values` after provisioning.\nWhether the column is a repeated values column."]
    pub fn repeated_values(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repeated_values", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDataTable_ScopeInfo {
    data_access_scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ChronicleDataTable_ScopeInfo { }

impl terrars::ToListMappable for ChronicleDataTable_ScopeInfo {
    type O = terrars::BlockAssignable<ChronicleDataTable_ScopeInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDataTable_ScopeInfo {
    #[doc =
        "Contains the list of scope names of the data table. If the list is empty,\nthe data table is treated as unscoped. The scope names should be\nfull resource names and should be of the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope_name}\""]
    pub data_access_scopes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildChronicleDataTable_ScopeInfo {
    pub fn build(self) -> ChronicleDataTable_ScopeInfo {
        ChronicleDataTable_ScopeInfo { data_access_scopes: self.data_access_scopes }
    }
}

#[allow(dead_code)]
pub struct ChronicleDataTable_ScopeInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDataTable_ScopeInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleDataTable_ScopeInfoRef {
        ChronicleDataTable_ScopeInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDataTable_ScopeInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_access_scopes` after provisioning.\nContains the list of scope names of the data table. If the list is empty,\nthe data table is treated as unscoped. The scope names should be\nfull resource names and should be of the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope_name}\""]
    pub fn data_access_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_access_scopes", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleDataTable_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleDataTable_Timeouts {
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

impl terrars::ToListMappable for ChronicleDataTable_Timeouts {
    type O = terrars::BlockAssignable<ChronicleDataTable_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleDataTable_Timeouts {}

impl BuildChronicleDataTable_Timeouts {
    pub fn build(self) -> ChronicleDataTable_Timeouts {
        ChronicleDataTable_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleDataTable_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleDataTable_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleDataTable_TimeoutsRef {
        ChronicleDataTable_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleDataTable_TimeoutsRef {
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
