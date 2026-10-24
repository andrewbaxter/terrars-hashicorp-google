use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeSharedflowData {
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
    config_bundle: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detect_md5hash: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    org_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApigeeSharedflowTimeoutsEl>,
}

struct ApigeeSharedflow_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApigeeSharedflowData>,
}

#[derive(Clone)]
pub struct ApigeeSharedflow(std::rc::Rc<ApigeeSharedflow_>);

impl ApigeeSharedflow {
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
        "Set the field `detect_md5hash`.\nA hash of local config bundle in string, user needs to use a Terraform Hash function of their choice. A change in hash will trigger an update."]
    pub fn set_detect_md5hash(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().detect_md5hash = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApigeeSharedflowTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `config_bundle` after provisioning.\nPath to the config zip bundle"]
    pub fn config_bundle(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_bundle", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `detect_md5hash` after provisioning.\nA hash of local config bundle in string, user needs to use a Terraform Hash function of their choice. A change in hash will trigger an update."]
    pub fn detect_md5hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detect_md5hash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_revision_id` after provisioning.\nThe id of the most recently created revision for this shared flow."]
    pub fn latest_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `md5hash` after provisioning.\nBase 64 MD5 hash of the uploaded config bundle."]
    pub fn md5hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.md5hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meta_data` after provisioning.\nMetadata describing the shared flow."]
    pub fn meta_data(&self) -> terrars::ListRef<ApigeeSharedflowMetaDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.meta_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the shared flow."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization name associated with the Apigee instance."]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision` after provisioning.\nA list of revisions of this shared flow."]
    pub fn revision(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSharedflowTimeoutsElRef {
        ApigeeSharedflowTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApigeeSharedflow {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApigeeSharedflow { }

impl terrars::ToListMappable for ApigeeSharedflow {
    type O = terrars::ListRef<ApigeeSharedflowRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApigeeSharedflow_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apigee_sharedflow".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeSharedflow {
    pub tf_id: std::string::String,
    #[doc = "Path to the config zip bundle"]
    pub config_bundle: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the shared flow."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The Apigee Organization name associated with the Apigee instance."]
    pub org_id: terrars::PrimField<std::string::String>,
}

impl BuildApigeeSharedflow {
    pub fn build(self, stack: &mut terrars::Stack) -> ApigeeSharedflow {
        let out = ApigeeSharedflow(std::rc::Rc::new(ApigeeSharedflow_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApigeeSharedflowData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                config_bundle: self.config_bundle,
                detect_md5hash: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                org_id: self.org_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeSharedflowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeSharedflowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeSharedflowRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config_bundle` after provisioning.\nPath to the config zip bundle"]
    pub fn config_bundle(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_bundle", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `detect_md5hash` after provisioning.\nA hash of local config bundle in string, user needs to use a Terraform Hash function of their choice. A change in hash will trigger an update."]
    pub fn detect_md5hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detect_md5hash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_revision_id` after provisioning.\nThe id of the most recently created revision for this shared flow."]
    pub fn latest_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `md5hash` after provisioning.\nBase 64 MD5 hash of the uploaded config bundle."]
    pub fn md5hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.md5hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `meta_data` after provisioning.\nMetadata describing the shared flow."]
    pub fn meta_data(&self) -> terrars::ListRef<ApigeeSharedflowMetaDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.meta_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe ID of the shared flow."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization name associated with the Apigee instance."]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision` after provisioning.\nA list of revisions of this shared flow."]
    pub fn revision(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSharedflowTimeoutsElRef {
        ApigeeSharedflowTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApigeeSharedflowMetaDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_modified_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeSharedflowMetaDataEl {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_modified_at`.\n"]
    pub fn set_last_modified_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_modified_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sub_type`.\n"]
    pub fn set_sub_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sub_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeSharedflowMetaDataEl {
    type O = terrars::BlockAssignable<ApigeeSharedflowMetaDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeSharedflowMetaDataEl {}

impl BuildApigeeSharedflowMetaDataEl {
    pub fn build(self) -> ApigeeSharedflowMetaDataEl {
        ApigeeSharedflowMetaDataEl {
            created_at: core::default::Default::default(),
            last_modified_at: core::default::Default::default(),
            sub_type: core::default::Default::default(),
        }
    }
}

pub struct ApigeeSharedflowMetaDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeSharedflowMetaDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeSharedflowMetaDataElRef {
        ApigeeSharedflowMetaDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeSharedflowMetaDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `last_modified_at` after provisioning.\n"]
    pub fn last_modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_at", self.base))
    }

    #[doc = "Get a reference to the value of field `sub_type` after provisioning.\n"]
    pub fn sub_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sub_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeSharedflowTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeSharedflowTimeoutsEl {
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

impl terrars::ToListMappable for ApigeeSharedflowTimeoutsEl {
    type O = terrars::BlockAssignable<ApigeeSharedflowTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeSharedflowTimeoutsEl {}

impl BuildApigeeSharedflowTimeoutsEl {
    pub fn build(self) -> ApigeeSharedflowTimeoutsEl {
        ApigeeSharedflowTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ApigeeSharedflowTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeSharedflowTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeSharedflowTimeoutsElRef {
        ApigeeSharedflowTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeSharedflowTimeoutsElRef {
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
