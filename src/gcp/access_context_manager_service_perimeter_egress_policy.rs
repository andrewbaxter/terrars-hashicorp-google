use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerServicePerimeterEgressPolicyData {
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
    perimeter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_from: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterEgressPolicyEgressFromEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_to: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterEgressPolicyEgressToEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl>,
    dynamic: AccessContextManagerServicePerimeterEgressPolicyDynamic,
}

struct AccessContextManagerServicePerimeterEgressPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessContextManagerServicePerimeterEgressPolicyData>,
}

#[derive(Clone)]
pub struct AccessContextManagerServicePerimeterEgressPolicy(
    std::rc::Rc<AccessContextManagerServicePerimeterEgressPolicy_>,
);

impl AccessContextManagerServicePerimeterEgressPolicy {
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

    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressFromEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().egress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.egress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressToEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().egress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.egress_to = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl std::convert::Into<AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `perimeter` after provisioning.\nThe name of the Service Perimeter to add this resource to."]
    pub fn perimeter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.perimeter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AccessContextManagerServicePerimeterEgressPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessContextManagerServicePerimeterEgressPolicy { }

impl terrars::ToListMappable for AccessContextManagerServicePerimeterEgressPolicy {
    type O = terrars::ListRef<AccessContextManagerServicePerimeterEgressPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessContextManagerServicePerimeterEgressPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_access_context_manager_service_perimeter_egress_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerServicePerimeterEgressPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the Service Perimeter to add this resource to."]
    pub perimeter: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerServicePerimeterEgressPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessContextManagerServicePerimeterEgressPolicy {
        let out =
            AccessContextManagerServicePerimeterEgressPolicy(
                std::rc::Rc::new(AccessContextManagerServicePerimeterEgressPolicy_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(AccessContextManagerServicePerimeterEgressPolicyData {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        id: core::default::Default::default(),
                        perimeter: self.perimeter,
                        egress_from: core::default::Default::default(),
                        egress_to: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterEgressPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyRef {
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
        "Get a reference to the value of field `perimeter` after provisioning.\nThe name of the Service Perimeter to add this resource to."]
    pub fn perimeter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.perimeter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl {
    #[doc =
        "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn set_access_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl {}

impl BuildAccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl {
        AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl {
            access_level: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef {
        AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef {
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
struct AccessContextManagerServicePerimeterEgressPolicyEgressFromElDynamic {
    sources: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyEgressFromEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_restriction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl>>,
    dynamic: AccessContextManagerServicePerimeterEgressPolicyEgressFromElDynamic,
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressFromEl {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
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
                        ::BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl>,
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterEgressPolicyEgressFromEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressFromEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterEgressPolicyEgressFromEl {}

impl BuildAccessContextManagerServicePerimeterEgressPolicyEgressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterEgressPolicyEgressFromEl {
        AccessContextManagerServicePerimeterEgressPolicyEgressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            source_restriction: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef {
        AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn identities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_restriction", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl {
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
::ToListMappable for AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl {}

impl BuildAccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl {
        AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef {
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
struct AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars::DynamicBlock<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl>>,
    dynamic: AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl {
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
                            AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl,
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl {}

impl BuildAccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl {
        AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef {
        AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef {
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
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterEgressPolicyEgressToElDynamic {
    operations: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_resources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl>>,
    dynamic: AccessContextManagerServicePerimeterEgressPolicyEgressToElDynamic,
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToEl {
    #[doc =
        "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.external_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
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
                        ::BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl>,
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterEgressPolicyEgressToEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterEgressPolicyEgressToEl {}

impl BuildAccessContextManagerServicePerimeterEgressPolicyEgressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterEgressPolicyEgressToEl {
        AccessContextManagerServicePerimeterEgressPolicyEgressToEl {
            external_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterEgressPolicyEgressToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterEgressPolicyEgressToElRef {
        AccessContextManagerServicePerimeterEgressPolicyEgressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_resources", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl {
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterEgressPolicyTimeoutsEl {}

impl BuildAccessContextManagerServicePerimeterEgressPolicyTimeoutsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl {
        AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
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
struct AccessContextManagerServicePerimeterEgressPolicyDynamic {
    egress_from: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterEgressPolicyEgressFromEl>>,
    egress_to: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterEgressPolicyEgressToEl>>,
}
