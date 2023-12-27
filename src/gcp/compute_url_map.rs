use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeUrlMapData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_route_action: Option<Vec<ComputeUrlMapDefaultRouteActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_url_redirect: Option<Vec<ComputeUrlMapDefaultUrlRedirectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeUrlMapHeaderActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rule: Option<Vec<ComputeUrlMapHostRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_matcher: Option<Vec<ComputeUrlMapPathMatcherEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test: Option<Vec<ComputeUrlMapTestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeUrlMapTimeoutsEl>,
    dynamic: ComputeUrlMapDynamic,
}

struct ComputeUrlMap_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeUrlMapData>,
}

#[derive(Clone)]
pub struct ComputeUrlMap(Rc<ComputeUrlMap_>);

impl ComputeUrlMap {
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

    #[doc= "Set the field `default_service`.\nThe backend service or backend bucket to use when none of the given rules match."]
    pub fn set_default_service(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_service = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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

    #[doc= "Set the field `default_route_action`.\n"]
    pub fn set_default_route_action(self, v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_route_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_route_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_url_redirect`.\n"]
    pub fn set_default_url_redirect(self, v: impl Into<BlockAssignable<ComputeUrlMapDefaultUrlRedirectEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_url_redirect = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_url_redirect = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `header_action`.\n"]
    pub fn set_header_action(self, v: impl Into<BlockAssignable<ComputeUrlMapHeaderActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().header_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.header_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `host_rule`.\n"]
    pub fn set_host_rule(self, v: impl Into<BlockAssignable<ComputeUrlMapHostRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().host_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.host_rule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `path_matcher`.\n"]
    pub fn set_path_matcher(self, v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().path_matcher = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.path_matcher = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `test`.\n"]
    pub fn set_test(self, v: impl Into<BlockAssignable<ComputeUrlMapTestEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().test = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.test = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeUrlMapTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_service` after provisioning.\nThe backend service or backend bucket to use when none of the given rules match."]
    pub fn default_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this object. This\nfield is used in optimistic locking."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `map_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn map_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.map_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a lowercase\nletter, and all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> ListRef<ComputeUrlMapDefaultUrlRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeUrlMapHeaderActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> ListRef<ComputeUrlMapPathMatcherElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> ListRef<ComputeUrlMapTestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.test", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeUrlMapTimeoutsElRef {
        ComputeUrlMapTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ComputeUrlMap {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeUrlMap { }

impl ToListMappable for ComputeUrlMap {
    type O = ListRef<ComputeUrlMapRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeUrlMap_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_url_map".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeUrlMap {
    pub tf_id: String,
    #[doc= "Name of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a lowercase\nletter, and all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
}

impl BuildComputeUrlMap {
    pub fn build(self, stack: &mut Stack) -> ComputeUrlMap {
        let out = ComputeUrlMap(Rc::new(ComputeUrlMap_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeUrlMapData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_service: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                default_route_action: core::default::Default::default(),
                default_url_redirect: core::default::Default::default(),
                header_action: core::default::Default::default(),
                host_rule: core::default::Default::default(),
                path_matcher: core::default::Default::default(),
                test: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeUrlMapRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeUrlMapRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_service` after provisioning.\nThe backend service or backend bucket to use when none of the given rules match."]
    pub fn default_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this object. This\nfield is used in optimistic locking."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `map_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn map_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.map_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a lowercase\nletter, and all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> ListRef<ComputeUrlMapDefaultUrlRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeUrlMapHeaderActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> ListRef<ComputeUrlMapPathMatcherElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> ListRef<ComputeUrlMapTestElRef> {
        ListRef::new(self.shared().clone(), format!("{}.test", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeUrlMapTimeoutsElRef {
        ComputeUrlMapTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElCorsPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origin_regexes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<f64>>,
}

impl ComputeUrlMapDefaultRouteActionElCorsPolicyEl {
    #[doc= "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\nThis translates to the Access-Control-Allow-Credentials header."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn set_allow_origin_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origin_regexes = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\nSpecifies how long results of a preflight request can be cached in seconds.\nThis translates to the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElCorsPolicyEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElCorsPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElCorsPolicyEl {}

impl BuildComputeUrlMapDefaultRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElCorsPolicyEl {
        ComputeUrlMapDefaultRouteActionElCorsPolicyEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origin_regexes: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            disabled: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElCorsPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElCorsPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElCorsPolicyElRef {
        ComputeUrlMapDefaultRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\nThis translates to the Access-Control-Allow-Credentials header."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origin_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origin_regexes", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long results of a preflight request can be cached in seconds.\nThis translates to the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_status: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}

impl ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    #[doc= "Set the field `http_status`.\nThe HTTP status code used to abort the request.\nThe value must be between 200 and 599 inclusive."]
    pub fn set_http_status(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_status = Some(v.into());
        self
    }

    #[doc= "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {}

impl BuildComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
        ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
        ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request.\nThe value must be between 200 and 599 inclusive."]
    pub fn http_status(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }

    #[doc= "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}

impl ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }

    #[doc= "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {}

impl BuildComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
        ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
        ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElDynamic {
    fixed_delay: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_delay: Option<Vec<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>>,
    dynamic: ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElDynamic,
}

impl ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc= "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }

    #[doc= "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_delay = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {}

impl BuildComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
        ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
        ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }

    #[doc= "Get a reference to the value of field `fixed_delay` after provisioning.\n"]
    pub fn fixed_delay(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDynamic {
    abort: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>>,
    delay: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort: Option<Vec<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<Vec<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>>,
    dynamic: ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDynamic,
}

impl ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    #[doc= "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.abort = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.abort = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `delay`.\n"]
    pub fn set_delay(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.delay = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.delay = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl {}

impl BuildComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
        ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
        ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc= "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    backend_service: PrimField<String>,
}

impl ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl { }

impl ToListMappable for ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    #[doc= "The full or partial URL to the BackendService resource being mirrored to."]
    pub backend_service: PrimField<String>,
}

impl BuildComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
        ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl { backend_service: self.backend_service }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
        ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the BackendService resource being mirrored to."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}

impl ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }

    #[doc= "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {}

impl BuildComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
        ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapDefaultRouteActionElRetryPolicyElDynamic {
    per_try_timeout: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_try_timeout: Option<Vec<ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    dynamic: ComputeUrlMapDefaultRouteActionElRetryPolicyElDynamic,
}

impl ComputeUrlMapDefaultRouteActionElRetryPolicyEl {
    #[doc= "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn set_num_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_retries = Some(v.into());
        self
    }

    #[doc= "Set the field `retry_conditions`.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with any 5xx response code,\n  or if the backend service does not respond at all, example: disconnects, reset, read timeout,\n* connection failure, and refused streams.\n* gateway-error: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures connecting to backend services,\n  for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream:Loadbalancer will retry if the backend service resets the stream with a REFUSED_STREAM error code.\n  This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in the response header is set to unavailable"]
    pub fn set_retry_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.retry_conditions = Some(v.into());
        self
    }

    #[doc= "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_try_timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElRetryPolicyEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElRetryPolicyEl {}

impl BuildComputeUrlMapDefaultRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElRetryPolicyEl {
        ComputeUrlMapDefaultRouteActionElRetryPolicyEl {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElRetryPolicyElRef {
        ComputeUrlMapDefaultRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with any 5xx response code,\n  or if the backend service does not respond at all, example: disconnects, reset, read timeout,\n* connection failure, and refused streams.\n* gateway-error: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures connecting to backend services,\n  for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream:Loadbalancer will retry if the backend service resets the stream with a REFUSED_STREAM error code.\n  This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc= "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}

impl ComputeUrlMapDefaultRouteActionElTimeoutEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }

    #[doc= "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElTimeoutEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElTimeoutEl {}

impl BuildComputeUrlMapDefaultRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElTimeoutEl {
        ComputeUrlMapDefaultRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElTimeoutElRef {
        ComputeUrlMapDefaultRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix_rewrite: Option<PrimField<String>>,
}

impl ComputeUrlMapDefaultRouteActionElUrlRewriteEl {
    #[doc= "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host header is replaced\nwith contents of hostRewrite.\n\nThe value must be between 1 and 255 characters."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }

    #[doc= "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching portion of the\nrequest's path is replaced by pathPrefixRewrite.\n\nThe value must be between 1 and 1024 characters."]
    pub fn set_path_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_prefix_rewrite = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElUrlRewriteEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElUrlRewriteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElUrlRewriteEl {}

impl BuildComputeUrlMapDefaultRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElUrlRewriteEl {
        ComputeUrlMapDefaultRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElUrlRewriteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElUrlRewriteElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElUrlRewriteElRef {
        ComputeUrlMapDefaultRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host header is replaced\nwith contents of hostRewrite.\n\nThe value must be between 1 and 255 characters."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching portion of the\nrequest's path is replaced by pathPrefixRewrite.\n\nThe value must be between 1 and 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}

impl ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    #[doc= "Set the field `header_name`.\nThe name of the header to add."]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }

    #[doc= "Set the field `header_value`.\nThe value of the header to add."]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }

    #[doc= "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn set_replace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.replace = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    type O =
        BlockAssignable<
            ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {}

impl BuildComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
        ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
        ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}

impl ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    #[doc= "Set the field `header_name`.\nThe name of the header to add."]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }

    #[doc= "Set the field `header_value`.\nThe value of the header to add."]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }

    #[doc= "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn set_replace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.replace = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    type O =
        BlockAssignable<
            ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {}

impl BuildComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
        ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
        ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElDynamic {
    request_headers_to_add: Option<
        DynamicBlock<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl>,
    >,
    response_headers_to_add: Option<
        DynamicBlock<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add: Option<
        Vec<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_add: Option<
        Vec<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl>,
    >,
    dynamic: ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElDynamic,
}

impl ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[doc= "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request prior to\nforwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response prior to sending the\nresponse back to the client."]
    pub fn set_response_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {}

impl BuildComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
        ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
        ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request prior to\nforwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response prior to sending the\nresponse back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> ListRef<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> ListRef<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElDynamic {
    header_action: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>>,
    dynamic: ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElDynamic,
}

impl ComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    #[doc= "Set the field `backend_service`.\nThe full or partial URL to the default BackendService resource. Before forwarding the\nrequest to backendService, the loadbalancer applies any relevant headerActions\nspecified as part of this backendServiceWeight."]
    pub fn set_backend_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backend_service = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\nSpecifies the fraction of traffic sent to backendService, computed as\nweight / (sum of all weightedBackendService weights in routeAction) .\n\nThe selection of a backend service is determined only for new traffic. Once a user's request\nhas been directed to a backendService, subsequent requests will be sent to the same backendService\nas determined by the BackendService's session affinity policy.\n\nThe value must be between 0 and 1000"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }

    #[doc= "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl {}

impl BuildComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl {
        ComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl {
            backend_service: core::default::Default::default(),
            weight: core::default::Default::default(),
            header_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
        ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the default BackendService resource. Before forwarding the\nrequest to backendService, the loadbalancer applies any relevant headerActions\nspecified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as\nweight / (sum of all weightedBackendService weights in routeAction) .\n\nThe selection of a backend service is determined only for new traffic. Once a user's request\nhas been directed to a backendService, subsequent requests will be sent to the same backendService\nas determined by the BackendService's session affinity policy.\n\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc= "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapDefaultRouteActionElDynamic {
    cors_policy: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElCorsPolicyEl>>,
    fault_injection_policy: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl>>,
    request_mirror_policy: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl>>,
    retry_policy: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElRetryPolicyEl>>,
    timeout: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElTimeoutEl>>,
    url_rewrite: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElUrlRewriteEl>>,
    weighted_backend_services: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultRouteActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_policy: Option<Vec<ComputeUrlMapDefaultRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_injection_policy: Option<Vec<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_mirror_policy: Option<Vec<ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<ComputeUrlMapDefaultRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<ComputeUrlMapDefaultRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite: Option<Vec<ComputeUrlMapDefaultRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_backend_services: Option<Vec<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl>>,
    dynamic: ComputeUrlMapDefaultRouteActionElDynamic,
}

impl ComputeUrlMapDefaultRouteActionEl {
    #[doc= "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElCorsPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cors_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fault_injection_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fault_injection_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_mirror_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_mirror_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElTimeoutEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElUrlRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_rewrite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_backend_services = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultRouteActionEl {
    type O = BlockAssignable<ComputeUrlMapDefaultRouteActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultRouteActionEl {}

impl BuildComputeUrlMapDefaultRouteActionEl {
    pub fn build(self) -> ComputeUrlMapDefaultRouteActionEl {
        ComputeUrlMapDefaultRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapDefaultRouteActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultRouteActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultRouteActionElRef {
        ComputeUrlMapDefaultRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultRouteActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElCorsPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElFaultInjectionPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElRequestMirrorPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElUrlRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(&self) -> ListRef<ComputeUrlMapDefaultRouteActionElWeightedBackendServicesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapDefaultUrlRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_response_code: Option<PrimField<String>>,
    strip_query: PrimField<bool>,
}

impl ComputeUrlMapDefaultUrlRedirectEl {
    #[doc= "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn set_host_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.https_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn set_path_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_response_code = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapDefaultUrlRedirectEl {
    type O = BlockAssignable<ComputeUrlMapDefaultUrlRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapDefaultUrlRedirectEl {
    #[doc= "If set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained. The default is set to false.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: PrimField<bool>,
}

impl BuildComputeUrlMapDefaultUrlRedirectEl {
    pub fn build(self) -> ComputeUrlMapDefaultUrlRedirectEl {
        ComputeUrlMapDefaultUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}

pub struct ComputeUrlMapDefaultUrlRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapDefaultUrlRedirectElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapDefaultUrlRedirectElRef {
        ComputeUrlMapDefaultUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapDefaultUrlRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn host_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn https_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn path_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_response_code", self.base))
    }

    #[doc= "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained. The default is set to false.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapHeaderActionElRequestHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}

impl ComputeUrlMapHeaderActionElRequestHeadersToAddEl { }

impl ToListMappable for ComputeUrlMapHeaderActionElRequestHeadersToAddEl {
    type O = BlockAssignable<ComputeUrlMapHeaderActionElRequestHeadersToAddEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapHeaderActionElRequestHeadersToAddEl {
    #[doc= "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc= "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc= "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}

impl BuildComputeUrlMapHeaderActionElRequestHeadersToAddEl {
    pub fn build(self) -> ComputeUrlMapHeaderActionElRequestHeadersToAddEl {
        ComputeUrlMapHeaderActionElRequestHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeUrlMapHeaderActionElRequestHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapHeaderActionElRequestHeadersToAddElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapHeaderActionElRequestHeadersToAddElRef {
        ComputeUrlMapHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapHeaderActionElResponseHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}

impl ComputeUrlMapHeaderActionElResponseHeadersToAddEl { }

impl ToListMappable for ComputeUrlMapHeaderActionElResponseHeadersToAddEl {
    type O = BlockAssignable<ComputeUrlMapHeaderActionElResponseHeadersToAddEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapHeaderActionElResponseHeadersToAddEl {
    #[doc= "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc= "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc= "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}

impl BuildComputeUrlMapHeaderActionElResponseHeadersToAddEl {
    pub fn build(self) -> ComputeUrlMapHeaderActionElResponseHeadersToAddEl {
        ComputeUrlMapHeaderActionElResponseHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeUrlMapHeaderActionElResponseHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapHeaderActionElResponseHeadersToAddElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapHeaderActionElResponseHeadersToAddElRef {
        ComputeUrlMapHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapHeaderActionElDynamic {
    request_headers_to_add: Option<DynamicBlock<ComputeUrlMapHeaderActionElRequestHeadersToAddEl>>,
    response_headers_to_add: Option<DynamicBlock<ComputeUrlMapHeaderActionElResponseHeadersToAddEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add: Option<Vec<ComputeUrlMapHeaderActionElRequestHeadersToAddEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_add: Option<Vec<ComputeUrlMapHeaderActionElResponseHeadersToAddEl>>,
    dynamic: ComputeUrlMapHeaderActionElDynamic,
}

impl ComputeUrlMapHeaderActionEl {
    #[doc= "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapHeaderActionElRequestHeadersToAddEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapHeaderActionElResponseHeadersToAddEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapHeaderActionEl {
    type O = BlockAssignable<ComputeUrlMapHeaderActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapHeaderActionEl {}

impl BuildComputeUrlMapHeaderActionEl {
    pub fn build(self) -> ComputeUrlMapHeaderActionEl {
        ComputeUrlMapHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapHeaderActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapHeaderActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapHeaderActionElRef {
        ComputeUrlMapHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(&self) -> ListRef<ComputeUrlMapHeaderActionElRequestHeadersToAddElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(&self) -> ListRef<ComputeUrlMapHeaderActionElResponseHeadersToAddElRef> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapHostRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    hosts: SetField<PrimField<String>>,
    path_matcher: PrimField<String>,
}

impl ComputeUrlMapHostRuleEl {
    #[doc= "Set the field `description`.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapHostRuleEl {
    type O = BlockAssignable<ComputeUrlMapHostRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapHostRuleEl {
    #[doc= "The list of host patterns to match. They must be valid hostnames, except * will\nmatch any string of ([a-z0-9-.]*). In that case, * must be the first character\nand must be followed in the pattern by either - or .."]
    pub hosts: SetField<PrimField<String>>,
    #[doc= "The name of the PathMatcher to use to match the path portion of the URL if the\nhostRule matches the URL's host portion."]
    pub path_matcher: PrimField<String>,
}

impl BuildComputeUrlMapHostRuleEl {
    pub fn build(self) -> ComputeUrlMapHostRuleEl {
        ComputeUrlMapHostRuleEl {
            description: core::default::Default::default(),
            hosts: self.hosts,
            path_matcher: self.path_matcher,
        }
    }
}

pub struct ComputeUrlMapHostRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapHostRuleElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapHostRuleElRef {
        ComputeUrlMapHostRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapHostRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `hosts` after provisioning.\nThe list of host patterns to match. They must be valid hostnames, except * will\nmatch any string of ([a-z0-9-.]*). In that case, * must be the first character\nand must be followed in the pattern by either - or .."]
    pub fn hosts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc= "Get a reference to the value of field `path_matcher` after provisioning.\nThe name of the PathMatcher to use to match the path portion of the URL if the\nhostRule matches the URL's host portion."]
    pub fn path_matcher(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_matcher", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origin_regexes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<f64>>,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
    #[doc= "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\nThis translates to the Access-Control-Allow-Credentials header."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn set_allow_origin_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origin_regexes = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\nSpecifies how long results of a preflight request can be cached in seconds.\nThis translates to the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origin_regexes: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            disabled: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\nThis translates to the Access-Control-Allow-Credentials header."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origin_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origin_regexes", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long results of a preflight request can be cached in seconds.\nThis translates to the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_status: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    #[doc= "Set the field `http_status`.\nThe HTTP status code used to abort the request.\nThe value must be between 200 and 599 inclusive."]
    pub fn set_http_status(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_status = Some(v.into());
        self
    }

    #[doc= "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request.\nThe value must be between 200 and 599 inclusive."]
    pub fn http_status(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }

    #[doc= "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }

    #[doc= "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    type O =
        BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElDynamic {
    fixed_delay: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_delay: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>>,
    dynamic: ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElDynamic,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc= "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }

    #[doc= "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_delay = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }

    #[doc= "Get a reference to the value of field `fixed_delay` after provisioning.\n"]
    pub fn fixed_delay(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDynamic {
    abort: Option<DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl>>,
    delay: Option<DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl>>,
    dynamic: ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDynamic,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
    #[doc= "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.abort = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.abort = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `delay`.\n"]
    pub fn set_delay(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.delay = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.delay = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(&self) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc= "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
    backend_service: PrimField<String>,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
    #[doc= "The full or partial URL to the BackendService resource being mirrored to."]
    pub backend_service: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl { backend_service: self.backend_service }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the BackendService resource being mirrored to."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }

    #[doc= "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElDynamic {
    per_try_timeout: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_try_timeout: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    dynamic: ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElDynamic,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
    #[doc= "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn set_num_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_retries = Some(v.into());
        self
    }

    #[doc= "Set the field `retry_conditions`.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with any 5xx response code,\n  or if the backend service does not respond at all, example: disconnects, reset, read timeout,\n* connection failure, and refused streams.\n* gateway-error: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures connecting to backend services,\n  for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream:Loadbalancer will retry if the backend service resets the stream with a REFUSED_STREAM error code.\n  This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in the response header is set to unavailable"]
    pub fn set_retry_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.retry_conditions = Some(v.into());
        self
    }

    #[doc= "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_try_timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with any 5xx response code,\n  or if the backend service does not respond at all, example: disconnects, reset, read timeout,\n* connection failure, and refused streams.\n* gateway-error: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures connecting to backend services,\n  for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream:Loadbalancer will retry if the backend service resets the stream with a REFUSED_STREAM error code.\n  This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc= "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }

    #[doc= "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix_rewrite: Option<PrimField<String>>,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
    #[doc= "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host header is replaced\nwith contents of hostRewrite.\n\nThe value must be between 1 and 255 characters."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }

    #[doc= "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching portion of the\nrequest's path is replaced by pathPrefixRewrite.\n\nThe value must be between 1 and 1024 characters."]
    pub fn set_path_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_prefix_rewrite = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host header is replaced\nwith contents of hostRewrite.\n\nThe value must be between 1 and 255 characters."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching portion of the\nrequest's path is replaced by pathPrefixRewrite.\n\nThe value must be between 1 and 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    #[doc= "Set the field `header_name`.\nThe name of the header to add."]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }

    #[doc= "Set the field `header_value`.\nThe value of the header to add."]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }

    #[doc= "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn set_replace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.replace = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    type O =
        BlockAssignable<
            ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    #[doc= "Set the field `header_name`.\nThe name of the header to add."]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }

    #[doc= "Set the field `header_value`.\nThe value of the header to add."]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }

    #[doc= "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn set_replace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.replace = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    type O =
        BlockAssignable<
            ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElDynamic {
    request_headers_to_add: Option<
        DynamicBlock<
            ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    response_headers_to_add: Option<
        DynamicBlock<
            ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add: Option<
        Vec<
            ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_add: Option<
        Vec<
            ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
    dynamic: ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElDynamic,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[doc= "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request prior to\nforwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response prior to sending the\nresponse back to the client."]
    pub fn set_response_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request prior to\nforwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response prior to sending the\nresponse back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> ListRef<
        ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> ListRef<
        ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElDynamic {
    header_action: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>>,
    dynamic: ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElDynamic,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {
    #[doc= "Set the field `backend_service`.\nThe full or partial URL to the default BackendService resource. Before forwarding the\nrequest to backendService, the loadbalancer applies any relevant headerActions\nspecified as part of this backendServiceWeight."]
    pub fn set_backend_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backend_service = Some(v.into());
        self
    }

    #[doc= "Set the field `weight`.\nSpecifies the fraction of traffic sent to backendService, computed as\nweight / (sum of all weightedBackendService weights in routeAction) .\n\nThe selection of a backend service is determined only for new traffic. Once a user's request\nhas been directed to a backendService, subsequent requests will be sent to the same backendService\nas determined by the BackendService's session affinity policy.\n\nThe value must be between 0 and 1000"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }

    #[doc= "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {
            backend_service: core::default::Default::default(),
            weight: core::default::Default::default(),
            header_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the default BackendService resource. Before forwarding the\nrequest to backendService, the loadbalancer applies any relevant headerActions\nspecified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as\nweight / (sum of all weightedBackendService weights in routeAction) .\n\nThe selection of a backend service is determined only for new traffic. Once a user's request\nhas been directed to a backendService, subsequent requests will be sent to the same backendService\nas determined by the BackendService's session affinity policy.\n\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc= "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElDefaultRouteActionElDynamic {
    cors_policy: Option<DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl>>,
    fault_injection_policy: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl>,
    >,
    request_mirror_policy: Option<DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl>>,
    retry_policy: Option<DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl>>,
    timeout: Option<DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl>>,
    url_rewrite: Option<DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl>>,
    weighted_backend_services: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultRouteActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_policy: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_injection_policy: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_mirror_policy: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_backend_services: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl>>,
    dynamic: ComputeUrlMapPathMatcherElDefaultRouteActionElDynamic,
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionEl {
    #[doc= "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cors_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fault_injection_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fault_injection_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_mirror_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_mirror_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_rewrite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_backend_services = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultRouteActionEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultRouteActionEl {}

impl BuildComputeUrlMapPathMatcherElDefaultRouteActionEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultRouteActionEl {
        ComputeUrlMapPathMatcherElDefaultRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultRouteActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultRouteActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElDefaultRouteActionElRef {
        ComputeUrlMapPathMatcherElDefaultRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultRouteActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElDefaultUrlRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_response_code: Option<PrimField<String>>,
    strip_query: PrimField<bool>,
}

impl ComputeUrlMapPathMatcherElDefaultUrlRedirectEl {
    #[doc= "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn set_host_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.https_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn set_path_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_response_code = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElDefaultUrlRedirectEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElDefaultUrlRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElDefaultUrlRedirectEl {
    #[doc= "If set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElDefaultUrlRedirectEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElDefaultUrlRedirectEl {
        ComputeUrlMapPathMatcherElDefaultUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElDefaultUrlRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElDefaultUrlRedirectElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElDefaultUrlRedirectElRef {
        ComputeUrlMapPathMatcherElDefaultUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElDefaultUrlRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn host_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn https_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn path_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_response_code", self.base))
    }

    #[doc= "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}

impl ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
    #[doc= "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc= "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc= "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
        ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef {
        ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}

impl ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
    #[doc= "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc= "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc= "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
        ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef {
        ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElHeaderActionElDynamic {
    request_headers_to_add: Option<DynamicBlock<ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl>>,
    response_headers_to_add: Option<DynamicBlock<ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add: Option<Vec<ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_add: Option<Vec<ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl>>,
    dynamic: ComputeUrlMapPathMatcherElHeaderActionElDynamic,
}

impl ComputeUrlMapPathMatcherElHeaderActionEl {
    #[doc= "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElHeaderActionEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElHeaderActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElHeaderActionEl {}

impl BuildComputeUrlMapPathMatcherElHeaderActionEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElHeaderActionEl {
        ComputeUrlMapPathMatcherElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElHeaderActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElHeaderActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElHeaderActionElRef {
        ComputeUrlMapPathMatcherElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(&self) -> ListRef<ComputeUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(&self) -> ListRef<ComputeUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origin_regexes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<ListField<PrimField<String>>>,
    disabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<f64>>,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    #[doc= "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origin_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origin_regexes = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    #[doc= "If true, specifies the CORS policy is disabled."]
    pub disabled: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origin_regexes: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            disabled: self.disabled,
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origin_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origin_regexes", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
    http_status: PrimField<f64>,
    percentage: PrimField<f64>,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
    #[doc= "The HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub http_status: PrimField<f64>,
    #[doc= "The percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub percentage: PrimField<f64>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: self.http_status,
            percentage: self.percentage,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub fn http_status(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }

    #[doc= "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    type O =
        BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc= "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElDynamic {
    fixed_delay: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    percentage: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_delay: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>>,
    dynamic: ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElDynamic,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc= "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_delay = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc= "The percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub percentage: PrimField<f64>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: self.percentage,
            fixed_delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }

    #[doc= "Get a reference to the value of field `fixed_delay` after provisioning.\n"]
    pub fn fixed_delay(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDynamic {
    abort: Option<DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl>>,
    delay: Option<DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl>>,
    dynamic: ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDynamic,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    #[doc= "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.abort = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.abort = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `delay`.\n"]
    pub fn set_delay(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.delay = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.delay = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(&self) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc= "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    backend_service: PrimField<String>,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    #[doc= "The BackendService resource being mirrored to."]
    pub backend_service: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
            backend_service: self.backend_service,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backend_service` after provisioning.\nThe BackendService resource being mirrored to."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc= "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElDynamic {
    per_try_timeout: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_try_timeout: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    dynamic: ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElDynamic,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    #[doc= "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn set_num_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_retries = Some(v.into());
        self
    }

    #[doc= "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\nany 5xx response code, or if the backend service does not respond at all,\nexample: disconnects, reset, read timeout, connection failure, and refused\nstreams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\nconnecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\nCurrently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\nREFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\nheader is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\ngRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\nheader is set to resource-exhausted\n* unavailable: Loadbalancer will retry if\nthe gRPC status code in the response header is set to unavailable"]
    pub fn set_retry_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.retry_conditions = Some(v.into());
        self
    }

    #[doc= "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_try_timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\nany 5xx response code, or if the backend service does not respond at all,\nexample: disconnects, reset, read timeout, connection failure, and refused\nstreams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\nconnecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\nCurrently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\nREFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\nheader is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\ngRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\nheader is set to resource-exhausted\n* unavailable: Loadbalancer will retry if\nthe gRPC status code in the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc= "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    #[doc= "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix_rewrite: Option<PrimField<String>>,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    #[doc= "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }

    #[doc= "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn set_path_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_prefix_rewrite = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    type O =
        BlockAssignable<
            ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    #[doc= "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc= "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc= "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    type O =
        BlockAssignable<
            ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    #[doc= "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc= "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc= "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElDynamic {
    request_headers_to_add: Option<
        DynamicBlock<
            ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    response_headers_to_add: Option<
        DynamicBlock<
            ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add: Option<
        Vec<
            ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_add: Option<
        Vec<
            ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
    dynamic: ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElDynamic,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[doc= "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
    type O =
        BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> ListRef<
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> ListRef<
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElDynamic {
    header_action: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    backend_service: PrimField<String>,
    weight: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl>>,
    dynamic: ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElDynamic,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    #[doc= "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    #[doc= "The default BackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub backend_service: PrimField<String>,
    #[doc= "Specifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub weight: PrimField<f64>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
            backend_service: self.backend_service,
            weight: self.weight,
            header_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backend_service` after provisioning.\nThe default BackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc= "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElDynamic {
    cors_policy: Option<DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>>,
    fault_injection_policy: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl>,
    >,
    request_mirror_policy: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl>,
    >,
    retry_policy: Option<DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>>,
    timeout: Option<DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>>,
    url_rewrite: Option<DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>>,
    weighted_backend_services: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_policy: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_injection_policy: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_mirror_policy: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_backend_services: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl>>,
    dynamic: ComputeUrlMapPathMatcherElPathRuleElRouteActionElDynamic,
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionEl {
    #[doc= "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cors_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fault_injection_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fault_injection_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_mirror_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_mirror_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_rewrite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_backend_services = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElRouteActionEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElRouteActionEl {}

impl BuildComputeUrlMapPathMatcherElPathRuleElRouteActionEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionEl {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRouteActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRouteActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElPathRuleElRouteActionElRef {
        ComputeUrlMapPathMatcherElPathRuleElRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRouteActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_response_code: Option<PrimField<String>>,
    strip_query: PrimField<bool>,
}

impl ComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    #[doc= "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one\nthat was supplied in the request. The value must be between 1 and 255\ncharacters."]
    pub fn set_host_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https.\nIf set to false, the URL scheme of the redirected request will remain the\nsame as that of the request. This must only be set for UrlMaps used in\nTargetHttpProxys. Setting this true for TargetHttpsProxy is not\npermitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.https_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one\nthat was supplied in the request. pathRedirect cannot be supplied\ntogether with prefixRedirect. Supply one alone or neither. If neither is\nsupplied, the path of the original request will be used for the redirect.\nThe value must be between 1 and 1024 characters."]
    pub fn set_path_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the\nHttpRouteRuleMatch, retaining the remaining portion of the URL before\nredirecting the request. prefixRedirect cannot be supplied together with\npathRedirect. Supply one alone or neither. If neither is supplied, the\npath of the original request will be used for the redirect. The value\nmust be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_response_code = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    #[doc= "If set to true, any accompanying query portion of the original URL is\nremoved prior to redirecting the request. If set to false, the query\nportion of the original URL is retained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl {
        ComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
        ComputeUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one\nthat was supplied in the request. The value must be between 1 and 255\ncharacters."]
    pub fn host_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https.\nIf set to false, the URL scheme of the redirected request will remain the\nsame as that of the request. This must only be set for UrlMaps used in\nTargetHttpProxys. Setting this true for TargetHttpsProxy is not\npermitted. The default is set to false."]
    pub fn https_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one\nthat was supplied in the request. pathRedirect cannot be supplied\ntogether with prefixRedirect. Supply one alone or neither. If neither is\nsupplied, the path of the original request will be used for the redirect.\nThe value must be between 1 and 1024 characters."]
    pub fn path_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the\nHttpRouteRuleMatch, retaining the remaining portion of the URL before\nredirecting the request. prefixRedirect cannot be supplied together with\npathRedirect. Supply one alone or neither. If neither is supplied, the\npath of the original request will be used for the redirect. The value\nmust be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_response_code", self.base))
    }

    #[doc= "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is\nremoved prior to redirecting the request. If set to false, the query\nportion of the original URL is retained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElPathRuleElDynamic {
    route_action: Option<DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElRouteActionEl>>,
    url_redirect: Option<DynamicBlock<ComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElPathRuleEl {
    paths: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_action: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElRouteActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_redirect: Option<Vec<ComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl>>,
    dynamic: ComputeUrlMapPathMatcherElPathRuleElDynamic,
}

impl ComputeUrlMapPathMatcherElPathRuleEl {
    #[doc= "Set the field `service`.\nThe backend service or backend bucket to use if any of the given paths match."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }

    #[doc= "Set the field `route_action`.\n"]
    pub fn set_route_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElRouteActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.route_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.route_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `url_redirect`.\n"]
    pub fn set_url_redirect(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleElUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_redirect = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_redirect = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElPathRuleEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElPathRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElPathRuleEl {
    #[doc= "The list of path patterns to match. Each must start with / and the only place a\n\\* is allowed is at the end following a /. The string fed to the path matcher\ndoes not include any text after the first ? or #, and those chars are not\nallowed here."]
    pub paths: SetField<PrimField<String>>,
}

impl BuildComputeUrlMapPathMatcherElPathRuleEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElPathRuleEl {
        ComputeUrlMapPathMatcherElPathRuleEl {
            paths: self.paths,
            service: core::default::Default::default(),
            route_action: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElPathRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElPathRuleElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElPathRuleElRef {
        ComputeUrlMapPathMatcherElPathRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElPathRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `paths` after provisioning.\nThe list of path patterns to match. Each must start with / and the only place a\n\\* is allowed is at the end following a /. The string fed to the path matcher\ndoes not include any text after the first ? or #, and those chars are not\nallowed here."]
    pub fn paths(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\nThe backend service or backend bucket to use if any of the given paths match."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc= "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(&self) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRouteActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }

    #[doc= "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(&self) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElUrlRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    #[doc= "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc= "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc= "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
        ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
        ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    #[doc= "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc= "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc= "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
        ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
        ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElDynamic {
    request_headers_to_add: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl>,
    >,
    response_headers_to_add: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_add: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl>>,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    #[doc= "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl {}

impl BuildComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl {
        ComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
        ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
    range_end: PrimField<f64>,
    range_start: PrimField<f64>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
    #[doc= "The end of the range (exclusive)."]
    pub range_end: PrimField<f64>,
    #[doc= "The start of the range (inclusive)."]
    pub range_start: PrimField<f64>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
            range_end: self.range_end,
            range_start: self.range_start,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `range_end` after provisioning.\nThe end of the range (exclusive)."]
    pub fn range_end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_end", self.base))
    }

    #[doc= "Get a reference to the value of field `range_start` after provisioning.\nThe start of the range (inclusive)."]
    pub fn range_start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_start", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElDynamic {
    range_match: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_match: Option<PrimField<String>>,
    header_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    present_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_match: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl>>,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    #[doc= "Set the field `exact_match`.\nThe value should exactly match contents of exactMatch. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn set_exact_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact_match = Some(v.into());
        self
    }

    #[doc= "Set the field `invert_match`.\nIf set to false, the headerMatch is considered a match if the match criteria\nabove are met. If set to true, the headerMatch is considered a match if the\nmatch criteria above are NOT met. Defaults to false."]
    pub fn set_invert_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_match = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix_match`.\nThe value of the header must start with the contents of prefixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn set_prefix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_match = Some(v.into());
        self
    }

    #[doc= "Set the field `present_match`.\nA header with the contents of headerName must exist. The match takes place\nwhether or not the request's header has a value or not. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn set_present_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.present_match = Some(v.into());
        self
    }

    #[doc= "Set the field `regex_match`.\nThe value of the header must match the regular expression specified in\nregexMatch. For regular expression grammar, please see:\nen.cppreference.com/w/cpp/regex/ecmascript  For matching against a port\nspecified in the HTTP request, use a headerMatch with headerName set to PORT and\na regular expression that satisfies the RFC2616 Host header's port specifier.\nOnly one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or\nrangeMatch must be set."]
    pub fn set_regex_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex_match = Some(v.into());
        self
    }

    #[doc= "Set the field `suffix_match`.\nThe value of the header must end with the contents of suffixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn set_suffix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix_match = Some(v.into());
        self
    }

    #[doc= "Set the field `range_match`.\n"]
    pub fn set_range_match(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range_match = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range_match = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    #[doc= "The name of the HTTP header to match. For matching against the HTTP request's\nauthority, use a headerMatch with the header name \":authority\". For matching a\nrequest's method, use the headerName \":method\"."]
    pub header_name: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
            exact_match: core::default::Default::default(),
            header_name: self.header_name,
            invert_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            present_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
            suffix_match: core::default::Default::default(),
            range_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exact_match` after provisioning.\nThe value should exactly match contents of exactMatch. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn exact_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the HTTP header to match. For matching against the HTTP request's\nauthority, use a headerMatch with the header name \":authority\". For matching a\nrequest's method, use the headerName \":method\"."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `invert_match` after provisioning.\nIf set to false, the headerMatch is considered a match if the match criteria\nabove are met. If set to true, the headerMatch is considered a match if the\nmatch criteria above are NOT met. Defaults to false."]
    pub fn invert_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_match", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix_match` after provisioning.\nThe value of the header must start with the contents of prefixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn prefix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }

    #[doc= "Get a reference to the value of field `present_match` after provisioning.\nA header with the contents of headerName must exist. The match takes place\nwhether or not the request's header has a value or not. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn present_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.present_match", self.base))
    }

    #[doc= "Get a reference to the value of field `regex_match` after provisioning.\nThe value of the header must match the regular expression specified in\nregexMatch. For regular expression grammar, please see:\nen.cppreference.com/w/cpp/regex/ecmascript  For matching against a port\nspecified in the HTTP request, use a headerMatch with headerName set to PORT and\na regular expression that satisfies the RFC2616 Host header's port specifier.\nOnly one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or\nrangeMatch must be set."]
    pub fn regex_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }

    #[doc= "Get a reference to the value of field `suffix_match` after provisioning.\nThe value of the header must end with the contents of suffixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn suffix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix_match", self.base))
    }

    #[doc= "Get a reference to the value of field `range_match` after provisioning.\n"]
    pub fn range_match(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range_match", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
    name: PrimField<String>,
    value: PrimField<String>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
    #[doc= "Name of metadata label. The name can have a maximum length of 1024 characters\nand must be at least 1 character long."]
    pub name: PrimField<String>,
    #[doc= "The value of the label must match the specified value. value can have a maximum\nlength of 1024 characters."]
    pub value: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of metadata label. The name can have a maximum length of 1024 characters\nand must be at least 1 character long."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\nThe value of the label must match the specified value. value can have a maximum\nlength of 1024 characters."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElDynamic {
    filter_labels: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    filter_match_criteria: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_labels: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl>>,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    #[doc= "Set the field `filter_labels`.\n"]
    pub fn set_filter_labels(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter_labels = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_labels = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    #[doc= "Specifies how individual filterLabel matches within the list of filterLabels\ncontribute towards the overall metadataFilter match. Supported values are:\n  - MATCH_ANY: At least one of the filterLabels must have a matching label in the\nprovided metadata.\n  - MATCH_ALL: All filterLabels must have matching labels in\nthe provided metadata. Possible values: [\"MATCH_ALL\", \"MATCH_ANY\"]"]
    pub filter_match_criteria: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
            filter_match_criteria: self.filter_match_criteria,
            filter_labels: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `filter_match_criteria` after provisioning.\nSpecifies how individual filterLabel matches within the list of filterLabels\ncontribute towards the overall metadataFilter match. Supported values are:\n  - MATCH_ANY: At least one of the filterLabels must have a matching label in the\nprovided metadata.\n  - MATCH_ALL: All filterLabels must have matching labels in\nthe provided metadata. Possible values: [\"MATCH_ALL\", \"MATCH_ANY\"]"]
    pub fn filter_match_criteria(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_match_criteria", self.base))
    }

    #[doc= "Get a reference to the value of field `filter_labels` after provisioning.\n"]
    pub fn filter_labels(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter_labels", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_match: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    present_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_match: Option<PrimField<String>>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    #[doc= "Set the field `exact_match`.\nThe queryParameterMatch matches if the value of the parameter exactly matches\nthe contents of exactMatch. Only one of presentMatch, exactMatch and regexMatch\nmust be set."]
    pub fn set_exact_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact_match = Some(v.into());
        self
    }

    #[doc= "Set the field `present_match`.\nSpecifies that the queryParameterMatch matches if the request contains the query\nparameter, irrespective of whether the parameter has a value or not. Only one of\npresentMatch, exactMatch and regexMatch must be set."]
    pub fn set_present_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.present_match = Some(v.into());
        self
    }

    #[doc= "Set the field `regex_match`.\nThe queryParameterMatch matches if the value of the parameter matches the\nregular expression specified by regexMatch. For the regular expression grammar,\nplease see en.cppreference.com/w/cpp/regex/ecmascript  Only one of presentMatch,\nexactMatch and regexMatch must be set."]
    pub fn set_regex_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex_match = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    #[doc= "The name of the query parameter to match. The query parameter must exist in the\nrequest, in the absence of which the request match fails."]
    pub name: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
            exact_match: core::default::Default::default(),
            name: self.name,
            present_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exact_match` after provisioning.\nThe queryParameterMatch matches if the value of the parameter exactly matches\nthe contents of exactMatch. Only one of presentMatch, exactMatch and regexMatch\nmust be set."]
    pub fn exact_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the query parameter to match. The query parameter must exist in the\nrequest, in the absence of which the request match fails."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `present_match` after provisioning.\nSpecifies that the queryParameterMatch matches if the request contains the query\nparameter, irrespective of whether the parameter has a value or not. Only one of\npresentMatch, exactMatch and regexMatch must be set."]
    pub fn present_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.present_match", self.base))
    }

    #[doc= "Get a reference to the value of field `regex_match` after provisioning.\nThe queryParameterMatch matches if the value of the parameter matches the\nregular expression specified by regexMatch. For the regular expression grammar,\nplease see en.cppreference.com/w/cpp/regex/ecmascript  Only one of presentMatch,\nexactMatch and regexMatch must be set."]
    pub fn regex_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElDynamic {
    header_matches: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>>,
    metadata_filters: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>>,
    query_parameter_matches: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    full_path_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_template_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_matches: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_filters: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_parameter_matches: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl>>,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    #[doc= "Set the field `full_path_match`.\nFor satisfying the matchRule condition, the path of the request must exactly\nmatch the value specified in fullPathMatch after removing any query parameters\nand anchor that may be part of the original URL. FullPathMatch must be between 1\nand 1024 characters. Only one of prefixMatch, fullPathMatch or regexMatch must\nbe specified."]
    pub fn set_full_path_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.full_path_match = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore_case`.\nSpecifies that prefixMatch and fullPathMatch matches are case sensitive.\nDefaults to false."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }

    #[doc= "Set the field `path_template_match`.\nFor satisfying the matchRule condition, the path of the request\nmust match the wildcard pattern specified in pathTemplateMatch\nafter removing any query parameters and anchor that may be part\nof the original URL.\n\npathTemplateMatch must be between 1 and 255 characters\n(inclusive).  The pattern specified by pathTemplateMatch may\nhave at most 5 wildcard operators and at most 5 variable\ncaptures in total."]
    pub fn set_path_template_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_template_match = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix_match`.\nFor satisfying the matchRule condition, the request's path must begin with the\nspecified prefixMatch. prefixMatch must begin with a /. The value must be\nbetween 1 and 1024 characters. Only one of prefixMatch, fullPathMatch or\nregexMatch must be specified."]
    pub fn set_prefix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_match = Some(v.into());
        self
    }

    #[doc= "Set the field `regex_match`.\nFor satisfying the matchRule condition, the path of the request must satisfy the\nregular expression specified in regexMatch after removing any query parameters\nand anchor supplied with the original URL. For regular expression grammar please\nsee en.cppreference.com/w/cpp/regex/ecmascript  Only one of prefixMatch,\nfullPathMatch or regexMatch must be specified."]
    pub fn set_regex_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex_match = Some(v.into());
        self
    }

    #[doc= "Set the field `header_matches`.\n"]
    pub fn set_header_matches(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_matches = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_matches = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `metadata_filters`.\n"]
    pub fn set_metadata_filters(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata_filters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_filters = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `query_parameter_matches`.\n"]
    pub fn set_query_parameter_matches(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_parameter_matches = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameter_matches = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl {}

impl BuildComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl {
            full_path_match: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            path_template_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
            header_matches: core::default::Default::default(),
            metadata_filters: core::default::Default::default(),
            query_parameter_matches: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
        ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `full_path_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request must exactly\nmatch the value specified in fullPathMatch after removing any query parameters\nand anchor that may be part of the original URL. FullPathMatch must be between 1\nand 1024 characters. Only one of prefixMatch, fullPathMatch or regexMatch must\nbe specified."]
    pub fn full_path_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.full_path_match", self.base))
    }

    #[doc= "Get a reference to the value of field `ignore_case` after provisioning.\nSpecifies that prefixMatch and fullPathMatch matches are case sensitive.\nDefaults to false."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc= "Get a reference to the value of field `path_template_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request\nmust match the wildcard pattern specified in pathTemplateMatch\nafter removing any query parameters and anchor that may be part\nof the original URL.\n\npathTemplateMatch must be between 1 and 255 characters\n(inclusive).  The pattern specified by pathTemplateMatch may\nhave at most 5 wildcard operators and at most 5 variable\ncaptures in total."]
    pub fn path_template_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_template_match", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix_match` after provisioning.\nFor satisfying the matchRule condition, the request's path must begin with the\nspecified prefixMatch. prefixMatch must begin with a /. The value must be\nbetween 1 and 1024 characters. Only one of prefixMatch, fullPathMatch or\nregexMatch must be specified."]
    pub fn prefix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }

    #[doc= "Get a reference to the value of field `regex_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request must satisfy the\nregular expression specified in regexMatch after removing any query parameters\nand anchor supplied with the original URL. For regular expression grammar please\nsee en.cppreference.com/w/cpp/regex/ecmascript  Only one of prefixMatch,\nfullPathMatch or regexMatch must be specified."]
    pub fn regex_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }

    #[doc= "Get a reference to the value of field `header_matches` after provisioning.\n"]
    pub fn header_matches(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_matches", self.base))
    }

    #[doc= "Get a reference to the value of field `metadata_filters` after provisioning.\n"]
    pub fn metadata_filters(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metadata_filters", self.base))
    }

    #[doc= "Get a reference to the value of field `query_parameter_matches` after provisioning.\n"]
    pub fn query_parameter_matches(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.query_parameter_matches", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origin_regexes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<f64>>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    #[doc= "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origin_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origin_regexes = Some(v.into());
        self
    }

    #[doc= "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\nIf true, specifies the CORS policy is disabled.\nwhich indicates that the CORS policy is in effect. Defaults to false."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age`.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origin_regexes: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            disabled: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origin_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origin_regexes", self.base))
    }

    #[doc= "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled.\nwhich indicates that the CORS policy is in effect. Defaults to false."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_status: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    #[doc= "Set the field `http_status`.\nThe HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub fn set_http_status(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_status = Some(v.into());
        self
    }

    #[doc= "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub fn http_status(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }

    #[doc= "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    type O =
        BlockAssignable<
            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc= "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    pub fn build(
        self,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElDynamic {
    fixed_delay: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_delay: Option<
        Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>,
    >,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc= "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }

    #[doc= "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_delay = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }

    #[doc= "Get a reference to the value of field `fixed_delay` after provisioning.\n"]
    pub fn fixed_delay(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDynamic {
    abort: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl>>,
    delay: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl>>,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    #[doc= "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.abort = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.abort = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `delay`.\n"]
    pub fn set_delay(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.delay = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.delay = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc= "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    backend_service: PrimField<String>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl { }

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    #[doc= "The BackendService resource being mirrored to."]
    pub backend_service: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
            backend_service: self.backend_service,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backend_service` after provisioning.\nThe BackendService resource being mirrored to."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc= "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElDynamic {
    per_try_timeout: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    num_retries: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_try_timeout: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    #[doc= "Set the field `retry_conditions`.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\n  any 5xx response code, or if the backend service does not respond at all,\n  example: disconnects, reset, read timeout, connection failure, and refused\n  streams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n  502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\n  connecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\n  REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\n  header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\n  gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\n  header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in\n  the response header is set to unavailable"]
    pub fn set_retry_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.retry_conditions = Some(v.into());
        self
    }

    #[doc= "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_try_timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    #[doc= "Specifies the allowed number retries. This number must be > 0."]
    pub num_retries: PrimField<f64>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
            num_retries: self.num_retries,
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\n  any 5xx response code, or if the backend service does not respond at all,\n  example: disconnects, reset, read timeout, connection failure, and refused\n  streams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n  502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\n  connecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\n  REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\n  header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\n  gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\n  header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in\n  the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc= "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    #[doc= "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_template_rewrite: Option<PrimField<String>>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    #[doc= "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }

    #[doc= "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn set_path_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_prefix_rewrite = Some(v.into());
        self
    }

    #[doc= "Set the field `path_template_rewrite`.\nPrior to forwarding the request to the selected origin, if the\nrequest matched a pathTemplateMatch, the matching portion of the\nrequest's path is replaced re-written using the pattern specified\nby pathTemplateRewrite.\n\npathTemplateRewrite must be between 1 and 255 characters\n(inclusive), must start with a '/', and must only use variables\ncaptured by the route's pathTemplate matchers.\n\npathTemplateRewrite may only be used when all of a route's\nMatchRules specify pathTemplate.\n\nOnly one of pathPrefixRewrite and pathTemplateRewrite may be\nspecified."]
    pub fn set_path_template_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_template_rewrite = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
            path_template_rewrite: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `path_template_rewrite` after provisioning.\nPrior to forwarding the request to the selected origin, if the\nrequest matched a pathTemplateMatch, the matching portion of the\nrequest's path is replaced re-written using the pattern specified\nby pathTemplateRewrite.\n\npathTemplateRewrite must be between 1 and 255 characters\n(inclusive), must start with a '/', and must only use variables\ncaptured by the route's pathTemplate matchers.\n\npathTemplateRewrite may only be used when all of a route's\nMatchRules specify pathTemplate.\n\nOnly one of pathPrefixRewrite and pathTemplateRewrite may be\nspecified."]
    pub fn path_template_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_template_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {

}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    type O =
        BlockAssignable<
            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    #[doc= "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc= "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc= "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {

}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    type O =
        BlockAssignable<
            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    #[doc= "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc= "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc= "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc= "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElDynamic {
    request_headers_to_add: Option<
        DynamicBlock<
            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    response_headers_to_add: Option<
        DynamicBlock<
            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add: Option<
        Vec<
            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_add: Option<
        Vec<
            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[doc= "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }

    #[doc= "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
    type O =
        BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc= "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> ListRef<
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc= "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> ListRef<
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElDynamic {
    header_action: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    backend_service: PrimField<String>,
    weight: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<
        Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl>,
    >,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    #[doc= "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    #[doc= "The default BackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub backend_service: PrimField<String>,
    #[doc= "Specifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub weight: PrimField<f64>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
            backend_service: self.backend_service,
            weight: self.weight,
            header_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backend_service` after provisioning.\nThe default BackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }

    #[doc= "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc= "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElDynamic {
    cors_policy: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>>,
    fault_injection_policy: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl>,
    >,
    request_mirror_policy: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl>,
    >,
    retry_policy: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>>,
    timeout: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>>,
    url_rewrite: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>>,
    weighted_backend_services: Option<
        DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_policy: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_injection_policy: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_mirror_policy: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_backend_services: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl>>,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElRouteActionElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionEl {
    #[doc= "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cors_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fault_injection_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fault_injection_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_mirror_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_mirror_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_rewrite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_backend_services = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElRouteActionEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionEl {}

impl BuildComputeUrlMapPathMatcherElRouteRulesElRouteActionEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionEl {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_response_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strip_query: Option<PrimField<bool>>,
}

impl ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    #[doc= "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn set_host_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https. If set\nto false, the URL scheme of the redirected request will remain the same as that\nof the request. This must only be set for UrlMaps used in TargetHttpProxys.\nSetting this true for TargetHttpsProxy is not permitted. Defaults to false."]
    pub fn set_https_redirect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.https_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. Only one of pathRedirect or prefixRedirect must be\nspecified. The value must be between 1 and 1024 characters."]
    pub fn set_path_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request."]
    pub fn set_prefix_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_redirect = Some(v.into());
        self
    }

    #[doc= "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method will be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case, the request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_response_code = Some(v.into());
        self
    }

    #[doc= "Set the field `strip_query`.\nIf set to true, any accompanying query portion of the original URL is removed\nprior to redirecting the request. If set to false, the query portion of the\noriginal URL is retained. Defaults to false."]
    pub fn set_strip_query(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strip_query = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl {}

impl BuildComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
        ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
        ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn host_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https. If set\nto false, the URL scheme of the redirected request will remain the same as that\nof the request. This must only be set for UrlMaps used in TargetHttpProxys.\nSetting this true for TargetHttpsProxy is not permitted. Defaults to false."]
    pub fn https_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. Only one of pathRedirect or prefixRedirect must be\nspecified. The value must be between 1 and 1024 characters."]
    pub fn path_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request."]
    pub fn prefix_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method will be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case, the request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_response_code", self.base))
    }

    #[doc= "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed\nprior to redirecting the request. If set to false, the query portion of the\noriginal URL is retained. Defaults to false."]
    pub fn strip_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElRouteRulesElDynamic {
    header_action: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl>>,
    match_rules: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl>>,
    route_action: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElRouteActionEl>>,
    url_redirect: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherElRouteRulesEl {
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_rules: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_action: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElRouteActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_redirect: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl>>,
    dynamic: ComputeUrlMapPathMatcherElRouteRulesElDynamic,
}

impl ComputeUrlMapPathMatcherElRouteRulesEl {
    #[doc= "Set the field `service`.\nThe backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }

    #[doc= "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `match_rules`.\n"]
    pub fn set_match_rules(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_rules = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `route_action`.\n"]
    pub fn set_route_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElRouteActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.route_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.route_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `url_redirect`.\n"]
    pub fn set_url_redirect(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_redirect = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_redirect = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherElRouteRulesEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherElRouteRulesEl {
    #[doc= "For routeRules within a given pathMatcher, priority determines the order\nin which load balancer will interpret routeRules. RouteRules are evaluated\nin order of priority, from the lowest to highest number. The priority of\na rule decreases as its number increases (1, 2, 3, N+1). The first rule\nthat matches the request is applied.\n\nYou cannot configure two or more routeRules with the same priority.\nPriority for each rule must be set to a number between 0 and\n2147483647 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules\nin the future without affecting the rest of the rules. For example,\n1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers to which\nyou could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the\nfuture without any impact on existing rules."]
    pub priority: PrimField<f64>,
}

impl BuildComputeUrlMapPathMatcherElRouteRulesEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherElRouteRulesEl {
        ComputeUrlMapPathMatcherElRouteRulesEl {
            priority: self.priority,
            service: core::default::Default::default(),
            header_action: core::default::Default::default(),
            match_rules: core::default::Default::default(),
            route_action: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRouteRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRouteRulesElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElRouteRulesElRef {
        ComputeUrlMapPathMatcherElRouteRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRouteRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nFor routeRules within a given pathMatcher, priority determines the order\nin which load balancer will interpret routeRules. RouteRules are evaluated\nin order of priority, from the lowest to highest number. The priority of\na rule decreases as its number increases (1, 2, 3, N+1). The first rule\nthat matches the request is applied.\n\nYou cannot configure two or more routeRules with the same priority.\nPriority for each rule must be set to a number between 0 and\n2147483647 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules\nin the future without affecting the rest of the rules. For example,\n1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers to which\nyou could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the\nfuture without any impact on existing rules."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\nThe backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc= "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElHeaderActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc= "Get a reference to the value of field `match_rules` after provisioning.\n"]
    pub fn match_rules(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElMatchRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match_rules", self.base))
    }

    #[doc= "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRouteActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }

    #[doc= "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElUrlRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapPathMatcherElDynamic {
    default_route_action: Option<DynamicBlock<ComputeUrlMapPathMatcherElDefaultRouteActionEl>>,
    default_url_redirect: Option<DynamicBlock<ComputeUrlMapPathMatcherElDefaultUrlRedirectEl>>,
    header_action: Option<DynamicBlock<ComputeUrlMapPathMatcherElHeaderActionEl>>,
    path_rule: Option<DynamicBlock<ComputeUrlMapPathMatcherElPathRuleEl>>,
    route_rules: Option<DynamicBlock<ComputeUrlMapPathMatcherElRouteRulesEl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMapPathMatcherEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_route_action: Option<Vec<ComputeUrlMapPathMatcherElDefaultRouteActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_url_redirect: Option<Vec<ComputeUrlMapPathMatcherElDefaultUrlRedirectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeUrlMapPathMatcherElHeaderActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_rule: Option<Vec<ComputeUrlMapPathMatcherElPathRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_rules: Option<Vec<ComputeUrlMapPathMatcherElRouteRulesEl>>,
    dynamic: ComputeUrlMapPathMatcherElDynamic,
}

impl ComputeUrlMapPathMatcherEl {
    #[doc= "Set the field `default_service`.\nThe backend service or backend bucket to use when none of the given paths match."]
    pub fn set_default_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_service = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `default_route_action`.\n"]
    pub fn set_default_route_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultRouteActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_route_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_route_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_url_redirect`.\n"]
    pub fn set_default_url_redirect(
        mut self,
        v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElDefaultUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_url_redirect = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_url_redirect = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `header_action`.\n"]
    pub fn set_header_action(mut self, v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElHeaderActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `path_rule`.\n"]
    pub fn set_path_rule(mut self, v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElPathRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.path_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.path_rule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `route_rules`.\n"]
    pub fn set_route_rules(mut self, v: impl Into<BlockAssignable<ComputeUrlMapPathMatcherElRouteRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.route_rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.route_rules = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeUrlMapPathMatcherEl {
    type O = BlockAssignable<ComputeUrlMapPathMatcherEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapPathMatcherEl {
    #[doc= "The name to which this PathMatcher is referred by the HostRule."]
    pub name: PrimField<String>,
}

impl BuildComputeUrlMapPathMatcherEl {
    pub fn build(self) -> ComputeUrlMapPathMatcherEl {
        ComputeUrlMapPathMatcherEl {
            default_service: core::default::Default::default(),
            description: core::default::Default::default(),
            name: self.name,
            default_route_action: core::default::Default::default(),
            default_url_redirect: core::default::Default::default(),
            header_action: core::default::Default::default(),
            path_rule: core::default::Default::default(),
            route_rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeUrlMapPathMatcherElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapPathMatcherElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapPathMatcherElRef {
        ComputeUrlMapPathMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapPathMatcherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_service` after provisioning.\nThe backend service or backend bucket to use when none of the given paths match."]
    pub fn default_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_service", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name to which this PathMatcher is referred by the HostRule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> ListRef<ComputeUrlMapPathMatcherElDefaultRouteActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.base))
    }

    #[doc= "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> ListRef<ComputeUrlMapPathMatcherElDefaultUrlRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.base))
    }

    #[doc= "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeUrlMapPathMatcherElHeaderActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc= "Get a reference to the value of field `path_rule` after provisioning.\n"]
    pub fn path_rule(&self) -> ListRef<ComputeUrlMapPathMatcherElPathRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path_rule", self.base))
    }

    #[doc= "Get a reference to the value of field `route_rules` after provisioning.\n"]
    pub fn route_rules(&self) -> ListRef<ComputeUrlMapPathMatcherElRouteRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapTestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    host: PrimField<String>,
    path: PrimField<String>,
    service: PrimField<String>,
}

impl ComputeUrlMapTestEl {
    #[doc= "Set the field `description`.\nDescription of this test case."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeUrlMapTestEl {
    type O = BlockAssignable<ComputeUrlMapTestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapTestEl {
    #[doc= "Host portion of the URL."]
    pub host: PrimField<String>,
    #[doc= "Path portion of the URL."]
    pub path: PrimField<String>,
    #[doc= "The backend service or backend bucket link that should be matched by this test."]
    pub service: PrimField<String>,
}

impl BuildComputeUrlMapTestEl {
    pub fn build(self) -> ComputeUrlMapTestEl {
        ComputeUrlMapTestEl {
            description: core::default::Default::default(),
            host: self.host,
            path: self.path,
            service: self.service,
        }
    }
}

pub struct ComputeUrlMapTestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapTestElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapTestElRef {
        ComputeUrlMapTestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapTestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of this test case."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\nHost portion of the URL."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\nPath portion of the URL."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `service` after provisioning.\nThe backend service or backend bucket link that should be matched by this test."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMapTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeUrlMapTimeoutsEl {
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

impl ToListMappable for ComputeUrlMapTimeoutsEl {
    type O = BlockAssignable<ComputeUrlMapTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMapTimeoutsEl {}

impl BuildComputeUrlMapTimeoutsEl {
    pub fn build(self) -> ComputeUrlMapTimeoutsEl {
        ComputeUrlMapTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeUrlMapTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeUrlMapTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeUrlMapTimeoutsElRef {
        ComputeUrlMapTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMapTimeoutsElRef {
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
struct ComputeUrlMapDynamic {
    default_route_action: Option<DynamicBlock<ComputeUrlMapDefaultRouteActionEl>>,
    default_url_redirect: Option<DynamicBlock<ComputeUrlMapDefaultUrlRedirectEl>>,
    header_action: Option<DynamicBlock<ComputeUrlMapHeaderActionEl>>,
    host_rule: Option<DynamicBlock<ComputeUrlMapHostRuleEl>>,
    path_matcher: Option<DynamicBlock<ComputeUrlMapPathMatcherEl>>,
    test: Option<DynamicBlock<ComputeUrlMapTestEl>>,
}
