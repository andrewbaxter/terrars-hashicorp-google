use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeEnvKeystoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    env_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeEnvKeystoreTimeoutsEl>,
}

struct ApigeeEnvKeystore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeEnvKeystoreData>,
}

#[derive(Clone)]
pub struct ApigeeEnvKeystore(Rc<ApigeeEnvKeystore_>);

impl ApigeeEnvKeystore {
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

    #[doc= "Set the field `name`.\nThe name of the newly created keystore."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeEnvKeystoreTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `aliases` after provisioning.\nAliases in this keystore."]
    pub fn aliases(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.aliases", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `env_id` after provisioning.\nThe Apigee environment group associated with the Apigee environment,\nin the format 'organizations/{{org_name}}/environments/{{env_name}}'."]
    pub fn env_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.env_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the newly created keystore."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeEnvKeystoreTimeoutsElRef {
        ApigeeEnvKeystoreTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ApigeeEnvKeystore {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApigeeEnvKeystore { }

impl ToListMappable for ApigeeEnvKeystore {
    type O = ListRef<ApigeeEnvKeystoreRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApigeeEnvKeystore_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_env_keystore".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeEnvKeystore {
    pub tf_id: String,
    #[doc= "The Apigee environment group associated with the Apigee environment,\nin the format 'organizations/{{org_name}}/environments/{{env_name}}'."]
    pub env_id: PrimField<String>,
}

impl BuildApigeeEnvKeystore {
    pub fn build(self, stack: &mut Stack) -> ApigeeEnvKeystore {
        let out = ApigeeEnvKeystore(Rc::new(ApigeeEnvKeystore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeEnvKeystoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                env_id: self.env_id,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeEnvKeystoreRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeEnvKeystoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeEnvKeystoreRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `aliases` after provisioning.\nAliases in this keystore."]
    pub fn aliases(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.aliases", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `env_id` after provisioning.\nThe Apigee environment group associated with the Apigee environment,\nin the format 'organizations/{{org_name}}/environments/{{env_name}}'."]
    pub fn env_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.env_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the newly created keystore."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeEnvKeystoreTimeoutsElRef {
        ApigeeEnvKeystoreTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApigeeEnvKeystoreTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl ApigeeEnvKeystoreTimeoutsEl {
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

impl ToListMappable for ApigeeEnvKeystoreTimeoutsEl {
    type O = BlockAssignable<ApigeeEnvKeystoreTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeEnvKeystoreTimeoutsEl {}

impl BuildApigeeEnvKeystoreTimeoutsEl {
    pub fn build(self) -> ApigeeEnvKeystoreTimeoutsEl {
        ApigeeEnvKeystoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ApigeeEnvKeystoreTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeEnvKeystoreTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeEnvKeystoreTimeoutsElRef {
        ApigeeEnvKeystoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeEnvKeystoreTimeoutsElRef {
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
