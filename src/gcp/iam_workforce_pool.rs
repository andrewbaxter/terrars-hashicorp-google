use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IamWorkforcePoolData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    workforce_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_restrictions: std::option::Option<std::vec::Vec<IamWorkforcePoolAccessRestrictionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IamWorkforcePoolTimeoutsEl>,
    dynamic: IamWorkforcePoolDynamic,
}

struct IamWorkforcePool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IamWorkforcePoolData>,
}

#[derive(Clone)]
pub struct IamWorkforcePool(std::rc::Rc<IamWorkforcePool_>);

impl IamWorkforcePool {
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

    #[doc = "Set the field `description`.\nA user-specified description of the pool. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens,\nor use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nA user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_duration`.\nDuration that the Google Cloud access tokens, console sign-in sessions,\nand 'gcloud' sign-in sessions from this pool are valid.\nMust be greater than 15 minutes (900s) and less than 12 hours (43200s).\nIf 'sessionDuration' is not configured, minted credentials have a default duration of one hour (3600s).\nA duration in seconds with up to nine fractional digits, ending with ''s''. Example: \"'3.5s'\"."]
    pub fn set_session_duration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().session_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_restrictions`.\n"]
    pub fn set_access_restrictions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkforcePoolAccessRestrictionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_restrictions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_restrictions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IamWorkforcePoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the pool. Cannot exceed 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens,\nor use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the pool.\nFormat: 'locations/{location}/workforcePools/{workforcePoolId}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nImmutable. The resource name of the parent. Format: 'organizations/{org-id}'."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nDuration that the Google Cloud access tokens, console sign-in sessions,\nand 'gcloud' sign-in sessions from this pool are valid.\nMust be greater than 15 minutes (900s) and less than 12 hours (43200s).\nIf 'sessionDuration' is not configured, minted credentials have a default duration of one hour (3600s).\nA duration in seconds with up to nine fractional digits, ending with ''s''. Example: \"'3.5s'\"."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the pool.\n * STATE_UNSPECIFIED: State unspecified.\n * ACTIVE: The pool is active, and may be used in Google Cloud policies.\n * DELETED: The pool is soft-deleted. Soft-deleted pools are permanently deleted\n   after approximately 30 days. You can restore a soft-deleted pool using\n   [workforcePools.undelete](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools/undelete#google.iam.admin.v1.WorkforcePools.UndeleteWorkforcePool).\n   You cannot reuse the ID of a soft-deleted pool until it is permanently deleted.\n   While a pool is deleted, you cannot use it to exchange tokens, or use\n   existing tokens to access resources. If the pool is undeleted, existing\n   tokens grant access again."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe name of the pool. The ID must be a globally unique string of 6 to 63 lowercase letters,\ndigits, or hyphens. It must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workforce_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workforce_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_restrictions` after provisioning.\n"]
    pub fn access_restrictions(&self) -> terrars::ListRef<IamWorkforcePoolAccessRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolTimeoutsElRef {
        IamWorkforcePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for IamWorkforcePool {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IamWorkforcePool { }

impl terrars::ToListMappable for IamWorkforcePool {
    type O = terrars::ListRef<IamWorkforcePoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IamWorkforcePool_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_iam_workforce_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamWorkforcePool {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Immutable. The resource name of the parent. Format: 'organizations/{org-id}'."]
    pub parent: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the pool. The ID must be a globally unique string of 6 to 63 lowercase letters,\ndigits, or hyphens. It must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub workforce_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkforcePool {
    pub fn build(self, stack: &mut terrars::Stack) -> IamWorkforcePool {
        let out = IamWorkforcePool(std::rc::Rc::new(IamWorkforcePool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IamWorkforcePoolData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
                session_duration: core::default::Default::default(),
                workforce_pool_id: self.workforce_pool_id,
                access_restrictions: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamWorkforcePoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamWorkforcePoolRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-specified description of the pool. Cannot exceed 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens,\nor use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-specified display name of the pool in Google Cloud Console. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. The resource name of the pool.\nFormat: 'locations/{location}/workforcePools/{workforcePoolId}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nImmutable. The resource name of the parent. Format: 'organizations/{org-id}'."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `session_duration` after provisioning.\nDuration that the Google Cloud access tokens, console sign-in sessions,\nand 'gcloud' sign-in sessions from this pool are valid.\nMust be greater than 15 minutes (900s) and less than 12 hours (43200s).\nIf 'sessionDuration' is not configured, minted credentials have a default duration of one hour (3600s).\nA duration in seconds with up to nine fractional digits, ending with ''s''. Example: \"'3.5s'\"."]
    pub fn session_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of the pool.\n * STATE_UNSPECIFIED: State unspecified.\n * ACTIVE: The pool is active, and may be used in Google Cloud policies.\n * DELETED: The pool is soft-deleted. Soft-deleted pools are permanently deleted\n   after approximately 30 days. You can restore a soft-deleted pool using\n   [workforcePools.undelete](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools/undelete#google.iam.admin.v1.WorkforcePools.UndeleteWorkforcePool).\n   You cannot reuse the ID of a soft-deleted pool until it is permanently deleted.\n   While a pool is deleted, you cannot use it to exchange tokens, or use\n   existing tokens to access resources. If the pool is undeleted, existing\n   tokens grant access again."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workforce_pool_id` after provisioning.\nThe name of the pool. The ID must be a globally unique string of 6 to 63 lowercase letters,\ndigits, or hyphens. It must start with a letter, and cannot have a trailing hyphen.\nThe prefix 'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workforce_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workforce_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_restrictions` after provisioning.\n"]
    pub fn access_restrictions(&self) -> terrars::ListRef<IamWorkforcePoolAccessRestrictionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_restrictions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkforcePoolTimeoutsElRef {
        IamWorkforcePoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
    #[doc = "Set the field `domain`.\nDomain name of the service.\nExample: console.cloud.google"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
    type O = terrars::BlockAssignable<IamWorkforcePoolAccessRestrictionsElAllowedServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolAccessRestrictionsElAllowedServicesEl {}

impl BuildIamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
    pub fn build(self) -> IamWorkforcePoolAccessRestrictionsElAllowedServicesEl {
        IamWorkforcePoolAccessRestrictionsElAllowedServicesEl { domain: core::default::Default::default() }
    }
}

pub struct IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef {
        IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nDomain name of the service.\nExample: console.cloud.google"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamWorkforcePoolAccessRestrictionsElDynamic {
    allowed_services: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkforcePoolAccessRestrictionsElAllowedServicesEl>>,
}

#[derive(Serialize)]
pub struct IamWorkforcePoolAccessRestrictionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_programmatic_signin: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_services: std::option::Option<std::vec::Vec<IamWorkforcePoolAccessRestrictionsElAllowedServicesEl>>,
    dynamic: IamWorkforcePoolAccessRestrictionsElDynamic,
}

impl IamWorkforcePoolAccessRestrictionsEl {
    #[doc =
        "Set the field `disable_programmatic_signin`.\nDisable programmatic sign-in by disabling token issue via the Security Token API endpoint.\nSee [Security Token Service API](https://cloud.google.com/iam/docs/reference/sts/rest)."]
    pub fn set_disable_programmatic_signin(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_programmatic_signin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_services`.\n"]
    pub fn set_allowed_services(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamWorkforcePoolAccessRestrictionsElAllowedServicesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamWorkforcePoolAccessRestrictionsEl {
    type O = terrars::BlockAssignable<IamWorkforcePoolAccessRestrictionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolAccessRestrictionsEl {}

impl BuildIamWorkforcePoolAccessRestrictionsEl {
    pub fn build(self) -> IamWorkforcePoolAccessRestrictionsEl {
        IamWorkforcePoolAccessRestrictionsEl {
            disable_programmatic_signin: core::default::Default::default(),
            allowed_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IamWorkforcePoolAccessRestrictionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolAccessRestrictionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkforcePoolAccessRestrictionsElRef {
        IamWorkforcePoolAccessRestrictionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolAccessRestrictionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_programmatic_signin` after provisioning.\nDisable programmatic sign-in by disabling token issue via the Security Token API endpoint.\nSee [Security Token Service API](https://cloud.google.com/iam/docs/reference/sts/rest)."]
    pub fn disable_programmatic_signin(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_programmatic_signin", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_services` after provisioning.\n"]
    pub fn allowed_services(&self) -> terrars::ListRef<IamWorkforcePoolAccessRestrictionsElAllowedServicesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_services", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkforcePoolTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkforcePoolTimeoutsEl {
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

impl terrars::ToListMappable for IamWorkforcePoolTimeoutsEl {
    type O = terrars::BlockAssignable<IamWorkforcePoolTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkforcePoolTimeoutsEl {}

impl BuildIamWorkforcePoolTimeoutsEl {
    pub fn build(self) -> IamWorkforcePoolTimeoutsEl {
        IamWorkforcePoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IamWorkforcePoolTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkforcePoolTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamWorkforcePoolTimeoutsElRef {
        IamWorkforcePoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkforcePoolTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct IamWorkforcePoolDynamic {
    access_restrictions: std::option::Option<terrars::DynamicBlock<IamWorkforcePoolAccessRestrictionsEl>>,
}
