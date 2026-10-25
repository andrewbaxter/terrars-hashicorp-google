#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionUrlMapData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
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
    default_route_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_url_redirect: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultUrlRedirect>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_HeaderAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rule: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_HostRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_matcher: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_Test>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRegionUrlMap_Timeouts>,
    dynamic: ComputeRegionUrlMapDynamic,
}

struct ComputeRegionUrlMap_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRegionUrlMapData_>,
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultUrlRedirect>>,
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

    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_HeaderAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().header_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.header_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `host_rule`.\n"]
    pub fn set_host_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_HostRule>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher>>,
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
    pub fn set_test(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_Test>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRegionUrlMap_Timeouts>) -> Self {
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn default_route_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultUrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> terrars::ListRef<ComputeRegionUrlMap_TestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionUrlMap_TimeoutsRef {
        ComputeRegionUrlMap_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(ComputeRegionUrlMapData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_service: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                default_route_action: core::default::Default::default(),
                default_url_redirect: core::default::Default::default(),
                header_action: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn default_route_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultUrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> terrars::ListRef<ComputeRegionUrlMap_TestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionUrlMap_TimeoutsRef {
        ComputeRegionUrlMap_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDynamic {
    default_route_action: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction>>,
    default_url_redirect: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultUrlRedirect>>,
    header_action: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_HeaderAction>>,
    host_rule: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_HostRule>>,
    path_matcher: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher>>,
    test: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_Test>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_CorsPolicy {
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

impl ComputeRegionUrlMap_DefaultRouteAction_CorsPolicy {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_CorsPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_CorsPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_CorsPolicy {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_CorsPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_CorsPolicy {
        ComputeRegionUrlMap_DefaultRouteAction_CorsPolicy {
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

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_CorsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_CorsPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_CorsPolicyRef {
        ComputeRegionUrlMap_DefaultRouteAction_CorsPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_CorsPolicyRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_status: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
        ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef {
        ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
        ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
        ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayDynamic {
    fixed_delay: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay>>,
    dynamic: ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayDynamic,
}

impl ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
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
                            ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
        ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef {
        ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicyDynamic {
    abort: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort>>,
    delay: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay>>,
    dynamic: ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicyDynamic,
}

impl ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort>,
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
                        terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy {
        ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicyRef {
        ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend_service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy {
    #[doc =
        "Set the field `backend_service`.\nThe full or partial URL to the RegionBackendService resource being mirrored to.\nThe backend service configured for a mirroring policy must reference backends that are of the same type as the original backend service matched in the URL map.\nServerless NEG backends are not currently supported as a mirrored backend service."]
    pub fn set_backend_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backend_service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy {
        ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy {
            backend_service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicyRef {
        ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicyRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
        ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef {
        ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_DefaultRouteAction_RetryPolicyDynamic {
    per_try_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>>,
    dynamic: ComputeRegionUrlMap_DefaultRouteAction_RetryPolicyDynamic,
}

impl ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy {
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
                        terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_RetryPolicy {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_RetryPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy {
        ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_RetryPolicyRef {
        ComputeRegionUrlMap_DefaultRouteAction_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_RetryPolicyRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMap_DefaultRouteAction_Timeout {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_Timeout {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_Timeout {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_Timeout {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_Timeout {
        ComputeRegionUrlMap_DefaultRouteAction_Timeout {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_TimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_TimeoutRef {
        ComputeRegionUrlMap_DefaultRouteAction_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_TimeoutRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_DefaultRouteAction_UrlRewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMap_DefaultRouteAction_UrlRewrite {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_UrlRewrite {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_UrlRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_UrlRewrite {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_UrlRewrite {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_UrlRewrite {
        ComputeRegionUrlMap_DefaultRouteAction_UrlRewrite {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_UrlRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_UrlRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_UrlRewriteRef {
        ComputeRegionUrlMap_DefaultRouteAction_UrlRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_UrlRewriteRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd {}

impl BuildComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd {
    pub fn build(self) -> ComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd {
        ComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_HeaderAction_RequestHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_HeaderAction_RequestHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_HeaderAction_RequestHeadersToAddRef {
        ComputeRegionUrlMap_HeaderAction_RequestHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_HeaderAction_RequestHeadersToAddRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd {}

impl BuildComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd {
    pub fn build(self) -> ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd {
        ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAddRef {
        ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAddRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_HeaderActionDynamic {
    request_headers_to_add: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd>>,
    response_headers_to_add: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_HeaderAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_add: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_add: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd>>,
    dynamic: ComputeRegionUrlMap_HeaderActionDynamic,
}

impl ComputeRegionUrlMap_HeaderAction {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_HeaderAction_RequestHeadersToAdd>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAdd>>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_HeaderAction {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_HeaderAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_HeaderAction {}

impl BuildComputeRegionUrlMap_HeaderAction {
    pub fn build(self) -> ComputeRegionUrlMap_HeaderAction {
        ComputeRegionUrlMap_HeaderAction {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_HeaderActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_HeaderActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMap_HeaderActionRef {
        ComputeRegionUrlMap_HeaderActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_HeaderActionRef {
    #[allow(dead_code)]
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
    pub fn request_headers_to_add(&self) -> terrars::ListRef<ComputeRegionUrlMap_HeaderAction_RequestHeadersToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(&self) -> terrars::ListRef<ComputeRegionUrlMap_HeaderAction_ResponseHeadersToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServicesDynamic {
    header_action: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_HeaderAction>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_HeaderAction>>,
    dynamic: ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServicesDynamic,
}

impl ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_HeaderAction>>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices {}

impl BuildComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices {
        ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices {
            backend_service: core::default::Default::default(),
            weight: core::default::Default::default(),
            header_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServicesRef {
        ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServicesRef {
    #[allow(dead_code)]
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
    pub fn header_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_DefaultRouteActionDynamic {
    cors_policy: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_CorsPolicy>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy>>,
    timeout: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_Timeout>>,
    url_rewrite: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_UrlRewrite>>,
    weighted_backend_services: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_DefaultRouteAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_CorsPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_Timeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_UrlRewrite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_backend_services: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices>>,
    dynamic: ComputeRegionUrlMap_DefaultRouteActionDynamic,
}

impl ComputeRegionUrlMap_DefaultRouteAction {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_CorsPolicy>>,
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy>>,
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_Timeout>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_UrlRewrite>>,
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServices>>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultRouteAction {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultRouteAction {}

impl BuildComputeRegionUrlMap_DefaultRouteAction {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultRouteAction {
        ComputeRegionUrlMap_DefaultRouteAction {
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

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultRouteActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultRouteActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMap_DefaultRouteActionRef {
        ComputeRegionUrlMap_DefaultRouteActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultRouteActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_CorsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_RequestMirrorPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_UrlRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_WeightedBackendServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_DefaultUrlRedirect {
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

impl ComputeRegionUrlMap_DefaultUrlRedirect {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_DefaultUrlRedirect {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_DefaultUrlRedirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_DefaultUrlRedirect {
    #[doc =
        "If set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMap_DefaultUrlRedirect {
    pub fn build(self) -> ComputeRegionUrlMap_DefaultUrlRedirect {
        ComputeRegionUrlMap_DefaultUrlRedirect {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_DefaultUrlRedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_DefaultUrlRedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMap_DefaultUrlRedirectRef {
        ComputeRegionUrlMap_DefaultUrlRedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_DefaultUrlRedirectRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_HostRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    hosts: terrars::SetField<terrars::PrimField<std::string::String>>,
    path_matcher: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMap_HostRule {
    #[doc =
        "Set the field `description`.\nAn optional description of this HostRule. Provide this property\nwhen you create the resource."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMap_HostRule {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_HostRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_HostRule {
    #[doc =
        "The list of host patterns to match. They must be valid\nhostnames, except * will match any string of ([a-z0-9-.]*). In\nthat case, * must be the first character and must be followed in\nthe pattern by either - or .."]
    pub hosts: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The name of the PathMatcher to use to match the path portion of\nthe URL if the hostRule matches the URL's host portion."]
    pub path_matcher: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_HostRule {
    pub fn build(self) -> ComputeRegionUrlMap_HostRule {
        ComputeRegionUrlMap_HostRule {
            description: core::default::Default::default(),
            hosts: self.hosts,
            path_matcher: self.path_matcher,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_HostRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_HostRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMap_HostRuleRef {
        ComputeRegionUrlMap_HostRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_HostRuleRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy {
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

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy {
    #[doc =
        "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\nThis translates to the Access-Control-Allow-Credentials header."]
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
        "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
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
        "Set the field `disabled`.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy {}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy {
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

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicyRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\nThis translates to the Access-Control-Allow-Credentials header."]
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
        "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origin_regexes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origin_regexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
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
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration {
    #[doc =
        "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDurationRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy {
    backend_service: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy { }

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy {
    #[doc = "The full or partial URL to the BackendService resource being mirrored to."]
    pub backend_service: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy {
            backend_service: self.backend_service,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicyRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the BackendService resource being mirrored to."]
    pub fn backend_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicyDynamic {
    per_try_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicyDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy {
    #[doc =
        "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_conditions`.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with any 5xx response code,\n  or if the backend service does not respond at all, example: disconnects, reset, read timeout,\n* connection failure, and refused streams.\n* gateway-error: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures connecting to backend services,\n  for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream:Loadbalancer will retry if the backend service resets the stream with a REFUSED_STREAM error code.\n  This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in the response header is set to unavailable"]
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
                        terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy {}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicyRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with any 5xx response code,\n  or if the backend service does not respond at all, example: disconnects, reset, read timeout,\n* connection failure, and refused streams.\n* gateway-error: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures connecting to backend services,\n  for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream:Loadbalancer will retry if the backend service resets the stream with a REFUSED_STREAM error code.\n  This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout {}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_TimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_TimeoutRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_TimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_template_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite {
    #[doc =
        "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host header is replaced\nwith contents of hostRewrite.\n\nThe value must be between 1 and 255 characters."]
    pub fn set_host_rewrite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching portion of the\nrequest's path is replaced by pathPrefixRewrite.\n\nThe value must be between 1 and 1024 characters."]
    pub fn set_path_prefix_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_prefix_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_template_rewrite`.\nIf specified, the pattern rewrites the URL path (based on the :path header) using the HTTP template syntax.\n\nA corresponding pathTemplateMatch must be specified. Any template variables must exist in the pathTemplateMatch field.\n\n* At least one variable must be specified in the pathTemplateMatch field\n* You can omit variables from the rewritten URL\n* The * and ** operators cannot be matched unless they have a corresponding variable name - e.g. {format=*} or {var=**}.\n\nFor example, a pathTemplateMatch of /static/{format=**} could be rewritten as /static/content/{format} to prefix\n/content to the URL. Variables can also be re-ordered in a rewrite, so that /{country}/{format}/{suffix=**} can be\nrewritten as /content/{format}/{country}/{suffix}.\n\nAt least one non-empty routeRules[].matchRules[].path_template_match is required.\n\nOnly one of pathPrefixRewrite or pathTemplateRewrite may be specified."]
    pub fn set_path_template_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_template_rewrite = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite {}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
            path_template_rewrite: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewriteRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewriteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host header is replaced\nwith contents of hostRewrite.\n\nThe value must be between 1 and 255 characters."]
    pub fn host_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching portion of the\nrequest's path is replaced by pathPrefixRewrite.\n\nThe value must be between 1 and 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_template_rewrite` after provisioning.\nIf specified, the pattern rewrites the URL path (based on the :path header) using the HTTP template syntax.\n\nA corresponding pathTemplateMatch must be specified. Any template variables must exist in the pathTemplateMatch field.\n\n* At least one variable must be specified in the pathTemplateMatch field\n* You can omit variables from the rewritten URL\n* The * and ** operators cannot be matched unless they have a corresponding variable name - e.g. {format=*} or {var=**}.\n\nFor example, a pathTemplateMatch of /static/{format=**} could be rewritten as /static/content/{format} to prefix\n/content to the URL. Variables can also be re-ordered in a rewrite, so that /{country}/{format}/{suffix=**} can be\nrewritten as /content/{format}/{country}/{suffix}.\n\nAt least one non-empty routeRules[].matchRules[].path_template_match is required.\n\nOnly one of pathPrefixRewrite or pathTemplateRewrite may be specified."]
    pub fn path_template_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_template_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
    #[doc = "Set the field `header_name`.\nThe name of the header to add."]
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
        "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn set_replace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
    #[doc = "Set the field `header_name`.\nThe name of the header to add."]
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
        "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn set_replace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
    type O =
        terrars
        ::BlockAssignable<
            ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderActionDynamic {
    request_headers_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd,
        >,
    >,
    response_headers_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd,
        >,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction {
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
            ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_add: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd,
        >,
    >,
    dynamic: ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderActionDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction {
    #[doc =
        "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request prior to\nforwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.request_headers_to_remove = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response prior to sending the\nresponse back to the client."]
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
                            ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd,
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
                            ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd,
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
::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction {}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderActionRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request prior to\nforwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_remove", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response prior to sending the\nresponse back to the client."]
    pub fn response_headers_to_remove(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_remove", self.base))
    }

    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> terrars
    ::ListRef<
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> terrars
    ::ListRef<
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServicesDynamic {
    header_action: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServicesDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices {
    #[doc =
        "Set the field `backend_service`.\nThe full or partial URL to the default BackendService resource. Before forwarding the\nrequest to backendService, the loadbalancer applies any relevant headerActions\nspecified as part of this backendServiceWeight."]
    pub fn set_backend_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backend_service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `weight`.\nSpecifies the fraction of traffic sent to backendService, computed as\nweight / (sum of all weightedBackendService weights in routeAction) .\n\nThe selection of a backend service is determined only for new traffic. Once a user's request\nhas been directed to a backendService, subsequent requests will be sent to the same backendService\nas determined by the BackendService's session affinity policy.\n\nThe value must be between 0 and 1000"]
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
                            ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderAction,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices {}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices {
            backend_service: core::default::Default::default(),
            weight: core::default::Default::default(),
            header_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServicesRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the default BackendService resource. Before forwarding the\nrequest to backendService, the loadbalancer applies any relevant headerActions\nspecified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as\nweight / (sum of all weightedBackendService weights in routeAction) .\n\nThe selection of a backend service is determined only for new traffic. Once a user's request\nhas been directed to a backendService, subsequent requests will be sent to the same backendService\nas determined by the BackendService's session affinity policy.\n\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_DefaultRouteActionDynamic {
    cors_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy>>,
    max_stream_duration: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy>>,
    retry_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy>>,
    timeout: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout>>,
    url_rewrite: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite>>,
    weighted_backend_services: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_stream_duration: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_backend_services: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_DefaultRouteActionDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteAction {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicy>>,
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicy>>,
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

    #[doc = "Set the field `max_stream_duration`.\n"]
    pub fn set_max_stream_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDuration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.max_stream_duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.max_stream_duration = std::option::Option::Some(d);
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
                        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicy>,
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicy>>,
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_Timeout>>,
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewrite>>,
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
                        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServices>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_DefaultRouteAction {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction {}

impl BuildComputeRegionUrlMap_PathMatcher_DefaultRouteAction {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteAction {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteAction {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            max_stream_duration: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_DefaultRouteActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_DefaultRouteActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_DefaultRouteActionRef {
        ComputeRegionUrlMap_PathMatcher_DefaultRouteActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_DefaultRouteActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_CorsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_DefaultRouteAction_FaultInjectionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\n"]
    pub fn max_stream_duration(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_MaxStreamDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_stream_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RequestMirrorPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_UrlRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction_WeightedBackendServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
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

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
    #[doc = "If true, specifies the CORS policy is disabled."]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
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

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
    http_status: terrars::PrimField<f64>,
    percentage: terrars::PrimField<f64>,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort { }

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
    #[doc = "The HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub http_status: terrars::PrimField<f64>,
    #[doc =
        "The percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub percentage: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
            http_status: self.http_status,
            percentage: self.percentage,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    type O =
        terrars
        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayDynamic {
    fixed_delay: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
    percentage: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std
    ::option
    ::Option<
        std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay>,
    >,
    dynamic: ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
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
                            ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
    #[doc =
        "The percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub percentage: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
            percentage: self.percentage,
            fixed_delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef {
    #[allow(dead_code)]
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
    ::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyDynamic {
    abort: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort>>,
    delay: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
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
                            ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort,
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
                            ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
    backend_service: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy { }

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
    #[doc = "The RegionBackendService resource being mirrored to."]
    pub backend_service: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
            backend_service: self.backend_service,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyDynamic {
    per_try_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
    #[doc = "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n- 5xx: Loadbalancer will attempt a retry if the backend service responds with\nany 5xx response code, or if the backend service does not respond at all,\nfor example: disconnects, reset, read timeout, connection failure, and refused\nstreams.\n- gateway-error: Similar to 5xx, but only applies to response codes\n502, 503 or 504.\n- connect-failure: Loadbalancer will retry on failures\nconnecting to backend services, for example due to connection timeouts.\n- retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\nCurrently the only retriable error supported is 409.\n- refused-stream: Loadbalancer will retry if the backend service resets the stream with a\nREFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n- cancelled: Loadbalancer will retry if the gRPC status code in the response\nheader is set to cancelled\n- deadline-exceeded: Loadbalancer will retry if the\ngRPC status code in the response header is set to deadline-exceeded\n- resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\nheader is set to resource-exhausted\n- unavailable: Loadbalancer will retry if\nthe gRPC status code in the response header is set to unavailable"]
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
                            ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n- 5xx: Loadbalancer will attempt a retry if the backend service responds with\nany 5xx response code, or if the backend service does not respond at all,\nfor example: disconnects, reset, read timeout, connection failure, and refused\nstreams.\n- gateway-error: Similar to 5xx, but only applies to response codes\n502, 503 or 504.\n- connect-failure: Loadbalancer will retry on failures\nconnecting to backend services, for example due to connection timeouts.\n- retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\nCurrently the only retriable error supported is 409.\n- refused-stream: Loadbalancer will retry if the backend service resets the stream with a\nREFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n- cancelled: Loadbalancer will retry if the gRPC status code in the response\nheader is set to cancelled\n- deadline-exceeded: Loadbalancer will retry if the\ngRPC status code in the response header is set to deadline-exceeded\n- resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\nheader is set to resource-exhausted\n- unavailable: Loadbalancer will retry if\nthe gRPC status code in the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd { }

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd {
    #[doc = "The name of the header."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc =
        "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd {
        ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAddRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAddRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<bool>,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd { }

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd {
    #[doc = "The name of the header."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc =
        "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd {
        ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAddRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAddRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderActionDynamic {
    request_headers_to_add: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd>>,
    response_headers_to_add: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_add: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_add: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderActionDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction {
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
                        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAdd>,
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
                            ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAdd,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction {}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction {
        ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderActionRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderActionRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_RequestHeadersToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction_ResponseHeadersToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesDynamic {
    header_action: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
    backend_service: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction>>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
    #[doc =
        "The default RegionBackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub backend_service: terrars::PrimField<std::string::String>,
    #[doc =
        "Specifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub weight: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
            backend_service: self.backend_service,
            weight: self.weight,
            header_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef {
    #[allow(dead_code)]
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
    pub fn header_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteActionDynamic {
    cors_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>>,
    retry_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy>>,
    timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
    url_rewrite: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite>>,
    weighted_backend_services: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_backend_services: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_PathRule_RouteActionDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy>>,
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
                        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy>,
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
                        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>,
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
                        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy>,
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite>>,
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
                            ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction {}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_RouteAction {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction {
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

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_RouteActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_RouteActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_RouteActionRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_RouteActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_RouteActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect {
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

impl ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect {
    #[doc =
        "If set to true, any accompanying query portion of the original URL is removed\nprior to redirecting the request. If set to false, the query portion of the\noriginal URL is retained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: terrars::PrimField<bool>,
}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect {
        ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirectRef {
        ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirectRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_PathMatcher_PathRuleDynamic {
    route_action: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction>>,
    url_redirect: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRule {
    paths: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_redirect: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_PathRuleDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_PathRule {
    #[doc =
        "Set the field `service`.\nThe region backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_action`.\n"]
    pub fn set_route_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirect>>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_PathRule {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_PathRule {
    #[doc =
        "The list of path patterns to match. Each must start with / and the only place a\n\\* is allowed is at the end following a /. The string fed to the path matcher\ndoes not include any text after the first ? or #, and those chars are not\nallowed here."]
    pub paths: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeRegionUrlMap_PathMatcher_PathRule {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_PathRule {
        ComputeRegionUrlMap_PathMatcher_PathRule {
            paths: self.paths,
            service: core::default::Default::default(),
            route_action: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_PathRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_PathRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMap_PathMatcher_PathRuleRef {
        ComputeRegionUrlMap_PathMatcher_PathRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_PathRuleRef {
    #[allow(dead_code)]
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
    pub fn route_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }

    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_UrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
    range_end: terrars::PrimField<f64>,
    range_start: terrars::PrimField<f64>,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch { }

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
    #[doc = "The end of the range (exclusive)."]
    pub range_end: terrars::PrimField<f64>,
    #[doc = "The start of the range (inclusive)."]
    pub range_start: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
            range_end: self.range_end,
            range_start: self.range_start,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesDynamic {
    range_match: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
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
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
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
                            ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
    #[doc =
        "The name of the HTTP header to match. For matching against the HTTP request's\nauthority, use a headerMatch with the header name \":authority\". For matching a\nrequest's method, use the headerName \":method\"."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
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

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range_match", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels { }

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
    #[doc =
        "Name of metadata label. The name can have a maximum length of 1024 characters\nand must be at least 1 character long."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The value of the label must match the specified value. value can have a maximum\nlength of 1024 characters."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersDynamic {
    filter_labels: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
    filter_match_criteria: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_labels: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
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
                            ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
    #[doc =
        "Specifies how individual filterLabel matches within the list of filterLabels\ncontribute towards the overall metadataFilter match. Supported values are:\n\n* MATCH_ANY: At least one of the filterLabels must have a matching label in the\nprovided metadata.\n* MATCH_ALL: All filterLabels must have matching labels in\nthe provided metadata. Possible values: [\"MATCH_ALL\", \"MATCH_ANY\"]"]
    pub filter_match_criteria: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
            filter_match_criteria: self.filter_match_criteria,
            filter_labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_labels", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_match: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
    #[doc =
        "The name of the query parameter to match. The query parameter must exist in the\nrequest, in the absence of which the request match fails."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
            exact_match: core::default::Default::default(),
            name: self.name,
            present_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRulesDynamic {
    header_matches: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches>>,
    metadata_filters: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters>>,
    query_parameter_matches: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    full_path_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_case: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_template_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_matches: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_filters: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameter_matches: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRulesDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules {
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
        "Set the field `path_template_match`.\nFor satisfying the matchRule condition, the path of the request\nmust match the wildcard pattern specified in pathTemplateMatch\nafter removing any query parameters and anchor that may be part\nof the original URL.\n\npathTemplateMatch must be between 1 and 255 characters\n(inclusive).  The pattern specified by pathTemplateMatch may\nhave at most 5 wildcard operators and at most 5 variable\ncaptures in total."]
    pub fn set_path_template_match(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_template_match = std::option::Option::Some(v.into());
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
                        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches>,
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
                        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters>,
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
                        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules {}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules {
            full_path_match: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            path_template_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
            header_matches: core::default::Default::default(),
            metadata_filters: core::default::Default::default(),
            query_parameter_matches: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRulesRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRulesRef {
    #[allow(dead_code)]
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
        "Get a reference to the value of field `path_template_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request\nmust match the wildcard pattern specified in pathTemplateMatch\nafter removing any query parameters and anchor that may be part\nof the original URL.\n\npathTemplateMatch must be between 1 and 255 characters\n(inclusive).  The pattern specified by pathTemplateMatch may\nhave at most 5 wildcard operators and at most 5 variable\ncaptures in total."]
    pub fn path_template_match(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_template_match", self.base))
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
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_matches", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_filters` after provisioning.\n"]
    pub fn metadata_filters(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `query_parameter_matches` after provisioning.\n"]
    pub fn query_parameter_matches(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_parameter_matches", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
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

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
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

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_status: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef {
    #[allow(dead_code)]
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

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayDynamic {
    fixed_delay: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std
    ::option
    ::Option<
        std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay>,
    >,
    dynamic: ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
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
                            ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef {
    #[allow(dead_code)]
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
    ::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyDynamic {
    abort: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort>,
    >,
    delay: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
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
                            ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort,
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
                            ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyDynamic {
    per_try_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
    num_retries: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
    #[doc =
        "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\n  any 5xx response code, or if the backend service does not respond at all,\n  for example: disconnects, reset, read timeout, connection failure, and refused\n  streams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n  502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\n  connecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\n  REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\n  header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\n  gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\n  header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in\n  the response header is set to unavailable"]
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
                            ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
    #[doc = "Specifies the allowed number retries. This number must be > 0."]
    pub num_retries: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
            num_retries: self.num_retries,
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn num_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\n  any 5xx response code, or if the backend service does not respond at all,\n  for example: disconnects, reset, read timeout, connection failure, and refused\n  streams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n  502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\n  connecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\n  REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\n  header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\n  gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\n  header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in\n  the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
    path_pattern: terrars::PrimField<std::string::String>,
    path_substitution: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite { }

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
    type O =
        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
    #[doc =
        "The regular expression used to match against the URL path.\nIt uses RE2 syntax with the following constraints:\n\n* Any single character operators are allowed.\n* Groups may only contain a submatch operator, and may not\n  contain character repetition (for example, '.*').\n* Character repetition (for example, '.*') may only be used in\n  a regex together with empty string operators, other\n  repetitions, ranges, and repetitions of ranges.\n* Ranges may only contain character ranges, digit ranges, and\n  symbols allowed for ranges."]
    pub path_pattern: terrars::PrimField<std::string::String>,
    #[doc =
        "The substitution used to rewrite the parts of the URL path\nmatched by pathPattern. May reference capture groups from\npathPattern."]
    pub path_substitution: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
            path_pattern: self.path_pattern,
            path_substitution: self.path_substitution,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path_pattern` after provisioning.\nThe regular expression used to match against the URL path.\nIt uses RE2 syntax with the following constraints:\n\n* Any single character operators are allowed.\n* Groups may only contain a submatch operator, and may not\n  contain character repetition (for example, '.*').\n* Character repetition (for example, '.*') may only be used in\n  a regex together with empty string operators, other\n  repetitions, ranges, and repetitions of ranges.\n* Ranges may only contain character ranges, digit ranges, and\n  symbols allowed for ranges."]
    pub fn path_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_pattern", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_substitution` after provisioning.\nThe substitution used to rewrite the parts of the URL path\nmatched by pathPattern. May reference capture groups from\npathPattern."]
    pub fn path_substitution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_substitution", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteDynamic {
    regex_rewrite: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_template_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_rewrite: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
    #[doc =
        "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn set_host_rewrite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters.\n\nOnly one of pathPrefixRewrite, pathTemplateRewrite, or regexRewrite may be\nspecified."]
    pub fn set_path_prefix_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_prefix_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_template_rewrite`.\nPrior to forwarding the request to the selected origin, if the\nrequest matched a pathTemplateMatch, the matching portion of the\nrequest's path is replaced re-written using the pattern specified\nby pathTemplateRewrite.\n\npathTemplateRewrite must be between 1 and 255 characters\n(inclusive), must start with a '/', and must only use variables\ncaptured by the route's pathTemplate matchers.\n\npathTemplateRewrite may only be used when all of a route's\nMatchRules specify pathTemplate.\n\nOnly one of pathPrefixRewrite, pathTemplateRewrite, or regexRewrite may be\nspecified."]
    pub fn set_path_template_rewrite(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.path_template_rewrite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex_rewrite`.\n"]
    pub fn set_regex_rewrite(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_rewrite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_rewrite = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
            path_template_rewrite: core::default::Default::default(),
            regex_rewrite: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn host_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters.\n\nOnly one of pathPrefixRewrite, pathTemplateRewrite, or regexRewrite may be\nspecified."]
    pub fn path_prefix_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_prefix_rewrite", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_template_rewrite` after provisioning.\nPrior to forwarding the request to the selected origin, if the\nrequest matched a pathTemplateMatch, the matching portion of the\nrequest's path is replaced re-written using the pattern specified\nby pathTemplateRewrite.\n\npathTemplateRewrite must be between 1 and 255 characters\n(inclusive), must start with a '/', and must only use variables\ncaptured by the route's pathTemplate matchers.\n\npathTemplateRewrite may only be used when all of a route's\nMatchRules specify pathTemplate.\n\nOnly one of pathPrefixRewrite, pathTemplateRewrite, or regexRewrite may be\nspecified."]
    pub fn path_template_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_template_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_rewrite` after provisioning.\n"]
    pub fn regex_rewrite(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteActionDynamic {
    cors_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>>,
    retry_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy>>,
    timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
    url_rewrite: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite>>,
    weighted_backend_services: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_backend_services: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_RouteRules_RouteActionDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy>,
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
                        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy>,
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
                        ::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>,
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
                        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy>,
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
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
                        terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite>,
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
                            ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction {}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction {
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

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_RouteActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_RouteActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_RouteActionRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_RouteActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_RouteActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect {
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

impl ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect {}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect {
        ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirectRef {
        ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirectRef {
    #[allow(dead_code)]
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
struct ComputeRegionUrlMap_PathMatcher_RouteRulesDynamic {
    header_action: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction>>,
    match_rules: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules>>,
    route_action: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction>>,
    url_redirect: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRules {
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_rules: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_redirect: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect>>,
    dynamic: ComputeRegionUrlMap_PathMatcher_RouteRulesDynamic,
}

impl ComputeRegionUrlMap_PathMatcher_RouteRules {
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
                    ::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderAction>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRules>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteAction>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirect>>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher_RouteRules {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher_RouteRules {
    #[doc =
        "For routeRules within a given pathMatcher, priority determines the order\nin which load balancer will interpret routeRules. RouteRules are evaluated\nin order of priority, from the lowest to highest number. The priority of\na rule decreases as its number increases (1, 2, 3, N+1). The first rule\nthat matches the request is applied.\n\nYou cannot configure two or more routeRules with the same priority.\nPriority for each rule must be set to a number between 0 and\n2147483647 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules\nin the future without affecting the rest of the rules. For example,\n1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers to which\nyou could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the\nfuture without any impact on existing rules."]
    pub priority: terrars::PrimField<f64>,
}

impl BuildComputeRegionUrlMap_PathMatcher_RouteRules {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher_RouteRules {
        ComputeRegionUrlMap_PathMatcher_RouteRules {
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

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcher_RouteRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcher_RouteRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMap_PathMatcher_RouteRulesRef {
        ComputeRegionUrlMap_PathMatcher_RouteRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcher_RouteRulesRef {
    #[allow(dead_code)]
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
    pub fn header_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_rules` after provisioning.\n"]
    pub fn match_rules(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_MatchRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_RouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }

    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRules_UrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionUrlMap_PathMatcherDynamic {
    default_route_action: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction>>,
    default_url_redirect: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_DefaultUrlRedirect>>,
    header_action: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_HeaderAction>>,
    path_rule: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_PathRule>>,
    route_rules: std::option::Option<terrars::DynamicBlock<ComputeRegionUrlMap_PathMatcher_RouteRules>>,
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_PathMatcher {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_route_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_url_redirect: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_DefaultUrlRedirect>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_HeaderAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_rule: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_PathRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_rules: std::option::Option<std::vec::Vec<ComputeRegionUrlMap_PathMatcher_RouteRules>>,
    dynamic: ComputeRegionUrlMap_PathMatcherDynamic,
}

impl ComputeRegionUrlMap_PathMatcher {
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

    #[doc = "Set the field `default_route_action`.\n"]
    pub fn set_default_route_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_DefaultRouteAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_route_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_route_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_url_redirect`.\n"]
    pub fn set_default_url_redirect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_DefaultUrlRedirect>>,
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

    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_HeaderAction>>,
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

    #[doc = "Set the field `path_rule`.\n"]
    pub fn set_path_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_PathRule>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher_RouteRules>>,
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

impl terrars::ToListMappable for ComputeRegionUrlMap_PathMatcher {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_PathMatcher>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_PathMatcher {
    #[doc = "The name to which this PathMatcher is referred by the HostRule."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_PathMatcher {
    pub fn build(self) -> ComputeRegionUrlMap_PathMatcher {
        ComputeRegionUrlMap_PathMatcher {
            default_service: core::default::Default::default(),
            description: core::default::Default::default(),
            name: self.name,
            default_route_action: core::default::Default::default(),
            default_url_redirect: core::default::Default::default(),
            header_action: core::default::Default::default(),
            path_rule: core::default::Default::default(),
            route_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_PathMatcherRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_PathMatcherRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMap_PathMatcherRef {
        ComputeRegionUrlMap_PathMatcherRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_PathMatcherRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_DefaultRouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.base))
    }

    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> terrars::ListRef<ComputeRegionUrlMap_DefaultUrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeRegionUrlMap_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `path_rule` after provisioning.\n"]
    pub fn path_rule(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_PathRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `route_rules` after provisioning.\n"]
    pub fn route_rules(&self) -> terrars::ListRef<ComputeRegionUrlMap_PathMatcher_RouteRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionUrlMap_Test {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    host: terrars::PrimField<std::string::String>,
    path: terrars::PrimField<std::string::String>,
    service: terrars::PrimField<std::string::String>,
}

impl ComputeRegionUrlMap_Test {
    #[doc = "Set the field `description`.\nDescription of this test case."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionUrlMap_Test {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_Test>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_Test {
    #[doc = "Host portion of the URL."]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = "Path portion of the URL."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "A reference to expected RegionBackendService resource the given URL should be mapped to."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionUrlMap_Test {
    pub fn build(self) -> ComputeRegionUrlMap_Test {
        ComputeRegionUrlMap_Test {
            description: core::default::Default::default(),
            host: self.host,
            path: self.path,
            service: self.service,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_TestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_TestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMap_TestRef {
        ComputeRegionUrlMap_TestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_TestRef {
    #[allow(dead_code)]
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
pub struct ComputeRegionUrlMap_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionUrlMap_Timeouts {
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

impl terrars::ToListMappable for ComputeRegionUrlMap_Timeouts {
    type O = terrars::BlockAssignable<ComputeRegionUrlMap_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionUrlMap_Timeouts {}

impl BuildComputeRegionUrlMap_Timeouts {
    pub fn build(self) -> ComputeRegionUrlMap_Timeouts {
        ComputeRegionUrlMap_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionUrlMap_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionUrlMap_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionUrlMap_TimeoutsRef {
        ComputeRegionUrlMap_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionUrlMap_TimeoutsRef {
    #[allow(dead_code)]
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
