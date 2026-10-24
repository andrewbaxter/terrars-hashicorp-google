use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerServicePerimeterIngressPolicyData {
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
    ingress_from: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicyIngressFromEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_to: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicyIngressToEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicyDynamic,
}

struct AccessContextManagerServicePerimeterIngressPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccessContextManagerServicePerimeterIngressPolicyData>,
}

#[derive(Clone)]
pub struct AccessContextManagerServicePerimeterIngressPolicy(
    std::rc::Rc<AccessContextManagerServicePerimeterIngressPolicy_>,
);

impl AccessContextManagerServicePerimeterIngressPolicy {
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

    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressFromEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingress_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingress_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressToEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingress_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingress_to = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl std::convert::Into<AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl>,
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

    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AccessContextManagerServicePerimeterIngressPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AccessContextManagerServicePerimeterIngressPolicy { }

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicy {
    type O = terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AccessContextManagerServicePerimeterIngressPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_access_context_manager_service_perimeter_ingress_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the Service Perimeter to add this resource to."]
    pub perimeter: terrars::PrimField<std::string::String>,
}

impl BuildAccessContextManagerServicePerimeterIngressPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> AccessContextManagerServicePerimeterIngressPolicy {
        let out =
            AccessContextManagerServicePerimeterIngressPolicy(
                std::rc::Rc::new(AccessContextManagerServicePerimeterIngressPolicy_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(AccessContextManagerServicePerimeterIngressPolicyData {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        id: core::default::Default::default(),
                        perimeter: self.perimeter,
                        ingress_from: core::default::Default::default(),
                        ingress_to: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerServicePerimeterIngressPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicyRef {
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

    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(&self) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {}

impl BuildAccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
        AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef {
        AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef {
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
struct AccessContextManagerServicePerimeterIngressPolicyIngressFromElDynamic {
    sources: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std::option::Option<std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl>>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicyIngressFromElDynamic,
}

impl AccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
    #[doc =
        "Set the field `identities`.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn set_identities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.identities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
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
                        ::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesEl>,
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressFromEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicyIngressFromEl {}

impl BuildAccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
        AccessContextManagerServicePerimeterIngressPolicyIngressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef {
        AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicyIngressFromElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn identities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressFromElSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl {
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
::ToListMappable for AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl {
    type O =
        terrars
        ::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl {}

impl BuildAccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl {
        AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef {
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
struct AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElDynamic {
    method_selectors: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_selectors: std
    ::option
    ::Option<
        std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
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
                            AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsEl,
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {}

impl BuildAccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
        AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef {
        AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef {
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
    ::ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElMethodSelectorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterIngressPolicyIngressToElDynamic {
    operations: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std
    ::option
    ::Option<std::vec::Vec<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl>>,
    dynamic: AccessContextManagerServicePerimeterIngressPolicyIngressToElDynamic,
}

impl AccessContextManagerServicePerimeterIngressPolicyIngressToEl {
    #[doc =
        "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
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
                        ::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsEl>,
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicyIngressToEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyIngressToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicyIngressToEl {}

impl BuildAccessContextManagerServicePerimeterIngressPolicyIngressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyIngressToEl {
        AccessContextManagerServicePerimeterIngressPolicyIngressToEl {
            resources: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterIngressPolicyIngressToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicyIngressToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyIngressToElRef {
        AccessContextManagerServicePerimeterIngressPolicyIngressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicyIngressToElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> terrars::ListRef<AccessContextManagerServicePerimeterIngressPolicyIngressToElOperationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
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

impl terrars::ToListMappable for AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {}

impl BuildAccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
        AccessContextManagerServicePerimeterIngressPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterIngressPolicyTimeoutsElRef {
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
struct AccessContextManagerServicePerimeterIngressPolicyDynamic {
    ingress_from: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicyIngressFromEl>>,
    ingress_to: std
    ::option
    ::Option<terrars::DynamicBlock<AccessContextManagerServicePerimeterIngressPolicyIngressToEl>>,
}
