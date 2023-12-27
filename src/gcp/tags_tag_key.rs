use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct TagsTagKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose_data: Option<RecField<PrimField<String>>>,
    short_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<TagsTagKeyTimeoutsEl>,
}

struct TagsTagKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TagsTagKeyData>,
}

#[derive(Clone)]
pub struct TagsTagKey(Rc<TagsTagKey_>);

impl TagsTagKey {
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

    #[doc= "Set the field `description`.\nUser-assigned description of the TagKey. Must not exceed 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `purpose`.\nOptional. A purpose cannot be changed once set.\n\nA purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag. Possible values: [\"GCE_FIREWALL\"]"]
    pub fn set_purpose(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().purpose = Some(v.into());
        self
    }

    #[doc= "Set the field `purpose_data`.\nOptional. Purpose data cannot be changed once set.\n\nPurpose data corresponds to the policy system that the tag is intended for. For example, the GCE_FIREWALL purpose expects data in the following format: 'network = \"<project-name>/<vpc-name>\"'."]
    pub fn set_purpose_data(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().purpose_data = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<TagsTagKeyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation time.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nUser-assigned description of the TagKey. Must not exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe generated numeric id for the TagKey."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespaced_name` after provisioning.\nOutput only. Namespaced name of the TagKey."]
    pub fn namespaced_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespaced_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent` after provisioning.\nInput only. The resource name of the new TagKey's parent. Must be of the form organizations/{org_id} or projects/{project_id_or_number}."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose` after provisioning.\nOptional. A purpose cannot be changed once set.\n\nA purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag. Possible values: [\"GCE_FIREWALL\"]"]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose_data` after provisioning.\nOptional. Purpose data cannot be changed once set.\n\nPurpose data corresponds to the policy system that the tag is intended for. For example, the GCE_FIREWALL purpose expects data in the following format: 'network = \"<project-name>/<vpc-name>\"'."]
    pub fn purpose_data(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.purpose_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `short_name` after provisioning.\nInput only. The user friendly name for a TagKey. The short name should be unique for TagKeys within the same tag namespace.\n\nThe short name must be 1-63 characters, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between."]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TagsTagKeyTimeoutsElRef {
        TagsTagKeyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for TagsTagKey {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for TagsTagKey { }

impl ToListMappable for TagsTagKey {
    type O = ListRef<TagsTagKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TagsTagKey_ {
    fn extract_resource_type(&self) -> String {
        "google_tags_tag_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTagsTagKey {
    pub tf_id: String,
    #[doc= "Input only. The resource name of the new TagKey's parent. Must be of the form organizations/{org_id} or projects/{project_id_or_number}."]
    pub parent: PrimField<String>,
    #[doc= "Input only. The user friendly name for a TagKey. The short name should be unique for TagKeys within the same tag namespace.\n\nThe short name must be 1-63 characters, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between."]
    pub short_name: PrimField<String>,
}

impl BuildTagsTagKey {
    pub fn build(self, stack: &mut Stack) -> TagsTagKey {
        let out = TagsTagKey(Rc::new(TagsTagKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TagsTagKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                purpose: core::default::Default::default(),
                purpose_data: core::default::Default::default(),
                short_name: self.short_name,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TagsTagKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for TagsTagKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TagsTagKeyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation time.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nUser-assigned description of the TagKey. Must not exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe generated numeric id for the TagKey."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `namespaced_name` after provisioning.\nOutput only. Namespaced name of the TagKey."]
    pub fn namespaced_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespaced_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent` after provisioning.\nInput only. The resource name of the new TagKey's parent. Must be of the form organizations/{org_id} or projects/{project_id_or_number}."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose` after provisioning.\nOptional. A purpose cannot be changed once set.\n\nA purpose denotes that this Tag is intended for use in policies of a specific policy engine, and will involve that policy engine in management operations involving this Tag. Possible values: [\"GCE_FIREWALL\"]"]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `purpose_data` after provisioning.\nOptional. Purpose data cannot be changed once set.\n\nPurpose data corresponds to the policy system that the tag is intended for. For example, the GCE_FIREWALL purpose expects data in the following format: 'network = \"<project-name>/<vpc-name>\"'."]
    pub fn purpose_data(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.purpose_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `short_name` after provisioning.\nInput only. The user friendly name for a TagKey. The short name should be unique for TagKeys within the same tag namespace.\n\nThe short name must be 1-63 characters, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between."]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TagsTagKeyTimeoutsElRef {
        TagsTagKeyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TagsTagKeyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl TagsTagKeyTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for TagsTagKeyTimeoutsEl {
    type O = BlockAssignable<TagsTagKeyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTagsTagKeyTimeoutsEl {}

impl BuildTagsTagKeyTimeoutsEl {
    pub fn build(self) -> TagsTagKeyTimeoutsEl {
        TagsTagKeyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct TagsTagKeyTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TagsTagKeyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> TagsTagKeyTimeoutsElRef {
        TagsTagKeyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TagsTagKeyTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
