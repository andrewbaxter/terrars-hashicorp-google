use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct EndpointsServiceData {
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
    grpc_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    openapi_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protoc_output_base64: std::option::Option<terrars::PrimField<std::string::String>>,
    service_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EndpointsServiceTimeoutsEl>,
}

struct EndpointsService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EndpointsServiceData>,
}

#[derive(Clone)]
pub struct EndpointsService(std::rc::Rc<EndpointsService_>);

impl EndpointsService {
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
        "Set the field `grpc_config`.\nThe full text of the Service Config YAML file (Example located here). If provided, must also provide protoc_output_base64. open_api config must not be provided."]
    pub fn set_grpc_config(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().grpc_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `openapi_config`.\nThe full text of the OpenAPI YAML configuration as described here. Either this, or both of grpc_config and protoc_output_base64 must be specified."]
    pub fn set_openapi_config(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().openapi_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe project ID that the service belongs to. If not provided, provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `protoc_output_base64`.\nThe full contents of the Service Descriptor File generated by protoc. This should be a compiled .pb file, base64-encoded."]
    pub fn set_protoc_output_base64(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protoc_output_base64 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EndpointsServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `apis` after provisioning.\nA list of API objects."]
    pub fn apis(&self) -> terrars::ListRef<EndpointsServiceApisElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apis", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config_id` after provisioning.\nThe autogenerated ID for the configuration that is rolled out as part of the creation of this resource. Must be provided to compute engine instances as a tag."]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_address` after provisioning.\nThe address at which the service can be found - usually the same as the service name."]
    pub fn dns_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\nA list of Endpoint objects."]
    pub fn endpoints(&self) -> terrars::ListRef<EndpointsServiceEndpointsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `grpc_config` after provisioning.\nThe full text of the Service Config YAML file (Example located here). If provided, must also provide protoc_output_base64. open_api config must not be provided."]
    pub fn grpc_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grpc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `openapi_config` after provisioning.\nThe full text of the OpenAPI YAML configuration as described here. Either this, or both of grpc_config and protoc_output_base64 must be specified."]
    pub fn openapi_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.openapi_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project ID that the service belongs to. If not provided, provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protoc_output_base64` after provisioning.\nThe full contents of the Service Descriptor File generated by protoc. This should be a compiled .pb file, base64-encoded."]
    pub fn protoc_output_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protoc_output_base64", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the service. Usually of the form $apiname.endpoints.$projectid.cloud.goog."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EndpointsServiceTimeoutsElRef {
        EndpointsServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EndpointsService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EndpointsService { }

impl terrars::ToListMappable for EndpointsService {
    type O = terrars::ListRef<EndpointsServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EndpointsService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_endpoints_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEndpointsService {
    pub tf_id: std::string::String,
    #[doc = "The name of the service. Usually of the form $apiname.endpoints.$projectid.cloud.goog."]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildEndpointsService {
    pub fn build(self, stack: &mut terrars::Stack) -> EndpointsService {
        let out = EndpointsService(std::rc::Rc::new(EndpointsService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EndpointsServiceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                grpc_config: core::default::Default::default(),
                id: core::default::Default::default(),
                openapi_config: core::default::Default::default(),
                project: core::default::Default::default(),
                protoc_output_base64: core::default::Default::default(),
                service_name: self.service_name,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EndpointsServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EndpointsServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EndpointsServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apis` after provisioning.\nA list of API objects."]
    pub fn apis(&self) -> terrars::ListRef<EndpointsServiceApisElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apis", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config_id` after provisioning.\nThe autogenerated ID for the configuration that is rolled out as part of the creation of this resource. Must be provided to compute engine instances as a tag."]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_address` after provisioning.\nThe address at which the service can be found - usually the same as the service name."]
    pub fn dns_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\nA list of Endpoint objects."]
    pub fn endpoints(&self) -> terrars::ListRef<EndpointsServiceEndpointsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `grpc_config` after provisioning.\nThe full text of the Service Config YAML file (Example located here). If provided, must also provide protoc_output_base64. open_api config must not be provided."]
    pub fn grpc_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grpc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `openapi_config` after provisioning.\nThe full text of the OpenAPI YAML configuration as described here. Either this, or both of grpc_config and protoc_output_base64 must be specified."]
    pub fn openapi_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.openapi_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project ID that the service belongs to. If not provided, provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protoc_output_base64` after provisioning.\nThe full contents of the Service Descriptor File generated by protoc. This should be a compiled .pb file, base64-encoded."]
    pub fn protoc_output_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protoc_output_base64", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the service. Usually of the form $apiname.endpoints.$projectid.cloud.goog."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EndpointsServiceTimeoutsElRef {
        EndpointsServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EndpointsServiceApisElMethodsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    syntax: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EndpointsServiceApisElMethodsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_type`.\n"]
    pub fn set_request_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_type`.\n"]
    pub fn set_response_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `syntax`.\n"]
    pub fn set_syntax(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.syntax = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EndpointsServiceApisElMethodsEl {
    type O = terrars::BlockAssignable<EndpointsServiceApisElMethodsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEndpointsServiceApisElMethodsEl {}

impl BuildEndpointsServiceApisElMethodsEl {
    pub fn build(self) -> EndpointsServiceApisElMethodsEl {
        EndpointsServiceApisElMethodsEl {
            name: core::default::Default::default(),
            request_type: core::default::Default::default(),
            response_type: core::default::Default::default(),
            syntax: core::default::Default::default(),
        }
    }
}

pub struct EndpointsServiceApisElMethodsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EndpointsServiceApisElMethodsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EndpointsServiceApisElMethodsElRef {
        EndpointsServiceApisElMethodsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EndpointsServiceApisElMethodsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `request_type` after provisioning.\n"]
    pub fn request_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_type", self.base))
    }

    #[doc = "Get a reference to the value of field `response_type` after provisioning.\n"]
    pub fn response_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_type", self.base))
    }

    #[doc = "Get a reference to the value of field `syntax` after provisioning.\n"]
    pub fn syntax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.syntax", self.base))
    }
}

#[derive(Serialize)]
pub struct EndpointsServiceApisEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methods: std::option::Option<terrars::ListField<EndpointsServiceApisElMethodsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    syntax: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EndpointsServiceApisEl {
    #[doc = "Set the field `methods`.\n"]
    pub fn set_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<EndpointsServiceApisElMethodsEl>>,
    ) -> Self {
        self.methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `syntax`.\n"]
    pub fn set_syntax(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.syntax = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EndpointsServiceApisEl {
    type O = terrars::BlockAssignable<EndpointsServiceApisEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEndpointsServiceApisEl {}

impl BuildEndpointsServiceApisEl {
    pub fn build(self) -> EndpointsServiceApisEl {
        EndpointsServiceApisEl {
            methods: core::default::Default::default(),
            name: core::default::Default::default(),
            syntax: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct EndpointsServiceApisElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EndpointsServiceApisElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EndpointsServiceApisElRef {
        EndpointsServiceApisElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EndpointsServiceApisElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `methods` after provisioning.\n"]
    pub fn methods(&self) -> terrars::ListRef<EndpointsServiceApisElMethodsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `syntax` after provisioning.\n"]
    pub fn syntax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.syntax", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct EndpointsServiceEndpointsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EndpointsServiceEndpointsEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EndpointsServiceEndpointsEl {
    type O = terrars::BlockAssignable<EndpointsServiceEndpointsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEndpointsServiceEndpointsEl {}

impl BuildEndpointsServiceEndpointsEl {
    pub fn build(self) -> EndpointsServiceEndpointsEl {
        EndpointsServiceEndpointsEl {
            address: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct EndpointsServiceEndpointsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EndpointsServiceEndpointsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EndpointsServiceEndpointsElRef {
        EndpointsServiceEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EndpointsServiceEndpointsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct EndpointsServiceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EndpointsServiceTimeoutsEl {
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

impl terrars::ToListMappable for EndpointsServiceTimeoutsEl {
    type O = terrars::BlockAssignable<EndpointsServiceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEndpointsServiceTimeoutsEl {}

impl BuildEndpointsServiceTimeoutsEl {
    pub fn build(self) -> EndpointsServiceTimeoutsEl {
        EndpointsServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EndpointsServiceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EndpointsServiceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EndpointsServiceTimeoutsElRef {
        EndpointsServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EndpointsServiceTimeoutsElRef {
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
