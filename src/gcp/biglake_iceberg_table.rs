#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BiglakeIcebergTableData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_spec: std::option::Option<std::vec::Vec<BiglakeIcebergTable_PartitionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<BiglakeIcebergTable_Schema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort_order: std::option::Option<std::vec::Vec<BiglakeIcebergTable_SortOrder>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BiglakeIcebergTable_Timeouts>,
    dynamic: BiglakeIcebergTableDynamic,
}

struct BiglakeIcebergTable_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BiglakeIcebergTableData_>,
}

#[derive(Clone)]
pub struct BiglakeIcebergTable(std::rc::Rc<BiglakeIcebergTable_>);

impl BiglakeIcebergTable {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location of the table."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\nUser-defined properties for the table."]
    pub fn set_properties(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partition_spec`.\n"]
    pub fn set_partition_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeIcebergTable_PartitionSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partition_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partition_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(self, v: impl std::convert::Into<terrars::BlockAssignable<BiglakeIcebergTable_Schema>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sort_order`.\n"]
    pub fn set_sort_order(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeIcebergTable_SortOrder>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sort_order = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sort_order = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BiglakeIcebergTable_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `catalog` after provisioning.\nThe name of the IcebergCatalog."]
    pub fn catalog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the table."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the table."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe parent namespace of the table."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nUser-defined properties for the table."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_spec` after provisioning.\n"]
    pub fn partition_spec(&self) -> terrars::ListRef<BiglakeIcebergTable_PartitionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<BiglakeIcebergTable_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort_order` after provisioning.\n"]
    pub fn sort_order(&self) -> terrars::ListRef<BiglakeIcebergTable_SortOrderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeIcebergTable_TimeoutsRef {
        BiglakeIcebergTable_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BiglakeIcebergTable {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BiglakeIcebergTable { }

impl terrars::ToListMappable for BiglakeIcebergTable {
    type O = terrars::ListRef<BiglakeIcebergTableRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BiglakeIcebergTable_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_biglake_iceberg_table".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBiglakeIcebergTable {
    pub tf_id: std::string::String,
    #[doc = "The name of the IcebergCatalog."]
    pub catalog: terrars::PrimField<std::string::String>,
    #[doc = "The name of the table."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The parent namespace of the table."]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeIcebergTable {
    pub fn build(self, stack: &mut terrars::Stack) -> BiglakeIcebergTable {
        let out = BiglakeIcebergTable(std::rc::Rc::new(BiglakeIcebergTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BiglakeIcebergTableData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                catalog: self.catalog,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                namespace: self.namespace,
                project: core::default::Default::default(),
                properties: core::default::Default::default(),
                partition_spec: core::default::Default::default(),
                schema: core::default::Default::default(),
                sort_order: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BiglakeIcebergTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergTableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BiglakeIcebergTableRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog` after provisioning.\nThe name of the IcebergCatalog."]
    pub fn catalog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the table."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the table."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe parent namespace of the table."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nUser-defined properties for the table."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_spec` after provisioning.\n"]
    pub fn partition_spec(&self) -> terrars::ListRef<BiglakeIcebergTable_PartitionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partition_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<BiglakeIcebergTable_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort_order` after provisioning.\n"]
    pub fn sort_order(&self) -> terrars::ListRef<BiglakeIcebergTable_SortOrderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort_order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeIcebergTable_TimeoutsRef {
        BiglakeIcebergTable_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BiglakeIcebergTableDynamic {
    partition_spec: std::option::Option<terrars::DynamicBlock<BiglakeIcebergTable_PartitionSpec>>,
    schema: std::option::Option<terrars::DynamicBlock<BiglakeIcebergTable_Schema>>,
    sort_order: std::option::Option<terrars::DynamicBlock<BiglakeIcebergTable_SortOrder>>,
}

#[derive(Serialize)]
pub struct BiglakeIcebergTable_PartitionSpec_Fields {
    name: terrars::PrimField<std::string::String>,
    source_id: terrars::PrimField<f64>,
    transform: terrars::PrimField<std::string::String>,
}

impl BiglakeIcebergTable_PartitionSpec_Fields { }

impl terrars::ToListMappable for BiglakeIcebergTable_PartitionSpec_Fields {
    type O = terrars::BlockAssignable<BiglakeIcebergTable_PartitionSpec_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergTable_PartitionSpec_Fields {
    #[doc = "The name of the partition field."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The source field ID for the partition field."]
    pub source_id: terrars::PrimField<f64>,
    #[doc = "The transform to apply to the source field."]
    pub transform: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeIcebergTable_PartitionSpec_Fields {
    pub fn build(self) -> BiglakeIcebergTable_PartitionSpec_Fields {
        BiglakeIcebergTable_PartitionSpec_Fields {
            name: self.name,
            source_id: self.source_id,
            transform: self.transform,
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergTable_PartitionSpec_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergTable_PartitionSpec_FieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeIcebergTable_PartitionSpec_FieldsRef {
        BiglakeIcebergTable_PartitionSpec_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergTable_PartitionSpec_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `field_id` after provisioning.\nThe unique identifier of the partition field."]
    pub fn field_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the partition field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_id` after provisioning.\nThe source field ID for the partition field."]
    pub fn source_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transform` after provisioning.\nThe transform to apply to the source field."]
    pub fn transform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform", self.base))
    }
}

#[derive(Serialize, Default)]
struct BiglakeIcebergTable_PartitionSpecDynamic {
    fields: std::option::Option<terrars::DynamicBlock<BiglakeIcebergTable_PartitionSpec_Fields>>,
}

#[derive(Serialize)]
pub struct BiglakeIcebergTable_PartitionSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<std::vec::Vec<BiglakeIcebergTable_PartitionSpec_Fields>>,
    dynamic: BiglakeIcebergTable_PartitionSpecDynamic,
}

impl BiglakeIcebergTable_PartitionSpec {
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeIcebergTable_PartitionSpec_Fields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergTable_PartitionSpec {
    type O = terrars::BlockAssignable<BiglakeIcebergTable_PartitionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergTable_PartitionSpec {}

impl BuildBiglakeIcebergTable_PartitionSpec {
    pub fn build(self) -> BiglakeIcebergTable_PartitionSpec {
        BiglakeIcebergTable_PartitionSpec {
            fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergTable_PartitionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergTable_PartitionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeIcebergTable_PartitionSpecRef {
        BiglakeIcebergTable_PartitionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergTable_PartitionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `spec_id` after provisioning.\nThe unique identifier of the partition spec."]
    pub fn spec_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spec_id", self.base))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<BiglakeIcebergTable_PartitionSpec_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergTable_Schema_Fields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    doc: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<f64>,
    name: terrars::PrimField<std::string::String>,
    required: terrars::PrimField<bool>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl BiglakeIcebergTable_Schema_Fields {
    #[doc = "Set the field `doc`.\nA description of the field."]
    pub fn set_doc(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.doc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergTable_Schema_Fields {
    type O = terrars::BlockAssignable<BiglakeIcebergTable_Schema_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergTable_Schema_Fields {
    #[doc = "The unique identifier of the field."]
    pub id: terrars::PrimField<f64>,
    #[doc = "The name of the field."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Whether the field is required."]
    pub required: terrars::PrimField<bool>,
    #[doc = "The type of the field."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeIcebergTable_Schema_Fields {
    pub fn build(self) -> BiglakeIcebergTable_Schema_Fields {
        BiglakeIcebergTable_Schema_Fields {
            doc: core::default::Default::default(),
            id: self.id,
            name: self.name,
            required: self.required,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergTable_Schema_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergTable_Schema_FieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeIcebergTable_Schema_FieldsRef {
        BiglakeIcebergTable_Schema_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergTable_Schema_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `doc` after provisioning.\nA description of the field."]
    pub fn doc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.doc", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the field."]
    pub fn id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\nWhether the field is required."]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the field."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BiglakeIcebergTable_SchemaDynamic {
    fields: std::option::Option<terrars::DynamicBlock<BiglakeIcebergTable_Schema_Fields>>,
}

#[derive(Serialize)]
pub struct BiglakeIcebergTable_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identifier_field_ids: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<std::vec::Vec<BiglakeIcebergTable_Schema_Fields>>,
    dynamic: BiglakeIcebergTable_SchemaDynamic,
}

impl BiglakeIcebergTable_Schema {
    #[doc = "Set the field `identifier_field_ids`.\nThe field IDs that make up the identifier for the table."]
    pub fn set_identifier_field_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.identifier_field_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe type of the schema."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeIcebergTable_Schema_Fields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergTable_Schema {
    type O = terrars::BlockAssignable<BiglakeIcebergTable_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergTable_Schema {}

impl BuildBiglakeIcebergTable_Schema {
    pub fn build(self) -> BiglakeIcebergTable_Schema {
        BiglakeIcebergTable_Schema {
            identifier_field_ids: core::default::Default::default(),
            type_: core::default::Default::default(),
            fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergTable_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergTable_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeIcebergTable_SchemaRef {
        BiglakeIcebergTable_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergTable_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identifier_field_ids` after provisioning.\nThe field IDs that make up the identifier for the table."]
    pub fn identifier_field_ids(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identifier_field_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema_id` after provisioning.\nThe unique identifier of the schema."]
    pub fn schema_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the schema."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<BiglakeIcebergTable_Schema_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergTable_SortOrder_Fields {
    direction: terrars::PrimField<std::string::String>,
    null_order: terrars::PrimField<std::string::String>,
    source_id: terrars::PrimField<f64>,
    transform: terrars::PrimField<std::string::String>,
}

impl BiglakeIcebergTable_SortOrder_Fields { }

impl terrars::ToListMappable for BiglakeIcebergTable_SortOrder_Fields {
    type O = terrars::BlockAssignable<BiglakeIcebergTable_SortOrder_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergTable_SortOrder_Fields {
    #[doc = "The sort direction for the sort field. Possible values: \"asc\", \"desc\"."]
    pub direction: terrars::PrimField<std::string::String>,
    #[doc = "The null ordering for the sort field. Possible values: \"nulls-first\", \"nulls-last\"."]
    pub null_order: terrars::PrimField<std::string::String>,
    #[doc = "The source field ID for the sort field."]
    pub source_id: terrars::PrimField<f64>,
    #[doc = "The transform to apply to the source field."]
    pub transform: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeIcebergTable_SortOrder_Fields {
    pub fn build(self) -> BiglakeIcebergTable_SortOrder_Fields {
        BiglakeIcebergTable_SortOrder_Fields {
            direction: self.direction,
            null_order: self.null_order,
            source_id: self.source_id,
            transform: self.transform,
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergTable_SortOrder_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergTable_SortOrder_FieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeIcebergTable_SortOrder_FieldsRef {
        BiglakeIcebergTable_SortOrder_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergTable_SortOrder_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe sort direction for the sort field. Possible values: \"asc\", \"desc\"."]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `null_order` after provisioning.\nThe null ordering for the sort field. Possible values: \"nulls-first\", \"nulls-last\"."]
    pub fn null_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.null_order", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_id` after provisioning.\nThe source field ID for the sort field."]
    pub fn source_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transform` after provisioning.\nThe transform to apply to the source field."]
    pub fn transform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform", self.base))
    }
}

#[derive(Serialize, Default)]
struct BiglakeIcebergTable_SortOrderDynamic {
    fields: std::option::Option<terrars::DynamicBlock<BiglakeIcebergTable_SortOrder_Fields>>,
}

#[derive(Serialize)]
pub struct BiglakeIcebergTable_SortOrder {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<std::vec::Vec<BiglakeIcebergTable_SortOrder_Fields>>,
    dynamic: BiglakeIcebergTable_SortOrderDynamic,
}

impl BiglakeIcebergTable_SortOrder {
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeIcebergTable_SortOrder_Fields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergTable_SortOrder {
    type O = terrars::BlockAssignable<BiglakeIcebergTable_SortOrder>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergTable_SortOrder {}

impl BuildBiglakeIcebergTable_SortOrder {
    pub fn build(self) -> BiglakeIcebergTable_SortOrder {
        BiglakeIcebergTable_SortOrder {
            fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergTable_SortOrderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergTable_SortOrderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeIcebergTable_SortOrderRef {
        BiglakeIcebergTable_SortOrderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergTable_SortOrderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `order_id` after provisioning.\nThe unique identifier of the sort order."]
    pub fn order_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order_id", self.base))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<BiglakeIcebergTable_SortOrder_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergTable_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BiglakeIcebergTable_Timeouts {
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

impl terrars::ToListMappable for BiglakeIcebergTable_Timeouts {
    type O = terrars::BlockAssignable<BiglakeIcebergTable_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergTable_Timeouts {}

impl BuildBiglakeIcebergTable_Timeouts {
    pub fn build(self) -> BiglakeIcebergTable_Timeouts {
        BiglakeIcebergTable_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergTable_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergTable_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeIcebergTable_TimeoutsRef {
        BiglakeIcebergTable_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergTable_TimeoutsRef {
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
