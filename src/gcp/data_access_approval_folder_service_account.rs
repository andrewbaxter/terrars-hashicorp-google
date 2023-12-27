use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataAccessApprovalFolderServiceAccountData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    folder_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataAccessApprovalFolderServiceAccount_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAccessApprovalFolderServiceAccountData>,
}

#[derive(Clone)]
pub struct DataAccessApprovalFolderServiceAccount(Rc<DataAccessApprovalFolderServiceAccount_>);

impl DataAccessApprovalFolderServiceAccount {
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

    #[doc= "Get a reference to the value of field `account_email` after provisioning.\n"]
    pub fn account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder_id` after provisioning.\n"]
    pub fn folder_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}

impl Referable for DataAccessApprovalFolderServiceAccount {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataAccessApprovalFolderServiceAccount { }

impl ToListMappable for DataAccessApprovalFolderServiceAccount {
    type O = ListRef<DataAccessApprovalFolderServiceAccountRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAccessApprovalFolderServiceAccount_ {
    fn extract_datasource_type(&self) -> String {
        "google_access_approval_folder_service_account".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccessApprovalFolderServiceAccount {
    pub tf_id: String,
    #[doc= ""]
    pub folder_id: PrimField<String>,
}

impl BuildDataAccessApprovalFolderServiceAccount {
    pub fn build(self, stack: &mut Stack) -> DataAccessApprovalFolderServiceAccount {
        let out = DataAccessApprovalFolderServiceAccount(Rc::new(DataAccessApprovalFolderServiceAccount_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAccessApprovalFolderServiceAccountData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                folder_id: self.folder_id,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccessApprovalFolderServiceAccountRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAccessApprovalFolderServiceAccountRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccessApprovalFolderServiceAccountRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `account_email` after provisioning.\n"]
    pub fn account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_email", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder_id` after provisioning.\n"]
    pub fn folder_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }
}
