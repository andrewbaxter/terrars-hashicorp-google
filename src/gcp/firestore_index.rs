use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FirestoreIndexData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_scope: Option<PrimField<String>>,
    collection: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<FirestoreIndexFieldsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirestoreIndexTimeoutsEl>,
    dynamic: FirestoreIndexDynamic,
}

struct FirestoreIndex_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirestoreIndexData>,
}

#[derive(Clone)]
pub struct FirestoreIndex(Rc<FirestoreIndex_>);

impl FirestoreIndex {
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

    #[doc= "Set the field `api_scope`.\nThe API scope at which a query is run. Default value: \"ANY_API\" Possible values: [\"ANY_API\", \"DATASTORE_MODE_API\"]"]
    pub fn set_api_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_scope = Some(v.into());
        self
    }

    #[doc= "Set the field `database`.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn set_database(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `query_scope`.\nThe scope at which a query is run. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\", \"COLLECTION_RECURSIVE\"]"]
    pub fn set_query_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().query_scope = Some(v.into());
        self
    }

    #[doc= "Set the field `fields`.\n"]
    pub fn set_fields(self, v: impl Into<BlockAssignable<FirestoreIndexFieldsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fields = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fields = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirestoreIndexTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `api_scope` after provisioning.\nThe API scope at which a query is run. Default value: \"ANY_API\" Possible values: [\"ANY_API\", \"DATASTORE_MODE_API\"]"]
    pub fn api_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `collection` after provisioning.\nThe collection being indexed."]
    pub fn collection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA server defined name for this index. Format:\n'projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/indexes/{{server_generated_id}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `query_scope` after provisioning.\nThe scope at which a query is run. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\", \"COLLECTION_RECURSIVE\"]"]
    pub fn query_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> ListRef<FirestoreIndexFieldsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreIndexTimeoutsElRef {
        FirestoreIndexTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for FirestoreIndex {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FirestoreIndex { }

impl ToListMappable for FirestoreIndex {
    type O = ListRef<FirestoreIndexRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FirestoreIndex_ {
    fn extract_resource_type(&self) -> String {
        "google_firestore_index".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFirestoreIndex {
    pub tf_id: String,
    #[doc= "The collection being indexed."]
    pub collection: PrimField<String>,
}

impl BuildFirestoreIndex {
    pub fn build(self, stack: &mut Stack) -> FirestoreIndex {
        let out = FirestoreIndex(Rc::new(FirestoreIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirestoreIndexData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_scope: core::default::Default::default(),
                collection: self.collection,
                database: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                query_scope: core::default::Default::default(),
                fields: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FirestoreIndexRef {
    shared: StackShared,
    base: String,
}

impl Ref for FirestoreIndexRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FirestoreIndexRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_scope` after provisioning.\nThe API scope at which a query is run. Default value: \"ANY_API\" Possible values: [\"ANY_API\", \"DATASTORE_MODE_API\"]"]
    pub fn api_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `collection` after provisioning.\nThe collection being indexed."]
    pub fn collection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\nThe Firestore database id. Defaults to '\"(default)\"'."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nA server defined name for this index. Format:\n'projects/{{project}}/databases/{{database}}/collectionGroups/{{collection}}/indexes/{{server_generated_id}}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `query_scope` after provisioning.\nThe scope at which a query is run. Default value: \"COLLECTION\" Possible values: [\"COLLECTION\", \"COLLECTION_GROUP\", \"COLLECTION_RECURSIVE\"]"]
    pub fn query_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query_scope", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> ListRef<FirestoreIndexFieldsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fields", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirestoreIndexTimeoutsElRef {
        FirestoreIndexTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FirestoreIndexFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    array_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<String>>,
}

impl FirestoreIndexFieldsEl {
    #[doc= "Set the field `array_config`.\nIndicates that this field supports operations on arrayValues. Only one of 'order' and 'arrayConfig' can\nbe specified. Possible values: [\"CONTAINS\"]"]
    pub fn set_array_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.array_config = Some(v.into());
        self
    }

    #[doc= "Set the field `field_path`.\nName of the field."]
    pub fn set_field_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_path = Some(v.into());
        self
    }

    #[doc= "Set the field `order`.\nIndicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=.\nOnly one of 'order' and 'arrayConfig' can be specified. Possible values: [\"ASCENDING\", \"DESCENDING\"]"]
    pub fn set_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.order = Some(v.into());
        self
    }
}

impl ToListMappable for FirestoreIndexFieldsEl {
    type O = BlockAssignable<FirestoreIndexFieldsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreIndexFieldsEl {}

impl BuildFirestoreIndexFieldsEl {
    pub fn build(self) -> FirestoreIndexFieldsEl {
        FirestoreIndexFieldsEl {
            array_config: core::default::Default::default(),
            field_path: core::default::Default::default(),
            order: core::default::Default::default(),
        }
    }
}

pub struct FirestoreIndexFieldsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FirestoreIndexFieldsElRef {
    fn new(shared: StackShared, base: String) -> FirestoreIndexFieldsElRef {
        FirestoreIndexFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreIndexFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `array_config` after provisioning.\nIndicates that this field supports operations on arrayValues. Only one of 'order' and 'arrayConfig' can\nbe specified. Possible values: [\"CONTAINS\"]"]
    pub fn array_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.array_config", self.base))
    }

    #[doc= "Get a reference to the value of field `field_path` after provisioning.\nName of the field."]
    pub fn field_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_path", self.base))
    }

    #[doc= "Get a reference to the value of field `order` after provisioning.\nIndicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=.\nOnly one of 'order' and 'arrayConfig' can be specified. Possible values: [\"ASCENDING\", \"DESCENDING\"]"]
    pub fn order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
}

#[derive(Serialize)]
pub struct FirestoreIndexTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl FirestoreIndexTimeoutsEl {
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
}

impl ToListMappable for FirestoreIndexTimeoutsEl {
    type O = BlockAssignable<FirestoreIndexTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirestoreIndexTimeoutsEl {}

impl BuildFirestoreIndexTimeoutsEl {
    pub fn build(self) -> FirestoreIndexTimeoutsEl {
        FirestoreIndexTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct FirestoreIndexTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FirestoreIndexTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirestoreIndexTimeoutsElRef {
        FirestoreIndexTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirestoreIndexTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct FirestoreIndexDynamic {
    fields: Option<DynamicBlock<FirestoreIndexFieldsEl>>,
}
