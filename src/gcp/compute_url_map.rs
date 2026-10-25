#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeUrlMapData_ {
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
    default_custom_error_response_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_DefaultCustomErrorResponsePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_route_action: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_url_redirect: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultUrlRedirect>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeUrlMap_HeaderAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rule: std::option::Option<std::vec::Vec<ComputeUrlMap_HostRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_matcher: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test: std::option::Option<std::vec::Vec<ComputeUrlMap_Test>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeUrlMap_Timeouts>,
    dynamic: ComputeUrlMapDynamic,
}

struct ComputeUrlMap_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeUrlMapData_>,
}

#[derive(Clone)]
pub struct ComputeUrlMap(std::rc::Rc<ComputeUrlMap_>);

impl ComputeUrlMap {
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
        "Set the field `default_service`.\nThe backend service or backend bucket to use when none of the given rules match."]
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
        "Set the field `description`.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
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

    #[doc = "Set the field `default_custom_error_response_policy`.\n"]
    pub fn set_default_custom_error_response_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultCustomErrorResponsePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_custom_error_response_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_custom_error_response_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_route_action`.\n"]
    pub fn set_default_route_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultUrlRedirect>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_HeaderAction>>,
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
    pub fn set_host_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_HostRule>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher>>,
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
    pub fn set_test(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_Test>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeUrlMap_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_service` after provisioning.\nThe backend service or backend bucket to use when none of the given rules match."]
    pub fn default_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this object. This\nfield is used in optimistic locking."]
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
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a lowercase\nletter, and all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_custom_error_response_policy` after provisioning.\n"]
    pub fn default_custom_error_response_policy(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_DefaultCustomErrorResponsePolicyRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.default_custom_error_response_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> terrars::ListRef<ComputeUrlMap_DefaultUrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeUrlMap_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> terrars::ListRef<ComputeUrlMap_TestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeUrlMap_TimeoutsRef {
        ComputeUrlMap_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeUrlMap {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeUrlMap { }

impl terrars::ToListMappable for ComputeUrlMap {
    type O = terrars::ListRef<ComputeUrlMapRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeUrlMap_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_url_map".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeUrlMap {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a lowercase\nletter, and all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeUrlMap {
        let out = ComputeUrlMap(std::rc::Rc::new(ComputeUrlMap_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeUrlMapData_ {
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
                default_custom_error_response_policy: core::default::Default::default(),
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

pub struct ComputeUrlMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeUrlMapRef {
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
        "Get a reference to the value of field `default_service` after provisioning.\nThe backend service or backend bucket to use when none of the given rules match."]
    pub fn default_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this object. This\nfield is used in optimistic locking."]
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
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The\nname must be 1-63 characters long, and comply with RFC1035. Specifically, the\nname must be 1-63 characters long and match the regular expression\n'[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character must be a lowercase\nletter, and all following characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_custom_error_response_policy` after provisioning.\n"]
    pub fn default_custom_error_response_policy(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_DefaultCustomErrorResponsePolicyRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.default_custom_error_response_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> terrars::ListRef<ComputeUrlMap_DefaultUrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeUrlMap_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_matcher", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> terrars::ListRef<ComputeUrlMap_TestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeUrlMap_TimeoutsRef {
        ComputeUrlMap_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMapDynamic {
    default_custom_error_response_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultCustomErrorResponsePolicy>>,
    default_route_action: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction>>,
    default_url_redirect: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultUrlRedirect>>,
    header_action: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_HeaderAction>>,
    host_rule: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_HostRule>>,
    path_matcher: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher>>,
    test: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_Test>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_response_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_response_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
    #[doc =
        "Set the field `match_response_codes`.\nValid values include:\n- A number between 400 and 599: For example 401 or 503, in which case the load balancer applies the policy if the error code exactly matches this value.\n- 5xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 500 to 599.\n- 4xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 400 to 499.\nValues must be unique within matchResponseCodes and across all errorResponseRules of CustomErrorResponsePolicy."]
    pub fn set_match_response_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_response_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `override_response_code`.\nThe HTTP status code returned with the response containing the custom error content.\nIf overrideResponseCode is not supplied, the same response code returned by the original backend bucket or backend service is returned to the client."]
    pub fn set_override_response_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.override_response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nThe full path to a file within backendBucket. For example: /errors/defaultError.html\npath must start with a leading slash. path cannot have trailing slashes.\nIf the file is not available in backendBucket or the load balancer cannot reach the BackendBucket, a simple Not Found Error is returned to the client.\nThe value must be from 1 to 1024 characters."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule {}

impl BuildComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
    pub fn build(self) -> ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
        ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
            match_response_codes: core::default::Default::default(),
            override_response_code: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef {
        ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `match_response_codes` after provisioning.\nValid values include:\n- A number between 400 and 599: For example 401 or 503, in which case the load balancer applies the policy if the error code exactly matches this value.\n- 5xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 500 to 599.\n- 4xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 400 to 499.\nValues must be unique within matchResponseCodes and across all errorResponseRules of CustomErrorResponsePolicy."]
    pub fn match_response_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_response_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `override_response_code` after provisioning.\nThe HTTP status code returned with the response containing the custom error content.\nIf overrideResponseCode is not supplied, the same response code returned by the original backend bucket or backend service is returned to the client."]
    pub fn override_response_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe full path to a file within backendBucket. For example: /errors/defaultError.html\npath must start with a leading slash. path cannot have trailing slashes.\nIf the file is not available in backendBucket or the load balancer cannot reach the BackendBucket, a simple Not Found Error is returned to the client.\nThe value must be from 1 to 1024 characters."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_DefaultCustomErrorResponsePolicyDynamic {
    error_response_rule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultCustomErrorResponsePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_response_rule: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule>>,
    dynamic: ComputeUrlMap_DefaultCustomErrorResponsePolicyDynamic,
}

impl ComputeUrlMap_DefaultCustomErrorResponsePolicy {
    #[doc =
        "Set the field `error_service`.\nThe full or partial URL to the BackendBucket resource that contains the custom error content. Examples are:\n\nhttps://www.googleapis.com/compute/v1/projects/project/global/backendBuckets/myBackendBucket\ncompute/v1/projects/project/global/backendBuckets/myBackendBucket\nglobal/backendBuckets/myBackendBucket\n\nIf errorService is not specified at lower levels like pathMatcher, pathRule and routeRule, an errorService specified at a higher level in the UrlMap will be used. If UrlMap.defaultCustomErrorResponsePolicy contains one or more errorResponseRules[], it must specify errorService.\nIf load balancer cannot reach the backendBucket, a simple Not Found Error will be returned, with the original response code (or overrideResponseCode if configured)."]
    pub fn set_error_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_response_rule`.\n"]
    pub fn set_error_response_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRule>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_response_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_response_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultCustomErrorResponsePolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultCustomErrorResponsePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultCustomErrorResponsePolicy {}

impl BuildComputeUrlMap_DefaultCustomErrorResponsePolicy {
    pub fn build(self) -> ComputeUrlMap_DefaultCustomErrorResponsePolicy {
        ComputeUrlMap_DefaultCustomErrorResponsePolicy {
            error_service: core::default::Default::default(),
            error_response_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultCustomErrorResponsePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultCustomErrorResponsePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultCustomErrorResponsePolicyRef {
        ComputeUrlMap_DefaultCustomErrorResponsePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultCustomErrorResponsePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `error_service` after provisioning.\nThe full or partial URL to the BackendBucket resource that contains the custom error content. Examples are:\n\nhttps://www.googleapis.com/compute/v1/projects/project/global/backendBuckets/myBackendBucket\ncompute/v1/projects/project/global/backendBuckets/myBackendBucket\nglobal/backendBuckets/myBackendBucket\n\nIf errorService is not specified at lower levels like pathMatcher, pathRule and routeRule, an errorService specified at a higher level in the UrlMap will be used. If UrlMap.defaultCustomErrorResponsePolicy contains one or more errorResponseRules[], it must specify errorService.\nIf load balancer cannot reach the backendBucket, a simple Not Found Error will be returned, with the original response code (or overrideResponseCode if configured)."]
    pub fn error_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_service", self.base))
    }

    #[doc = "Get a reference to the value of field `error_response_rule` after provisioning.\n"]
    pub fn error_response_rule(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_response_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_query_parameters: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_host: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_protocol: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_query_string: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_cookie_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_header_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_query_parameters: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy {
    #[doc =
        "Set the field `excluded_query_parameters`.\nNames of query string parameters to exclude in cache keys. All other\nparameters will be included. Either specify excludedQueryParameters\nor includedQueryParameters, not both. '&' and '=' will be percent\nencoded and not treated as delimiters. Note: This field applies to\nroutes that use backend services. Attempting to set it on a route that\npoints exclusively to Backend Buckets will result in a configuration\nerror. For routes that point to a Backend Bucket, use\nincludedQueryParameters to define which parameters should be part of\nthe cache key."]
    pub fn set_excluded_query_parameters(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_query_parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_host`.\nIf true, requests to different hosts will be cached separately. Note:\nThis setting is only applicable to routes that use a Backend Service.\nIt does not affect requests served by a Backend Bucket, as the host is\nnever included in a Backend Bucket's cache key. Attempting to set it on\na route that points exclusively to Backend Buckets will result in a\nconfiguration error."]
    pub fn set_include_host(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_protocol`.\nIf true, http and https requests will be cached separately. Note: This\nsetting is only applicable to routes that use a Backend Service. It\ndoes not affect requests served by a Backend Bucket, as the protocol is\nnever included in a Backend Bucket's cache key. Attempting to set on a\nroute that points exclusively to Backend Buckets will result in a\nconfiguration error."]
    pub fn set_include_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_query_string`.\nIf true, include query string parameters in the cache key according to\nincludedQueryParameters and excludedQueryParameters. If neither is\nset, the entire query string will be included. If false, the query\nstring will be excluded from the cache key entirely. Note: This field\napplies to routes that use backend services. Attempting to set it on a\nroute that points exclusively to Backend Buckets will result in a\nconfiguration error. For routes that point to a Backend Bucket, use\nincludedQueryParameters to define which parameters should be part of\nthe cache key."]
    pub fn set_include_query_string(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_query_string = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `included_cookie_names`.\nAllows HTTP cookies (by name) to be used in the cache key. The\nname=value pair will be used in the cache key Cloud CDN generates.\nNote: This setting is only applicable to routes that use a Backend\nService. It does not affect requests served by a Backend Bucket.\nAttempting to set it on a route that points exclusively to Backend\nBuckets will result in a configuration error. Up to 5 cookie names can\nbe specified."]
    pub fn set_included_cookie_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_cookie_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `included_header_names`.\nAllows HTTP request headers (by name) to be used in the cache key."]
    pub fn set_included_header_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_header_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `included_query_parameters`.\nNames of query string parameters to include in cache keys. All other\nparameters will be excluded. Either specify includedQueryParameters\nor excludedQueryParameters, not both. '&' and '=' will be percent\nencoded and not treated as delimiters."]
    pub fn set_included_query_parameters(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_query_parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy {}

impl BuildComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy {
            excluded_query_parameters: core::default::Default::default(),
            include_host: core::default::Default::default(),
            include_protocol: core::default::Default::default(),
            include_query_string: core::default::Default::default(),
            included_cookie_names: core::default::Default::default(),
            included_header_names: core::default::Default::default(),
            included_query_parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicyRef {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `excluded_query_parameters` after provisioning.\nNames of query string parameters to exclude in cache keys. All other\nparameters will be included. Either specify excludedQueryParameters\nor includedQueryParameters, not both. '&' and '=' will be percent\nencoded and not treated as delimiters. Note: This field applies to\nroutes that use backend services. Attempting to set it on a route that\npoints exclusively to Backend Buckets will result in a configuration\nerror. For routes that point to a Backend Bucket, use\nincludedQueryParameters to define which parameters should be part of\nthe cache key."]
    pub fn excluded_query_parameters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_query_parameters", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_host` after provisioning.\nIf true, requests to different hosts will be cached separately. Note:\nThis setting is only applicable to routes that use a Backend Service.\nIt does not affect requests served by a Backend Bucket, as the host is\nnever included in a Backend Bucket's cache key. Attempting to set it on\na route that points exclusively to Backend Buckets will result in a\nconfiguration error."]
    pub fn include_host(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_protocol` after provisioning.\nIf true, http and https requests will be cached separately. Note: This\nsetting is only applicable to routes that use a Backend Service. It\ndoes not affect requests served by a Backend Bucket, as the protocol is\nnever included in a Backend Bucket's cache key. Attempting to set on a\nroute that points exclusively to Backend Buckets will result in a\nconfiguration error."]
    pub fn include_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_query_string` after provisioning.\nIf true, include query string parameters in the cache key according to\nincludedQueryParameters and excludedQueryParameters. If neither is\nset, the entire query string will be included. If false, the query\nstring will be excluded from the cache key entirely. Note: This field\napplies to routes that use backend services. Attempting to set it on a\nroute that points exclusively to Backend Buckets will result in a\nconfiguration error. For routes that point to a Backend Bucket, use\nincludedQueryParameters to define which parameters should be part of\nthe cache key."]
    pub fn include_query_string(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_query_string", self.base))
    }

    #[doc =
        "Get a reference to the value of field `included_cookie_names` after provisioning.\nAllows HTTP cookies (by name) to be used in the cache key. The\nname=value pair will be used in the cache key Cloud CDN generates.\nNote: This setting is only applicable to routes that use a Backend\nService. It does not affect requests served by a Backend Bucket.\nAttempting to set it on a route that points exclusively to Backend\nBuckets will result in a configuration error. Up to 5 cookie names can\nbe specified."]
    pub fn included_cookie_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_cookie_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `included_header_names` after provisioning.\nAllows HTTP request headers (by name) to be used in the cache key."]
    pub fn included_header_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_header_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `included_query_parameters` after provisioning.\nNames of query string parameters to include in cache keys. All other\nparameters will be excluded. Either specify includedQueryParameters\nor excludedQueryParameters, not both. '&' and '=' will be percent\nencoded and not treated as delimiters."]
    pub fn included_query_parameters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_query_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtlRef {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtlRef {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtlRef {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_TtlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_TtlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_TtlRef {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_TtlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_TtlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicyDynamic {
    ttl: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl>>,
    dynamic: ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicyDynamic,
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy {
    #[doc =
        "Set the field `code`.\nThe HTTP status code to define a TTL against. Only HTTP status codes\n300, 301, 302, 307, 308, 404, 405, 410, 421, 451 and 501 can be\nspecified as values, and you cannot specify a status code more than\nonce."]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_Ttl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ttl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ttl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy {}

impl BuildComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy {
            code: core::default::Default::default(),
            ttl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicyRef {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nThe HTTP status code to define a TTL against. Only HTTP status codes\n300, 301, 302, 307, 308, 404, 405, 410, 421, 451 and 501 can be\nspecified as values, and you cannot specify a status code more than\nonce."]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy_TtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStaleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStaleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStaleRef {
        ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStaleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStaleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_DefaultRouteAction_CachePolicyDynamic {
    cache_key_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy>>,
    client_ttl: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl>>,
    default_ttl: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl>>,
    max_ttl: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl>>,
    negative_caching_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy>>,
    serve_while_stale: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_bypass_request_header_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negative_caching: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_coalescing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_ttl: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ttl: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_ttl: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negative_caching_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serve_while_stale: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale>>,
    dynamic: ComputeUrlMap_DefaultRouteAction_CachePolicyDynamic,
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicy {
    #[doc =
        "Set the field `cache_bypass_request_header_names`.\nBypass the cache when the specified request headers are matched by name,\ne.g. Pragma or Authorization headers. Values are case-insensitive. Up to 5\nheader names can be specified. The cache is bypassed for all cacheMode\nvalues."]
    pub fn set_cache_bypass_request_header_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cache_bypass_request_header_names = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cache_mode`.\nSpecifies the cache setting for all responses from this route. If not\nspecified, Cloud CDN uses CACHE_ALL_STATIC mode. Possible values: [\"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"CACHE_ALL_STATIC\"]"]
    pub fn set_cache_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `negative_caching`.\nNegative caching allows per-status code TTLs to be set, in order to apply\nfine-grained caching for common errors or redirects. This can reduce the\nload on your origin and improve end-user experience by reducing response\nlatency. When the cacheMode is set to CACHE_ALL_STATIC or\nUSE_ORIGIN_HEADERS, negative caching applies to responses with the\nspecified response code that lack any Cache-Control, Expires, or\nPragma: no-cache directives. When the cacheMode is set to\nFORCE_CACHE_ALL, negative caching applies to all responses with the\nspecified response code, and overrides any caching headers. By default,\nCloud CDN applies the following TTLs to these HTTP status codes:\n* 300 (Multiple Choice), 301, 308 (Permanent Redirects): 10m\n* 404 (Not Found), 410 (Gone), 451 (Unavailable For Legal Reasons): 120s\n* 405 (Method Not Found), 501 (Not Implemented): 60s\nThese defaults can be overridden in negativeCachingPolicy. If not\nspecified, Cloud CDN applies negative caching by default."]
    pub fn set_negative_caching(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.negative_caching = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_coalescing`.\nIf true then Cloud CDN will combine multiple concurrent cache fill\nrequests into a small number of requests to the origin. If not specified,\nCloud CDN applies request coalescing by default."]
    pub fn set_request_coalescing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.request_coalescing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_key_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_key_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_ttl`.\n"]
    pub fn set_client_ttl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_ttl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_ttl = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_ttl`.\n"]
    pub fn set_default_ttl(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_ttl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_ttl = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `max_ttl`.\n"]
    pub fn set_max_ttl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.max_ttl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.max_ttl = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `negative_caching_policy`.\n"]
    pub fn set_negative_caching_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicy>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.negative_caching_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.negative_caching_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `serve_while_stale`.\n"]
    pub fn set_serve_while_stale(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStale>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.serve_while_stale = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.serve_while_stale = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_CachePolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_CachePolicy {}

impl BuildComputeUrlMap_DefaultRouteAction_CachePolicy {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_CachePolicy {
        ComputeUrlMap_DefaultRouteAction_CachePolicy {
            cache_bypass_request_header_names: core::default::Default::default(),
            cache_mode: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            request_coalescing: core::default::Default::default(),
            cache_key_policy: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            serve_while_stale: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_CachePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_CachePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_CachePolicyRef {
        ComputeUrlMap_DefaultRouteAction_CachePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_CachePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache_bypass_request_header_names` after provisioning.\nBypass the cache when the specified request headers are matched by name,\ne.g. Pragma or Authorization headers. Values are case-insensitive. Up to 5\nheader names can be specified. The cache is bypassed for all cacheMode\nvalues."]
    pub fn cache_bypass_request_header_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_bypass_request_header_names", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cache_mode` after provisioning.\nSpecifies the cache setting for all responses from this route. If not\nspecified, Cloud CDN uses CACHE_ALL_STATIC mode. Possible values: [\"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"CACHE_ALL_STATIC\"]"]
    pub fn cache_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `negative_caching` after provisioning.\nNegative caching allows per-status code TTLs to be set, in order to apply\nfine-grained caching for common errors or redirects. This can reduce the\nload on your origin and improve end-user experience by reducing response\nlatency. When the cacheMode is set to CACHE_ALL_STATIC or\nUSE_ORIGIN_HEADERS, negative caching applies to responses with the\nspecified response code that lack any Cache-Control, Expires, or\nPragma: no-cache directives. When the cacheMode is set to\nFORCE_CACHE_ALL, negative caching applies to all responses with the\nspecified response code, and overrides any caching headers. By default,\nCloud CDN applies the following TTLs to these HTTP status codes:\n* 300 (Multiple Choice), 301, 308 (Permanent Redirects): 10m\n* 404 (Not Found), 410 (Gone), 451 (Unavailable For Legal Reasons): 120s\n* 405 (Method Not Found), 501 (Not Implemented): 60s\nThese defaults can be overridden in negativeCachingPolicy. If not\nspecified, Cloud CDN applies negative caching by default."]
    pub fn negative_caching(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.negative_caching", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_coalescing` after provisioning.\nIf true then Cloud CDN will combine multiple concurrent cache fill\nrequests into a small number of requests to the origin. If not specified,\nCloud CDN applies request coalescing by default."]
    pub fn request_coalescing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_coalescing", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CachePolicy_CacheKeyPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_key_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `client_ttl` after provisioning.\n"]
    pub fn client_ttl(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CachePolicy_ClientTtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CachePolicy_DefaultTtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CachePolicy_MaxTtlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `negative_caching_policy` after provisioning.\n"]
    pub fn negative_caching_policy(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CachePolicy_NegativeCachingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.negative_caching_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `serve_while_stale` after provisioning.\n"]
    pub fn serve_while_stale(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CachePolicy_ServeWhileStaleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.serve_while_stale", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_CorsPolicy {
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

impl ComputeUrlMap_DefaultRouteAction_CorsPolicy {
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_CorsPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CorsPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_CorsPolicy {}

impl BuildComputeUrlMap_DefaultRouteAction_CorsPolicy {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_CorsPolicy {
        ComputeUrlMap_DefaultRouteAction_CorsPolicy {
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
pub struct ComputeUrlMap_DefaultRouteAction_CorsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_CorsPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_CorsPolicyRef {
        ComputeUrlMap_DefaultRouteAction_CorsPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_CorsPolicyRef {
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
pub struct ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_status: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {}

impl BuildComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
        ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef {
        ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef {
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
pub struct ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {}

impl BuildComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
        ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
        ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
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
struct ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayDynamic {
    fixed_delay: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay>>,
    dynamic: ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayDynamic,
}

impl ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
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
                        ::BlockAssignable<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelay>,
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {}

impl BuildComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
        ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef {
        ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef {
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
    ) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay_FixedDelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicyDynamic {
    abort: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort>>,
    delay: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay>>,
    dynamic: ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicyDynamic,
}

impl ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Abort>>,
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
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_Delay>>,
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy {}

impl BuildComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy {
        ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicyRef {
        ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_AbortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy_DelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_MaxStreamDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_DefaultRouteAction_MaxStreamDuration {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_MaxStreamDuration {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_MaxStreamDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_MaxStreamDuration {
    #[doc =
        "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_DefaultRouteAction_MaxStreamDuration {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_MaxStreamDuration {
        ComputeUrlMap_DefaultRouteAction_MaxStreamDuration {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_MaxStreamDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_MaxStreamDurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_MaxStreamDurationRef {
        ComputeUrlMap_DefaultRouteAction_MaxStreamDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_MaxStreamDurationRef {
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
pub struct ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy {
    backend_service: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy { }

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy {
    #[doc = "The full or partial URL to the BackendService resource being mirrored to."]
    pub backend_service: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy {
        ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy { backend_service: self.backend_service }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicyRef {
        ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicyRef {
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

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {}

impl BuildComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
        ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef {
        ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef {
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
struct ComputeUrlMap_DefaultRouteAction_RetryPolicyDynamic {
    per_try_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>>,
    dynamic: ComputeUrlMap_DefaultRouteAction_RetryPolicyDynamic,
}

impl ComputeUrlMap_DefaultRouteAction_RetryPolicy {
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
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeout>>,
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_RetryPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_RetryPolicy {}

impl BuildComputeUrlMap_DefaultRouteAction_RetryPolicy {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_RetryPolicy {
        ComputeUrlMap_DefaultRouteAction_RetryPolicy {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_RetryPolicyRef {
        ComputeUrlMap_DefaultRouteAction_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_RetryPolicyRef {
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
    pub fn per_try_timeout(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_RetryPolicy_PerTryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeUrlMap_DefaultRouteAction_Timeout {
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_Timeout {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_Timeout {}

impl BuildComputeUrlMap_DefaultRouteAction_Timeout {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_Timeout {
        ComputeUrlMap_DefaultRouteAction_Timeout {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_TimeoutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_DefaultRouteAction_TimeoutRef {
        ComputeUrlMap_DefaultRouteAction_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_TimeoutRef {
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
pub struct ComputeUrlMap_DefaultRouteAction_UrlRewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeUrlMap_DefaultRouteAction_UrlRewrite {
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
}

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_UrlRewrite {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_UrlRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_UrlRewrite {}

impl BuildComputeUrlMap_DefaultRouteAction_UrlRewrite {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_UrlRewrite {
        ComputeUrlMap_DefaultRouteAction_UrlRewrite {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_UrlRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_UrlRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_UrlRewriteRef {
        ComputeUrlMap_DefaultRouteAction_UrlRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_UrlRewriteRef {
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
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
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
::ToListMappable for ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
    type O =
        terrars
        ::BlockAssignable<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {}

impl BuildComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
        ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef {
        ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef {
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
pub struct ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
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
::ToListMappable for ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
    type O =
        terrars
        ::BlockAssignable<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {}

impl BuildComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
        ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef {
        ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef {
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
struct ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderActionDynamic {
    request_headers_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd>,
    >,
    response_headers_to_add: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_add: std
    ::option
    ::Option<
        std::vec::Vec<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_add: std
    ::option
    ::Option<
        std::vec::Vec<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd>,
    >,
    dynamic: ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderActionDynamic,
}

impl ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction {
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
                            ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAdd,
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
                            ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAdd,
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction {}

impl BuildComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction {
        ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderActionRef {
        ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderActionRef {
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
    ::ListRef<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_RequestHeadersToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> terrars
    ::ListRef<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction_ResponseHeadersToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_DefaultRouteAction_WeightedBackendServicesDynamic {
    header_action: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction_WeightedBackendServices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backend_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction>>,
    dynamic: ComputeUrlMap_DefaultRouteAction_WeightedBackendServicesDynamic,
}

impl ComputeUrlMap_DefaultRouteAction_WeightedBackendServices {
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
                        ::BlockAssignable<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderAction>,
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction_WeightedBackendServices {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction_WeightedBackendServices {}

impl BuildComputeUrlMap_DefaultRouteAction_WeightedBackendServices {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction_WeightedBackendServices {
        ComputeUrlMap_DefaultRouteAction_WeightedBackendServices {
            backend_service: core::default::Default::default(),
            weight: core::default::Default::default(),
            header_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_DefaultRouteAction_WeightedBackendServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteAction_WeightedBackendServicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_DefaultRouteAction_WeightedBackendServicesRef {
        ComputeUrlMap_DefaultRouteAction_WeightedBackendServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteAction_WeightedBackendServicesRef {
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
    ) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_DefaultRouteActionDynamic {
    cache_policy: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CachePolicy>>,
    cors_policy: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CorsPolicy>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy>>,
    max_stream_duration: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_MaxStreamDuration>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_RetryPolicy>>,
    timeout: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_Timeout>>,
    url_rewrite: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_UrlRewrite>>,
    weighted_backend_services: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultRouteAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CachePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CorsPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_stream_duration: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_MaxStreamDuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_Timeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_UrlRewrite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_backend_services: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices>>,
    dynamic: ComputeUrlMap_DefaultRouteActionDynamic,
}

impl ComputeUrlMap_DefaultRouteAction {
    #[doc = "Set the field `cache_policy`.\n"]
    pub fn set_cache_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CorsPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_MaxStreamDuration>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_RetryPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_Timeout>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_UrlRewrite>>,
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
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_WeightedBackendServices>>,
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultRouteAction {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultRouteAction {}

impl BuildComputeUrlMap_DefaultRouteAction {
    pub fn build(self) -> ComputeUrlMap_DefaultRouteAction {
        ComputeUrlMap_DefaultRouteAction {
            cache_policy: core::default::Default::default(),
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
pub struct ComputeUrlMap_DefaultRouteActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultRouteActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_DefaultRouteActionRef {
        ComputeUrlMap_DefaultRouteActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultRouteActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_policy` after provisioning.\n"]
    pub fn cache_policy(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CachePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CorsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_FaultInjectionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\n"]
    pub fn max_stream_duration(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_MaxStreamDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_stream_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_RequestMirrorPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_UrlRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_WeightedBackendServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_DefaultUrlRedirect {
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

impl ComputeUrlMap_DefaultUrlRedirect {
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

impl terrars::ToListMappable for ComputeUrlMap_DefaultUrlRedirect {
    type O = terrars::BlockAssignable<ComputeUrlMap_DefaultUrlRedirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_DefaultUrlRedirect {
    #[doc =
        "If set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained. The default is set to false.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: terrars::PrimField<bool>,
}

impl BuildComputeUrlMap_DefaultUrlRedirect {
    pub fn build(self) -> ComputeUrlMap_DefaultUrlRedirect {
        ComputeUrlMap_DefaultUrlRedirect {
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
pub struct ComputeUrlMap_DefaultUrlRedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_DefaultUrlRedirectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_DefaultUrlRedirectRef {
        ComputeUrlMap_DefaultUrlRedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_DefaultUrlRedirectRef {
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
        "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained. The default is set to false.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_HeaderAction_RequestHeadersToAdd {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<bool>,
}

impl ComputeUrlMap_HeaderAction_RequestHeadersToAdd { }

impl terrars::ToListMappable for ComputeUrlMap_HeaderAction_RequestHeadersToAdd {
    type O = terrars::BlockAssignable<ComputeUrlMap_HeaderAction_RequestHeadersToAdd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_HeaderAction_RequestHeadersToAdd {
    #[doc = "The name of the header."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc =
        "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: terrars::PrimField<bool>,
}

impl BuildComputeUrlMap_HeaderAction_RequestHeadersToAdd {
    pub fn build(self) -> ComputeUrlMap_HeaderAction_RequestHeadersToAdd {
        ComputeUrlMap_HeaderAction_RequestHeadersToAdd {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_HeaderAction_RequestHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_HeaderAction_RequestHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_HeaderAction_RequestHeadersToAddRef {
        ComputeUrlMap_HeaderAction_RequestHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_HeaderAction_RequestHeadersToAddRef {
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
pub struct ComputeUrlMap_HeaderAction_ResponseHeadersToAdd {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    replace: terrars::PrimField<bool>,
}

impl ComputeUrlMap_HeaderAction_ResponseHeadersToAdd { }

impl terrars::ToListMappable for ComputeUrlMap_HeaderAction_ResponseHeadersToAdd {
    type O = terrars::BlockAssignable<ComputeUrlMap_HeaderAction_ResponseHeadersToAdd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_HeaderAction_ResponseHeadersToAdd {
    #[doc = "The name of the header."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc =
        "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: terrars::PrimField<bool>,
}

impl BuildComputeUrlMap_HeaderAction_ResponseHeadersToAdd {
    pub fn build(self) -> ComputeUrlMap_HeaderAction_ResponseHeadersToAdd {
        ComputeUrlMap_HeaderAction_ResponseHeadersToAdd {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_HeaderAction_ResponseHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_HeaderAction_ResponseHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_HeaderAction_ResponseHeadersToAddRef {
        ComputeUrlMap_HeaderAction_ResponseHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_HeaderAction_ResponseHeadersToAddRef {
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
struct ComputeUrlMap_HeaderActionDynamic {
    request_headers_to_add: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_HeaderAction_RequestHeadersToAdd>>,
    response_headers_to_add: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_HeaderAction_ResponseHeadersToAdd>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_HeaderAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_remove: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_add: std::option::Option<std::vec::Vec<ComputeUrlMap_HeaderAction_RequestHeadersToAdd>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_headers_to_add: std::option::Option<std::vec::Vec<ComputeUrlMap_HeaderAction_ResponseHeadersToAdd>>,
    dynamic: ComputeUrlMap_HeaderActionDynamic,
}

impl ComputeUrlMap_HeaderAction {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_HeaderAction_RequestHeadersToAdd>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_HeaderAction_ResponseHeadersToAdd>>,
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

impl terrars::ToListMappable for ComputeUrlMap_HeaderAction {
    type O = terrars::BlockAssignable<ComputeUrlMap_HeaderAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_HeaderAction {}

impl BuildComputeUrlMap_HeaderAction {
    pub fn build(self) -> ComputeUrlMap_HeaderAction {
        ComputeUrlMap_HeaderAction {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_HeaderActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_HeaderActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_HeaderActionRef {
        ComputeUrlMap_HeaderActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_HeaderActionRef {
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
    pub fn request_headers_to_add(&self) -> terrars::ListRef<ComputeUrlMap_HeaderAction_RequestHeadersToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(&self) -> terrars::ListRef<ComputeUrlMap_HeaderAction_ResponseHeadersToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_headers_to_add", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_HostRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    hosts: terrars::SetField<terrars::PrimField<std::string::String>>,
    path_matcher: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_HostRule {
    #[doc =
        "Set the field `description`.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_HostRule {
    type O = terrars::BlockAssignable<ComputeUrlMap_HostRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_HostRule {
    #[doc =
        "The list of host patterns to match. They must be valid hostnames, except * will\nmatch any string of ([a-z0-9-.]*). In that case, * must be the first character\nand must be followed in the pattern by either - or .."]
    pub hosts: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The name of the PathMatcher to use to match the path portion of the URL if the\nhostRule matches the URL's host portion."]
    pub path_matcher: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_HostRule {
    pub fn build(self) -> ComputeUrlMap_HostRule {
        ComputeUrlMap_HostRule {
            description: core::default::Default::default(),
            hosts: self.hosts,
            path_matcher: self.path_matcher,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_HostRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_HostRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_HostRuleRef {
        ComputeUrlMap_HostRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_HostRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hosts` after provisioning.\nThe list of host patterns to match. They must be valid hostnames, except * will\nmatch any string of ([a-z0-9-.]*). In that case, * must be the first character\nand must be followed in the pattern by either - or .."]
    pub fn hosts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_matcher` after provisioning.\nThe name of the PathMatcher to use to match the path portion of the URL if the\nhostRule matches the URL's host portion."]
    pub fn path_matcher(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_matcher", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_response_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_response_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
    #[doc =
        "Set the field `match_response_codes`.\nValid values include:\n- A number between 400 and 599: For example 401 or 503, in which case the load balancer applies the policy if the error code exactly matches this value.\n- 5xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 500 to 599.\n- 4xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 400 to 499.\nValues must be unique within matchResponseCodes and across all errorResponseRules of CustomErrorResponsePolicy."]
    pub fn set_match_response_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_response_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `override_response_code`.\nThe HTTP status code returned with the response containing the custom error content.\nIf overrideResponseCode is not supplied, the same response code returned by the original backend bucket or backend service is returned to the client."]
    pub fn set_override_response_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.override_response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nThe full path to a file within backendBucket . For example: /errors/defaultError.html\npath must start with a leading slash. path cannot have trailing slashes.\nIf the file is not available in backendBucket or the load balancer cannot reach the BackendBucket, a simple Not Found Error is returned to the client.\nThe value must be from 1 to 1024 characters"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule {}

impl BuildComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
        ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule {
            match_response_codes: core::default::Default::default(),
            override_response_code: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef {
        ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `match_response_codes` after provisioning.\nValid values include:\n- A number between 400 and 599: For example 401 or 503, in which case the load balancer applies the policy if the error code exactly matches this value.\n- 5xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 500 to 599.\n- 4xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 400 to 499.\nValues must be unique within matchResponseCodes and across all errorResponseRules of CustomErrorResponsePolicy."]
    pub fn match_response_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_response_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `override_response_code` after provisioning.\nThe HTTP status code returned with the response containing the custom error content.\nIf overrideResponseCode is not supplied, the same response code returned by the original backend bucket or backend service is returned to the client."]
    pub fn override_response_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe full path to a file within backendBucket . For example: /errors/defaultError.html\npath must start with a leading slash. path cannot have trailing slashes.\nIf the file is not available in backendBucket or the load balancer cannot reach the BackendBucket, a simple Not Found Error is returned to the client.\nThe value must be from 1 to 1024 characters"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicyDynamic {
    error_response_rule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_response_rule: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule>>,
    dynamic: ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicyDynamic,
}

impl ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy {
    #[doc =
        "Set the field `error_service`.\nThe full or partial URL to the BackendBucket resource that contains the custom error content. Examples are:\nhttps://www.googleapis.com/compute/v1/projects/project/global/backendBuckets/myBackendBucket\ncompute/v1/projects/project/global/backendBuckets/myBackendBucket\nglobal/backendBuckets/myBackendBucket\nIf errorService is not specified at lower levels like pathMatcher, pathRule and routeRule, an errorService specified at a higher level in the UrlMap will be used. If UrlMap.defaultCustomErrorResponsePolicy contains one or more errorResponseRules[], it must specify errorService.\nIf load balancer cannot reach the backendBucket, a simple Not Found Error will be returned, with the original response code (or overrideResponseCode if configured)."]
    pub fn set_error_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_response_rule`.\n"]
    pub fn set_error_response_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_response_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_response_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy {}

impl BuildComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy {
        ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy {
            error_service: core::default::Default::default(),
            error_response_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicyRef {
        ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `error_service` after provisioning.\nThe full or partial URL to the BackendBucket resource that contains the custom error content. Examples are:\nhttps://www.googleapis.com/compute/v1/projects/project/global/backendBuckets/myBackendBucket\ncompute/v1/projects/project/global/backendBuckets/myBackendBucket\nglobal/backendBuckets/myBackendBucket\nIf errorService is not specified at lower levels like pathMatcher, pathRule and routeRule, an errorService specified at a higher level in the UrlMap will be used. If UrlMap.defaultCustomErrorResponsePolicy contains one or more errorResponseRules[], it must specify errorService.\nIf load balancer cannot reach the backendBucket, a simple Not Found Error will be returned, with the original response code (or overrideResponseCode if configured)."]
    pub fn error_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_service", self.base))
    }

    #[doc = "Get a reference to the value of field `error_response_rule` after provisioning.\n"]
    pub fn error_response_rule(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy_ErrorResponseRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_response_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_DefaultUrlRedirect {
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

impl ComputeUrlMap_PathMatcher_DefaultUrlRedirect {
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_DefaultUrlRedirect {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_DefaultUrlRedirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_DefaultUrlRedirect {
    #[doc =
        "If set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: terrars::PrimField<bool>,
}

impl BuildComputeUrlMap_PathMatcher_DefaultUrlRedirect {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_DefaultUrlRedirect {
        ComputeUrlMap_PathMatcher_DefaultUrlRedirect {
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
pub struct ComputeUrlMap_PathMatcher_DefaultUrlRedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_DefaultUrlRedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_DefaultUrlRedirectRef {
        ComputeUrlMap_PathMatcher_DefaultUrlRedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_DefaultUrlRedirectRef {
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
pub struct ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_response_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_response_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule {
    #[doc =
        "Set the field `match_response_codes`.\nValid values include:\n\n- A number between 400 and 599: For example 401 or 503, in which case the load balancer applies the policy if the error code exactly matches this value.\n- 5xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 500 to 599.\n- 4xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 400 to 499.\n\nValues must be unique within matchResponseCodes and across all errorResponseRules of CustomErrorResponsePolicy."]
    pub fn set_match_response_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_response_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `override_response_code`.\nThe HTTP status code returned with the response containing the custom error content.\nIf overrideResponseCode is not supplied, the same response code returned by the original backend bucket or backend service is returned to the client."]
    pub fn set_override_response_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.override_response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nThe full path to a file within backendBucket . For example: /errors/defaultError.html\npath must start with a leading slash. path cannot have trailing slashes.\nIf the file is not available in backendBucket or the load balancer cannot reach the BackendBucket, a simple Not Found Error is returned to the client.\nThe value must be from 1 to 1024 characters"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule {
    type O =
        terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule {}

impl BuildComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule {
        ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule {
            match_response_codes: core::default::Default::default(),
            override_response_code: core::default::Default::default(),
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRuleRef {
        ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `match_response_codes` after provisioning.\nValid values include:\n\n- A number between 400 and 599: For example 401 or 503, in which case the load balancer applies the policy if the error code exactly matches this value.\n- 5xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 500 to 599.\n- 4xx: Load Balancer will apply the policy if the backend service responds with any response code in the range of 400 to 499.\n\nValues must be unique within matchResponseCodes and across all errorResponseRules of CustomErrorResponsePolicy."]
    pub fn match_response_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_response_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `override_response_code` after provisioning.\nThe HTTP status code returned with the response containing the custom error content.\nIf overrideResponseCode is not supplied, the same response code returned by the original backend bucket or backend service is returned to the client."]
    pub fn override_response_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe full path to a file within backendBucket . For example: /errors/defaultError.html\npath must start with a leading slash. path cannot have trailing slashes.\nIf the file is not available in backendBucket or the load balancer cannot reach the BackendBucket, a simple Not Found Error is returned to the client.\nThe value must be from 1 to 1024 characters"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicyDynamic {
    error_response_rule: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_response_rule: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule>>,
    dynamic: ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicyDynamic,
}

impl ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy {
    #[doc =
        "Set the field `error_service`.\nThe full or partial URL to the BackendBucket resource that contains the custom error content. Examples are:\n\nhttps://www.googleapis.com/compute/v1/projects/project/global/backendBuckets/myBackendBucket\ncompute/v1/projects/project/global/backendBuckets/myBackendBucket\nglobal/backendBuckets/myBackendBucket\n\nIf errorService is not specified at lower levels like pathMatcher, pathRule and routeRule, an errorService specified at a higher level in the UrlMap will be used. If UrlMap.defaultCustomErrorResponsePolicy contains one or more errorResponseRules[], it must specify errorService.\nIf load balancer cannot reach the backendBucket, a simple Not Found Error will be returned, with the original response code (or overrideResponseCode if configured)."]
    pub fn set_error_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_response_rule`.\n"]
    pub fn set_error_response_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.error_response_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.error_response_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy {}

impl BuildComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy {
        ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy {
            error_service: core::default::Default::default(),
            error_response_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicyRef {
        ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `error_service` after provisioning.\nThe full or partial URL to the BackendBucket resource that contains the custom error content. Examples are:\n\nhttps://www.googleapis.com/compute/v1/projects/project/global/backendBuckets/myBackendBucket\ncompute/v1/projects/project/global/backendBuckets/myBackendBucket\nglobal/backendBuckets/myBackendBucket\n\nIf errorService is not specified at lower levels like pathMatcher, pathRule and routeRule, an errorService specified at a higher level in the UrlMap will be used. If UrlMap.defaultCustomErrorResponsePolicy contains one or more errorResponseRules[], it must specify errorService.\nIf load balancer cannot reach the backendBucket, a simple Not Found Error will be returned, with the original response code (or overrideResponseCode if configured)."]
    pub fn error_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_service", self.base))
    }

    #[doc = "Get a reference to the value of field `error_response_rule` after provisioning.\n"]
    pub fn error_response_rule(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy_ErrorResponseRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_response_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
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

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
    #[doc = "If true, specifies the CORS policy is disabled."]
    pub disabled: terrars::PrimField<bool>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy {
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
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef {
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
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
    http_status: terrars::PrimField<f64>,
    percentage: terrars::PrimField<f64>,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort { }

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
    #[doc = "The HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub http_status: terrars::PrimField<f64>,
    #[doc =
        "The percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub percentage: terrars::PrimField<f64>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort {
            http_status: self.http_status,
            percentage: self.percentage,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef {
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
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    type O =
        terrars
        ::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef {
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
struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayDynamic {
    fixed_delay: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
    percentage: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay>>,
    dynamic: ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayDynamic,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
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
                            ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
    #[doc =
        "The percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub percentage: terrars::PrimField<f64>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay {
            percentage: self.percentage,
            fixed_delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef {
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
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyDynamic {
    abort: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort>>,
    delay: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay>>,
    dynamic: ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyDynamic,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Abort>,
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
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_AbortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_DelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDurationRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDurationRef {
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
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
    backend_service: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy { }

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
    #[doc = "The BackendService resource being mirrored to."]
    pub backend_service: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy { backend_service: self.backend_service }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend_service` after provisioning.\nThe BackendService resource being mirrored to."]
    pub fn backend_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef {
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
struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyDynamic {
    per_try_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    num_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>>,
    dynamic: ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyDynamic,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
    #[doc = "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn set_num_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.num_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\nany 5xx response code, or if the backend service does not respond at all,\nfor example: disconnects, reset, read timeout, connection failure, and refused\nstreams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\nconnecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\nCurrently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\nREFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\nheader is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\ngRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\nheader is set to resource-exhausted\n* unavailable: Loadbalancer will retry if\nthe gRPC status code in the response header is set to unavailable"]
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
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef {
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
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\nany 5xx response code, or if the backend service does not respond at all,\nfor example: disconnects, reset, read timeout, connection failure, and refused\nstreams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\nconnecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\nCurrently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\nREFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\nheader is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\ngRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\nheader is set to resource-exhausted\n* unavailable: Loadbalancer will retry if\nthe gRPC status code in the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef {
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
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef {
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

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesDynamic {
    header_action: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_HeaderAction>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
    backend_service: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeUrlMap_HeaderAction>>,
    dynamic: ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesDynamic,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_HeaderAction>>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
    #[doc =
        "The default BackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub backend_service: terrars::PrimField<std::string::String>,
    #[doc =
        "Specifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub weight: terrars::PrimField<f64>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices {
            backend_service: self.backend_service,
            weight: self.weight,
            header_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend_service` after provisioning.\nThe default BackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeUrlMap_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcher_PathRule_RouteActionDynamic {
    cache_policy: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CachePolicy>>,
    cors_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy>>,
    max_stream_duration: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>>,
    retry_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy>>,
    timeout: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
    url_rewrite: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite>>,
    weighted_backend_services: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CachePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_stream_duration: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_backend_services: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>>,
    dynamic: ComputeUrlMap_PathMatcher_PathRule_RouteActionDynamic,
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteAction {
    #[doc = "Set the field `cache_policy`.\n"]
    pub fn set_cache_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicy>>,
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
                        terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy>,
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

    #[doc = "Set the field `max_stream_duration`.\n"]
    pub fn set_max_stream_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration>,
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
                        terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>,
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
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
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
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewrite>>,
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
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_RouteAction {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_RouteAction {}

impl BuildComputeUrlMap_PathMatcher_PathRule_RouteAction {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_RouteAction {
        ComputeUrlMap_PathMatcher_PathRule_RouteAction {
            cache_policy: core::default::Default::default(),
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
pub struct ComputeUrlMap_PathMatcher_PathRule_RouteActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_RouteActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_RouteActionRef {
        ComputeUrlMap_PathMatcher_PathRule_RouteActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_RouteActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_policy` after provisioning.\n"]
    pub fn cache_policy(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CachePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_CorsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\n"]
    pub fn max_stream_duration(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_stream_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_UrlRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule_UrlRedirect {
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

impl ComputeUrlMap_PathMatcher_PathRule_UrlRedirect {
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule_UrlRedirect {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_UrlRedirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule_UrlRedirect {
    #[doc =
        "If set to true, any accompanying query portion of the original URL is\nremoved prior to redirecting the request. If set to false, the query\nportion of the original URL is retained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: terrars::PrimField<bool>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule_UrlRedirect {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule_UrlRedirect {
        ComputeUrlMap_PathMatcher_PathRule_UrlRedirect {
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
pub struct ComputeUrlMap_PathMatcher_PathRule_UrlRedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRule_UrlRedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_PathRule_UrlRedirectRef {
        ComputeUrlMap_PathMatcher_PathRule_UrlRedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRule_UrlRedirectRef {
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
        "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is\nremoved prior to redirecting the request. If set to false, the query\nportion of the original URL is retained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcher_PathRuleDynamic {
    custom_error_response_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy>>,
    route_action: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction>>,
    url_redirect: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_UrlRedirect>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_PathRule {
    paths: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_error_response_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_action: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_redirect: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_UrlRedirect>>,
    dynamic: ComputeUrlMap_PathMatcher_PathRuleDynamic,
}

impl ComputeUrlMap_PathMatcher_PathRule {
    #[doc = "Set the field `service`.\nThe backend service or backend bucket to use if any of the given paths match."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_error_response_policy`.\n"]
    pub fn set_custom_error_response_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_error_response_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_error_response_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `route_action`.\n"]
    pub fn set_route_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_UrlRedirect>>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_PathRule {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_PathRule {
    #[doc =
        "The list of path patterns to match. Each must start with / and the only place a\n\\* is allowed is at the end following a /. The string fed to the path matcher\ndoes not include any text after the first ? or #, and those chars are not\nallowed here."]
    pub paths: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeUrlMap_PathMatcher_PathRule {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_PathRule {
        ComputeUrlMap_PathMatcher_PathRule {
            paths: self.paths,
            service: core::default::Default::default(),
            custom_error_response_policy: core::default::Default::default(),
            route_action: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_PathRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_PathRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_PathMatcher_PathRuleRef {
        ComputeUrlMap_PathMatcher_PathRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_PathRuleRef {
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
        "Get a reference to the value of field `service` after provisioning.\nThe backend service or backend bucket to use if any of the given paths match."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_error_response_policy` after provisioning.\n"]
    pub fn custom_error_response_policy(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_error_response_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }

    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_UrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
    range_end: terrars::PrimField<f64>,
    range_start: terrars::PrimField<f64>,
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch { }

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
    #[doc = "The end of the range (exclusive)."]
    pub range_end: terrars::PrimField<f64>,
    #[doc = "The start of the range (inclusive)."]
    pub range_start: terrars::PrimField<f64>,
}

impl BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch {
            range_end: self.range_end,
            range_start: self.range_start,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef {
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
struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesDynamic {
    range_match: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
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
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch>>,
    dynamic: ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesDynamic,
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
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
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatch>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
    #[doc =
        "The name of the HTTP header to match. For matching against the HTTP request's\nauthority, use a headerMatch with the header name \":authority\". For matching a\nrequest's method, use the headerName \":method\"."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches {
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
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef {
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
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches_RangeMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range_match", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels { }

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
    #[doc =
        "Name of metadata label. The name can have a maximum length of 1024 characters\nand must be at least 1 character long."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The value of the label must match the specified value. value can have a maximum\nlength of 1024 characters."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef {
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
struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersDynamic {
    filter_labels: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
    filter_match_criteria: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_labels: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels>>,
    dynamic: ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersDynamic,
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
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
                            ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabels,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
    #[doc =
        "Specifies how individual filterLabel matches within the list of filterLabels\ncontribute towards the overall metadataFilter match. Supported values are:\n  - MATCH_ANY: At least one of the filterLabels must have a matching label in the\nprovided metadata.\n  - MATCH_ALL: All filterLabels must have matching labels in\nthe provided metadata. Possible values: [\"MATCH_ALL\", \"MATCH_ANY\"]"]
    pub filter_match_criteria: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters {
            filter_match_criteria: self.filter_match_criteria,
            filter_labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter_match_criteria` after provisioning.\nSpecifies how individual filterLabel matches within the list of filterLabels\ncontribute towards the overall metadataFilter match. Supported values are:\n  - MATCH_ANY: At least one of the filterLabels must have a matching label in the\nprovided metadata.\n  - MATCH_ALL: All filterLabels must have matching labels in\nthe provided metadata. Possible values: [\"MATCH_ALL\", \"MATCH_ANY\"]"]
    pub fn filter_match_criteria(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_match_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_labels` after provisioning.\n"]
    pub fn filter_labels(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters_FilterLabelsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_labels", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact_match: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    present_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
    #[doc =
        "The name of the query parameter to match. The query parameter must exist in the\nrequest, in the absence of which the request match fails."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches {
            exact_match: core::default::Default::default(),
            name: self.name,
            present_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef {
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
struct ComputeUrlMap_PathMatcher_RouteRules_MatchRulesDynamic {
    header_matches: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches>>,
    metadata_filters: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters>>,
    query_parameter_matches: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRules {
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
    header_matches: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_filters: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameter_matches: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches>>,
    dynamic: ComputeUrlMap_PathMatcher_RouteRules_MatchRulesDynamic,
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRules {
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
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatches>>,
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
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFilters>>,
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
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatches>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_MatchRules {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules {}

impl BuildComputeUrlMap_PathMatcher_RouteRules_MatchRules {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRules {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRules {
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
pub struct ComputeUrlMap_PathMatcher_RouteRules_MatchRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_MatchRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_MatchRulesRef {
        ComputeUrlMap_PathMatcher_RouteRules_MatchRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_MatchRulesRef {
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
    pub fn header_matches(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_HeaderMatchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_matches", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_filters` after provisioning.\n"]
    pub fn metadata_filters(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_MetadataFiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `query_parameter_matches` after provisioning.\n"]
    pub fn query_parameter_matches(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_MatchRules_QueryParameterMatchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_parameter_matches", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
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

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {}

impl BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy {
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
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef {
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
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_status: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {}

impl BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef {
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
struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayDynamic {
    fixed_delay: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay>,
    >,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_delay: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay>>,
    dynamic: ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayDynamic,
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
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
                            ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelay,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {}

impl BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef {
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
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_FaultInjectionPolicy_Delay_FixedDelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyDynamic {
    abort: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort>>,
    delay: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    abort: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delay: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay>>,
    dynamic: ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyDynamic,
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Abort>,
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
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_Delay>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {}

impl BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_AbortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }

    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy_DelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyDynamic {
    per_try_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
    num_retries: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_try_timeout: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>>,
    dynamic: ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyDynamic,
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
    #[doc =
        "Set the field `retry_conditions`.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\n  any 5xx response code, or if the backend service does not respond at all,\n  for example: disconnects, reset, read timeout, connection failure, and refused\n  streams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n  502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\n  connecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\n  REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\n  header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\n  gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\n  header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in\n  the response header is set to unavailable"]
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
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeout>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
    #[doc = "Specifies the allowed number retries. This number must be > 0."]
    pub num_retries: terrars::PrimField<f64>,
}

impl BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy {
            num_retries: self.num_retries,
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef {
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
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\n  any 5xx response code, or if the backend service does not respond at all,\n  for example: disconnects, reset, read timeout, connection failure, and refused\n  streams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n  502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\n  connecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\n  REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\n  header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\n  gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\n  header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in\n  the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RetryPolicy_PerTryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_try_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
    path_pattern: terrars::PrimField<std::string::String>,
    path_substitution: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite { }

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
    #[doc =
        "The regular expression used to match against the URL path.\nIt uses RE2 syntax with the following constraints:\n\n* Any single character operators are allowed.\n* Groups may only contain a submatch operator, and may not\n  contain character repetition (for example, '.*').\n* Character repetition (for example, '.*') may only be used in\n  a regex together with empty string operators, other\n  repetitions, ranges, and repetitions of ranges.\n* Ranges may only contain character ranges, digit ranges, and\n  symbols allowed for ranges."]
    pub path_pattern: terrars::PrimField<std::string::String>,
    #[doc =
        "The substitution used to rewrite the parts of the URL path\nmatched by pathPattern. May reference capture groups from\npathPattern."]
    pub path_substitution: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite {
            path_pattern: self.path_pattern,
            path_substitution: self.path_substitution,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef {
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
struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteDynamic {
    regex_rewrite: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_prefix_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_template_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_rewrite: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite>>,
    dynamic: ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteDynamic,
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
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
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewrite>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {}

impl BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
            path_template_rewrite: core::default::Default::default(),
            regex_rewrite: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef {
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
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite_RegexRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcher_RouteRules_RouteActionDynamic {
    cache_policy: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction_CachePolicy>>,
    cors_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy>>,
    fault_injection_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy>>,
    max_stream_duration: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration>>,
    request_mirror_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>>,
    retry_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy>>,
    timeout: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
    url_rewrite: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite>>,
    weighted_backend_services: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction_CachePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_injection_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_stream_duration: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_mirror_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_backend_services: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>>,
    dynamic: ComputeUrlMap_PathMatcher_RouteRules_RouteActionDynamic,
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteAction {
    #[doc = "Set the field `cache_policy`.\n"]
    pub fn set_cache_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction_CachePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicy>>,
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
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicy>,
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

    #[doc = "Set the field `max_stream_duration`.\n"]
    pub fn set_max_stream_duration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDuration>,
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
                        terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicy>,
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
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_Timeout>>,
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
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewrite>>,
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
                        ::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServices>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_RouteAction {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction {}

impl BuildComputeUrlMap_PathMatcher_RouteRules_RouteAction {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_RouteAction {
        ComputeUrlMap_PathMatcher_RouteRules_RouteAction {
            cache_policy: core::default::Default::default(),
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
pub struct ComputeUrlMap_PathMatcher_RouteRules_RouteActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_RouteActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_RouteActionRef {
        ComputeUrlMap_PathMatcher_RouteRules_RouteActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_RouteActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_policy` after provisioning.\n"]
    pub fn cache_policy(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteAction_CachePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_CorsPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_FaultInjectionPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fault_injection_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\n"]
    pub fn max_stream_duration(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_MaxStreamDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_stream_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_RequestMirrorPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_mirror_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_RouteAction_UrlRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_RouteAction_WeightedBackendServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_backend_services", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules_UrlRedirect {
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

impl ComputeUrlMap_PathMatcher_RouteRules_UrlRedirect {
    #[doc =
        "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn set_host_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https. If set\nto false, the URL scheme of the redirected request will remain the same as that\nof the request. This must only be set for UrlMaps used in TargetHttpProxys.\nSetting this true for TargetHttpsProxy is not permitted. Defaults to false."]
    pub fn set_https_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.https_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. Only one of pathRedirect or prefixRedirect must be\nspecified. The value must be between 1 and 1024 characters."]
    pub fn set_path_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request."]
    pub fn set_prefix_redirect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_redirect = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method will be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case, the request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redirect_response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `strip_query`.\nIf set to true, any accompanying query portion of the original URL is removed\nprior to redirecting the request. If set to false, the query portion of the\noriginal URL is retained. Defaults to false."]
    pub fn set_strip_query(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strip_query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules_UrlRedirect {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_UrlRedirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules_UrlRedirect {}

impl BuildComputeUrlMap_PathMatcher_RouteRules_UrlRedirect {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules_UrlRedirect {
        ComputeUrlMap_PathMatcher_RouteRules_UrlRedirect {
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
pub struct ComputeUrlMap_PathMatcher_RouteRules_UrlRedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRules_UrlRedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeUrlMap_PathMatcher_RouteRules_UrlRedirectRef {
        ComputeUrlMap_PathMatcher_RouteRules_UrlRedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRules_UrlRedirectRef {
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
        "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https. If set\nto false, the URL scheme of the redirected request will remain the same as that\nof the request. This must only be set for UrlMaps used in TargetHttpProxys.\nSetting this true for TargetHttpsProxy is not permitted. Defaults to false."]
    pub fn https_redirect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. Only one of pathRedirect or prefixRedirect must be\nspecified. The value must be between 1 and 1024 characters."]
    pub fn path_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request."]
    pub fn prefix_redirect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_redirect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method will be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case, the request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed\nprior to redirecting the request. If set to false, the query portion of the\noriginal URL is retained. Defaults to false."]
    pub fn strip_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcher_RouteRulesDynamic {
    custom_error_response_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy>>,
    header_action: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_HeaderAction>>,
    match_rules: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_MatchRules>>,
    route_action: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_RouteAction>>,
    url_redirect: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules_UrlRedirect>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher_RouteRules {
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_error_response_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeUrlMap_HeaderAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_rules: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_MatchRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_action: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_RouteAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_redirect: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules_UrlRedirect>>,
    dynamic: ComputeUrlMap_PathMatcher_RouteRulesDynamic,
}

impl ComputeUrlMap_PathMatcher_RouteRules {
    #[doc =
        "Set the field `service`.\nThe backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_error_response_policy`.\n"]
    pub fn set_custom_error_response_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_error_response_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_error_response_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_HeaderAction>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_MatchRules>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_RouteAction>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules_UrlRedirect>>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher_RouteRules {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher_RouteRules {
    #[doc =
        "For routeRules within a given pathMatcher, priority determines the order\nin which load balancer will interpret routeRules. RouteRules are evaluated\nin order of priority, from the lowest to highest number. The priority of\na rule decreases as its number increases (1, 2, 3, N+1). The first rule\nthat matches the request is applied.\n\nYou cannot configure two or more routeRules with the same priority.\nPriority for each rule must be set to a number between 0 and\n2147483647 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules\nin the future without affecting the rest of the rules. For example,\n1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers to which\nyou could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the\nfuture without any impact on existing rules."]
    pub priority: terrars::PrimField<f64>,
}

impl BuildComputeUrlMap_PathMatcher_RouteRules {
    pub fn build(self) -> ComputeUrlMap_PathMatcher_RouteRules {
        ComputeUrlMap_PathMatcher_RouteRules {
            priority: self.priority,
            service: core::default::Default::default(),
            custom_error_response_policy: core::default::Default::default(),
            header_action: core::default::Default::default(),
            match_rules: core::default::Default::default(),
            route_action: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_PathMatcher_RouteRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcher_RouteRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_PathMatcher_RouteRulesRef {
        ComputeUrlMap_PathMatcher_RouteRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcher_RouteRulesRef {
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
        "Get a reference to the value of field `service` after provisioning.\nThe backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_error_response_policy` after provisioning.\n"]
    pub fn custom_error_response_policy(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRule_CustomErrorResponsePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_error_response_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeUrlMap_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_rules` after provisioning.\n"]
    pub fn match_rules(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_MatchRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_RouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }

    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRules_UrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_PathMatcherDynamic {
    default_custom_error_response_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy>>,
    default_route_action: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_DefaultRouteAction>>,
    default_url_redirect: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_DefaultUrlRedirect>>,
    header_action: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_HeaderAction>>,
    path_rule: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_PathRule>>,
    route_rules: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_PathMatcher_RouteRules>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_PathMatcher {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_custom_error_response_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_route_action: std::option::Option<std::vec::Vec<ComputeUrlMap_DefaultRouteAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_url_redirect: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_DefaultUrlRedirect>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeUrlMap_HeaderAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_rule: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_PathRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_rules: std::option::Option<std::vec::Vec<ComputeUrlMap_PathMatcher_RouteRules>>,
    dynamic: ComputeUrlMap_PathMatcherDynamic,
}

impl ComputeUrlMap_PathMatcher {
    #[doc =
        "Set the field `default_service`.\nThe backend service or backend bucket to use when none of the given paths match."]
    pub fn set_default_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_custom_error_response_policy`.\n"]
    pub fn set_default_custom_error_response_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_custom_error_response_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_custom_error_response_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_route_action`.\n"]
    pub fn set_default_route_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_DefaultRouteAction>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_DefaultUrlRedirect>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_HeaderAction>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_PathRule>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_PathMatcher_RouteRules>>,
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

impl terrars::ToListMappable for ComputeUrlMap_PathMatcher {
    type O = terrars::BlockAssignable<ComputeUrlMap_PathMatcher>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_PathMatcher {
    #[doc = "The name to which this PathMatcher is referred by the HostRule."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_PathMatcher {
    pub fn build(self) -> ComputeUrlMap_PathMatcher {
        ComputeUrlMap_PathMatcher {
            default_service: core::default::Default::default(),
            description: core::default::Default::default(),
            name: self.name,
            default_custom_error_response_policy: core::default::Default::default(),
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
pub struct ComputeUrlMap_PathMatcherRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_PathMatcherRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_PathMatcherRef {
        ComputeUrlMap_PathMatcherRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_PathMatcherRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_service` after provisioning.\nThe backend service or backend bucket to use when none of the given paths match."]
    pub fn default_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create\nthe resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name to which this PathMatcher is referred by the HostRule."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `default_custom_error_response_policy` after provisioning.\n"]
    pub fn default_custom_error_response_policy(
        &self,
    ) -> terrars::ListRef<ComputeUrlMap_PathMatcher_DefaultCustomErrorResponsePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_custom_error_response_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> terrars::ListRef<ComputeUrlMap_DefaultRouteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_route_action", self.base))
    }

    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_DefaultUrlRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_url_redirect", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeUrlMap_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `path_rule` after provisioning.\n"]
    pub fn path_rule(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_PathRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `route_rules` after provisioning.\n"]
    pub fn route_rules(&self) -> terrars::ListRef<ComputeUrlMap_PathMatcher_RouteRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_Test_Headers {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl ComputeUrlMap_Test_Headers { }

impl terrars::ToListMappable for ComputeUrlMap_Test_Headers {
    type O = terrars::BlockAssignable<ComputeUrlMap_Test_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_Test_Headers {
    #[doc = "Header name."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Header value."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_Test_Headers {
    pub fn build(self) -> ComputeUrlMap_Test_Headers {
        ComputeUrlMap_Test_Headers {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_Test_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_Test_HeadersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_Test_HeadersRef {
        ComputeUrlMap_Test_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_Test_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nHeader name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nHeader value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeUrlMap_TestDynamic {
    headers: std::option::Option<terrars::DynamicBlock<ComputeUrlMap_Test_Headers>>,
}

#[derive(Serialize)]
pub struct ComputeUrlMap_Test {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_output_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_redirect_response_code: std::option::Option<terrars::PrimField<f64>>,
    host: terrars::PrimField<std::string::String>,
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::vec::Vec<ComputeUrlMap_Test_Headers>>,
    dynamic: ComputeUrlMap_TestDynamic,
}

impl ComputeUrlMap_Test {
    #[doc = "Set the field `description`.\nDescription of this test case."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expected_output_url`.\nThe expected output URL evaluated by the load balancer containing the scheme, host, path and query parameters.\n\nFor rules that forward requests to backends, the test passes only when expectedOutputUrl matches the request forwarded by the load balancer to backends. For rules with urlRewrite, the test verifies that the forwarded request matches hostRewrite and pathPrefixRewrite in the urlRewrite action. When service is specified, expectedOutputUrl's scheme is ignored.\n\nFor rules with urlRedirect, the test passes only if expectedOutputUrl matches the URL in the load balancer's redirect response. If urlRedirect specifies httpsRedirect, the test passes only if the scheme in expectedOutputUrl is also set to HTTPS. If urlRedirect specifies stripQuery, the test passes only if expectedOutputUrl does not contain any query parameters.\n\nexpectedOutputUrl is optional when service is specified."]
    pub fn set_expected_output_url(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.expected_output_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expected_redirect_response_code`.\nFor rules with urlRedirect, the test passes only if expectedRedirectResponseCode matches the HTTP status code in load balancer's redirect response.\n\nexpectedRedirectResponseCode cannot be set when service is set."]
    pub fn set_expected_redirect_response_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.expected_redirect_response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\nThe backend service or backend bucket link that should be matched by this test."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeUrlMap_Test_Headers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeUrlMap_Test {
    type O = terrars::BlockAssignable<ComputeUrlMap_Test>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_Test {
    #[doc = "Host portion of the URL."]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = "Path portion of the URL."]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildComputeUrlMap_Test {
    pub fn build(self) -> ComputeUrlMap_Test {
        ComputeUrlMap_Test {
            description: core::default::Default::default(),
            expected_output_url: core::default::Default::default(),
            expected_redirect_response_code: core::default::Default::default(),
            host: self.host,
            path: self.path,
            service: core::default::Default::default(),
            headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_TestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_TestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_TestRef {
        ComputeUrlMap_TestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_TestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of this test case."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expected_output_url` after provisioning.\nThe expected output URL evaluated by the load balancer containing the scheme, host, path and query parameters.\n\nFor rules that forward requests to backends, the test passes only when expectedOutputUrl matches the request forwarded by the load balancer to backends. For rules with urlRewrite, the test verifies that the forwarded request matches hostRewrite and pathPrefixRewrite in the urlRewrite action. When service is specified, expectedOutputUrl's scheme is ignored.\n\nFor rules with urlRedirect, the test passes only if expectedOutputUrl matches the URL in the load balancer's redirect response. If urlRedirect specifies httpsRedirect, the test passes only if the scheme in expectedOutputUrl is also set to HTTPS. If urlRedirect specifies stripQuery, the test passes only if expectedOutputUrl does not contain any query parameters.\n\nexpectedOutputUrl is optional when service is specified."]
    pub fn expected_output_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_output_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expected_redirect_response_code` after provisioning.\nFor rules with urlRedirect, the test passes only if expectedRedirectResponseCode matches the HTTP status code in load balancer's redirect response.\n\nexpectedRedirectResponseCode cannot be set when service is set."]
    pub fn expected_redirect_response_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_redirect_response_code", self.base))
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
        "Get a reference to the value of field `service` after provisioning.\nThe backend service or backend bucket link that should be matched by this test."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<ComputeUrlMap_Test_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeUrlMap_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeUrlMap_Timeouts {
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

impl terrars::ToListMappable for ComputeUrlMap_Timeouts {
    type O = terrars::BlockAssignable<ComputeUrlMap_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeUrlMap_Timeouts {}

impl BuildComputeUrlMap_Timeouts {
    pub fn build(self) -> ComputeUrlMap_Timeouts {
        ComputeUrlMap_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeUrlMap_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeUrlMap_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeUrlMap_TimeoutsRef {
        ComputeUrlMap_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeUrlMap_TimeoutsRef {
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
