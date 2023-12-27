use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FolderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FolderTimeoutsEl>,
}

struct Folder_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FolderData>,
}

#[derive(Clone)]
pub struct Folder(Rc<Folder_>);

impl Folder {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FolderTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the Folder was created. Assigned by the server. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe folder's display name. A folder's display name must be unique amongst its siblings, e.g. no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder_id` after provisioning.\nThe folder id from the name \"folders/{folder_id}\""]
    pub fn folder_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lifecycle_state` after provisioning.\nThe lifecycle state of the folder such as ACTIVE or DELETE_REQUESTED."]
    pub fn lifecycle_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Folder. Its format is folders/{folder_id}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the parent Folder or Organization. Must be of the form folders/{folder_id} or organizations/{org_id}."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FolderTimeoutsElRef {
        FolderTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for Folder {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Folder { }

impl ToListMappable for Folder {
    type O = ListRef<FolderRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Folder_ {
    fn extract_resource_type(&self) -> String {
        "google_folder".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFolder {
    pub tf_id: String,
    #[doc= "The folder's display name. A folder's display name must be unique amongst its siblings, e.g. no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters."]
    pub display_name: PrimField<String>,
    #[doc= "The resource name of the parent Folder or Organization. Must be of the form folders/{folder_id} or organizations/{org_id}."]
    pub parent: PrimField<String>,
}

impl BuildFolder {
    pub fn build(self, stack: &mut Stack) -> Folder {
        let out = Folder(Rc::new(Folder_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FolderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                display_name: self.display_name,
                id: core::default::Default::default(),
                parent: self.parent,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FolderRef {
    shared: StackShared,
    base: String,
}

impl Ref for FolderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FolderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the Folder was created. Assigned by the server. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe folder's display name. A folder's display name must be unique amongst its siblings, e.g. no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder_id` after provisioning.\nThe folder id from the name \"folders/{folder_id}\""]
    pub fn folder_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lifecycle_state` after provisioning.\nThe lifecycle state of the folder such as ACTIVE or DELETE_REQUESTED."]
    pub fn lifecycle_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Folder. Its format is folders/{folder_id}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the parent Folder or Organization. Must be of the form folders/{folder_id} or organizations/{org_id}."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FolderTimeoutsElRef {
        FolderTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FolderTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl FolderTimeoutsEl {
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

    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for FolderTimeoutsEl {
    type O = BlockAssignable<FolderTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFolderTimeoutsEl {}

impl BuildFolderTimeoutsEl {
    pub fn build(self) -> FolderTimeoutsEl {
        FolderTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FolderTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FolderTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FolderTimeoutsElRef {
        FolderTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FolderTimeoutsElRef {
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

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
