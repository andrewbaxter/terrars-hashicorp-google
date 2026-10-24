use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerServicePerimetersData {
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
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_perimeters: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AccessContextManagerServicePerimetersTimeoutsEl>,
    dynamic: AccessContextManagerServicePerimetersDynamic,
}

struct AccessContextManagerServicePerimeters_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessContextManagerServicePerimetersData>,
}

#[derive(Clone)]
pub struct AccessContextManagerServicePerimeters(std::rc::Rc<AccessContextManagerServicePerimeters_>);

impl AccessContextManagerServicePerimeters {
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

    #[doc = "Set the field `service_perimeters`.\n"]
    pub fn set_service_perimeters(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_perimeters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_perimeters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AccessContextManagerServicePerimetersTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_perimeters` after provisioning.\n"]
    pub fn service_perimeters(&self) -> terrars::ListRef<AccessContextManagerServicePerimetersServicePerimetersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_perimeters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimetersTimeoutsElRef {
        AccessContextManagerServicePerimetersTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AccessContextManagerServicePerimeters {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessContextManagerServicePerimeters { }

impl terrars::ToListMappable for AccessContextManagerServicePerimeters {
    type O = terrars::ListRef<AccessContextManagerServicePerimetersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessContextManagerServicePerimeters_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_access_context_manager_service_perimeters".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerServicePerimeters {
    pub tf_id: std::string::String,
    #[doc = "The AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerServicePerimeters {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessContextManagerServicePerimeters {
        let out = AccessContextManagerServicePerimeters(std::rc::Rc::new(AccessContextManagerServicePerimeters_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccessContextManagerServicePerimetersData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                service_perimeters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerServicePerimetersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerServicePerimetersRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_perimeters` after provisioning.\n"]
    pub fn service_perimeters(&self) -> terrars::ListRef<AccessContextManagerServicePerimetersServicePerimetersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_perimeters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimetersTimeoutsElRef {
        AccessContextManagerServicePerimetersTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl {
    #[doc =
        "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl {
            access_level: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_level` after provisioning.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn access_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElDynamic {
    sources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_restriction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_restriction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            source_restriction: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_restriction", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    #[doc =
        "Set the field `method`.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
        >,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl {
    #[doc =
        "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElDynamic {
    operations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl {
    #[doc =
        "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.external_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl {
            external_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.external_resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElOperationsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElDynamic {
    egress_from: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl>,
    >,
    egress_to: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_from: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_to: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl>>,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {
    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl {
            egress_from: core::default::Default::default(),
            egress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElEgressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_to", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl {
    #[doc =
        "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_level` after provisioning.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn access_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElDynamic {
    sources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElSourcesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    #[doc =
        "Set the field `method`.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
        >,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl {
    #[doc =
        "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElDynamic {
    operations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl {
    #[doc =
        "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl {
            resources: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElOperationsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElDynamic {
    ingress_from: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl>,
    >,
    ingress_to: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_from: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_to: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {
    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl {
            ingress_from: core::default::Default::default(),
            ingress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElIngressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_restriction: std::option::Option<terrars::PrimField<bool>>,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
    #[doc =
        "Set the field `allowed_services`.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn set_allowed_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_services = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_restriction`.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn set_enable_restriction(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_restriction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl {
            allowed_services: core::default::Default::default(),
            enable_restriction: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_services` after provisioning.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn allowed_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_services", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_restriction` after provisioning.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn enable_restriction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_restriction", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElSpecElDynamic {
    egress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl>>,
    ingress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl>>,
    vpc_accessible_services: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_levels: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_policies: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_policies: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_accessible_services: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl>>,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElSpecElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecEl {
    #[doc =
        "Set the field `access_levels`.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn set_access_levels(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restricted_services`.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn set_restricted_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.restricted_services = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress_policies`.\n"]
    pub fn set_egress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_policies`.\n"]
    pub fn set_ingress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_accessible_services`.\n"]
    pub fn set_vpc_accessible_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_accessible_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_accessible_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElSpecEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElSpecEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElSpecEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElSpecEl {
        AccessContextManagerServicePerimetersServicePerimetersElSpecEl {
            access_levels: core::default::Default::default(),
            resources: core::default::Default::default(),
            restricted_services: core::default::Default::default(),
            egress_policies: core::default::Default::default(),
            ingress_policies: core::default::Default::default(),
            vpc_accessible_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElSpecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElSpecElRef {
        AccessContextManagerServicePerimetersServicePerimetersElSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_levels` after provisioning.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn access_levels(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access_levels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restricted_services` after provisioning.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn restricted_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.restricted_services", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_policies` after provisioning.\n"]
    pub fn egress_policies(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElEgressPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_policies` after provisioning.\n"]
    pub fn ingress_policies(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElIngressPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_accessible_services` after provisioning.\n"]
    pub fn vpc_accessible_services(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElVpcAccessibleServicesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_accessible_services", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl {
    #[doc =
        "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl {
            access_level: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_level` after provisioning.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn access_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElDynamic {
    sources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_restriction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_restriction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            source_restriction: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_restriction", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElSourcesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    #[doc =
        "Set the field `method`.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
        >,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl {
    #[doc =
        "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElDynamic {
    operations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl {
    #[doc =
        "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.external_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl {
            external_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.external_resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElOperationsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElDynamic {
    egress_from: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl>,
    >,
    egress_to: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_from: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_to: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl {
            egress_from: core::default::Default::default(),
            egress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElEgressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_to", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl {
    #[doc =
        "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_level` after provisioning.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn access_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElDynamic {
    sources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn identities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElSourcesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    #[doc =
        "Set the field `method`.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
        >,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl {
    #[doc =
        "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method_selectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElDynamic {
    operations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl,
        >,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl {
    #[doc =
        "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl {
    type O =
        terrars
        ::BlockAssignable<
            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl {
            resources: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars
    ::ListRef<
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElOperationsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElDynamic {
    ingress_from: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl>,
    >,
    ingress_to: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_from: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_to: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl>,
    >,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_to = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
    type O =
        terrars::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl {
            ingress_from: core::default::Default::default(),
            ingress_to: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(
        &self,
    ) -> terrars
    ::ListRef<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesElIngressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_restriction: std::option::Option<terrars::PrimField<bool>>,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {
    #[doc =
        "Set the field `allowed_services`.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn set_allowed_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_services = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_restriction`.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn set_enable_restriction(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_restriction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl {
            allowed_services: core::default::Default::default(),
            enable_restriction: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_services` after provisioning.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn allowed_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_services", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_restriction` after provisioning.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn enable_restriction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_restriction", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElStatusElDynamic {
    egress_policies: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl>>,
    ingress_policies: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl>,
    >,
    vpc_accessible_services: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_levels: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_policies: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_policies: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_accessible_services: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl>>,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElStatusElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusEl {
    #[doc =
        "Set the field `access_levels`.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn set_access_levels(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access_levels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restricted_services`.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn set_restricted_services(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.restricted_services = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress_policies`.\n"]
    pub fn set_egress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_policies`.\n"]
    pub fn set_ingress_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElIngressPoliciesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingress_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_accessible_services`.\n"]
    pub fn set_vpc_accessible_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_accessible_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_accessible_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersElStatusEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersElStatusEl {}

impl BuildAccessContextManagerServicePerimetersServicePerimetersElStatusEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersElStatusEl {
        AccessContextManagerServicePerimetersServicePerimetersElStatusEl {
            access_levels: core::default::Default::default(),
            resources: core::default::Default::default(),
            restricted_services: core::default::Default::default(),
            egress_policies: core::default::Default::default(),
            ingress_policies: core::default::Default::default(),
            vpc_accessible_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElStatusElRef {
        AccessContextManagerServicePerimetersServicePerimetersElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_levels` after provisioning.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn access_levels(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access_levels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restricted_services` after provisioning.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn restricted_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.restricted_services", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_policies` after provisioning.\n"]
    pub fn egress_policies(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimetersServicePerimetersElStatusElEgressPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_accessible_services` after provisioning.\n"]
    pub fn vpc_accessible_services(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimetersServicePerimetersElStatusElVpcAccessibleServicesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_accessible_services", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimetersServicePerimetersElDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElSpecEl>>,
    status: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersElStatusEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersServicePerimetersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    perimeter_type: std::option::Option<terrars::PrimField<std::string::String>>,
    title: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_explicit_dry_run_spec: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElSpecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimetersServicePerimetersElStatusEl>>,
    dynamic: AccessContextManagerServicePerimetersServicePerimetersElDynamic,
}

impl AccessContextManagerServicePerimetersServicePerimetersEl {
    #[doc =
        "Set the field `description`.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `perimeter_type`.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn set_perimeter_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.perimeter_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_explicit_dry_run_spec`.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn set_use_explicit_dry_run_spec(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_explicit_dry_run_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElSpecEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersElStatusEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.status = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimetersServicePerimetersEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimetersServicePerimetersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersServicePerimetersEl {
    #[doc =
        "Resource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerServicePerimetersServicePerimetersEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersServicePerimetersEl {
        AccessContextManagerServicePerimetersServicePerimetersEl {
            description: core::default::Default::default(),
            name: self.name,
            perimeter_type: core::default::Default::default(),
            title: self.title,
            use_explicit_dry_run_spec: core::default::Default::default(),
            spec: core::default::Default::default(),
            status: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersServicePerimetersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersServicePerimetersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersServicePerimetersElRef {
        AccessContextManagerServicePerimetersServicePerimetersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersServicePerimetersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the AccessPolicy was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `perimeter_type` after provisioning.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn perimeter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.perimeter_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the AccessPolicy was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_explicit_dry_run_spec` after provisioning.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn use_explicit_dry_run_spec(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_explicit_dry_run_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AccessContextManagerServicePerimetersServicePerimetersElSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::ListRef<AccessContextManagerServicePerimetersServicePerimetersElStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimetersTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimetersTimeoutsEl {
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

impl terrars::ToListMappable for AccessContextManagerServicePerimetersTimeoutsEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimetersTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimetersTimeoutsEl {}

impl BuildAccessContextManagerServicePerimetersTimeoutsEl {
    pub fn build(self) -> AccessContextManagerServicePerimetersTimeoutsEl {
        AccessContextManagerServicePerimetersTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimetersTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimetersTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimetersTimeoutsElRef {
        AccessContextManagerServicePerimetersTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimetersTimeoutsElRef {
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
struct AccessContextManagerServicePerimetersDynamic {
    service_perimeters: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimetersServicePerimetersEl>>,
}
