#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleReferenceListData_ {
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
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    reference_list_id: terrars::PrimField<std::string::String>,
    syntax_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entries: std::option::Option<std::vec::Vec<ChronicleReferenceList_Entries>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope_info: std::option::Option<std::vec::Vec<ChronicleReferenceList_ScopeInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleReferenceList_Timeouts>,
    dynamic: ChronicleReferenceListDynamic,
}

struct ChronicleReferenceList_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleReferenceListData_>,
}

#[derive(Clone)]
pub struct ChronicleReferenceList(std::rc::Rc<ChronicleReferenceList_>);

impl ChronicleReferenceList {
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

    #[doc = "Set the field `entries`.\n"]
    pub fn set_entries(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleReferenceList_Entries>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entries = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope_info`.\n"]
    pub fn set_scope_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleReferenceList_ScopeInfo>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleReferenceList_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nRequired. A user-provided description of the reference list."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The unique display name of the reference list."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the reference list.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/referenceLists/{reference_list}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reference_list_id` after provisioning.\nRequired. The ID to use for the reference list. This is also the display name for\nthe reference list. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Has length < 256.\n- Must be unique."]
    pub fn reference_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_list_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp when the reference list was last updated."]
    pub fn revision_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_associations_count` after provisioning.\nOutput only. The count of self-authored rules using the reference list."]
    pub fn rule_associations_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_associations_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nOutput only. The resource names for the associated self-authored Rules that use this\nreference list.\nThis is returned only when the view is REFERENCE_LIST_VIEW_FULL."]
    pub fn rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `syntax_type` after provisioning.\nPossible values:\nREFERENCE_LIST_SYNTAX_TYPE_PLAIN_TEXT_STRING\nREFERENCE_LIST_SYNTAX_TYPE_REGEX\nREFERENCE_LIST_SYNTAX_TYPE_CIDR"]
    pub fn syntax_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.syntax_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> terrars::ListRef<ChronicleReferenceList_EntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_info` after provisioning.\n"]
    pub fn scope_info(&self) -> terrars::ListRef<ChronicleReferenceList_ScopeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleReferenceList_TimeoutsRef {
        ChronicleReferenceList_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleReferenceList {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleReferenceList { }

impl terrars::ToListMappable for ChronicleReferenceList {
    type O = terrars::ListRef<ChronicleReferenceListRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleReferenceList_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_reference_list".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleReferenceList {
    pub tf_id: std::string::String,
    #[doc = "Required. A user-provided description of the reference list."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. The ID to use for the reference list. This is also the display name for\nthe reference list. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Has length < 256.\n- Must be unique."]
    pub reference_list_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Possible values:\nREFERENCE_LIST_SYNTAX_TYPE_PLAIN_TEXT_STRING\nREFERENCE_LIST_SYNTAX_TYPE_REGEX\nREFERENCE_LIST_SYNTAX_TYPE_CIDR"]
    pub syntax_type: terrars::PrimField<std::string::String>,
}

impl BuildChronicleReferenceList {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleReferenceList {
        let out = ChronicleReferenceList(std::rc::Rc::new(ChronicleReferenceList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleReferenceListData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: self.description,
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                reference_list_id: self.reference_list_id,
                syntax_type: self.syntax_type,
                entries: core::default::Default::default(),
                scope_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleReferenceListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleReferenceListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleReferenceListRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nRequired. A user-provided description of the reference list."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOutput only. The unique display name of the reference list."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the reference list.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/referenceLists/{reference_list}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reference_list_id` after provisioning.\nRequired. The ID to use for the reference list. This is also the display name for\nthe reference list. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Has length < 256.\n- Must be unique."]
    pub fn reference_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_list_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp when the reference list was last updated."]
    pub fn revision_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_associations_count` after provisioning.\nOutput only. The count of self-authored rules using the reference list."]
    pub fn rule_associations_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_associations_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rules` after provisioning.\nOutput only. The resource names for the associated self-authored Rules that use this\nreference list.\nThis is returned only when the view is REFERENCE_LIST_VIEW_FULL."]
    pub fn rules(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `syntax_type` after provisioning.\nPossible values:\nREFERENCE_LIST_SYNTAX_TYPE_PLAIN_TEXT_STRING\nREFERENCE_LIST_SYNTAX_TYPE_REGEX\nREFERENCE_LIST_SYNTAX_TYPE_CIDR"]
    pub fn syntax_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.syntax_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> terrars::ListRef<ChronicleReferenceList_EntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope_info` after provisioning.\n"]
    pub fn scope_info(&self) -> terrars::ListRef<ChronicleReferenceList_ScopeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleReferenceList_TimeoutsRef {
        ChronicleReferenceList_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleReferenceListDynamic {
    entries: std::option::Option<terrars::DynamicBlock<ChronicleReferenceList_Entries>>,
    scope_info: std::option::Option<terrars::DynamicBlock<ChronicleReferenceList_ScopeInfo>>,
}

#[derive(Serialize)]
pub struct ChronicleReferenceList_Entries {
    value: terrars::PrimField<std::string::String>,
}

impl ChronicleReferenceList_Entries { }

impl terrars::ToListMappable for ChronicleReferenceList_Entries {
    type O = terrars::BlockAssignable<ChronicleReferenceList_Entries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleReferenceList_Entries {
    #[doc = "Required. The value of the entry. Maximum length is 512 characters."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildChronicleReferenceList_Entries {
    pub fn build(self) -> ChronicleReferenceList_Entries {
        ChronicleReferenceList_Entries { value: self.value }
    }
}

#[allow(dead_code)]
pub struct ChronicleReferenceList_EntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleReferenceList_EntriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleReferenceList_EntriesRef {
        ChronicleReferenceList_EntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleReferenceList_EntriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nRequired. The value of the entry. Maximum length is 512 characters."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleReferenceList_ScopeInfo_ReferenceListScope {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ChronicleReferenceList_ScopeInfo_ReferenceListScope {
    #[doc =
        "Set the field `scope_names`.\nOptional. The list of scope names of the reference list. The scope names should be\nfull resource names and should be of the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope_name}\"."]
    pub fn set_scope_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scope_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleReferenceList_ScopeInfo_ReferenceListScope {
    type O = terrars::BlockAssignable<ChronicleReferenceList_ScopeInfo_ReferenceListScope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleReferenceList_ScopeInfo_ReferenceListScope {}

impl BuildChronicleReferenceList_ScopeInfo_ReferenceListScope {
    pub fn build(self) -> ChronicleReferenceList_ScopeInfo_ReferenceListScope {
        ChronicleReferenceList_ScopeInfo_ReferenceListScope { scope_names: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ChronicleReferenceList_ScopeInfo_ReferenceListScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleReferenceList_ScopeInfo_ReferenceListScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleReferenceList_ScopeInfo_ReferenceListScopeRef {
        ChronicleReferenceList_ScopeInfo_ReferenceListScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleReferenceList_ScopeInfo_ReferenceListScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scope_names` after provisioning.\nOptional. The list of scope names of the reference list. The scope names should be\nfull resource names and should be of the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope_name}\"."]
    pub fn scope_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_names", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleReferenceList_ScopeInfoDynamic {
    reference_list_scope: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleReferenceList_ScopeInfo_ReferenceListScope>>,
}

#[derive(Serialize)]
pub struct ChronicleReferenceList_ScopeInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_list_scope: std::option::Option<std::vec::Vec<ChronicleReferenceList_ScopeInfo_ReferenceListScope>>,
    dynamic: ChronicleReferenceList_ScopeInfoDynamic,
}

impl ChronicleReferenceList_ScopeInfo {
    #[doc = "Set the field `reference_list_scope`.\n"]
    pub fn set_reference_list_scope(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleReferenceList_ScopeInfo_ReferenceListScope>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reference_list_scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reference_list_scope = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleReferenceList_ScopeInfo {
    type O = terrars::BlockAssignable<ChronicleReferenceList_ScopeInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleReferenceList_ScopeInfo {}

impl BuildChronicleReferenceList_ScopeInfo {
    pub fn build(self) -> ChronicleReferenceList_ScopeInfo {
        ChronicleReferenceList_ScopeInfo {
            reference_list_scope: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleReferenceList_ScopeInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleReferenceList_ScopeInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleReferenceList_ScopeInfoRef {
        ChronicleReferenceList_ScopeInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleReferenceList_ScopeInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reference_list_scope` after provisioning.\n"]
    pub fn reference_list_scope(&self) -> terrars::ListRef<ChronicleReferenceList_ScopeInfo_ReferenceListScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_list_scope", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleReferenceList_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleReferenceList_Timeouts {
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

impl terrars::ToListMappable for ChronicleReferenceList_Timeouts {
    type O = terrars::BlockAssignable<ChronicleReferenceList_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleReferenceList_Timeouts {}

impl BuildChronicleReferenceList_Timeouts {
    pub fn build(self) -> ChronicleReferenceList_Timeouts {
        ChronicleReferenceList_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleReferenceList_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleReferenceList_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleReferenceList_TimeoutsRef {
        ChronicleReferenceList_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleReferenceList_TimeoutsRef {
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
