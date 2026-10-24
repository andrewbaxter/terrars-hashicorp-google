use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OrganizationIamAuditConfigData {
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
    org_id: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_log_config: std::option::Option<std::vec::Vec<OrganizationIamAuditConfigAuditLogConfigEl>>,
    dynamic: OrganizationIamAuditConfigDynamic,
}

struct OrganizationIamAuditConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OrganizationIamAuditConfigData>,
}

#[derive(Clone)]
pub struct OrganizationIamAuditConfig(std::rc::Rc<OrganizationIamAuditConfig_>);

impl OrganizationIamAuditConfig {
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

    #[doc = "Set the field `audit_log_config`.\n"]
    pub fn set_audit_log_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OrganizationIamAuditConfigAuditLogConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audit_log_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audit_log_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag of iam policy"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe numeric ID of the organization in which you want to manage the audit logging config."]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nService which will be enabled for audit logging. The special value allServices covers all services."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }
}

impl terrars::Referable for OrganizationIamAuditConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OrganizationIamAuditConfig { }

impl terrars::ToListMappable for OrganizationIamAuditConfig {
    type O = terrars::ListRef<OrganizationIamAuditConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OrganizationIamAuditConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_organization_iam_audit_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOrganizationIamAuditConfig {
    pub tf_id: std::string::String,
    #[doc = "The numeric ID of the organization in which you want to manage the audit logging config."]
    pub org_id: terrars::PrimField<std::string::String>,
    #[doc = "Service which will be enabled for audit logging. The special value allServices covers all services."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildOrganizationIamAuditConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> OrganizationIamAuditConfig {
        let out = OrganizationIamAuditConfig(std::rc::Rc::new(OrganizationIamAuditConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OrganizationIamAuditConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                org_id: self.org_id,
                service: self.service,
                audit_log_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OrganizationIamAuditConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrganizationIamAuditConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OrganizationIamAuditConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag of iam policy"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe numeric ID of the organization in which you want to manage the audit logging config."]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nService which will be enabled for audit logging. The special value allServices covers all services."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OrganizationIamAuditConfigAuditLogConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exempted_members: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    log_type: terrars::PrimField<std::string::String>,
}

impl OrganizationIamAuditConfigAuditLogConfigEl {
    #[doc =
        "Set the field `exempted_members`.\nIdentities that do not cause logging for this type of permission. Each entry can have one of the following values:user:{emailid}: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com. serviceAccount:{emailid}: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com. group:{emailid}: An email address that represents a Google group. For example, admins@example.com. domain:{domain}: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com."]
    pub fn set_exempted_members(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exempted_members = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OrganizationIamAuditConfigAuditLogConfigEl {
    type O = terrars::BlockAssignable<OrganizationIamAuditConfigAuditLogConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrganizationIamAuditConfigAuditLogConfigEl {
    #[doc =
        "Permission type for which logging is to be configured. Must be one of DATA_READ, DATA_WRITE, or ADMIN_READ."]
    pub log_type: terrars::PrimField<std::string::String>,
}

impl BuildOrganizationIamAuditConfigAuditLogConfigEl {
    pub fn build(self) -> OrganizationIamAuditConfigAuditLogConfigEl {
        OrganizationIamAuditConfigAuditLogConfigEl {
            exempted_members: core::default::Default::default(),
            log_type: self.log_type,
        }
    }
}

pub struct OrganizationIamAuditConfigAuditLogConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrganizationIamAuditConfigAuditLogConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OrganizationIamAuditConfigAuditLogConfigElRef {
        OrganizationIamAuditConfigAuditLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrganizationIamAuditConfigAuditLogConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exempted_members` after provisioning.\nIdentities that do not cause logging for this type of permission. Each entry can have one of the following values:user:{emailid}: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com. serviceAccount:{emailid}: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com. group:{emailid}: An email address that represents a Google group. For example, admins@example.com. domain:{domain}: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com."]
    pub fn exempted_members(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.exempted_members", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_type` after provisioning.\nPermission type for which logging is to be configured. Must be one of DATA_READ, DATA_WRITE, or ADMIN_READ."]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct OrganizationIamAuditConfigDynamic {
    audit_log_config: std::option::Option<terrars::DynamicBlock<OrganizationIamAuditConfigAuditLogConfigEl>>,
}
