#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataplexEntryLinkData_ {
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
    entry_group_id: terrars::PrimField<std::string::String>,
    entry_link_id: terrars::PrimField<std::string::String>,
    entry_link_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aspects: std::option::Option<std::vec::Vec<DataplexEntryLink_Aspects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_references: std::option::Option<std::vec::Vec<DataplexEntryLink_EntryReferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataplexEntryLink_Timeouts>,
    dynamic: DataplexEntryLinkDynamic,
}

struct DataplexEntryLink_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataplexEntryLinkData_>,
}

#[derive(Clone)]
pub struct DataplexEntryLink(std::rc::Rc<DataplexEntryLink_>);

impl DataplexEntryLink {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aspects`.\n"]
    pub fn set_aspects(self, v: impl std::convert::Into<terrars::BlockAssignable<DataplexEntryLink_Aspects>>) -> Self {
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

    #[doc = "Set the field `entry_references`.\n"]
    pub fn set_entry_references(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexEntryLink_EntryReferences>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entry_references = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entry_references = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataplexEntryLink_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Entry Link was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_group_id` after provisioning.\nThe id of the entry group this entry link is in."]
    pub fn entry_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_group_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_link_id` after provisioning.\nThe id of the entry link to create."]
    pub fn entry_link_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_link_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_link_type` after provisioning.\nRelative resource name of the Entry Link Type used to create this Entry Link. For example:\nprojects/dataplex-types/locations/global/entryLinkTypes/definition"]
    pub fn entry_link_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_link_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the entry."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the Entry Link, of the form:\nprojects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entryLinks/{entry_link_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Entry Link was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aspects` after provisioning.\n"]
    pub fn aspects(&self) -> terrars::ListRef<DataplexEntryLink_AspectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aspects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_references` after provisioning.\n"]
    pub fn entry_references(&self) -> terrars::ListRef<DataplexEntryLink_EntryReferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry_references", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexEntryLink_TimeoutsRef {
        DataplexEntryLink_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataplexEntryLink {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataplexEntryLink { }

impl terrars::ToListMappable for DataplexEntryLink {
    type O = terrars::ListRef<DataplexEntryLinkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataplexEntryLink_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataplex_entry_link".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataplexEntryLink {
    pub tf_id: std::string::String,
    #[doc = "The id of the entry group this entry link is in."]
    pub entry_group_id: terrars::PrimField<std::string::String>,
    #[doc = "The id of the entry link to create."]
    pub entry_link_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Relative resource name of the Entry Link Type used to create this Entry Link. For example:\nprojects/dataplex-types/locations/global/entryLinkTypes/definition"]
    pub entry_link_type: terrars::PrimField<std::string::String>,
    #[doc = "The location for the entry."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataplexEntryLink {
    pub fn build(self, stack: &mut terrars::Stack) -> DataplexEntryLink {
        let out = DataplexEntryLink(std::rc::Rc::new(DataplexEntryLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataplexEntryLinkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                entry_group_id: self.entry_group_id,
                entry_link_id: self.entry_link_id,
                entry_link_type: self.entry_link_type,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                aspects: core::default::Default::default(),
                entry_references: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataplexEntryLinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntryLinkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataplexEntryLinkRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the Entry Link was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_group_id` after provisioning.\nThe id of the entry group this entry link is in."]
    pub fn entry_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_group_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_link_id` after provisioning.\nThe id of the entry link to create."]
    pub fn entry_link_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_link_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_link_type` after provisioning.\nRelative resource name of the Entry Link Type used to create this Entry Link. For example:\nprojects/dataplex-types/locations/global/entryLinkTypes/definition"]
    pub fn entry_link_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_link_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the entry."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the Entry Link, of the form:\nprojects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entryLinks/{entry_link_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the Entry Link was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aspects` after provisioning.\n"]
    pub fn aspects(&self) -> terrars::ListRef<DataplexEntryLink_AspectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aspects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entry_references` after provisioning.\n"]
    pub fn entry_references(&self) -> terrars::ListRef<DataplexEntryLink_EntryReferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entry_references", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataplexEntryLink_TimeoutsRef {
        DataplexEntryLink_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataplexEntryLinkDynamic {
    aspects: std::option::Option<terrars::DynamicBlock<DataplexEntryLink_Aspects>>,
    entry_references: std::option::Option<terrars::DynamicBlock<DataplexEntryLink_EntryReferences>>,
}

#[derive(Serialize)]
pub struct DataplexEntryLink_Aspects_Aspect {
    data: terrars::PrimField<std::string::String>,
}

impl DataplexEntryLink_Aspects_Aspect { }

impl terrars::ToListMappable for DataplexEntryLink_Aspects_Aspect {
    type O = terrars::BlockAssignable<DataplexEntryLink_Aspects_Aspect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexEntryLink_Aspects_Aspect {
    #[doc =
        "The content of the aspect in JSON form, according to its aspect type schema. The maximum size of the field is 120KB (encoded as UTF-8)."]
    pub data: terrars::PrimField<std::string::String>,
}

impl BuildDataplexEntryLink_Aspects_Aspect {
    pub fn build(self) -> DataplexEntryLink_Aspects_Aspect {
        DataplexEntryLink_Aspects_Aspect { data: self.data }
    }
}

#[allow(dead_code)]
pub struct DataplexEntryLink_Aspects_AspectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntryLink_Aspects_AspectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexEntryLink_Aspects_AspectRef {
        DataplexEntryLink_Aspects_AspectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexEntryLink_Aspects_AspectRef {
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
        "Get a reference to the value of field `path` after provisioning.\nThe path in the entry link under which the aspect is attached."]
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
struct DataplexEntryLink_AspectsDynamic {
    aspect: std::option::Option<terrars::DynamicBlock<DataplexEntryLink_Aspects_Aspect>>,
}

#[derive(Serialize)]
pub struct DataplexEntryLink_Aspects {
    aspect_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aspect: std::option::Option<std::vec::Vec<DataplexEntryLink_Aspects_Aspect>>,
    dynamic: DataplexEntryLink_AspectsDynamic,
}

impl DataplexEntryLink_Aspects {
    #[doc = "Set the field `aspect`.\n"]
    pub fn set_aspect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataplexEntryLink_Aspects_Aspect>>,
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

impl terrars::ToListMappable for DataplexEntryLink_Aspects {
    type O = terrars::BlockAssignable<DataplexEntryLink_Aspects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexEntryLink_Aspects {
    #[doc =
        "The map keys of the Aspects which the service should modify.\nIt should be the aspect type reference in the format '{project_number}.{location_id}.{aspect_type_id}'."]
    pub aspect_key: terrars::PrimField<std::string::String>,
}

impl BuildDataplexEntryLink_Aspects {
    pub fn build(self) -> DataplexEntryLink_Aspects {
        DataplexEntryLink_Aspects {
            aspect_key: self.aspect_key,
            aspect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexEntryLink_AspectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntryLink_AspectsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexEntryLink_AspectsRef {
        DataplexEntryLink_AspectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexEntryLink_AspectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aspect_key` after provisioning.\nThe map keys of the Aspects which the service should modify.\nIt should be the aspect type reference in the format '{project_number}.{location_id}.{aspect_type_id}'."]
    pub fn aspect_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aspect_key", self.base))
    }

    #[doc = "Get a reference to the value of field `aspect` after provisioning.\n"]
    pub fn aspect(&self) -> terrars::ListRef<DataplexEntryLink_Aspects_AspectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aspect", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexEntryLink_EntryReferences {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexEntryLink_EntryReferences {
    #[doc =
        "Set the field `path`.\nThe path in the Entry that is referenced in the Entry Link.\nEmpty path denotes that the Entry itself is referenced in the Entry Link."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe reference type of the Entry. Possible values: [\"SOURCE\", \"TARGET\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataplexEntryLink_EntryReferences {
    type O = terrars::BlockAssignable<DataplexEntryLink_EntryReferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexEntryLink_EntryReferences {
    #[doc =
        "The relative resource name of the referenced Entry, of the form:\nprojects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataplexEntryLink_EntryReferences {
    pub fn build(self) -> DataplexEntryLink_EntryReferences {
        DataplexEntryLink_EntryReferences {
            name: self.name,
            path: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexEntryLink_EntryReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntryLink_EntryReferencesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexEntryLink_EntryReferencesRef {
        DataplexEntryLink_EntryReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexEntryLink_EntryReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the referenced Entry, of the form:\nprojects/{project_id_or_number}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe path in the Entry that is referenced in the Entry Link.\nEmpty path denotes that the Entry itself is referenced in the Entry Link."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe reference type of the Entry. Possible values: [\"SOURCE\", \"TARGET\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataplexEntryLink_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataplexEntryLink_Timeouts {
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

impl terrars::ToListMappable for DataplexEntryLink_Timeouts {
    type O = terrars::BlockAssignable<DataplexEntryLink_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataplexEntryLink_Timeouts {}

impl BuildDataplexEntryLink_Timeouts {
    pub fn build(self) -> DataplexEntryLink_Timeouts {
        DataplexEntryLink_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataplexEntryLink_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataplexEntryLink_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataplexEntryLink_TimeoutsRef {
        DataplexEntryLink_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataplexEntryLink_TimeoutsRef {
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
