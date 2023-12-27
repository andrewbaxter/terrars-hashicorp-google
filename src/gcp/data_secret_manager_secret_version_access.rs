use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataSecretManagerSecretVersionAccessData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    secret: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

struct DataSecretManagerSecretVersionAccess_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSecretManagerSecretVersionAccessData>,
}

#[derive(Clone)]
pub struct DataSecretManagerSecretVersionAccess(Rc<DataSecretManagerSecretVersionAccess_>);

impl DataSecretManagerSecretVersionAccess {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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

    #[doc= "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_data` after provisioning.\n"]
    pub fn secret_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl Referable for DataSecretManagerSecretVersionAccess {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSecretManagerSecretVersionAccess { }

impl ToListMappable for DataSecretManagerSecretVersionAccess {
    type O = ListRef<DataSecretManagerSecretVersionAccessRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSecretManagerSecretVersionAccess_ {
    fn extract_datasource_type(&self) -> String {
        "google_secret_manager_secret_version_access".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecretManagerSecretVersionAccess {
    pub tf_id: String,
    #[doc= ""]
    pub secret: PrimField<String>,
}

impl BuildDataSecretManagerSecretVersionAccess {
    pub fn build(self, stack: &mut Stack) -> DataSecretManagerSecretVersionAccess {
        let out = DataSecretManagerSecretVersionAccess(Rc::new(DataSecretManagerSecretVersionAccess_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSecretManagerSecretVersionAccessData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                secret: self.secret,
                version: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecretManagerSecretVersionAccessRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSecretManagerSecretVersionAccessRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSecretManagerSecretVersionAccessRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret_data` after provisioning.\n"]
    pub fn secret_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}
