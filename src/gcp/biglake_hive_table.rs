#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BiglakeHiveTableData_ {
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
    catalog: terrars::PrimField<std::string::String>,
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_expanded_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    view_original_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_keys: std::option::Option<std::vec::Vec<BiglakeHiveTable_PartitionKeys>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_descriptor: std::option::Option<std::vec::Vec<BiglakeHiveTable_StorageDescriptor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BiglakeHiveTable_Timeouts>,
    dynamic: BiglakeHiveTableDynamic,
}

struct BiglakeHiveTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BiglakeHiveTableData_>,
}

#[derive(Clone)]
pub struct BiglakeHiveTable(std::rc::Rc<BiglakeHiveTable_>);

impl BiglakeHiveTable {
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

    #[doc = "Set the field `description`.\nDescription of the table."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\nAdditional parameters associated with the table."]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_expanded_text`.\nExpanded view text for Hive views. Empty for non-view."]
    pub fn set_view_expanded_text(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().view_expanded_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `view_original_text`.\nOriginal view text for Hive views. Empty for non-view."]
    pub fn set_view_original_text(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().view_original_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partition_keys`.\n"]
    pub fn set_partition_keys(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeHiveTable_PartitionKeys>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partition_keys = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partition_keys = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_descriptor`.\n"]
    pub fn set_storage_descriptor(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_descriptor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_descriptor = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BiglakeHiveTable_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `catalog` after provisioning.\nThe Hive catalog where the table is located."]
    pub fn catalog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time of the table."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe Hive database where the table is located."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the table."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_access_time` after provisioning.\nOutput only. Last access time of the table."]
    pub fn last_access_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_access_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the table."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nAdditional parameters associated with the table."]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_type` after provisioning.\nOutput only. The type of the table."]
    pub fn table_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The update time of the table"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `view_expanded_text` after provisioning.\nExpanded view text for Hive views. Empty for non-view."]
    pub fn view_expanded_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_expanded_text", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `view_original_text` after provisioning.\nOriginal view text for Hive views. Empty for non-view."]
    pub fn view_original_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_original_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> terrars::ListRef<BiglakeHiveTable_PartitionKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<BiglakeHiveTable_StorageDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeHiveTable_TimeoutsRef {
        BiglakeHiveTable_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BiglakeHiveTable {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BiglakeHiveTable { }

impl terrars::ToListMappable for BiglakeHiveTable {
    type O = terrars::ListRef<BiglakeHiveTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BiglakeHiveTable_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_biglake_hive_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBiglakeHiveTable {
    pub tf_id: std::string::String,
    #[doc = "The Hive catalog where the table is located."]
    pub catalog: terrars::PrimField<std::string::String>,
    #[doc = "The Hive database where the table is located."]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = "The name of the table."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeHiveTable {
    pub fn build(self, stack: &mut terrars::Stack) -> BiglakeHiveTable {
        let out = BiglakeHiveTable(std::rc::Rc::new(BiglakeHiveTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BiglakeHiveTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                catalog: self.catalog,
                database: self.database,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parameters: core::default::Default::default(),
                project: core::default::Default::default(),
                view_expanded_text: core::default::Default::default(),
                view_original_text: core::default::Default::default(),
                partition_keys: core::default::Default::default(),
                storage_descriptor: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BiglakeHiveTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeHiveTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BiglakeHiveTableRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `catalog` after provisioning.\nThe Hive catalog where the table is located."]
    pub fn catalog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time of the table."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe Hive database where the table is located."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the table."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_access_time` after provisioning.\nOutput only. Last access time of the table."]
    pub fn last_access_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_access_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the table."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nAdditional parameters associated with the table."]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `table_type` after provisioning.\nOutput only. The type of the table."]
    pub fn table_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The update time of the table"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `view_expanded_text` after provisioning.\nExpanded view text for Hive views. Empty for non-view."]
    pub fn view_expanded_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_expanded_text", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `view_original_text` after provisioning.\nOriginal view text for Hive views. Empty for non-view."]
    pub fn view_original_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.view_original_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> terrars::ListRef<BiglakeHiveTable_PartitionKeysRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> terrars::ListRef<BiglakeHiveTable_StorageDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeHiveTable_TimeoutsRef {
        BiglakeHiveTable_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BiglakeHiveTableDynamic {
    partition_keys: std::option::Option<terrars::DynamicBlock<BiglakeHiveTable_PartitionKeys>>,
    storage_descriptor: std::option::Option<terrars::DynamicBlock<BiglakeHiveTable_StorageDescriptor>>,
}

#[derive(Serialize)]
pub struct BiglakeHiveTable_PartitionKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BiglakeHiveTable_PartitionKeys {
    #[doc = "Set the field `comment`.\nComment of the field."]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeHiveTable_PartitionKeys {
    type O = terrars::BlockAssignable<BiglakeHiveTable_PartitionKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeHiveTable_PartitionKeys {
    #[doc = "Name of the field."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Type of the field."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeHiveTable_PartitionKeys {
    pub fn build(self) -> BiglakeHiveTable_PartitionKeys {
        BiglakeHiveTable_PartitionKeys {
            comment: core::default::Default::default(),
            name: self.name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeHiveTable_PartitionKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeHiveTable_PartitionKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeHiveTable_PartitionKeysRef {
        BiglakeHiveTable_PartitionKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeHiveTable_PartitionKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\nComment of the field."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the field."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeHiveTable_StorageDescriptor_Columns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BiglakeHiveTable_StorageDescriptor_Columns {
    #[doc = "Set the field `comment`.\nComment of the field."]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeHiveTable_StorageDescriptor_Columns {
    type O = terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeHiveTable_StorageDescriptor_Columns {
    #[doc = "Name of the field."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Type of the field."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeHiveTable_StorageDescriptor_Columns {
    pub fn build(self) -> BiglakeHiveTable_StorageDescriptor_Columns {
        BiglakeHiveTable_StorageDescriptor_Columns {
            comment: core::default::Default::default(),
            name: self.name,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeHiveTable_StorageDescriptor_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeHiveTable_StorageDescriptor_ColumnsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeHiveTable_StorageDescriptor_ColumnsRef {
        BiglakeHiveTable_StorageDescriptor_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeHiveTable_StorageDescriptor_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\nComment of the field."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the field."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeHiveTable_StorageDescriptor_SerdeInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deserializer_class: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serde_type: std::option::Option<terrars::PrimField<std::string::String>>,
    serialization_lib: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serializer_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BiglakeHiveTable_StorageDescriptor_SerdeInfo {
    #[doc = "Set the field `description`.\nDescription of the SerDe."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deserializer_class`.\nThe fully qualified Java class name of the deserializer."]
    pub fn set_deserializer_class(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deserializer_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\nParameters of the SerDe."]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `serde_type`.\nThe SerDe type. Possible values: [\"SERDE_TYPE_UNSPECIFIED\", \"HIVE\", \"SCHEMA_REGISTRY\"]"]
    pub fn set_serde_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.serde_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serializer_class`.\nThe fully qualified Java class name of the serializer."]
    pub fn set_serializer_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.serializer_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeHiveTable_StorageDescriptor_SerdeInfo {
    type O = terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_SerdeInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeHiveTable_StorageDescriptor_SerdeInfo {
    #[doc = "Name of the SerDe, table name by default."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The fully qualified Java class name of the serialization library."]
    pub serialization_lib: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeHiveTable_StorageDescriptor_SerdeInfo {
    pub fn build(self) -> BiglakeHiveTable_StorageDescriptor_SerdeInfo {
        BiglakeHiveTable_StorageDescriptor_SerdeInfo {
            description: core::default::Default::default(),
            deserializer_class: core::default::Default::default(),
            name: self.name,
            parameters: core::default::Default::default(),
            serde_type: core::default::Default::default(),
            serialization_lib: self.serialization_lib,
            serializer_class: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeHiveTable_StorageDescriptor_SerdeInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeHiveTable_StorageDescriptor_SerdeInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeHiveTable_StorageDescriptor_SerdeInfoRef {
        BiglakeHiveTable_StorageDescriptor_SerdeInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeHiveTable_StorageDescriptor_SerdeInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the SerDe."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deserializer_class` after provisioning.\nThe fully qualified Java class name of the deserializer."]
    pub fn deserializer_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deserializer_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the SerDe, table name by default."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\nParameters of the SerDe."]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serde_type` after provisioning.\nThe SerDe type. Possible values: [\"SERDE_TYPE_UNSPECIFIED\", \"HIVE\", \"SCHEMA_REGISTRY\"]"]
    pub fn serde_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serde_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serialization_lib` after provisioning.\nThe fully qualified Java class name of the serialization library."]
    pub fn serialization_lib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serialization_lib", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serializer_class` after provisioning.\nThe fully qualified Java class name of the serializer."]
    pub fn serializer_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serializer_class", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues {
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues { }

impl terrars::ToListMappable for BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues {
    type O = terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues {
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues {
    pub fn build(self) -> BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues {
        BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues { values: self.values }
    }
}

#[allow(dead_code)]
pub struct BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValuesRef {
        BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations {
    location: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations { }

impl terrars::ToListMappable for BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations {
    type O = terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations {
    #[doc = ""]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations {
    pub fn build(self) -> BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations {
        BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations {
            location: self.location,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocationsRef {
        BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct BiglakeHiveTable_StorageDescriptor_SkewedInfoDynamic {
    skewed_col_values: std
    ::option
    ::Option<terrars::DynamicBlock<BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues>>,
    skewed_key_values_locations: std
    ::option
    ::Option<terrars::DynamicBlock<BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations>>,
}

#[derive(Serialize)]
pub struct BiglakeHiveTable_StorageDescriptor_SkewedInfo {
    skewed_col_names: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_col_values: std::option::Option<std::vec::Vec<BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_key_values_locations: std
    ::option
    ::Option<std::vec::Vec<BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations>>,
    dynamic: BiglakeHiveTable_StorageDescriptor_SkewedInfoDynamic,
}

impl BiglakeHiveTable_StorageDescriptor_SkewedInfo {
    #[doc = "Set the field `skewed_col_values`.\n"]
    pub fn set_skewed_col_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValues>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.skewed_col_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.skewed_col_values = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `skewed_key_values_locations`.\n"]
    pub fn set_skewed_key_values_locations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocations>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.skewed_key_values_locations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.skewed_key_values_locations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BiglakeHiveTable_StorageDescriptor_SkewedInfo {
    type O = terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_SkewedInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeHiveTable_StorageDescriptor_SkewedInfo {
    #[doc = "The column names that are skewed."]
    pub skewed_col_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBiglakeHiveTable_StorageDescriptor_SkewedInfo {
    pub fn build(self) -> BiglakeHiveTable_StorageDescriptor_SkewedInfo {
        BiglakeHiveTable_StorageDescriptor_SkewedInfo {
            skewed_col_names: self.skewed_col_names,
            skewed_col_values: core::default::Default::default(),
            skewed_key_values_locations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeHiveTable_StorageDescriptor_SkewedInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeHiveTable_StorageDescriptor_SkewedInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeHiveTable_StorageDescriptor_SkewedInfoRef {
        BiglakeHiveTable_StorageDescriptor_SkewedInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeHiveTable_StorageDescriptor_SkewedInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `skewed_col_names` after provisioning.\nThe column names that are skewed."]
    pub fn skewed_col_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skewed_col_names", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_col_values` after provisioning.\n"]
    pub fn skewed_col_values(
        &self,
    ) -> terrars::ListRef<BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedColValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skewed_col_values", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_key_values_locations` after provisioning.\n"]
    pub fn skewed_key_values_locations(
        &self,
    ) -> terrars::ListRef<BiglakeHiveTable_StorageDescriptor_SkewedInfo_SkewedKeyValuesLocationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skewed_key_values_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeHiveTable_StorageDescriptor_SortCols {
    col: terrars::PrimField<std::string::String>,
    order: terrars::PrimField<f64>,
}

impl BiglakeHiveTable_StorageDescriptor_SortCols { }

impl terrars::ToListMappable for BiglakeHiveTable_StorageDescriptor_SortCols {
    type O = terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_SortCols>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeHiveTable_StorageDescriptor_SortCols {
    #[doc = "The column name."]
    pub col: terrars::PrimField<std::string::String>,
    #[doc = "Sort order: 1 for Ascending, 0 for Descending."]
    pub order: terrars::PrimField<f64>,
}

impl BuildBiglakeHiveTable_StorageDescriptor_SortCols {
    pub fn build(self) -> BiglakeHiveTable_StorageDescriptor_SortCols {
        BiglakeHiveTable_StorageDescriptor_SortCols {
            col: self.col,
            order: self.order,
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeHiveTable_StorageDescriptor_SortColsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeHiveTable_StorageDescriptor_SortColsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeHiveTable_StorageDescriptor_SortColsRef {
        BiglakeHiveTable_StorageDescriptor_SortColsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeHiveTable_StorageDescriptor_SortColsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `col` after provisioning.\nThe column name."]
    pub fn col(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.col", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nSort order: 1 for Ascending, 0 for Descending."]
    pub fn order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
}

#[derive(Serialize, Default)]
struct BiglakeHiveTable_StorageDescriptorDynamic {
    columns: std::option::Option<terrars::DynamicBlock<BiglakeHiveTable_StorageDescriptor_Columns>>,
    serde_info: std::option::Option<terrars::DynamicBlock<BiglakeHiveTable_StorageDescriptor_SerdeInfo>>,
    skewed_info: std::option::Option<terrars::DynamicBlock<BiglakeHiveTable_StorageDescriptor_SkewedInfo>>,
    sort_cols: std::option::Option<terrars::DynamicBlock<BiglakeHiveTable_StorageDescriptor_SortCols>>,
}

#[derive(Serialize)]
pub struct BiglakeHiveTable_StorageDescriptor {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_cols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compressed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_buckets: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_as_sub_dirs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<std::vec::Vec<BiglakeHiveTable_StorageDescriptor_Columns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serde_info: std::option::Option<std::vec::Vec<BiglakeHiveTable_StorageDescriptor_SerdeInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skewed_info: std::option::Option<std::vec::Vec<BiglakeHiveTable_StorageDescriptor_SkewedInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_cols: std::option::Option<std::vec::Vec<BiglakeHiveTable_StorageDescriptor_SortCols>>,
    dynamic: BiglakeHiveTable_StorageDescriptorDynamic,
}

impl BiglakeHiveTable_StorageDescriptor {
    #[doc = "Set the field `bucket_cols`.\nReducer grouping columns, clustering columns, and bucketing columns."]
    pub fn set_bucket_cols(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.bucket_cols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compressed`.\nWhether the table data is compressed."]
    pub fn set_compressed(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.compressed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_format`.\nThe fully qualified Java class name of the input format."]
    pub fn set_input_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.input_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location_uri`.\nThe Cloud Storage URI where the table data is located."]
    pub fn set_location_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `num_buckets`.\nThe number of buckets in the table."]
    pub fn set_num_buckets(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_format`.\nThe fully qualified Java class name of the output format."]
    pub fn set_output_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\nKey-value pairs for the storage descriptor."]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stored_as_sub_dirs`.\nWhether the table is stored as sub directories."]
    pub fn set_stored_as_sub_dirs(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.stored_as_sub_dirs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_Columns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serde_info`.\n"]
    pub fn set_serde_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_SerdeInfo>>,
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

    #[doc = "Set the field `skewed_info`.\n"]
    pub fn set_skewed_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_SkewedInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.skewed_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.skewed_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sort_cols`.\n"]
    pub fn set_sort_cols(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor_SortCols>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sort_cols = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sort_cols = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BiglakeHiveTable_StorageDescriptor {
    type O = terrars::BlockAssignable<BiglakeHiveTable_StorageDescriptor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeHiveTable_StorageDescriptor {}

impl BuildBiglakeHiveTable_StorageDescriptor {
    pub fn build(self) -> BiglakeHiveTable_StorageDescriptor {
        BiglakeHiveTable_StorageDescriptor {
            bucket_cols: core::default::Default::default(),
            compressed: core::default::Default::default(),
            input_format: core::default::Default::default(),
            location_uri: core::default::Default::default(),
            num_buckets: core::default::Default::default(),
            output_format: core::default::Default::default(),
            parameters: core::default::Default::default(),
            stored_as_sub_dirs: core::default::Default::default(),
            columns: core::default::Default::default(),
            serde_info: core::default::Default::default(),
            skewed_info: core::default::Default::default(),
            sort_cols: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeHiveTable_StorageDescriptorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeHiveTable_StorageDescriptorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeHiveTable_StorageDescriptorRef {
        BiglakeHiveTable_StorageDescriptorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeHiveTable_StorageDescriptorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_cols` after provisioning.\nReducer grouping columns, clustering columns, and bucketing columns."]
    pub fn bucket_cols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_cols", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compressed` after provisioning.\nWhether the table data is compressed."]
    pub fn compressed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compressed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `input_format` after provisioning.\nThe fully qualified Java class name of the input format."]
    pub fn input_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location_uri` after provisioning.\nThe Cloud Storage URI where the table data is located."]
    pub fn location_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `num_buckets` after provisioning.\nThe number of buckets in the table."]
    pub fn num_buckets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_buckets", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_format` after provisioning.\nThe fully qualified Java class name of the output format."]
    pub fn output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nKey-value pairs for the storage descriptor."]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stored_as_sub_dirs` after provisioning.\nWhether the table is stored as sub directories."]
    pub fn stored_as_sub_dirs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stored_as_sub_dirs", self.base))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<BiglakeHiveTable_StorageDescriptor_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `serde_info` after provisioning.\n"]
    pub fn serde_info(&self) -> terrars::ListRef<BiglakeHiveTable_StorageDescriptor_SerdeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serde_info", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_info` after provisioning.\n"]
    pub fn skewed_info(&self) -> terrars::ListRef<BiglakeHiveTable_StorageDescriptor_SkewedInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skewed_info", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_cols` after provisioning.\n"]
    pub fn sort_cols(&self) -> terrars::ListRef<BiglakeHiveTable_StorageDescriptor_SortColsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_cols", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeHiveTable_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BiglakeHiveTable_Timeouts {
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

impl terrars::ToListMappable for BiglakeHiveTable_Timeouts {
    type O = terrars::BlockAssignable<BiglakeHiveTable_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeHiveTable_Timeouts {}

impl BuildBiglakeHiveTable_Timeouts {
    pub fn build(self) -> BiglakeHiveTable_Timeouts {
        BiglakeHiveTable_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeHiveTable_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeHiveTable_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeHiveTable_TimeoutsRef {
        BiglakeHiveTable_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeHiveTable_TimeoutsRef {
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
