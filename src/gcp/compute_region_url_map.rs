use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionUrlMapData {
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
    default_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_route_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_url_redirect: std::option::Option<std::vec::Vec<ComputeRegionUrlMapDefaultUrlRedirectEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rule: std::option::Option<std::vec::Vec<ComputeRegionUrlMapHostRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_matcher: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test: std::option::Option<std::vec::Vec<ComputeRegionUrlMapTestEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRegionUrlMapTimeoutsEl>,
    dynamic: ComputeRegionUrlMapDynamic,
}

struct ComputeRegionUrlMap_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRegionUrlMapData>,
}

#[derive(Clone)]
pub struct ComputeRegionUrlMap(std::rc::Rc<ComputeRegionUrlMap_>);

impl ComputeRegionUrlMap {
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

    #[doc =
        "Set the field `default_service`.\nThe full or partial URL of the defaultService resource to which traffic is directed if\nnone of the hostRules match. If defaultRouteAction is additionally specified, advanced\nrouting actions like URL Rewrites, etc. take effect prior to sending the request to the\nbackend. However, if defaultService is specified, defaultRouteAction cannot contain any\nweightedBackendServices. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified.  Only one of defaultService,\ndefaultUrlRedirect or defaultRouteAction.weightedBackendService must be set."]
    pub fn set_default_service(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe Region in which the url map should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_route_action`.\n"]
    pub fn set_default_route_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_route_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_route_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_url_redirect`.\n"]
    pub fn set_default_url_redirect(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapDefaultUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_url_redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_url_redirect = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `host_rule`.\n"]
    pub fn set_host_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapHostRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().host_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.host_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `path_matcher`.\n"]
    pub fn set_path_matcher(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().path_matcher = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.path_matcher = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `test`.\n"]
    pub fn set_test(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapTestEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().test = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.test = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRegionUrlMapTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_service` after provisioning.\nThe full or partial URL of the defaultService resource to which traffic is directed if\nnone of the hostRules match. If defaultRouteAction is additionally specified, advanced\nrouting actions like URL Rewrites, etc. take effect prior to sending the request to the\nbackend. However, if defaultService is specified, defaultRouteAction cannot contain any\nweightedBackendServices. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified.  Only one of defaultService,\ndefaultUrlRedirect or defaultRouteAction.weightedBackendService must be set."]
    pub fn default_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `map_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn map_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe Region in which the url map should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> terrars::ListRef<ComputeRegionUrlMapDefaultUrlRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> terrars::ListRef<ComputeRegionUrlMapTestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionUrlMapTimeoutsElRef {
        ComputeRegionUrlMapTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeRegionUrlMap {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeRegionUrlMap { }

impl terrars::ToListMappable for ComputeRegionUrlMap {
    type O = terrars::ListRef<ComputeRegionUrlMapRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeRegionUrlMap_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_region_url_map".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRegionUrlMap {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeRegionUrlMap {
        let out = ComputeRegionUrlMap(std::rc::Rc::new(ComputeRegionUrlMap_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeRegionUrlMapData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_service: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                default_route_action: core::default::Default::default(),
                default_url_redirect: core::default::Default::default(),
                host_rule: core::default::Default::default(),
                path_matcher: core::default::Default::default(),
                test: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRegionUrlMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRegionUrlMapRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_service` after provisioning.\nThe full or partial URL of the defaultService resource to which traffic is directed if\nnone of the hostRules match. If defaultRouteAction is additionally specified, advanced\nrouting actions like URL Rewrites, etc. take effect prior to sending the request to the\nbackend. However, if defaultService is specified, defaultRouteAction cannot contain any\nweightedBackendServices. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified.  Only one of defaultService,\ndefaultUrlRedirect or defaultRouteAction.weightedBackendService must be set."]
    pub fn default_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `map_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn map_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe Region in which the url map should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> terrars::ListRef<ComputeRegionUrlMapDefaultUrlRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> terrars::ListRef<ComputeRegionUrlMapTestElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionUrlMapTimeoutsElRef {
        ComputeRegionUrlMapTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origin_regexes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
    #[doc =
        "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials. This field translates to the Access-Control-Allow-Credentials header.\nDefault is false."]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_origin_regexes`.\nSpecifies the regualar expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn set_allow_origin_regexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_origin_regexes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nIf true, the setting specifies the CORS policy is disabled. The default value of false, which indicates that the CORS policy is in effect."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_age`.\nSpecifies how long results of a preflight request can be cached in seconds.\nThis translates to the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
        ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
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

pub struct ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef {
        ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials. This field translates to the Access-Control-Allow-Credentials header.\nDefault is false."]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regualar expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origin_regexes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origin_regexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf true, the setting specifies the CORS policy is disabled. The default value of false, which indicates that the CORS policy is in effect."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long results of a preflight request can be cached in seconds.\nThis translates to the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_status: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    #[doc =
        "Set the field `http_status`.\nThe HTTP status code used to abort the request.\nThe value must be between 200 and 599 inclusive."]
    pub fn set_http_status(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.http_status = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request.\nThe value must be between 200 and 599 inclusive."]
    pub fn http_status(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElDynamic {
    fixed_delay: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>>,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElDynamic,
}

impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc =
        "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fixed_delay = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_delay` after provisioning.\n"]
    pub fn fixed_delay(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDynamic {
    abort: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>>,
    delay: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std::option::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>>,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDynamic,
}

impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.abort = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.abort = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `delay`.\n"]
    pub fn set_delay(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delay = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delay = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(&self) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend_service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    #[doc =
        "Set the field `backend_service`.\nThe full or partial URL to the RegionBackendService resource being mirrored to.\nThe backend service configured for a mirroring policy must reference backends that are of the same type as the original backend service matched in the URL map.\nServerless NEG backends are not currently supported as a mirrored backend service."]
    pub fn set_backend_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backend_service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
        ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
            backend_service: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
        ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the RegionBackendService resource being mirrored to.\nThe backend service configured for a mirroring policy must reference backends that are of the same type as the original backend service matched in the URL map.\nServerless NEG backends are not currently supported as a mirrored backend service."]
    pub fn backend_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
        ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElDynamic {
    per_try_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElDynamic,
}

impl ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
    #[doc =
        "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry policy applies.\nValid values are listed below. Only the following codes are supported when the URL map is bound to target gRPC proxy that has validateForProxyless field set to true: cancelled, deadline-exceeded, internal, resource-exhausted, unavailable.\n  - 5xx : retry is attempted if the instance or endpoint responds with any 5xx response code, or if the instance or endpoint does not respond at all. For example, disconnects, reset, read timeout, connection failure, and refused streams.\n  - gateway-error : Similar to 5xx, but only applies to response codes 502, 503 or 504.\n  - connect-failure : a retry is attempted on failures connecting to the instance or endpoint. For example, connection timeouts.\n  - retriable-4xx : a retry is attempted if the instance or endpoint responds with a 4xx response code. The only error that you can retry is error code 409.\n  - refused-stream : a retry is attempted if the instance or endpoint resets the stream with a REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n  - cancelled : a retry is attempted if the gRPC status code in the response header is set to cancelled.\n  - deadline-exceeded : a retry is attempted if the gRPC status code in the response header is set to deadline-exceeded.\n  - internal :  a retry is attempted if the gRPC status code in the response header is set to internal.\n  - resource-exhausted : a retry is attempted if the gRPC status code in the response header is set to resource-exhausted.\n  - unavailable : a retry is attempted if the gRPC status code in the response header is set to unavailable."]
    pub fn set_retry_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.retry_conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.per_try_timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
        ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef {
        ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry policy applies.\nValid values are listed below. Only the following codes are supported when the URL map is bound to target gRPC proxy that has validateForProxyless field set to true: cancelled, deadline-exceeded, internal, resource-exhausted, unavailable.\n  - 5xx : retry is attempted if the instance or endpoint responds with any 5xx response code, or if the instance or endpoint does not respond at all. For example, disconnects, reset, read timeout, connection failure, and refused streams.\n  - gateway-error : Similar to 5xx, but only applies to response codes 502, 503 or 504.\n  - connect-failure : a retry is attempted on failures connecting to the instance or endpoint. For example, connection timeouts.\n  - retriable-4xx : a retry is attempted if the instance or endpoint responds with a 4xx response code. The only error that you can retry is error code 409.\n  - refused-stream : a retry is attempted if the instance or endpoint resets the stream with a REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n  - cancelled : a retry is attempted if the gRPC status code in the response header is set to cancelled.\n  - deadline-exceeded : a retry is attempted if the gRPC status code in the response header is set to deadline-exceeded.\n  - internal :  a retry is attempted if the gRPC status code in the response header is set to internal.\n  - resource-exhausted : a retry is attempted if the gRPC status code in the response header is set to resource-exhausted.\n  - unavailable : a retry is attempted if the gRPC status code in the response header is set to unavailable."]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive. Note: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElTimeoutEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
        ComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef {
        ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive. Note: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
    #[doc =
        "Set the field `host_rewrite`.\nBefore forwarding the request to the selected service, the request's host header is replaced with contents of hostRewrite.\nThe value must be from 1 to 255 characters."]
    pub fn set_host_rewrite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_prefix_rewrite`.\nBefore forwarding the request to the selected backend service, the matching portion of the request's path is replaced by pathPrefixRewrite.\nThe value must be from 1 to 1024 characters."]
    pub fn set_path_prefix_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_prefix_rewrite = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
        ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef {
        ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_rewrite` after provisioning.\nBefore forwarding the request to the selected service, the request's host header is replaced with contents of hostRewrite.\nThe value must be from 1 to 255 characters."]
    pub fn host_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nBefore forwarding the request to the selected backend service, the matching portion of the request's path is replaced by pathPrefixRewrite.\nThe value must be from 1 to 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    #[doc = "Set the field `header_name`.\nThe name of the header."]
    pub fn set_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_value`.\nThe value of the header to add."]
    pub fn set_header_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_value = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn set_replace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    #[doc = "Set the field `header_name`.\nThe name of the header."]
    pub fn set_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_value`.\nThe value of the header to add."]
    pub fn set_header_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_value = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn set_replace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElDynamic {
    request_headers_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    response_headers_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_add: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_add: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl>,
    >,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElDynamic,
}

impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[doc =
        "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request before forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.request_headers_to_remove = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response before sending the response back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.response_headers_to_remove = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_headers_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_headers_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request before forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response before sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> terrars
    ::ListRef<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> terrars
    ::ListRef<
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElDynamic {
    header_action: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>>,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElDynamic,
}

impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    #[doc =
        "Set the field `backend_service`.\nThe full or partial URL to the default BackendService resource. Before forwarding the request to backendService, the load balancer applies any relevant headerActions specified as part of this backendServiceWeight."]
    pub fn set_backend_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backend_service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `weight`.\nSpecifies the fraction of traffic sent to a backend service, computed as weight / (sum of all weightedBackendService weights in routeAction) .\nThe selection of a backend service is determined only for new traffic. Once a user's request has been directed to a backend service, subsequent requests are sent to the same backend service as determined by the backend service's session affinity policy.\nThe value must be from 0 to 1000."]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
            backend_service: core::default::Default::default(),
            weight: core::default::Default::default(),
            header_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the default BackendService resource. Before forwarding the request to backendService, the load balancer applies any relevant headerActions specified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to a backend service, computed as weight / (sum of all weightedBackendService weights in routeAction) .\nThe selection of a backend service is determined only for new traffic. Once a user's request has been directed to a backend service, subsequent requests are sent to the same backend service as determined by the backend service's session affinity policy.\nThe value must be from 0 to 1000."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElDynamic {
    cors_policy: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl>>,
    timeout: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElTimeoutEl>>,
    url_rewrite: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl>>,
    weighted_backend_services: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_backend_services: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl>>,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElDynamic,
}

impl ComputeRegionUrlMapDefaultRouteActionEl {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cors_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fault_injection_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fault_injection_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_mirror_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_mirror_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_rewrite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weighted_backend_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultRouteActionEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultRouteActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultRouteActionEl {}

impl BuildComputeRegionUrlMapDefaultRouteActionEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionEl {
        ComputeRegionUrlMapDefaultRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapDefaultRouteActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultRouteActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMapDefaultRouteActionElRef {
        ComputeRegionUrlMapDefaultRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultRouteActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultUrlRedirectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_redirect: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_response_code: std::option::Option<terrars::PrimField<std::string::String>>,
    strip_query: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMapDefaultUrlRedirectEl {
    #[doc =
        "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn set_host_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.https_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn set_path_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redirect_response_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapDefaultUrlRedirectEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapDefaultUrlRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapDefaultUrlRedirectEl {
    #[doc =
        "If set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMapDefaultUrlRedirectEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultUrlRedirectEl {
        ComputeRegionUrlMapDefaultUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}

pub struct ComputeRegionUrlMapDefaultUrlRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapDefaultUrlRedirectElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMapDefaultUrlRedirectElRef {
        ComputeRegionUrlMapDefaultUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapDefaultUrlRedirectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn host_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn https_redirect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn path_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapHostRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    hosts: terrars::SetField<terrars::PrimField<std::string::String>>,
    path_matcher: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapHostRuleEl {
    #[doc =
        "Set the field `description`.\nAn optional description of this HostRule. Provide this property\nwhen you create the resource."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapHostRuleEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapHostRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapHostRuleEl {
    #[doc =
        "The list of host patterns to match. They must be valid\nhostnames, except * will match any string of ([a-z0-9-.]*). In\nthat case, * must be the first character and must be followed in\nthe pattern by either - or .."]
    pub hosts: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The name of the PathMatcher to use to match the path portion of\nthe URL if the hostRule matches the URL's host portion."]
    pub path_matcher: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapHostRuleEl {
    pub fn build(self) -> ComputeRegionUrlMapHostRuleEl {
        ComputeRegionUrlMapHostRuleEl {
            description: core::default::Default::default(),
            hosts: self.hosts,
            path_matcher: self.path_matcher,
        }
    }
}

pub struct ComputeRegionUrlMapHostRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapHostRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMapHostRuleElRef {
        ComputeRegionUrlMapHostRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapHostRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this HostRule. Provide this property\nwhen you create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nThe list of host patterns to match. They must be valid\nhostnames, except * will match any string of ([a-z0-9-.]*). In\nthat case, * must be the first character and must be followed in\nthe pattern by either - or .."]
    pub fn hosts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_matcher` after provisioning.\nThe name of the PathMatcher to use to match the path portion of\nthe URL if the hostRule matches the URL's host portion."]
    pub fn path_matcher(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_matcher", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_redirect: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_response_code: std::option::Option<terrars::PrimField<std::string::String>>,
    strip_query: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
    #[doc =
        "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn set_host_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.https_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn set_path_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redirect_response_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
    #[doc =
        "If set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
        ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef {
        ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn host_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn https_redirect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn path_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origin_regexes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    disabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    #[doc =
        "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origin_regexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_origin_regexes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_age`.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    #[doc = "If true, specifies the CORS policy is disabled."]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
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

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origin_regexes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origin_regexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
    http_status: terrars::PrimField<f64>,
    percentage: terrars::PrimField<f64>,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl { }

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
    #[doc = "The HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub http_status: terrars::PrimField<f64>,
    #[doc =
        "The percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub percentage: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: self.http_status,
            percentage: self.percentage,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub fn http_status(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElDynamic {
    fixed_delay: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    percentage: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc = "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fixed_delay = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc =
        "The percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub percentage: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: self.percentage,
            fixed_delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_delay` after provisioning.\n"]
    pub fn fixed_delay(
        &self,
    ) -> terrars
    ::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDynamic {
    abort: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl>,
    >,
    delay: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.abort = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.abort = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `delay`.\n"]
    pub fn set_delay(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delay = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delay = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    backend_service: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl { }

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    #[doc = "The RegionBackendService resource being mirrored to."]
    pub backend_service: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
            backend_service: self.backend_service,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend_service` after provisioning.\nThe RegionBackendService resource being mirrored to."]
    pub fn backend_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElDynamic {
    per_try_timeout: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    #[doc = "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n- 5xx: Loadbalancer will attempt a retry if the backend service responds with\nany 5xx response code, or if the backend service does not respond at all,\nexample: disconnects, reset, read timeout, connection failure, and refused\nstreams.\n- gateway-error: Similar to 5xx, but only applies to response codes\n502, 503 or 504.\n- connect-failure: Loadbalancer will retry on failures\nconnecting to backend services, for example due to connection timeouts.\n- retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\nCurrently the only retriable error supported is 409.\n- refused-stream: Loadbalancer will retry if the backend service resets the stream with a\nREFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n- cancelled: Loadbalancer will retry if the gRPC status code in the response\nheader is set to cancelled\n- deadline-exceeded: Loadbalancer will retry if the\ngRPC status code in the response header is set to deadline-exceeded\n- resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\nheader is set to resource-exhausted\n- unavailable: Loadbalancer will retry if\nthe gRPC status code in the response header is set to unavailable"]
    pub fn set_retry_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.retry_conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.per_try_timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n- 5xx: Loadbalancer will attempt a retry if the backend service responds with\nany 5xx response code, or if the backend service does not respond at all,\nexample: disconnects, reset, read timeout, connection failure, and refused\nstreams.\n- gateway-error: Similar to 5xx, but only applies to response codes\n502, 503 or 504.\n- connect-failure: Loadbalancer will retry on failures\nconnecting to backend services, for example due to connection timeouts.\n- retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\nCurrently the only retriable error supported is 409.\n- refused-stream: Loadbalancer will retry if the backend service resets the stream with a\nREFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n- cancelled: Loadbalancer will retry if the gRPC status code in the response\nheader is set to cancelled\n- deadline-exceeded: Loadbalancer will retry if the\ngRPC status code in the response header is set to deadline-exceeded\n- resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\nheader is set to resource-exhausted\n- unavailable: Loadbalancer will retry if\nthe gRPC status code in the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    #[doc =
        "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn set_host_rewrite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn set_path_prefix_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_prefix_rewrite = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn host_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {

}

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    #[doc = "The name of the header."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc =
        "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {

}

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    #[doc = "The name of the header."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc =
        "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElDynamic {
    request_headers_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    response_headers_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_add: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_add: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[doc =
        "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.request_headers_to_remove = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.response_headers_to_remove = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_headers_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_headers_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> terrars
    ::ListRef<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> terrars
    ::ListRef<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElDynamic {
    header_action: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    backend_service: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std
    ::option
    ::Option<
        std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    #[doc =
        "The default RegionBackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub backend_service: terrars::PrimField<std::string::String>,
    #[doc =
        "Specifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub weight: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
            backend_service: self.backend_service,
            weight: self.weight,
            header_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend_service` after provisioning.\nThe default RegionBackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> terrars
    ::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElDynamic {
    cors_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl>>,
    retry_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>>,
    timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>>,
    url_rewrite: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>>,
    weighted_backend_services: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_backend_services: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cors_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fault_injection_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fault_injection_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_mirror_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_mirror_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_rewrite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weighted_backend_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_redirect: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_response_code: std::option::Option<terrars::PrimField<std::string::String>>,
    strip_query: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    #[doc =
        "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one\nthat was supplied in the request. The value must be between 1 and 255\ncharacters."]
    pub fn set_host_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https.\nIf set to false, the URL scheme of the redirected request will remain the\nsame as that of the request. This must only be set for UrlMaps used in\nTargetHttpProxys. Setting this true for TargetHttpsProxy is not\npermitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.https_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one\nthat was supplied in the request. pathRedirect cannot be supplied\ntogether with prefixRedirect. Supply one alone or neither. If neither is\nsupplied, the path of the original request will be used for the redirect.\nThe value must be between 1 and 1024 characters."]
    pub fn set_path_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the\nHttpRouteRuleMatch, retaining the remaining portion of the URL before\nredirecting the request. prefixRedirect cannot be supplied together with\npathRedirect. Supply one alone or neither. If neither is supplied, the\npath of the original request will be used for the redirect. The value\nmust be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redirect_response_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    #[doc =
        "If set to true, any accompanying query portion of the original URL is removed\nprior to redirecting the request. If set to false, the query portion of the\noriginal URL is retained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one\nthat was supplied in the request. The value must be between 1 and 255\ncharacters."]
    pub fn host_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https.\nIf set to false, the URL scheme of the redirected request will remain the\nsame as that of the request. This must only be set for UrlMaps used in\nTargetHttpProxys. Setting this true for TargetHttpsProxy is not\npermitted. The default is set to false."]
    pub fn https_redirect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one\nthat was supplied in the request. pathRedirect cannot be supplied\ntogether with prefixRedirect. Supply one alone or neither. If neither is\nsupplied, the path of the original request will be used for the redirect.\nThe value must be between 1 and 1024 characters."]
    pub fn path_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the\nHttpRouteRuleMatch, retaining the remaining portion of the URL before\nredirecting the request. prefixRedirect cannot be supplied together with\npathRedirect. Supply one alone or neither. If neither is supplied, the\npath of the original request will be used for the redirect. The value\nmust be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed\nprior to redirecting the request. If set to false, the query portion of the\noriginal URL is retained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElDynamic {
    route_action: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl>>,
    url_redirect: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleEl {
    paths: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_redirect: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElPathRuleEl {
    #[doc =
        "Set the field `service`.\nThe region backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_action`.\n"]
    pub fn set_route_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.route_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.route_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `url_redirect`.\n"]
    pub fn set_url_redirect(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_redirect = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleEl {
    #[doc =
        "The list of path patterns to match. Each must start with / and the only place a\n\\* is allowed is at the end following a /. The string fed to the path matcher\ndoes not include any text after the first ? or #, and those chars are not\nallowed here."]
    pub paths: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeRegionUrlMapPathMatcherElPathRuleEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleEl {
        ComputeRegionUrlMapPathMatcherElPathRuleEl {
            paths: self.paths,
            service: core::default::Default::default(),
            route_action: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMapPathMatcherElPathRuleElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElPathRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `paths` after provisioning.\nThe list of path patterns to match. Each must start with / and the only place a\n\\* is allowed is at the end following a /. The string fed to the path matcher\ndoes not include any text after the first ? or #, and those chars are not\nallowed here."]
    pub fn paths(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe region backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }

    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl { }

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    #[doc = "The name of the header."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc =
        "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl { }

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    #[doc = "The name of the header."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc =
        "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElDynamic {
    request_headers_to_add: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl>>,
    response_headers_to_add: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_add: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_add: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    #[doc =
        "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.request_headers_to_remove = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.response_headers_to_remove = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_headers_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_headers_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
    range_end: terrars::PrimField<f64>,
    range_start: terrars::PrimField<f64>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl { }

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
    #[doc = "The end of the range (exclusive)."]
    pub range_end: terrars::PrimField<f64>,
    #[doc = "The start of the range (inclusive)."]
    pub range_start: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
            range_end: self.range_end,
            range_start: self.range_start,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `range_end` after provisioning.\nThe end of the range (exclusive)."]
    pub fn range_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range_end", self.base))
    }

    #[doc =
        "Get a reference to the value of field `range_start` after provisioning.\nThe start of the range (inclusive)."]
    pub fn range_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.range_start", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElDynamic {
    range_match: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_match: std::option::Option<terrars::PrimField<std::string::String>>,
    header_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range_match: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    #[doc =
        "Set the field `exact_match`.\nThe value should exactly match contents of exactMatch. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn set_exact_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invert_match`.\nIf set to false, the headerMatch is considered a match if the match criteria\nabove are met. If set to true, the headerMatch is considered a match if the\nmatch criteria above are NOT met. Defaults to false."]
    pub fn set_invert_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_match`.\nThe value of the header must start with the contents of prefixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn set_prefix_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `present_match`.\nA header with the contents of headerName must exist. The match takes place\nwhether or not the request's header has a value or not. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn set_present_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.present_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `regex_match`.\nThe value of the header must match the regular expression specified in\nregexMatch. For regular expression grammar, please see:\nen.cppreference.com/w/cpp/regex/ecmascript  For matching against a port\nspecified in the HTTP request, use a headerMatch with headerName set to PORT and\na regular expression that satisfies the RFC2616 Host header's port specifier.\nOnly one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or\nrangeMatch must be set."]
    pub fn set_regex_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suffix_match`.\nThe value of the header must end with the contents of suffixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn set_suffix_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range_match`.\n"]
    pub fn set_range_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.range_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.range_match = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    #[doc =
        "The name of the HTTP header to match. For matching against the HTTP request's\nauthority, use a headerMatch with the header name \":authority\". For matching a\nrequest's method, use the headerName \":method\"."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
            exact_match: core::default::Default::default(),
            header_name: self.header_name,
            invert_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            present_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
            suffix_match: core::default::Default::default(),
            range_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exact_match` after provisioning.\nThe value should exactly match contents of exactMatch. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn exact_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_name` after provisioning.\nThe name of the HTTP header to match. For matching against the HTTP request's\nauthority, use a headerMatch with the header name \":authority\". For matching a\nrequest's method, use the headerName \":method\"."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_match` after provisioning.\nIf set to false, the headerMatch is considered a match if the match criteria\nabove are met. If set to true, the headerMatch is considered a match if the\nmatch criteria above are NOT met. Defaults to false."]
    pub fn invert_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_match` after provisioning.\nThe value of the header must start with the contents of prefixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn prefix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `present_match` after provisioning.\nA header with the contents of headerName must exist. The match takes place\nwhether or not the request's header has a value or not. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn present_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.present_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `regex_match` after provisioning.\nThe value of the header must match the regular expression specified in\nregexMatch. For regular expression grammar, please see:\nen.cppreference.com/w/cpp/regex/ecmascript  For matching against a port\nspecified in the HTTP request, use a headerMatch with headerName set to PORT and\na regular expression that satisfies the RFC2616 Host header's port specifier.\nOnly one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or\nrangeMatch must be set."]
    pub fn regex_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suffix_match` after provisioning.\nThe value of the header must end with the contents of suffixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn suffix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix_match", self.base))
    }

    #[doc = "Get a reference to the value of field `range_match` after provisioning.\n"]
    pub fn range_match(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range_match", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl { }

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
    #[doc =
        "Name of metadata label. The name can have a maximum length of 1024 characters\nand must be at least 1 character long."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The value of the label must match the specified value. value can have a maximum\nlength of 1024 characters."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of metadata label. The name can have a maximum length of 1024 characters\nand must be at least 1 character long."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe value of the label must match the specified value. value can have a maximum\nlength of 1024 characters."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElDynamic {
    filter_labels: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    filter_match_criteria: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_labels: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    #[doc = "Set the field `filter_labels`.\n"]
    pub fn set_filter_labels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_labels = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    #[doc =
        "Specifies how individual filterLabel matches within the list of filterLabels\ncontribute towards the overall metadataFilter match. Supported values are:\n\n* MATCH_ANY: At least one of the filterLabels must have a matching label in the\nprovided metadata.\n* MATCH_ALL: All filterLabels must have matching labels in\nthe provided metadata. Possible values: [\"MATCH_ALL\", \"MATCH_ANY\"]"]
    pub filter_match_criteria: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
            filter_match_criteria: self.filter_match_criteria,
            filter_labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter_match_criteria` after provisioning.\nSpecifies how individual filterLabel matches within the list of filterLabels\ncontribute towards the overall metadataFilter match. Supported values are:\n\n* MATCH_ANY: At least one of the filterLabels must have a matching label in the\nprovided metadata.\n* MATCH_ALL: All filterLabels must have matching labels in\nthe provided metadata. Possible values: [\"MATCH_ALL\", \"MATCH_ANY\"]"]
    pub fn filter_match_criteria(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_match_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_labels` after provisioning.\n"]
    pub fn filter_labels(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_labels", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_match: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    #[doc =
        "Set the field `exact_match`.\nThe queryParameterMatch matches if the value of the parameter exactly matches\nthe contents of exactMatch. Only one of presentMatch, exactMatch and regexMatch\nmust be set."]
    pub fn set_exact_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `present_match`.\nSpecifies that the queryParameterMatch matches if the request contains the query\nparameter, irrespective of whether the parameter has a value or not. Only one of\npresentMatch, exactMatch and regexMatch must be set."]
    pub fn set_present_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.present_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `regex_match`.\nThe queryParameterMatch matches if the value of the parameter matches the\nregular expression specified by regexMatch. For the regular expression grammar,\nplease see en.cppreference.com/w/cpp/regex/ecmascript  Only one of presentMatch,\nexactMatch and regexMatch must be set."]
    pub fn set_regex_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex_match = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    #[doc =
        "The name of the query parameter to match. The query parameter must exist in the\nrequest, in the absence of which the request match fails."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
            exact_match: core::default::Default::default(),
            name: self.name,
            present_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exact_match` after provisioning.\nThe queryParameterMatch matches if the value of the parameter exactly matches\nthe contents of exactMatch. Only one of presentMatch, exactMatch and regexMatch\nmust be set."]
    pub fn exact_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the query parameter to match. The query parameter must exist in the\nrequest, in the absence of which the request match fails."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `present_match` after provisioning.\nSpecifies that the queryParameterMatch matches if the request contains the query\nparameter, irrespective of whether the parameter has a value or not. Only one of\npresentMatch, exactMatch and regexMatch must be set."]
    pub fn present_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.present_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `regex_match` after provisioning.\nThe queryParameterMatch matches if the value of the parameter matches the\nregular expression specified by regexMatch. For the regular expression grammar,\nplease see en.cppreference.com/w/cpp/regex/ecmascript  Only one of presentMatch,\nexactMatch and regexMatch must be set."]
    pub fn regex_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElDynamic {
    header_matches: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>>,
    metadata_filters: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>>,
    query_parameter_matches: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    full_path_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_matches: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_filters: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameter_matches: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    #[doc =
        "Set the field `full_path_match`.\nFor satisfying the matchRule condition, the path of the request must exactly\nmatch the value specified in fullPathMatch after removing any query parameters\nand anchor that may be part of the original URL. FullPathMatch must be between 1\nand 1024 characters. Only one of prefixMatch, fullPathMatch or regexMatch must\nbe specified."]
    pub fn set_full_path_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.full_path_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_case`.\nSpecifies that prefixMatch and fullPathMatch matches are case sensitive.\nDefaults to false."]
    pub fn set_ignore_case(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_case = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_match`.\nFor satisfying the matchRule condition, the request's path must begin with the\nspecified prefixMatch. prefixMatch must begin with a /. The value must be\nbetween 1 and 1024 characters. Only one of prefixMatch, fullPathMatch or\nregexMatch must be specified."]
    pub fn set_prefix_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `regex_match`.\nFor satisfying the matchRule condition, the path of the request must satisfy the\nregular expression specified in regexMatch after removing any query parameters\nand anchor supplied with the original URL. For regular expression grammar please\nsee en.cppreference.com/w/cpp/regex/ecmascript  Only one of prefixMatch,\nfullPathMatch or regexMatch must be specified."]
    pub fn set_regex_match(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_matches`.\n"]
    pub fn set_header_matches(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_matches = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_matches = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata_filters`.\n"]
    pub fn set_metadata_filters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata_filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_filters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_parameter_matches`.\n"]
    pub fn set_query_parameter_matches(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_parameter_matches = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameter_matches = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
            full_path_match: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
            header_matches: core::default::Default::default(),
            metadata_filters: core::default::Default::default(),
            query_parameter_matches: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `full_path_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request must exactly\nmatch the value specified in fullPathMatch after removing any query parameters\nand anchor that may be part of the original URL. FullPathMatch must be between 1\nand 1024 characters. Only one of prefixMatch, fullPathMatch or regexMatch must\nbe specified."]
    pub fn full_path_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.full_path_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_case` after provisioning.\nSpecifies that prefixMatch and fullPathMatch matches are case sensitive.\nDefaults to false."]
    pub fn ignore_case(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_match` after provisioning.\nFor satisfying the matchRule condition, the request's path must begin with the\nspecified prefixMatch. prefixMatch must begin with a /. The value must be\nbetween 1 and 1024 characters. Only one of prefixMatch, fullPathMatch or\nregexMatch must be specified."]
    pub fn prefix_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `regex_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request must satisfy the\nregular expression specified in regexMatch after removing any query parameters\nand anchor supplied with the original URL. For regular expression grammar please\nsee en.cppreference.com/w/cpp/regex/ecmascript  Only one of prefixMatch,\nfullPathMatch or regexMatch must be specified."]
    pub fn regex_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }

    #[doc = "Get a reference to the value of field `header_matches` after provisioning.\n"]
    pub fn header_matches(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_matches", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_filters` after provisioning.\n"]
    pub fn metadata_filters(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `query_parameter_matches` after provisioning.\n"]
    pub fn query_parameter_matches(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_parameter_matches", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_credentials: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origin_regexes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    #[doc =
        "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn set_allow_credentials(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origin_regexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_origin_regexes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nIf true, specifies the CORS policy is disabled.\nwhich indicates that the CORS policy is in effect. Defaults to false."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_age`.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
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

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn allow_credentials(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_credentials", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origin_regexes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origin_regexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled.\nwhich indicates that the CORS policy is in effect. Defaults to false."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_status: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    #[doc =
        "Set the field `http_status`.\nThe HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub fn set_http_status(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.http_status = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub fn set_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub fn http_status(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElDynamic {
    fixed_delay: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc =
        "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fixed_delay = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `fixed_delay` after provisioning.\n"]
    pub fn fixed_delay(
        &self,
    ) -> terrars
    ::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDynamic {
    abort: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl>,
    >,
    delay: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.abort = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.abort = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `delay`.\n"]
    pub fn set_delay(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delay = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delay = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    backend_service: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl { }

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    #[doc = "The RegionBackendService resource being mirrored to."]
    pub backend_service: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
            backend_service: self.backend_service,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend_service` after provisioning.\nThe RegionBackendService resource being mirrored to."]
    pub fn backend_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElDynamic {
    per_try_timeout: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    num_retries: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    #[doc =
        "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\n  any 5xx response code, or if the backend service does not respond at all,\n  example: disconnects, reset, read timeout, connection failure, and refused\n  streams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n  502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\n  connecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\n  REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\n  header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\n  gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\n  header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in\n  the response header is set to unavailable"]
    pub fn set_retry_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.retry_conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.per_try_timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    #[doc = "Specifies the allowed number retries. This number must be > 0."]
    pub num_retries: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
            num_retries: self.num_retries,
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\n  any 5xx response code, or if the backend service does not respond at all,\n  example: disconnects, reset, read timeout, connection failure, and refused\n  streams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n  502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\n  connecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\n  REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\n  header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\n  gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\n  header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in\n  the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    #[doc =
        "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn set_host_rewrite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn set_path_prefix_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_prefix_rewrite = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn host_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {

}

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    #[doc = "The name of the header."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc =
        "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {

}

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    #[doc = "The name of the header."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc =
        "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElDynamic {
    request_headers_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    response_headers_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_add: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_add: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[doc =
        "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.request_headers_to_remove = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.response_headers_to_remove = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_headers_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_headers_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> terrars
    ::ListRef<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> terrars
    ::ListRef<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElDynamic {
    header_action: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    backend_service: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    #[doc =
        "The default RegionBackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub backend_service: terrars::PrimField<std::string::String>,
    #[doc =
        "Specifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub weight: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
            backend_service: self.backend_service,
            weight: self.weight,
            header_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend_service` after provisioning.\nThe default RegionBackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> terrars
    ::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElDynamic {
    cors_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl>>,
    retry_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>>,
    timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>>,
    url_rewrite: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>>,
    weighted_backend_services: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_backend_services: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cors_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fault_injection_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fault_injection_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_mirror_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_mirror_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_rewrite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weighted_backend_services = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_redirect: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_redirect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_response_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strip_query: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    #[doc =
        "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one\nthat was supplied in the request. The value must be between 1 and 255\ncharacters."]
    pub fn set_host_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https.\nIf set to false, the URL scheme of the redirected request will remain the\nsame as that of the request. This must only be set for UrlMaps used in\nTargetHttpProxys. Setting this true for TargetHttpsProxy is not\npermitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.https_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one\nthat was supplied in the request. pathRedirect cannot be supplied\ntogether with prefixRedirect. Supply one alone or neither. If neither is\nsupplied, the path of the original request will be used for the redirect.\nThe value must be between 1 and 1024 characters."]
    pub fn set_path_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the\nHttpRouteRuleMatch, retaining the remaining portion of the URL before\nredirecting the request. prefixRedirect cannot be supplied together with\npathRedirect. Supply one alone or neither. If neither is supplied, the\npath of the original request will be used for the redirect. The value\nmust be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redirect_response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strip_query`.\nIf set to true, any accompanying query portion of the original URL is\nremoved prior to redirecting the request. If set to false, the query\nportion of the original URL is retained. The default value is false."]
    pub fn set_strip_query(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strip_query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one\nthat was supplied in the request. The value must be between 1 and 255\ncharacters."]
    pub fn host_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https.\nIf set to false, the URL scheme of the redirected request will remain the\nsame as that of the request. This must only be set for UrlMaps used in\nTargetHttpProxys. Setting this true for TargetHttpsProxy is not\npermitted. The default is set to false."]
    pub fn https_redirect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one\nthat was supplied in the request. pathRedirect cannot be supplied\ntogether with prefixRedirect. Supply one alone or neither. If neither is\nsupplied, the path of the original request will be used for the redirect.\nThe value must be between 1 and 1024 characters."]
    pub fn path_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the\nHttpRouteRuleMatch, retaining the remaining portion of the URL before\nredirecting the request. prefixRedirect cannot be supplied together with\npathRedirect. Supply one alone or neither. If neither is supplied, the\npath of the original request will be used for the redirect. The value\nmust be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is\nremoved prior to redirecting the request. If set to false, the query\nportion of the original URL is retained. The default value is false."]
    pub fn strip_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElDynamic {
    header_action: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl>>,
    match_rules: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl>>,
    route_action: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl>>,
    url_redirect: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesEl {
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_rules: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_redirect: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElDynamic,
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesEl {
    #[doc =
        "Set the field `service`.\nThe region backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_rules`.\n"]
    pub fn set_match_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `route_action`.\n"]
    pub fn set_route_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.route_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.route_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `url_redirect`.\n"]
    pub fn set_url_redirect(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_redirect = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesEl {
    #[doc =
        "For routeRules within a given pathMatcher, priority determines the order\nin which load balancer will interpret routeRules. RouteRules are evaluated\nin order of priority, from the lowest to highest number. The priority of\na rule decreases as its number increases (1, 2, 3, N+1). The first rule\nthat matches the request is applied.\n\nYou cannot configure two or more routeRules with the same priority.\nPriority for each rule must be set to a number between 0 and\n2147483647 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules\nin the future without affecting the rest of the rules. For example,\n1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers to which\nyou could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the\nfuture without any impact on existing rules."]
    pub priority: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMapPathMatcherElRouteRulesEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesEl {
            priority: self.priority,
            service: core::default::Default::default(),
            header_action: core::default::Default::default(),
            match_rules: core::default::Default::default(),
            route_action: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRouteRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nFor routeRules within a given pathMatcher, priority determines the order\nin which load balancer will interpret routeRules. RouteRules are evaluated\nin order of priority, from the lowest to highest number. The priority of\na rule decreases as its number increases (1, 2, 3, N+1). The first rule\nthat matches the request is applied.\n\nYou cannot configure two or more routeRules with the same priority.\nPriority for each rule must be set to a number between 0 and\n2147483647 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules\nin the future without affecting the rest of the rules. For example,\n1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers to which\nyou could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the\nfuture without any impact on existing rules."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe region backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_rules` after provisioning.\n"]
    pub fn match_rules(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }

    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElDynamic {
    default_url_redirect: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl>>,
    path_rule: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleEl>>,
    route_rules: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_url_redirect: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_rule: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElPathRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_rules: std::option::Option<std::vec::Vec<ComputeRegionUrlMapPathMatcherElRouteRulesEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElDynamic,
}

impl ComputeRegionUrlMapPathMatcherEl {
    #[doc =
        "Set the field `default_service`.\nA reference to a RegionBackendService resource. This will be used if\nnone of the pathRules defined by this PathMatcher is matched by\nthe URL's path portion."]
    pub fn set_default_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_url_redirect`.\n"]
    pub fn set_default_url_redirect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_url_redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_url_redirect = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `path_rule`.\n"]
    pub fn set_path_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.path_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.path_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `route_rules`.\n"]
    pub fn set_route_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.route_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.route_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapPathMatcherEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapPathMatcherEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapPathMatcherEl {
    #[doc = "The name to which this PathMatcher is referred by the HostRule."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapPathMatcherEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherEl {
        ComputeRegionUrlMapPathMatcherEl {
            default_service: core::default::Default::default(),
            description: core::default::Default::default(),
            name: self.name,
            default_url_redirect: core::default::Default::default(),
            path_rule: core::default::Default::default(),
            route_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapPathMatcherElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapPathMatcherElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMapPathMatcherElRef {
        ComputeRegionUrlMapPathMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapPathMatcherElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_service` after provisioning.\nA reference to a RegionBackendService resource. This will be used if\nnone of the pathRules defined by this PathMatcher is matched by\nthe URL's path portion."]
    pub fn default_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name to which this PathMatcher is referred by the HostRule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.base))
    }

    #[doc = "Get a reference to the value of field `path_rule` after provisioning.\n"]
    pub fn path_rule(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `route_rules` after provisioning.\n"]
    pub fn route_rules(&self) -> terrars::ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapTestEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    host: terrars::PrimField<std::string::String>,
    path: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMapTestEl {
    #[doc = "Set the field `description`.\nDescription of this test case."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMapTestEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapTestEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapTestEl {
    #[doc = "Host portion of the URL."]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = "Path portion of the URL."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "A reference to expected RegionBackendService resource the given URL should be mapped to."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMapTestEl {
    pub fn build(self) -> ComputeRegionUrlMapTestEl {
        ComputeRegionUrlMapTestEl {
            description: core::default::Default::default(),
            host: self.host,
            path: self.path,
            service: self.service,
        }
    }
}

pub struct ComputeRegionUrlMapTestElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapTestElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMapTestElRef {
        ComputeRegionUrlMapTestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapTestElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of this test case."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nHost portion of the URL."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath portion of the URL."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nA reference to expected RegionBackendService resource the given URL should be mapped to."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMapTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMapTimeoutsEl {
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

impl terrars::ToListMappable for ComputeRegionUrlMapTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeRegionUrlMapTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMapTimeoutsEl {}

impl BuildComputeRegionUrlMapTimeoutsEl {
    pub fn build(self) -> ComputeRegionUrlMapTimeoutsEl {
        ComputeRegionUrlMapTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionUrlMapTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMapTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMapTimeoutsElRef {
        ComputeRegionUrlMapTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMapTimeoutsElRef {
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
struct ComputeRegionUrlMapDynamic {
    default_route_action: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultRouteActionEl>>,
    default_url_redirect: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapDefaultUrlRedirectEl>>,
    host_rule: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapHostRuleEl>>,
    path_matcher: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapPathMatcherEl>>,
    test: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMapTestEl>>,
}
