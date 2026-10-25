#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FirestoreIndexData_ {
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
    api_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    collection: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    density: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multikey: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_wait: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<std::vec::Vec<FirestoreIndex_Fields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FirestoreIndex_Timeouts>,
    dynamic: FirestoreIndexDynamic,
}

struct FirestoreIndex_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FirestoreIndexData_>,
}

#[derive(Clone)]
pub struct FirestoreIndex(std::rc::Rc<FirestoreIndex_>);

impl FirestoreIndex {
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
        "Set the field `api_scope`.\nThe API scope at which a query is run. Default value: \"ANY_API\" Possible values: [\"ANY_API\", \"DATASTORE_MODE_API\", \"MONGODB_COMPATIBLE_API\"]"]
    pub fn set_api_scope(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().api_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database`.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn set_database(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `density`.\nThe density configuration for this index. Possible values: [\"SPARSE_ALL\", \"SPARSE_ANY\", \"DENSE\"]"]
    pub fn set_density(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().density = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `multikey`.\nOptional. Whether the index is multikey. By default, the index is not multikey. For non-multikey indexes, none of the paths in the index definition reach or traverse an array, except via an explicit array index. For multikey indexes, at most one of the paths in the index definition reach or traverse an array, except via an explicit array index. Violations will result in errors. Note this field only applies to indexes with MONGODB_COMPATIBLE_API ApiScope."]
    pub fn set_multikey(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multikey = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_scope`.\nThe scope at which a query is run. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\", \"COLLECTION_RECURSIVE\"]"]
    pub fn set_query_scope(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().query_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_wait`.\nWhether to skip waiting for the index to be created."]
    pub fn set_skip_wait(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_wait = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unique`.\nWhether it is an unique index. Unique index ensures all values for the indexed field(s) are unique across documents."]
    pub fn set_unique(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().unique = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(self, v: impl std::convert::Into<terrars::BlockAssignable<FirestoreIndex_Fields>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FirestoreIndex_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `api_scope` after provisioning.\nThe API scope at which a query is run. Default value: \"ANY_API\" Possible values: [\"ANY_API\", \"DATASTORE_MODE_API\", \"MONGODB_COMPATIBLE_API\"]"]
    pub fn api_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collection` after provisioning.\nThe collection being indexed."]
    pub fn collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `density` after provisioning.\nThe density configuration for this index. Possible values: [\"SPARSE_ALL\", \"SPARSE_ANY\", \"DENSE\"]"]
    pub fn density(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.density", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `multikey` after provisioning.\nOptional. Whether the index is multikey. By default, the index is not multikey. For non-multikey indexes, none of the paths in the index definition reach or traverse an array, except via an explicit array index. For multikey indexes, at most one of the paths in the index definition reach or traverse an array, except via an explicit array index. Violations will result in errors. Note this field only applies to indexes with MONGODB_COMPATIBLE_API ApiScope."]
    pub fn multikey(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multikey", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA server defined name for this index. Format:\n'projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/indexes/{{server_generated_id}}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `query_scope` after provisioning.\nThe scope at which a query is run. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\", \"COLLECTION_RECURSIVE\"]"]
    pub fn query_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_wait` after provisioning.\nWhether to skip waiting for the index to be created."]
    pub fn skip_wait(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_wait", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unique` after provisioning.\nWhether it is an unique index. Unique index ensures all values for the indexed field(s) are unique across documents."]
    pub fn unique(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<FirestoreIndex_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreIndex_TimeoutsRef {
        FirestoreIndex_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FirestoreIndex {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FirestoreIndex { }

impl terrars::ToListMappable for FirestoreIndex {
    type O = terrars::ListRef<FirestoreIndexRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FirestoreIndex_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_firestore_index".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFirestoreIndex {
    pub tf_id: std::string::String,
    #[doc = "The collection being indexed."]
    pub collection: terrars::PrimField<std::string::String>,
}

impl BuildFirestoreIndex {
    pub fn build(self, stack: &mut terrars::Stack) -> FirestoreIndex {
        let out = FirestoreIndex(std::rc::Rc::new(FirestoreIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FirestoreIndexData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_scope: core::default::Default::default(),
                collection: self.collection,
                database: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                density: core::default::Default::default(),
                id: core::default::Default::default(),
                multikey: core::default::Default::default(),
                project: core::default::Default::default(),
                query_scope: core::default::Default::default(),
                skip_wait: core::default::Default::default(),
                unique: core::default::Default::default(),
                fields: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FirestoreIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FirestoreIndexRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_scope` after provisioning.\nThe API scope at which a query is run. Default value: \"ANY_API\" Possible values: [\"ANY_API\", \"DATASTORE_MODE_API\", \"MONGODB_COMPATIBLE_API\"]"]
    pub fn api_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collection` after provisioning.\nThe collection being indexed."]
    pub fn collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `density` after provisioning.\nThe density configuration for this index. Possible values: [\"SPARSE_ALL\", \"SPARSE_ANY\", \"DENSE\"]"]
    pub fn density(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.density", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `multikey` after provisioning.\nOptional. Whether the index is multikey. By default, the index is not multikey. For non-multikey indexes, none of the paths in the index definition reach or traverse an array, except via an explicit array index. For multikey indexes, at most one of the paths in the index definition reach or traverse an array, except via an explicit array index. Violations will result in errors. Note this field only applies to indexes with MONGODB_COMPATIBLE_API ApiScope."]
    pub fn multikey(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multikey", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA server defined name for this index. Format:\n'projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/indexes/{{server_generated_id}}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `query_scope` after provisioning.\nThe scope at which a query is run. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\", \"COLLECTION_RECURSIVE\"]"]
    pub fn query_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_wait` after provisioning.\nWhether to skip waiting for the index to be created."]
    pub fn skip_wait(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_wait", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unique` after provisioning.\nWhether it is an unique index. Unique index ensures all values for the indexed field(s) are unique across documents."]
    pub fn unique(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<FirestoreIndex_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreIndex_TimeoutsRef {
        FirestoreIndex_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FirestoreIndexDynamic {
    fields: std::option::Option<terrars::DynamicBlock<FirestoreIndex_Fields>>,
}

#[derive(Serialize)]
pub struct FirestoreIndex_Fields_SearchConfig_GeoSpec {
    geo_json_indexing_disabled: terrars::PrimField<bool>,
}

impl FirestoreIndex_Fields_SearchConfig_GeoSpec { }

impl terrars::ToListMappable for FirestoreIndex_Fields_SearchConfig_GeoSpec {
    type O = terrars::BlockAssignable<FirestoreIndex_Fields_SearchConfig_GeoSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreIndex_Fields_SearchConfig_GeoSpec {
    #[doc =
        "If true, disables GeoJSON indexing for the field. By default, GeoJSON points are indexed.\nFirestore GeoPoints are indexed regardless of the value of this field."]
    pub geo_json_indexing_disabled: terrars::PrimField<bool>,
}

impl BuildFirestoreIndex_Fields_SearchConfig_GeoSpec {
    pub fn build(self) -> FirestoreIndex_Fields_SearchConfig_GeoSpec {
        FirestoreIndex_Fields_SearchConfig_GeoSpec { geo_json_indexing_disabled: self.geo_json_indexing_disabled }
    }
}

#[allow(dead_code)]
pub struct FirestoreIndex_Fields_SearchConfig_GeoSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreIndex_Fields_SearchConfig_GeoSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirestoreIndex_Fields_SearchConfig_GeoSpecRef {
        FirestoreIndex_Fields_SearchConfig_GeoSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreIndex_Fields_SearchConfig_GeoSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `geo_json_indexing_disabled` after provisioning.\nIf true, disables GeoJSON indexing for the field. By default, GeoJSON points are indexed.\nFirestore GeoPoints are indexed regardless of the value of this field."]
    pub fn geo_json_indexing_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.geo_json_indexing_disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs {
    #[doc = "Set the field `index_type`.\nWays to index the text field value."]
    pub fn set_index_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_type`.\nHow to match the text field value."]
    pub fn set_match_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs {
    type O = terrars::BlockAssignable<FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs {}

impl BuildFirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs {
    pub fn build(self) -> FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs {
        FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs {
            index_type: core::default::Default::default(),
            match_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecsRef {
        FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `index_type` after provisioning.\nWays to index the text field value."]
    pub fn index_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `match_type` after provisioning.\nHow to match the text field value."]
    pub fn match_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirestoreIndex_Fields_SearchConfig_TextSpecDynamic {
    index_specs: std::option::Option<terrars::DynamicBlock<FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs>>,
}

#[derive(Serialize)]
pub struct FirestoreIndex_Fields_SearchConfig_TextSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_specs: std::option::Option<std::vec::Vec<FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs>>,
    dynamic: FirestoreIndex_Fields_SearchConfig_TextSpecDynamic,
}

impl FirestoreIndex_Fields_SearchConfig_TextSpec {
    #[doc = "Set the field `index_specs`.\n"]
    pub fn set_index_specs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.index_specs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.index_specs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FirestoreIndex_Fields_SearchConfig_TextSpec {
    type O = terrars::BlockAssignable<FirestoreIndex_Fields_SearchConfig_TextSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreIndex_Fields_SearchConfig_TextSpec {}

impl BuildFirestoreIndex_Fields_SearchConfig_TextSpec {
    pub fn build(self) -> FirestoreIndex_Fields_SearchConfig_TextSpec {
        FirestoreIndex_Fields_SearchConfig_TextSpec {
            index_specs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirestoreIndex_Fields_SearchConfig_TextSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreIndex_Fields_SearchConfig_TextSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirestoreIndex_Fields_SearchConfig_TextSpecRef {
        FirestoreIndex_Fields_SearchConfig_TextSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreIndex_Fields_SearchConfig_TextSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `index_specs` after provisioning.\n"]
    pub fn index_specs(&self) -> terrars::ListRef<FirestoreIndex_Fields_SearchConfig_TextSpec_IndexSpecsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_specs", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirestoreIndex_Fields_SearchConfigDynamic {
    geo_spec: std::option::Option<terrars::DynamicBlock<FirestoreIndex_Fields_SearchConfig_GeoSpec>>,
    text_spec: std::option::Option<terrars::DynamicBlock<FirestoreIndex_Fields_SearchConfig_TextSpec>>,
}

#[derive(Serialize)]
pub struct FirestoreIndex_Fields_SearchConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_spec: std::option::Option<std::vec::Vec<FirestoreIndex_Fields_SearchConfig_GeoSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_spec: std::option::Option<std::vec::Vec<FirestoreIndex_Fields_SearchConfig_TextSpec>>,
    dynamic: FirestoreIndex_Fields_SearchConfigDynamic,
}

impl FirestoreIndex_Fields_SearchConfig {
    #[doc = "Set the field `geo_spec`.\n"]
    pub fn set_geo_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirestoreIndex_Fields_SearchConfig_GeoSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_spec`.\n"]
    pub fn set_text_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirestoreIndex_Fields_SearchConfig_TextSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FirestoreIndex_Fields_SearchConfig {
    type O = terrars::BlockAssignable<FirestoreIndex_Fields_SearchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreIndex_Fields_SearchConfig {}

impl BuildFirestoreIndex_Fields_SearchConfig {
    pub fn build(self) -> FirestoreIndex_Fields_SearchConfig {
        FirestoreIndex_Fields_SearchConfig {
            geo_spec: core::default::Default::default(),
            text_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirestoreIndex_Fields_SearchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreIndex_Fields_SearchConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirestoreIndex_Fields_SearchConfigRef {
        FirestoreIndex_Fields_SearchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreIndex_Fields_SearchConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `geo_spec` after provisioning.\n"]
    pub fn geo_spec(&self) -> terrars::ListRef<FirestoreIndex_Fields_SearchConfig_GeoSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `text_spec` after provisioning.\n"]
    pub fn text_spec(&self) -> terrars::ListRef<FirestoreIndex_Fields_SearchConfig_TextSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct FirestoreIndex_Fields_VectorConfig_Flat {}

impl FirestoreIndex_Fields_VectorConfig_Flat { }

impl terrars::ToListMappable for FirestoreIndex_Fields_VectorConfig_Flat {
    type O = terrars::BlockAssignable<FirestoreIndex_Fields_VectorConfig_Flat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreIndex_Fields_VectorConfig_Flat {}

impl BuildFirestoreIndex_Fields_VectorConfig_Flat {
    pub fn build(self) -> FirestoreIndex_Fields_VectorConfig_Flat {
        FirestoreIndex_Fields_VectorConfig_Flat {}
    }
}

#[allow(dead_code)]
pub struct FirestoreIndex_Fields_VectorConfig_FlatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreIndex_Fields_VectorConfig_FlatRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirestoreIndex_Fields_VectorConfig_FlatRef {
        FirestoreIndex_Fields_VectorConfig_FlatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreIndex_Fields_VectorConfig_FlatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct FirestoreIndex_Fields_VectorConfigDynamic {
    flat: std::option::Option<terrars::DynamicBlock<FirestoreIndex_Fields_VectorConfig_Flat>>,
}

#[derive(Serialize)]
pub struct FirestoreIndex_Fields_VectorConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flat: std::option::Option<std::vec::Vec<FirestoreIndex_Fields_VectorConfig_Flat>>,
    dynamic: FirestoreIndex_Fields_VectorConfigDynamic,
}

impl FirestoreIndex_Fields_VectorConfig {
    #[doc =
        "Set the field `dimension`.\nThe resulting index will only include vectors of this dimension, and can be used for vector search\nwith the same dimension."]
    pub fn set_dimension(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flat`.\n"]
    pub fn set_flat(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirestoreIndex_Fields_VectorConfig_Flat>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.flat = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.flat = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FirestoreIndex_Fields_VectorConfig {
    type O = terrars::BlockAssignable<FirestoreIndex_Fields_VectorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreIndex_Fields_VectorConfig {}

impl BuildFirestoreIndex_Fields_VectorConfig {
    pub fn build(self) -> FirestoreIndex_Fields_VectorConfig {
        FirestoreIndex_Fields_VectorConfig {
            dimension: core::default::Default::default(),
            flat: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirestoreIndex_Fields_VectorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreIndex_Fields_VectorConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirestoreIndex_Fields_VectorConfigRef {
        FirestoreIndex_Fields_VectorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreIndex_Fields_VectorConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dimension` after provisioning.\nThe resulting index will only include vectors of this dimension, and can be used for vector search\nwith the same dimension."]
    pub fn dimension(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `flat` after provisioning.\n"]
    pub fn flat(&self) -> terrars::ListRef<FirestoreIndex_Fields_VectorConfig_FlatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flat", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirestoreIndex_FieldsDynamic {
    search_config: std::option::Option<terrars::DynamicBlock<FirestoreIndex_Fields_SearchConfig>>,
    vector_config: std::option::Option<terrars::DynamicBlock<FirestoreIndex_Fields_VectorConfig>>,
}

#[derive(Serialize)]
pub struct FirestoreIndex_Fields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    array_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search_config: std::option::Option<std::vec::Vec<FirestoreIndex_Fields_SearchConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_config: std::option::Option<std::vec::Vec<FirestoreIndex_Fields_VectorConfig>>,
    dynamic: FirestoreIndex_FieldsDynamic,
}

impl FirestoreIndex_Fields {
    #[doc =
        "Set the field `array_config`.\nIndicates that this field supports operations on arrayValues. Only one of 'order', 'arrayConfig', 'searchConfig' and\n'vectorConfig' can be specified. Possible values: [\"CONTAINS\"]"]
    pub fn set_array_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.array_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_path`.\nName of the field."]
    pub fn set_field_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nIndicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=.\nOnly one of 'order', 'arrayConfig', 'searchConfig' and 'vectorConfig' can be specified. Possible values: [\"ASCENDING\", \"DESCENDING\"]"]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `search_config`.\n"]
    pub fn set_search_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirestoreIndex_Fields_SearchConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.search_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.search_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vector_config`.\n"]
    pub fn set_vector_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirestoreIndex_Fields_VectorConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vector_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vector_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FirestoreIndex_Fields {
    type O = terrars::BlockAssignable<FirestoreIndex_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreIndex_Fields {}

impl BuildFirestoreIndex_Fields {
    pub fn build(self) -> FirestoreIndex_Fields {
        FirestoreIndex_Fields {
            array_config: core::default::Default::default(),
            field_path: core::default::Default::default(),
            order: core::default::Default::default(),
            search_config: core::default::Default::default(),
            vector_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirestoreIndex_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreIndex_FieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirestoreIndex_FieldsRef {
        FirestoreIndex_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreIndex_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `array_config` after provisioning.\nIndicates that this field supports operations on arrayValues. Only one of 'order', 'arrayConfig', 'searchConfig' and\n'vectorConfig' can be specified. Possible values: [\"CONTAINS\"]"]
    pub fn array_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.array_config", self.base))
    }

    #[doc = "Get a reference to the value of field `field_path` after provisioning.\nName of the field."]
    pub fn field_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nIndicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=.\nOnly one of 'order', 'arrayConfig', 'searchConfig' and 'vectorConfig' can be specified. Possible values: [\"ASCENDING\", \"DESCENDING\"]"]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `search_config` after provisioning.\n"]
    pub fn search_config(&self) -> terrars::ListRef<FirestoreIndex_Fields_SearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search_config", self.base))
    }

    #[doc = "Get a reference to the value of field `vector_config` after provisioning.\n"]
    pub fn vector_config(&self) -> terrars::ListRef<FirestoreIndex_Fields_VectorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vector_config", self.base))
    }
}

#[derive(Serialize)]
pub struct FirestoreIndex_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirestoreIndex_Timeouts {
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

impl terrars::ToListMappable for FirestoreIndex_Timeouts {
    type O = terrars::BlockAssignable<FirestoreIndex_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreIndex_Timeouts {}

impl BuildFirestoreIndex_Timeouts {
    pub fn build(self) -> FirestoreIndex_Timeouts {
        FirestoreIndex_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirestoreIndex_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirestoreIndex_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirestoreIndex_TimeoutsRef {
        FirestoreIndex_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreIndex_TimeoutsRef {
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
