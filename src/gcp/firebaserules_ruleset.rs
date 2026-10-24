use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FirebaserulesRulesetData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<FirebaserulesRulesetSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FirebaserulesRulesetTimeoutsEl>,
    dynamic: FirebaserulesRulesetDynamic,
}

struct FirebaserulesRuleset_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FirebaserulesRulesetData>,
}

#[derive(Clone)]
pub struct FirebaserulesRuleset(std::rc::Rc<FirebaserulesRuleset_>);

impl FirebaserulesRuleset {
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

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl std::convert::Into<terrars::BlockAssignable<FirebaserulesRulesetSourceEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FirebaserulesRulesetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time the `Ruleset` was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nOutput only. The metadata for this ruleset."]
    pub fn metadata(&self) -> terrars::ListRef<FirebaserulesRulesetMetadataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the `Ruleset`. The ruleset_id is auto generated by the service. Format: `projects/{project_id}/rulesets/{ruleset_id}`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<FirebaserulesRulesetSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaserulesRulesetTimeoutsElRef {
        FirebaserulesRulesetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FirebaserulesRuleset {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FirebaserulesRuleset { }

impl terrars::ToListMappable for FirebaserulesRuleset {
    type O = terrars::ListRef<FirebaserulesRulesetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FirebaserulesRuleset_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_firebaserules_ruleset".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFirebaserulesRuleset {
    pub tf_id: std::string::String,
}

impl BuildFirebaserulesRuleset {
    pub fn build(self, stack: &mut terrars::Stack) -> FirebaserulesRuleset {
        let out = FirebaserulesRuleset(std::rc::Rc::new(FirebaserulesRuleset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FirebaserulesRulesetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FirebaserulesRulesetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaserulesRulesetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FirebaserulesRulesetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time the `Ruleset` was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nOutput only. The metadata for this ruleset."]
    pub fn metadata(&self) -> terrars::ListRef<FirebaserulesRulesetMetadataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the `Ruleset`. The ruleset_id is auto generated by the service. Format: `projects/{project_id}/rulesets/{ruleset_id}`"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<FirebaserulesRulesetSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaserulesRulesetTimeoutsElRef {
        FirebaserulesRulesetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FirebaserulesRulesetMetadataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    services: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl FirebaserulesRulesetMetadataEl {
    #[doc = "Set the field `services`.\n"]
    pub fn set_services(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.services = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaserulesRulesetMetadataEl {
    type O = terrars::BlockAssignable<FirebaserulesRulesetMetadataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaserulesRulesetMetadataEl {}

impl BuildFirebaserulesRulesetMetadataEl {
    pub fn build(self) -> FirebaserulesRulesetMetadataEl {
        FirebaserulesRulesetMetadataEl { services: core::default::Default::default() }
    }
}

pub struct FirebaserulesRulesetMetadataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaserulesRulesetMetadataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaserulesRulesetMetadataElRef {
        FirebaserulesRulesetMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaserulesRulesetMetadataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `services` after provisioning.\n"]
    pub fn services(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.services", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaserulesRulesetSourceElFilesEl {
    content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl FirebaserulesRulesetSourceElFilesEl {
    #[doc = "Set the field `fingerprint`.\nFingerprint (e.g. github sha) associated with the `File`."]
    pub fn set_fingerprint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fingerprint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaserulesRulesetSourceElFilesEl {
    type O = terrars::BlockAssignable<FirebaserulesRulesetSourceElFilesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaserulesRulesetSourceElFilesEl {
    #[doc = "Textual Content."]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = "File name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildFirebaserulesRulesetSourceElFilesEl {
    pub fn build(self) -> FirebaserulesRulesetSourceElFilesEl {
        FirebaserulesRulesetSourceElFilesEl {
            content: self.content,
            fingerprint: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct FirebaserulesRulesetSourceElFilesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaserulesRulesetSourceElFilesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaserulesRulesetSourceElFilesElRef {
        FirebaserulesRulesetSourceElFilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaserulesRulesetSourceElFilesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nTextual Content."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint (e.g. github sha) associated with the `File`."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nFile name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirebaserulesRulesetSourceElDynamic {
    files: std::option::Option<terrars::DynamicBlock<FirebaserulesRulesetSourceElFilesEl>>,
}

#[derive(Serialize)]
pub struct FirebaserulesRulesetSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    files: std::option::Option<std::vec::Vec<FirebaserulesRulesetSourceElFilesEl>>,
    dynamic: FirebaserulesRulesetSourceElDynamic,
}

impl FirebaserulesRulesetSourceEl {
    #[doc =
        "Set the field `language`.\n`Language` of the `Source` bundle. If unspecified, the language will default to `FIREBASE_RULES`. Possible values: LANGUAGE_UNSPECIFIED, FIREBASE_RULES, EVENT_FLOW_TRIGGERS"]
    pub fn set_language(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `files`.\n"]
    pub fn set_files(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaserulesRulesetSourceElFilesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.files = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.files = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FirebaserulesRulesetSourceEl {
    type O = terrars::BlockAssignable<FirebaserulesRulesetSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaserulesRulesetSourceEl {}

impl BuildFirebaserulesRulesetSourceEl {
    pub fn build(self) -> FirebaserulesRulesetSourceEl {
        FirebaserulesRulesetSourceEl {
            language: core::default::Default::default(),
            files: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct FirebaserulesRulesetSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaserulesRulesetSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaserulesRulesetSourceElRef {
        FirebaserulesRulesetSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaserulesRulesetSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `language` after provisioning.\n`Language` of the `Source` bundle. If unspecified, the language will default to `FIREBASE_RULES`. Possible values: LANGUAGE_UNSPECIFIED, FIREBASE_RULES, EVENT_FLOW_TRIGGERS"]
    pub fn language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language", self.base))
    }

    #[doc = "Get a reference to the value of field `files` after provisioning.\n"]
    pub fn files(&self) -> terrars::ListRef<FirebaserulesRulesetSourceElFilesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.files", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaserulesRulesetTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaserulesRulesetTimeoutsEl {
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
}

impl terrars::ToListMappable for FirebaserulesRulesetTimeoutsEl {
    type O = terrars::BlockAssignable<FirebaserulesRulesetTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaserulesRulesetTimeoutsEl {}

impl BuildFirebaserulesRulesetTimeoutsEl {
    pub fn build(self) -> FirebaserulesRulesetTimeoutsEl {
        FirebaserulesRulesetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct FirebaserulesRulesetTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaserulesRulesetTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaserulesRulesetTimeoutsElRef {
        FirebaserulesRulesetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaserulesRulesetTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct FirebaserulesRulesetDynamic {
    source: std::option::Option<terrars::DynamicBlock<FirebaserulesRulesetSourceEl>>,
}
