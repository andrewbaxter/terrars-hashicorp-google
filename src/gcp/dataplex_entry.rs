#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataplexEntryData_ {
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
    entry_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_id: std::option::Option<terrars::PrimField<std::string::String>>,
    entry_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fully_qualified_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent_entry: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aspects: std::option::Option<std::vec::Vec<DataplexEntry_Aspects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_source: std::option::Option<std::vec::Vec<DataplexEntry_EntrySource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataplexEntry_Timeouts>,
    dynamic: DataplexEntryDynamic,
}

struct DataplexEntry_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataplexEntryData_>,
}

#[derive(Clone)]
pub struct DataplexEntry(std::rc::Rc<DataplexEntry_>);

impl DataplexEntry {
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

    #[doc = "Set the field `entry_group_id`.\nThe entry group id of the entry group the entry will be created in."]
    pub fn set_entry_group_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().entry_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entry_id`.\nThe entry id of the entry."]
    pub fn set_entry_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().entry_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fully_qualified_name`.\nA name for the entry that can be referenced by an external system. For more information, see https://cloud.google.com/dataplex/docs/fully-qualified-names.\nThe maximum size of the field is 4000 characters."]
    pub fn set_fully_qualified_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fully_qualified_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location where entry will be created."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parent_entry`.\nThe resource name of the parent entry, in the format projects/{project_number}/locations/{locationId}/entryGroups/{entryGroupId}/entries/{entryId}."]
    pub fn set_parent_entry(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent_entry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aspects`.\n"]
    pub fn set_aspects(self, v: impl std::convert::Into<terrars::BlockAssignable<DataplexEntry_Aspects>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aspects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aspects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `entry_source`.\n"]
    pub fn set_entry_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexEntry_EntrySource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entry_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entry_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataplexEntry_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Entry was created in Dataplex."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_group_id` after provisioning.\nThe entry group id of the entry group the entry will be created in."]
    pub fn entry_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\nThe entry id of the entry."]
    pub fn entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_type` after provisioning.\nThe relative resource name of the entry type that was used to create this entry, in the format projects/{project_number}/locations/{locationId}/entryTypes/{entryTypeId}."]
    pub fn entry_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fully_qualified_name` after provisioning.\nA name for the entry that can be referenced by an external system. For more information, see https://cloud.google.com/dataplex/docs/fully-qualified-names.\nThe maximum size of the field is 4000 characters."]
    pub fn fully_qualified_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fully_qualified_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where entry will be created."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the entry, in the format projects/{project_number}/locations/{locationId}/entryGroups/{entryGroupId}/entries/{entryId}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent_entry` after provisioning.\nThe resource name of the parent entry, in the format projects/{project_number}/locations/{locationId}/entryGroups/{entryGroupId}/entries/{entryId}."]
    pub fn parent_entry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_entry", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the entry was last updated in Dataplex."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aspects` after provisioning.\n"]
    pub fn aspects(&self) -> terrars::ListRef<DataplexEntry_AspectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aspects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_source` after provisioning.\n"]
    pub fn entry_source(&self) -> terrars::ListRef<DataplexEntry_EntrySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexEntry_TimeoutsRef {
        DataplexEntry_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataplexEntry {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataplexEntry { }

impl terrars::ToListMappable for DataplexEntry {
    type O = terrars::ListRef<DataplexEntryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataplexEntry_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataplex_entry".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataplexEntry {
    pub tf_id: std::string::String,
    #[doc =
        "The relative resource name of the entry type that was used to create this entry, in the format projects/{project_number}/locations/{locationId}/entryTypes/{entryTypeId}."]
    pub entry_type: terrars::PrimField<std::string::String>,
}

impl BuildDataplexEntry {
    pub fn build(self, stack: &mut terrars::Stack) -> DataplexEntry {
        let out = DataplexEntry(std::rc::Rc::new(DataplexEntry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataplexEntryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                entry_group_id: core::default::Default::default(),
                entry_id: core::default::Default::default(),
                entry_type: self.entry_type,
                fully_qualified_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                parent_entry: core::default::Default::default(),
                project: core::default::Default::default(),
                aspects: core::default::Default::default(),
                entry_source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataplexEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataplexEntryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Entry was created in Dataplex."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_group_id` after provisioning.\nThe entry group id of the entry group the entry will be created in."]
    pub fn entry_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\nThe entry id of the entry."]
    pub fn entry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_type` after provisioning.\nThe relative resource name of the entry type that was used to create this entry, in the format projects/{project_number}/locations/{locationId}/entryTypes/{entryTypeId}."]
    pub fn entry_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fully_qualified_name` after provisioning.\nA name for the entry that can be referenced by an external system. For more information, see https://cloud.google.com/dataplex/docs/fully-qualified-names.\nThe maximum size of the field is 4000 characters."]
    pub fn fully_qualified_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fully_qualified_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where entry will be created."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the entry, in the format projects/{project_number}/locations/{locationId}/entryGroups/{entryGroupId}/entries/{entryId}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent_entry` after provisioning.\nThe resource name of the parent entry, in the format projects/{project_number}/locations/{locationId}/entryGroups/{entryGroupId}/entries/{entryId}."]
    pub fn parent_entry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_entry", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the entry was last updated in Dataplex."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aspects` after provisioning.\n"]
    pub fn aspects(&self) -> terrars::ListRef<DataplexEntry_AspectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aspects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_source` after provisioning.\n"]
    pub fn entry_source(&self) -> terrars::ListRef<DataplexEntry_EntrySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexEntry_TimeoutsRef {
        DataplexEntry_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataplexEntryDynamic {
    aspects: std::option::Option<terrars::DynamicBlock<DataplexEntry_Aspects>>,
    entry_source: std::option::Option<terrars::DynamicBlock<DataplexEntry_EntrySource>>,
}

#[derive(Serialize)]
pub struct DataplexEntry_Aspects_Aspect {
    data: terrars::PrimField<std::string::String>,
}

impl DataplexEntry_Aspects_Aspect { }

impl terrars::ToListMappable for DataplexEntry_Aspects_Aspect {
    type O = terrars::BlockAssignable<DataplexEntry_Aspects_Aspect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexEntry_Aspects_Aspect {
    #[doc =
        "The content of the aspect in JSON form, according to its aspect type schema. The maximum size of the field is 120KB (encoded as UTF-8)."]
    pub data: terrars::PrimField<std::string::String>,
}

impl BuildDataplexEntry_Aspects_Aspect {
    pub fn build(self) -> DataplexEntry_Aspects_Aspect {
        DataplexEntry_Aspects_Aspect { data: self.data }
    }
}

#[allow(dead_code)]
pub struct DataplexEntry_Aspects_AspectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntry_Aspects_AspectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexEntry_Aspects_AspectRef {
        DataplexEntry_Aspects_AspectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexEntry_Aspects_AspectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aspect_type` after provisioning.\nThe resource name of the type used to create this Aspect."]
    pub fn aspect_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aspect_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Aspect was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data` after provisioning.\nThe content of the aspect in JSON form, according to its aspect type schema. The maximum size of the field is 120KB (encoded as UTF-8)."]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe path in the entry under which the aspect is attached."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Aspect was last modified."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexEntry_AspectsDynamic {
    aspect: std::option::Option<terrars::DynamicBlock<DataplexEntry_Aspects_Aspect>>,
}

#[derive(Serialize)]
pub struct DataplexEntry_Aspects {
    aspect_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aspect: std::option::Option<std::vec::Vec<DataplexEntry_Aspects_Aspect>>,
    dynamic: DataplexEntry_AspectsDynamic,
}

impl DataplexEntry_Aspects {
    #[doc = "Set the field `aspect`.\n"]
    pub fn set_aspect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexEntry_Aspects_Aspect>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aspect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aspect = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexEntry_Aspects {
    type O = terrars::BlockAssignable<DataplexEntry_Aspects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexEntry_Aspects {
    #[doc =
        "Depending on how the aspect is attached to the entry, the format of the aspect key can be one of the following:\n\nIf the aspect is attached directly to the entry: {project_number}.{locationId}.{aspectTypeId}\nIf the aspect is attached to an entry's path: {project_number}.{locationId}.{aspectTypeId}@{path}"]
    pub aspect_key: terrars::PrimField<std::string::String>,
}

impl BuildDataplexEntry_Aspects {
    pub fn build(self) -> DataplexEntry_Aspects {
        DataplexEntry_Aspects {
            aspect_key: self.aspect_key,
            aspect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexEntry_AspectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntry_AspectsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexEntry_AspectsRef {
        DataplexEntry_AspectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexEntry_AspectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aspect_key` after provisioning.\nDepending on how the aspect is attached to the entry, the format of the aspect key can be one of the following:\n\nIf the aspect is attached directly to the entry: {project_number}.{locationId}.{aspectTypeId}\nIf the aspect is attached to an entry's path: {project_number}.{locationId}.{aspectTypeId}@{path}"]
    pub fn aspect_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aspect_key", self.base))
    }

    #[doc = "Get a reference to the value of field `aspect` after provisioning.\n"]
    pub fn aspect(&self) -> terrars::ListRef<DataplexEntry_Aspects_AspectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aspect", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexEntry_EntrySource_Ancestors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexEntry_EntrySource_Ancestors {
    #[doc = "Set the field `name`.\nThe name of the ancestor resource."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe type of the ancestor resource."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexEntry_EntrySource_Ancestors {
    type O = terrars::BlockAssignable<DataplexEntry_EntrySource_Ancestors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexEntry_EntrySource_Ancestors {}

impl BuildDataplexEntry_EntrySource_Ancestors {
    pub fn build(self) -> DataplexEntry_EntrySource_Ancestors {
        DataplexEntry_EntrySource_Ancestors {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexEntry_EntrySource_AncestorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntry_EntrySource_AncestorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexEntry_EntrySource_AncestorsRef {
        DataplexEntry_EntrySource_AncestorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexEntry_EntrySource_AncestorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the ancestor resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the ancestor resource."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataplexEntry_EntrySourceDynamic {
    ancestors: std::option::Option<terrars::DynamicBlock<DataplexEntry_EntrySource_Ancestors>>,
}

#[derive(Serialize)]
pub struct DataplexEntry_EntrySource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ancestors: std::option::Option<std::vec::Vec<DataplexEntry_EntrySource_Ancestors>>,
    dynamic: DataplexEntry_EntrySourceDynamic,
}

impl DataplexEntry_EntrySource {
    #[doc = "Set the field `create_time`.\nThe time when the resource was created in the source system."]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA description of the data resource. Maximum length is 2,000 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nA user-friendly display name. Maximum length is 500 characters."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-defined labels. The maximum size of keys and values is 128 characters each.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform`.\nThe platform containing the source system. Maximum length is 64 characters."]
    pub fn set_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource`.\nThe name of the resource in the source system. Maximum length is 4,000 characters."]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system`.\nThe name of the source system. Maximum length is 64 characters."]
    pub fn set_system(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.system = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update_time`.\nThe time when the resource was last updated in the source system.\nIf the entry exists in the system and its EntrySource has updateTime populated,\nfurther updates to the EntrySource of the entry must provide incremental updates to its updateTime."]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ancestors`.\n"]
    pub fn set_ancestors(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexEntry_EntrySource_Ancestors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ancestors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ancestors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataplexEntry_EntrySource {
    type O = terrars::BlockAssignable<DataplexEntry_EntrySource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexEntry_EntrySource {}

impl BuildDataplexEntry_EntrySource {
    pub fn build(self) -> DataplexEntry_EntrySource {
        DataplexEntry_EntrySource {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            labels: core::default::Default::default(),
            platform: core::default::Default::default(),
            resource: core::default::Default::default(),
            system: core::default::Default::default(),
            update_time: core::default::Default::default(),
            ancestors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexEntry_EntrySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntry_EntrySourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexEntry_EntrySourceRef {
        DataplexEntry_EntrySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexEntry_EntrySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the resource was created in the source system."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the data resource. Maximum length is 2,000 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-friendly display name. Maximum length is 500 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels. The maximum size of keys and values is 128 characters each.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the resource in the source system. You can search the entry by this location.\nBy default, this should match the location of the entry group containing this entry.\nA different value allows capturing the source location for data external to Google Cloud."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `platform` after provisioning.\nThe platform containing the source system. Maximum length is 64 characters."]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nThe name of the resource in the source system. Maximum length is 4,000 characters."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc =
        "Get a reference to the value of field `system` after provisioning.\nThe name of the source system. Maximum length is 64 characters."]
    pub fn system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the resource was last updated in the source system.\nIf the entry exists in the system and its EntrySource has updateTime populated,\nfurther updates to the EntrySource of the entry must provide incremental updates to its updateTime."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `ancestors` after provisioning.\n"]
    pub fn ancestors(&self) -> terrars::ListRef<DataplexEntry_EntrySource_AncestorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ancestors", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexEntry_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexEntry_Timeouts {
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

impl terrars::ToListMappable for DataplexEntry_Timeouts {
    type O = terrars::BlockAssignable<DataplexEntry_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexEntry_Timeouts {}

impl BuildDataplexEntry_Timeouts {
    pub fn build(self) -> DataplexEntry_Timeouts {
        DataplexEntry_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexEntry_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntry_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexEntry_TimeoutsRef {
        DataplexEntry_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexEntry_TimeoutsRef {
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
