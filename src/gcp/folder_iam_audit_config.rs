use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FolderIamAuditConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    folder: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_log_config: Option<Vec<FolderIamAuditConfigAuditLogConfigEl>>,
    dynamic: FolderIamAuditConfigDynamic,
}

struct FolderIamAuditConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FolderIamAuditConfigData>,
}

#[derive(Clone)]
pub struct FolderIamAuditConfig(Rc<FolderIamAuditConfig_>);

impl FolderIamAuditConfig {
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

    #[doc= "Set the field `audit_log_config`.\n"]
    pub fn set_audit_log_config(self, v: impl Into<BlockAssignable<FolderIamAuditConfigAuditLogConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audit_log_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audit_log_config = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nThe etag of iam policy"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder` after provisioning.\n"]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\nService which will be enabled for audit logging. The special value allServices covers all services."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }
}

impl Referable for FolderIamAuditConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for FolderIamAuditConfig { }

impl ToListMappable for FolderIamAuditConfig {
    type O = ListRef<FolderIamAuditConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for FolderIamAuditConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_folder_iam_audit_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFolderIamAuditConfig {
    pub tf_id: String,
    #[doc= ""]
    pub folder: PrimField<String>,
    #[doc= "Service which will be enabled for audit logging. The special value allServices covers all services."]
    pub service: PrimField<String>,
}

impl BuildFolderIamAuditConfig {
    pub fn build(self, stack: &mut Stack) -> FolderIamAuditConfig {
        let out = FolderIamAuditConfig(Rc::new(FolderIamAuditConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FolderIamAuditConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                folder: self.folder,
                id: core::default::Default::default(),
                service: self.service,
                audit_log_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FolderIamAuditConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for FolderIamAuditConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FolderIamAuditConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\nThe etag of iam policy"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `folder` after provisioning.\n"]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.folder", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\nService which will be enabled for audit logging. The special value allServices covers all services."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FolderIamAuditConfigAuditLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exempted_members: Option<SetField<PrimField<String>>>,
    log_type: PrimField<String>,
}

impl FolderIamAuditConfigAuditLogConfigEl {
    #[doc= "Set the field `exempted_members`.\nIdentities that do not cause logging for this type of permission. Each entry can have one of the following values:user:{emailid}: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com. serviceAccount:{emailid}: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com. group:{emailid}: An email address that represents a Google group. For example, admins@example.com. domain:{domain}: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com."]
    pub fn set_exempted_members(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exempted_members = Some(v.into());
        self
    }
}

impl ToListMappable for FolderIamAuditConfigAuditLogConfigEl {
    type O = BlockAssignable<FolderIamAuditConfigAuditLogConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFolderIamAuditConfigAuditLogConfigEl {
    #[doc= "Permission type for which logging is to be configured. Must be one of DATA_READ, DATA_WRITE, or ADMIN_READ."]
    pub log_type: PrimField<String>,
}

impl BuildFolderIamAuditConfigAuditLogConfigEl {
    pub fn build(self) -> FolderIamAuditConfigAuditLogConfigEl {
        FolderIamAuditConfigAuditLogConfigEl {
            exempted_members: core::default::Default::default(),
            log_type: self.log_type,
        }
    }
}

pub struct FolderIamAuditConfigAuditLogConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FolderIamAuditConfigAuditLogConfigElRef {
    fn new(shared: StackShared, base: String) -> FolderIamAuditConfigAuditLogConfigElRef {
        FolderIamAuditConfigAuditLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FolderIamAuditConfigAuditLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exempted_members` after provisioning.\nIdentities that do not cause logging for this type of permission. Each entry can have one of the following values:user:{emailid}: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com. serviceAccount:{emailid}: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com. group:{emailid}: An email address that represents a Google group. For example, admins@example.com. domain:{domain}: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com."]
    pub fn exempted_members(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exempted_members", self.base))
    }

    #[doc= "Get a reference to the value of field `log_type` after provisioning.\nPermission type for which logging is to be configured. Must be one of DATA_READ, DATA_WRITE, or ADMIN_READ."]
    pub fn log_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct FolderIamAuditConfigDynamic {
    audit_log_config: Option<DynamicBlock<FolderIamAuditConfigAuditLogConfigEl>>,
}
