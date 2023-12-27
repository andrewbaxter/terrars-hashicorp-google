use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerServicePerimeterEgressPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    perimeter: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_from: Option<Vec<AccessContextManagerServicePerimeterEgressPolicyEgressFromEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_to: Option<Vec<AccessContextManagerServicePerimeterEgressPolicyEgressToEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl>,
    dynamic: AccessContextManagerServicePerimeterEgressPolicyDynamic,
}

struct AccessContextManagerServicePerimeterEgressPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessContextManagerServicePerimeterEgressPolicyData>,
}

#[derive(Clone)]
pub struct AccessContextManagerServicePerimeterEgressPolicy(Rc<AccessContextManagerServicePerimeterEgressPolicy_>);

impl AccessContextManagerServicePerimeterEgressPolicy {
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

    #[doc= "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressFromEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().egress_from = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.egress_from = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressToEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().egress_to = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.egress_to = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `perimeter` after provisioning.\nThe name of the Service Perimeter to add this resource to."]
    pub fn perimeter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.perimeter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(&self) -> ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef> {
        ListRef::new(self.shared().clone(), format!("{}.egress_from", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(&self) -> ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.egress_to", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for AccessContextManagerServicePerimeterEgressPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AccessContextManagerServicePerimeterEgressPolicy { }

impl ToListMappable for AccessContextManagerServicePerimeterEgressPolicy {
    type O = ListRef<AccessContextManagerServicePerimeterEgressPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AccessContextManagerServicePerimeterEgressPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_access_context_manager_service_perimeter_egress_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerServicePerimeterEgressPolicy {
    pub tf_id: String,
    #[doc= "The name of the Service Perimeter to add this resource to."]
    pub perimeter: PrimField<String>,
}

impl BuildAccessContextManagerServicePerimeterEgressPolicy {
    pub fn build(self, stack: &mut Stack) -> AccessContextManagerServicePerimeterEgressPolicy {
        let out =
            AccessContextManagerServicePerimeterEgressPolicy(
                Rc::new(AccessContextManagerServicePerimeterEgressPolicy_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: RefCell::new(AccessContextManagerServicePerimeterEgressPolicyData {
                        depends_on: core::default::Default::default(),
                        provider: None,
                        lifecycle: core::default::Default::default(),
                        for_each: None,
                        id: core::default::Default::default(),
                        perimeter: self.perimeter,
                        egress_from: core::default::Default::default(),
                        egress_to: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessContextManagerServicePerimeterEgressPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `perimeter` after provisioning.\nThe name of the Service Perimeter to add this resource to."]
    pub fn perimeter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.perimeter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(&self) -> ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef> {
        ListRef::new(self.shared().clone(), format!("{}.egress_from", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(&self) -> ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.egress_to", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl {
    #[doc= "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }
}

impl ToListMappable for AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef {
        AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_level` after provisioning.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterEgressPolicyEgressFromElDynamic {
    sources: Option<DynamicBlock<AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyEgressFromEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identities: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_restriction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<Vec<AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl>>,
    dynamic: AccessContextManagerServicePerimeterEgressPolicyEgressFromElDynamic,
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressFromEl {
    #[doc= "Set the field `identities`.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn set_identities(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.identities = Some(v.into());
        self
    }

    #[doc= "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }

    #[doc= "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_restriction = Some(v.into());
        self
    }

    #[doc= "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessContextManagerServicePerimeterEgressPolicyEgressFromEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressFromEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef {
        AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressFromElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn identities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }

    #[doc= "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.base))
    }

    #[doc= "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_restriction", self.base))
    }

    #[doc= "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressFromElSourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl {
    #[doc= "Set the field `method`.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
    pub fn set_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.method = Some(v.into());
        self
    }

    #[doc= "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn set_permission(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission = Some(v.into());
        self
    }
}

impl ToListMappable for AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl {
    type O =
        BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef {
        AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `method` after provisioning.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."]
    pub fn method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc= "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElDynamic {
    method_selectors: Option<
        DynamicBlock<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl>,
    >,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method_selectors: Option<
        Vec<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElDynamic,
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl {
    #[doc= "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }

    #[doc= "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.method_selectors = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef {
        AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc= "Get a reference to the value of field `method_selectors` after provisioning.\n"]
    pub fn method_selectors(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElMethodSelectorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.method_selectors", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterEgressPolicyEgressToElDynamic {
    operations: Option<DynamicBlock<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl>>,
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_resources: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operations: Option<Vec<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl>>,
    dynamic: AccessContextManagerServicePerimeterEgressPolicyEgressToElDynamic,
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToEl {
    #[doc= "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.external_resources = Some(v.into());
        self
    }

    #[doc= "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }

    #[doc= "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AccessContextManagerServicePerimeterEgressPolicyEgressToEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyEgressToEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct AccessContextManagerServicePerimeterEgressPolicyEgressToElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessContextManagerServicePerimeterEgressPolicyEgressToElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerServicePerimeterEgressPolicyEgressToElRef {
        AccessContextManagerServicePerimeterEgressPolicyEgressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyEgressToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.external_resources", self.base))
    }

    #[doc= "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc= "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(&self) -> ListRef<AccessContextManagerServicePerimeterEgressPolicyEgressToElOperationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl {
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

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterEgressPolicyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
        AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerServicePerimeterEgressPolicyTimeoutsElRef {
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

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterEgressPolicyDynamic {
    egress_from: Option<DynamicBlock<AccessContextManagerServicePerimeterEgressPolicyEgressFromEl>>,
    egress_to: Option<DynamicBlock<AccessContextManagerServicePerimeterEgressPolicyEgressToEl>>,
}
