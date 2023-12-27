use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigtableTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    change_stream_retention: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_family: Option<Vec<BigtableTableColumnFamilyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigtableTableTimeoutsEl>,
    dynamic: BigtableTableDynamic,
}

struct BigtableTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigtableTableData>,
}

#[derive(Clone)]
pub struct BigtableTable(Rc<BigtableTable_>);

impl BigtableTable {
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

    #[doc= "Set the field `change_stream_retention`.\nDuration to retain change stream data for the table. Set to 0 to disable. Must be between 1 and 7 days."]
    pub fn set_change_stream_retention(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().change_stream_retention = Some(v.into());
        self
    }

    #[doc= "Set the field `deletion_protection`.\nA field to make the table protected against data loss i.e. when set to PROTECTED, deleting the table, the column families in the table, and the instance containing the table would be prohibited. If not provided, currently deletion protection will be set to UNPROTECTED as it is the API default value."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `split_keys`.\nA list of predefined keys to split the table on. !> Warning: Modifying the split_keys of an existing table will cause Terraform to delete/recreate the entire google_bigtable_table resource."]
    pub fn set_split_keys(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().split_keys = Some(v.into());
        self
    }

    #[doc= "Set the field `column_family`.\n"]
    pub fn set_column_family(self, v: impl Into<BlockAssignable<BigtableTableColumnFamilyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().column_family = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.column_family = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigtableTableTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `change_stream_retention` after provisioning.\nDuration to retain change stream data for the table. Set to 0 to disable. Must be between 1 and 7 days."]
    pub fn change_stream_retention(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.change_stream_retention", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nA field to make the table protected against data loss i.e. when set to PROTECTED, deleting the table, the column families in the table, and the instance containing the table would be prohibited. If not provided, currently deletion protection will be set to UNPROTECTED as it is the API default value."]
    pub fn deletion_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_name` after provisioning.\nThe name of the Bigtable instance."]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the table. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `split_keys` after provisioning.\nA list of predefined keys to split the table on. !> Warning: Modifying the split_keys of an existing table will cause Terraform to delete/recreate the entire google_bigtable_table resource."]
    pub fn split_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.split_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableTableTimeoutsElRef {
        BigtableTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for BigtableTable {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BigtableTable { }

impl ToListMappable for BigtableTable {
    type O = ListRef<BigtableTableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BigtableTable_ {
    fn extract_resource_type(&self) -> String {
        "google_bigtable_table".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigtableTable {
    pub tf_id: String,
    #[doc= "The name of the Bigtable instance."]
    pub instance_name: PrimField<String>,
    #[doc= "The name of the table. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers."]
    pub name: PrimField<String>,
}

impl BuildBigtableTable {
    pub fn build(self, stack: &mut Stack) -> BigtableTable {
        let out = BigtableTable(Rc::new(BigtableTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigtableTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                change_stream_retention: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_name: self.instance_name,
                name: self.name,
                project: core::default::Default::default(),
                split_keys: core::default::Default::default(),
                column_family: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigtableTableRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigtableTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigtableTableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `change_stream_retention` after provisioning.\nDuration to retain change stream data for the table. Set to 0 to disable. Must be between 1 and 7 days."]
    pub fn change_stream_retention(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.change_stream_retention", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nA field to make the table protected against data loss i.e. when set to PROTECTED, deleting the table, the column families in the table, and the instance containing the table would be prohibited. If not provided, currently deletion protection will be set to UNPROTECTED as it is the API default value."]
    pub fn deletion_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_name` after provisioning.\nThe name of the Bigtable instance."]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the table. Must be 1-50 characters and must only contain hyphens, underscores, periods, letters and numbers."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `split_keys` after provisioning.\nA list of predefined keys to split the table on. !> Warning: Modifying the split_keys of an existing table will cause Terraform to delete/recreate the entire google_bigtable_table resource."]
    pub fn split_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.split_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableTableTimeoutsElRef {
        BigtableTableTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BigtableTableColumnFamilyEl {
    family: PrimField<String>,
}

impl BigtableTableColumnFamilyEl { }

impl ToListMappable for BigtableTableColumnFamilyEl {
    type O = BlockAssignable<BigtableTableColumnFamilyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableTableColumnFamilyEl {
    #[doc= "The name of the column family."]
    pub family: PrimField<String>,
}

impl BuildBigtableTableColumnFamilyEl {
    pub fn build(self) -> BigtableTableColumnFamilyEl {
        BigtableTableColumnFamilyEl { family: self.family }
    }
}

pub struct BigtableTableColumnFamilyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigtableTableColumnFamilyElRef {
    fn new(shared: StackShared, base: String) -> BigtableTableColumnFamilyElRef {
        BigtableTableColumnFamilyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableTableColumnFamilyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `family` after provisioning.\nThe name of the column family."]
    pub fn family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.family", self.base))
    }
}

#[derive(Serialize)]
pub struct BigtableTableTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BigtableTableTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for BigtableTableTimeoutsEl {
    type O = BlockAssignable<BigtableTableTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableTableTimeoutsEl {}

impl BuildBigtableTableTimeoutsEl {
    pub fn build(self) -> BigtableTableTimeoutsEl {
        BigtableTableTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BigtableTableTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigtableTableTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigtableTableTimeoutsElRef {
        BigtableTableTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableTableTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigtableTableDynamic {
    column_family: Option<DynamicBlock<BigtableTableColumnFamilyEl>>,
}
