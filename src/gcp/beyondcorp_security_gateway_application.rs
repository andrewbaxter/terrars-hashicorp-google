#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BeyondcorpSecurityGatewayApplicationData_ {
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
    application_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
    security_gateway_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_matchers: std::option::Option<std::vec::Vec<BeyondcorpSecurityGatewayApplication_EndpointMatchers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BeyondcorpSecurityGatewayApplication_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstreams: std::option::Option<std::vec::Vec<BeyondcorpSecurityGatewayApplication_Upstreams>>,
    dynamic: BeyondcorpSecurityGatewayApplicationDynamic,
}

struct BeyondcorpSecurityGatewayApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BeyondcorpSecurityGatewayApplicationData_>,
}

#[derive(Clone)]
pub struct BeyondcorpSecurityGatewayApplication(std::rc::Rc<BeyondcorpSecurityGatewayApplication_>);

impl BeyondcorpSecurityGatewayApplication {
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nOptional. An arbitrary user-provided name for the Application resource.\nCannot exceed 64 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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
        "Set the field `schema`.\nType of the external application. Possible values: [\"PROXY_GATEWAY\", \"API_GATEWAY\"]"]
    pub fn set_schema(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_matchers`.\n"]
    pub fn set_endpoint_matchers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_EndpointMatchers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_matchers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_matchers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BeyondcorpSecurityGatewayApplication_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upstreams`.\n"]
    pub fn set_upstreams(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upstreams = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upstreams = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `application_id` after provisioning.\nUser-settable Application resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. An arbitrary user-provided name for the Application resource.\nCannot exceed 64 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nType of the external application. Possible values: [\"PROXY_GATEWAY\", \"API_GATEWAY\"]"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_gateway_id` after provisioning.\nID of the Security Gateway resource this belongs to."]
    pub fn security_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_gateway_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the resource was last modified."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_matchers` after provisioning.\n"]
    pub fn endpoint_matchers(&self) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_EndpointMatchersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_matchers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BeyondcorpSecurityGatewayApplication_TimeoutsRef {
        BeyondcorpSecurityGatewayApplication_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `upstreams` after provisioning.\n"]
    pub fn upstreams(&self) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_UpstreamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstreams", self.extract_ref()))
    }
}

impl terrars::Referable for BeyondcorpSecurityGatewayApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BeyondcorpSecurityGatewayApplication { }

impl terrars::ToListMappable for BeyondcorpSecurityGatewayApplication {
    type O = terrars::ListRef<BeyondcorpSecurityGatewayApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BeyondcorpSecurityGatewayApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_beyondcorp_security_gateway_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication {
    pub tf_id: std::string::String,
    #[doc =
        "User-settable Application resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub application_id: terrars::PrimField<std::string::String>,
    #[doc = "ID of the Security Gateway resource this belongs to."]
    pub security_gateway_id: terrars::PrimField<std::string::String>,
}

impl BuildBeyondcorpSecurityGatewayApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> BeyondcorpSecurityGatewayApplication {
        let out = BeyondcorpSecurityGatewayApplication(std::rc::Rc::new(BeyondcorpSecurityGatewayApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BeyondcorpSecurityGatewayApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                application_id: self.application_id,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                schema: core::default::Default::default(),
                security_gateway_id: self.security_gateway_id,
                endpoint_matchers: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                upstreams: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BeyondcorpSecurityGatewayApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BeyondcorpSecurityGatewayApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application_id` after provisioning.\nUser-settable Application resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. An arbitrary user-provided name for the Application resource.\nCannot exceed 64 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nType of the external application. Possible values: [\"PROXY_GATEWAY\", \"API_GATEWAY\"]"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_gateway_id` after provisioning.\nID of the Security Gateway resource this belongs to."]
    pub fn security_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_gateway_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the resource was last modified."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_matchers` after provisioning.\n"]
    pub fn endpoint_matchers(&self) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_EndpointMatchersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint_matchers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BeyondcorpSecurityGatewayApplication_TimeoutsRef {
        BeyondcorpSecurityGatewayApplication_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `upstreams` after provisioning.\n"]
    pub fn upstreams(&self) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_UpstreamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstreams", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayApplicationDynamic {
    endpoint_matchers: std
    ::option
    ::Option<terrars::DynamicBlock<BeyondcorpSecurityGatewayApplication_EndpointMatchers>>,
    upstreams: std::option::Option<terrars::DynamicBlock<BeyondcorpSecurityGatewayApplication_Upstreams>>,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_EndpointMatchers {
    hostname: terrars::PrimField<std::string::String>,
    ports: terrars::ListField<terrars::PrimField<f64>>,
}

impl BeyondcorpSecurityGatewayApplication_EndpointMatchers { }

impl terrars::ToListMappable for BeyondcorpSecurityGatewayApplication_EndpointMatchers {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_EndpointMatchers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_EndpointMatchers {
    #[doc = "Required. Hostname of the application."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Optional. Ports of the application."]
    pub ports: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildBeyondcorpSecurityGatewayApplication_EndpointMatchers {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_EndpointMatchers {
        BeyondcorpSecurityGatewayApplication_EndpointMatchers {
            hostname: self.hostname,
            ports: self.ports,
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_EndpointMatchersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_EndpointMatchersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_EndpointMatchersRef {
        BeyondcorpSecurityGatewayApplication_EndpointMatchersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_EndpointMatchersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nRequired. Hostname of the application."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\nOptional. Ports of the application."]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BeyondcorpSecurityGatewayApplication_Timeouts {
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

impl terrars::ToListMappable for BeyondcorpSecurityGatewayApplication_Timeouts {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Timeouts {}

impl BuildBeyondcorpSecurityGatewayApplication_Timeouts {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Timeouts {
        BeyondcorpSecurityGatewayApplication_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_TimeoutsRef {
        BeyondcorpSecurityGatewayApplication_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_TimeoutsRef {
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

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy {
    regions: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy { }

impl terrars::ToListMappable for BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy {
    #[doc = "Required. List of regions where the application sends traffic to."]
    pub regions: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy {
        BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy { regions: self.regions }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicyRef {
        BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `regions` after provisioning.\nRequired. List of regions where the application sends traffic to."]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints {
    hostname: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints { }

impl terrars::ToListMappable for BeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints {
    #[doc = "Hostname of the endpoint."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Port of the endpoint."]
    pub port: terrars::PrimField<f64>,
}

impl BuildBeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints {
        BeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints {
            hostname: self.hostname,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_External_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_Upstreams_External_EndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_Upstreams_External_EndpointsRef {
        BeyondcorpSecurityGatewayApplication_Upstreams_External_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_External_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname of the endpoint."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort of the endpoint."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayApplication_Upstreams_ExternalDynamic {
    endpoints: std
    ::option
    ::Option<terrars::DynamicBlock<BeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints>>,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_External {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<std::vec::Vec<BeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints>>,
    dynamic: BeyondcorpSecurityGatewayApplication_Upstreams_ExternalDynamic,
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_External {
    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_External_Endpoints>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.endpoints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.endpoints = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGatewayApplication_Upstreams_External {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_External>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Upstreams_External {}

impl BuildBeyondcorpSecurityGatewayApplication_Upstreams_External {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Upstreams_External {
        BeyondcorpSecurityGatewayApplication_Upstreams_External {
            endpoints: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ExternalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_Upstreams_ExternalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_Upstreams_ExternalRef {
        BeyondcorpSecurityGatewayApplication_Upstreams_ExternalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ExternalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_Upstreams_External_EndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_Network {
    name: terrars::PrimField<std::string::String>,
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_Network { }

impl terrars::ToListMappable for BeyondcorpSecurityGatewayApplication_Upstreams_Network {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_Network>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Upstreams_Network {
    #[doc = "Required. Network name is of the format:\n'projects/{project}/global/networks/{network}'"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBeyondcorpSecurityGatewayApplication_Upstreams_Network {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Upstreams_Network {
        BeyondcorpSecurityGatewayApplication_Upstreams_Network { name: self.name }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_NetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_Upstreams_NetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_Upstreams_NetworkRef {
        BeyondcorpSecurityGatewayApplication_Upstreams_NetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_NetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nRequired. Network name is of the format:\n'projects/{project}/global/networks/{network}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo {
    #[doc =
        "Set the field `output_type`.\nThe output type of the delegated device info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo {
    type O =
        terrars
        ::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo {}

impl BuildBeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo {
        BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo {
            output_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfoRef {
        BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated device info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo {
    #[doc =
        "Set the field `output_type`.\nThe output type of the delegated group info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo {
    type O =
        terrars
        ::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo {}

impl BuildBeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo {
        BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo {
            output_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfoRef {
        BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated group info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo {
    #[doc =
        "Set the field `output_type`.\nThe output type of the delegated user info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo {
    type O =
        terrars
        ::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo {}

impl BuildBeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo {
        BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo {
            output_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfoRef {
        BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated user info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeadersDynamic {
    device_info: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo>,
    >,
    group_info: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo>,
    >,
    user_info: std
    ::option
    ::Option<
        terrars::DynamicBlock<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo>,
    >,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_info: std
    ::option
    ::Option<
        std::vec::Vec<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_info: std
    ::option
    ::Option<std::vec::Vec<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_info: std
    ::option
    ::Option<std::vec::Vec<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo>>,
    dynamic: BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeadersDynamic,
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders {
    #[doc =
        "Set the field `output_type`.\nDefault output type for all enabled headers. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_info`.\n"]
    pub fn set_device_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.device_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.device_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `group_info`.\n"]
    pub fn set_group_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.group_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.group_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_info`.\n"]
    pub fn set_user_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders {
    type O =
        terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders {}

impl BuildBeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders {
        BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders {
            output_type: core::default::Default::default(),
            device_info: core::default::Default::default(),
            group_info: core::default::Default::default(),
            user_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeadersRef {
        BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_type` after provisioning.\nDefault output type for all enabled headers. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }

    #[doc = "Get a reference to the value of field `device_info` after provisioning.\n"]
    pub fn device_info(
        &self,
    ) -> terrars
    ::ListRef<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_DeviceInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_info", self.base))
    }

    #[doc = "Get a reference to the value of field `group_info` after provisioning.\n"]
    pub fn group_info(
        &self,
    ) -> terrars
    ::ListRef<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_GroupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_info", self.base))
    }

    #[doc = "Get a reference to the value of field `user_info` after provisioning.\n"]
    pub fn user_info(
        &self,
    ) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders_UserInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_info", self.base))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocolDynamic {
    contextual_headers: std
    ::option
    ::Option<terrars::DynamicBlock<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders>>,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_client_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_identity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contextual_headers: std
    ::option
    ::Option<std::vec::Vec<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders>>,
    dynamic: BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocolDynamic,
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol {
    #[doc = "Set the field `allowed_client_headers`.\nThe configuration for the proxy."]
    pub fn set_allowed_client_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_client_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_ip`.\nClient IP configuration. The client IP address is included if true."]
    pub fn set_client_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.client_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_identity`.\nGateway identity configuration. Possible values: [\"RESOURCE_NAME\"]"]
    pub fn set_gateway_identity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gateway_identity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metadata_headers`.\nCustom resource specific headers along with the values.\nThe names should conform to RFC 9110:\n> Field names SHOULD constrain themselves to alphanumeric characters, \"-\",\n  and \".\", and SHOULD begin with a letter.\n> Field values SHOULD contain only ASCII printable characters and tab."]
    pub fn set_metadata_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contextual_headers`.\n"]
    pub fn set_contextual_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeaders,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.contextual_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.contextual_headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol {}

impl BuildBeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol {
        BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol {
            allowed_client_headers: core::default::Default::default(),
            client_ip: core::default::Default::default(),
            gateway_identity: core::default::Default::default(),
            metadata_headers: core::default::Default::default(),
            contextual_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocolRef {
        BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_client_headers` after provisioning.\nThe configuration for the proxy."]
    pub fn allowed_client_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_client_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_ip` after provisioning.\nClient IP configuration. The client IP address is included if true."]
    pub fn client_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gateway_identity` after provisioning.\nGateway identity configuration. Possible values: [\"RESOURCE_NAME\"]"]
    pub fn gateway_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_identity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata_headers` after provisioning.\nCustom resource specific headers along with the values.\nThe names should conform to RFC 9110:\n> Field names SHOULD constrain themselves to alphanumeric characters, \"-\",\n  and \".\", and SHOULD begin with a letter.\n> Field values SHOULD contain only ASCII printable characters and tab."]
    pub fn metadata_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `contextual_headers` after provisioning.\n"]
    pub fn contextual_headers(
        &self,
    ) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol_ContextualHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contextual_headers", self.base))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayApplication_UpstreamsDynamic {
    egress_policy: std
    ::option
    ::Option<terrars::DynamicBlock<BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy>>,
    external: std::option::Option<terrars::DynamicBlock<BeyondcorpSecurityGatewayApplication_Upstreams_External>>,
    network: std::option::Option<terrars::DynamicBlock<BeyondcorpSecurityGatewayApplication_Upstreams_Network>>,
    proxy_protocol: std
    ::option
    ::Option<terrars::DynamicBlock<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol>>,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplication_Upstreams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_policy: std::option::Option<std::vec::Vec<BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external: std::option::Option<std::vec::Vec<BeyondcorpSecurityGatewayApplication_Upstreams_External>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<std::vec::Vec<BeyondcorpSecurityGatewayApplication_Upstreams_Network>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_protocol: std::option::Option<std::vec::Vec<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol>>,
    dynamic: BeyondcorpSecurityGatewayApplication_UpstreamsDynamic,
}

impl BeyondcorpSecurityGatewayApplication_Upstreams {
    #[doc = "Set the field `egress_policy`.\n"]
    pub fn set_egress_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.egress_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `external`.\n"]
    pub fn set_external(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_External>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.external = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.external = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_Network>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proxy_protocol`.\n"]
    pub fn set_proxy_protocol(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocol>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proxy_protocol = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proxy_protocol = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGatewayApplication_Upstreams {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGatewayApplication_Upstreams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGatewayApplication_Upstreams {}

impl BuildBeyondcorpSecurityGatewayApplication_Upstreams {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplication_Upstreams {
        BeyondcorpSecurityGatewayApplication_Upstreams {
            egress_policy: core::default::Default::default(),
            external: core::default::Default::default(),
            network: core::default::Default::default(),
            proxy_protocol: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGatewayApplication_UpstreamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayApplication_UpstreamsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGatewayApplication_UpstreamsRef {
        BeyondcorpSecurityGatewayApplication_UpstreamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGatewayApplication_UpstreamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_policy` after provisioning.\n"]
    pub fn egress_policy(&self) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_Upstreams_EgressPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `external` after provisioning.\n"]
    pub fn external(&self) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_Upstreams_ExternalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_Upstreams_NetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_protocol` after provisioning.\n"]
    pub fn proxy_protocol(&self) -> terrars::ListRef<BeyondcorpSecurityGatewayApplication_Upstreams_ProxyProtocolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_protocol", self.base))
    }
}
