use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeHealthCheckData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeHealthCheck_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeHealthCheckData>,
}

#[derive(Clone)]
pub struct DataComputeHealthCheck(std::rc::Rc<DataComputeHealthCheck_>);

impl DataComputeHealthCheck {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `check_interval_sec` after provisioning.\nHow often (in seconds) to send a health check. The default value is 5\nseconds."]
    pub fn check_interval_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_interval_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grpc_health_check` after provisioning.\nA nested object resource"]
    pub fn grpc_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckGrpcHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_health_check", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `healthy_threshold` after provisioning.\nA so-far unhealthy instance will be marked healthy after this many\nconsecutive successes. The default value is 2."]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http2_health_check` after provisioning.\nA nested object resource"]
    pub fn http2_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckHttp2HealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_health_check` after provisioning.\nA nested object resource"]
    pub fn http_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckHttpHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `https_health_check` after provisioning.\nA nested object resource"]
    pub fn https_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckHttpsHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.https_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_config` after provisioning.\nConfigure logging on this health check."]
    pub fn log_config(&self) -> terrars::ListRef<DataComputeHealthCheckLogConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
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

    #[doc = "Get a reference to the value of field `ssl_health_check` after provisioning.\nA nested object resource"]
    pub fn ssl_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckSslHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_health_check` after provisioning.\nA nested object resource"]
    pub fn tcp_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckTcpHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_health_check", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout_sec` after provisioning.\nHow long (in seconds) to wait before claiming failure.\nThe default value is 5 seconds.  It is invalid for timeoutSec to have\ngreater value than checkIntervalSec."]
    pub fn timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the health check. One of HTTP, HTTPS, TCP, or SSL."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unhealthy_threshold` after provisioning.\nA so-far healthy instance will be marked unhealthy after this many\nconsecutive failures. The default value is 2."]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeHealthCheck {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeHealthCheck { }

impl terrars::ToListMappable for DataComputeHealthCheck {
    type O = terrars::ListRef<DataComputeHealthCheckRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeHealthCheck_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_health_check".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeHealthCheck {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataComputeHealthCheck {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeHealthCheck {
        let out = DataComputeHealthCheck(std::rc::Rc::new(DataComputeHealthCheck_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeHealthCheckData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeHealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeHealthCheckRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `check_interval_sec` after provisioning.\nHow often (in seconds) to send a health check. The default value is 5\nseconds."]
    pub fn check_interval_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_interval_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grpc_health_check` after provisioning.\nA nested object resource"]
    pub fn grpc_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckGrpcHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_health_check", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `healthy_threshold` after provisioning.\nA so-far unhealthy instance will be marked healthy after this many\nconsecutive successes. The default value is 2."]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http2_health_check` after provisioning.\nA nested object resource"]
    pub fn http2_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckHttp2HealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_health_check` after provisioning.\nA nested object resource"]
    pub fn http_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckHttpHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `https_health_check` after provisioning.\nA nested object resource"]
    pub fn https_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckHttpsHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.https_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_config` after provisioning.\nConfigure logging on this health check."]
    pub fn log_config(&self) -> terrars::ListRef<DataComputeHealthCheckLogConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
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

    #[doc = "Get a reference to the value of field `ssl_health_check` after provisioning.\nA nested object resource"]
    pub fn ssl_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckSslHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_health_check` after provisioning.\nA nested object resource"]
    pub fn tcp_health_check(&self) -> terrars::ListRef<DataComputeHealthCheckTcpHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_health_check", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout_sec` after provisioning.\nHow long (in seconds) to wait before claiming failure.\nThe default value is 5 seconds.  It is invalid for timeoutSec to have\ngreater value than checkIntervalSec."]
    pub fn timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the health check. One of HTTP, HTTPS, TCP, or SSL."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unhealthy_threshold` after provisioning.\nA so-far healthy instance will be marked unhealthy after this many\nconsecutive failures. The default value is 2."]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeHealthCheckGrpcHealthCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_specification: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeHealthCheckGrpcHealthCheckEl {
    #[doc = "Set the field `grpc_service_name`.\n"]
    pub fn set_grpc_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grpc_service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeHealthCheckGrpcHealthCheckEl {
    type O = terrars::BlockAssignable<DataComputeHealthCheckGrpcHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeHealthCheckGrpcHealthCheckEl {}

impl BuildDataComputeHealthCheckGrpcHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckGrpcHealthCheckEl {
        DataComputeHealthCheckGrpcHealthCheckEl {
            grpc_service_name: core::default::Default::default(),
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
        }
    }
}

pub struct DataComputeHealthCheckGrpcHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeHealthCheckGrpcHealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeHealthCheckGrpcHealthCheckElRef {
        DataComputeHealthCheckGrpcHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeHealthCheckGrpcHealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc_service_name` after provisioning.\n"]
    pub fn grpc_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grpc_service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeHealthCheckHttp2HealthCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_specification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeHealthCheckHttp2HealthCheckEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_header`.\n"]
    pub fn set_proxy_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_path`.\n"]
    pub fn set_request_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeHealthCheckHttp2HealthCheckEl {
    type O = terrars::BlockAssignable<DataComputeHealthCheckHttp2HealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeHealthCheckHttp2HealthCheckEl {}

impl BuildDataComputeHealthCheckHttp2HealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckHttp2HealthCheckEl {
        DataComputeHealthCheckHttp2HealthCheckEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request_path: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}

pub struct DataComputeHealthCheckHttp2HealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeHealthCheckHttp2HealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeHealthCheckHttp2HealthCheckElRef {
        DataComputeHealthCheckHttp2HealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeHealthCheckHttp2HealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\n"]
    pub fn proxy_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }

    #[doc = "Get a reference to the value of field `request_path` after provisioning.\n"]
    pub fn request_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeHealthCheckHttpHealthCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_specification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeHealthCheckHttpHealthCheckEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_header`.\n"]
    pub fn set_proxy_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_path`.\n"]
    pub fn set_request_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeHealthCheckHttpHealthCheckEl {
    type O = terrars::BlockAssignable<DataComputeHealthCheckHttpHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeHealthCheckHttpHealthCheckEl {}

impl BuildDataComputeHealthCheckHttpHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckHttpHealthCheckEl {
        DataComputeHealthCheckHttpHealthCheckEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request_path: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}

pub struct DataComputeHealthCheckHttpHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeHealthCheckHttpHealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeHealthCheckHttpHealthCheckElRef {
        DataComputeHealthCheckHttpHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeHealthCheckHttpHealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\n"]
    pub fn proxy_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }

    #[doc = "Get a reference to the value of field `request_path` after provisioning.\n"]
    pub fn request_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeHealthCheckHttpsHealthCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_specification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeHealthCheckHttpsHealthCheckEl {
    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_header`.\n"]
    pub fn set_proxy_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_path`.\n"]
    pub fn set_request_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeHealthCheckHttpsHealthCheckEl {
    type O = terrars::BlockAssignable<DataComputeHealthCheckHttpsHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeHealthCheckHttpsHealthCheckEl {}

impl BuildDataComputeHealthCheckHttpsHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckHttpsHealthCheckEl {
        DataComputeHealthCheckHttpsHealthCheckEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request_path: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}

pub struct DataComputeHealthCheckHttpsHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeHealthCheckHttpsHealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeHealthCheckHttpsHealthCheckElRef {
        DataComputeHealthCheckHttpsHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeHealthCheckHttpsHealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\n"]
    pub fn proxy_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }

    #[doc = "Get a reference to the value of field `request_path` after provisioning.\n"]
    pub fn request_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeHealthCheckLogConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComputeHealthCheckLogConfigEl {
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeHealthCheckLogConfigEl {
    type O = terrars::BlockAssignable<DataComputeHealthCheckLogConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeHealthCheckLogConfigEl {}

impl BuildDataComputeHealthCheckLogConfigEl {
    pub fn build(self) -> DataComputeHealthCheckLogConfigEl {
        DataComputeHealthCheckLogConfigEl { enable: core::default::Default::default() }
    }
}

pub struct DataComputeHealthCheckLogConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeHealthCheckLogConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeHealthCheckLogConfigElRef {
        DataComputeHealthCheckLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeHealthCheckLogConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeHealthCheckSslHealthCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_specification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeHealthCheckSslHealthCheckEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_header`.\n"]
    pub fn set_proxy_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request`.\n"]
    pub fn set_request(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeHealthCheckSslHealthCheckEl {
    type O = terrars::BlockAssignable<DataComputeHealthCheckSslHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeHealthCheckSslHealthCheckEl {}

impl BuildDataComputeHealthCheckSslHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckSslHealthCheckEl {
        DataComputeHealthCheckSslHealthCheckEl {
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}

pub struct DataComputeHealthCheckSslHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeHealthCheckSslHealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeHealthCheckSslHealthCheckElRef {
        DataComputeHealthCheckSslHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeHealthCheckSslHealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\n"]
    pub fn proxy_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }

    #[doc = "Get a reference to the value of field `request` after provisioning.\n"]
    pub fn request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeHealthCheckTcpHealthCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_specification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeHealthCheckTcpHealthCheckEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_name`.\n"]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_specification`.\n"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proxy_header`.\n"]
    pub fn set_proxy_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request`.\n"]
    pub fn set_request(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeHealthCheckTcpHealthCheckEl {
    type O = terrars::BlockAssignable<DataComputeHealthCheckTcpHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeHealthCheckTcpHealthCheckEl {}

impl BuildDataComputeHealthCheckTcpHealthCheckEl {
    pub fn build(self) -> DataComputeHealthCheckTcpHealthCheckEl {
        DataComputeHealthCheckTcpHealthCheckEl {
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}

pub struct DataComputeHealthCheckTcpHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeHealthCheckTcpHealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeHealthCheckTcpHealthCheckElRef {
        DataComputeHealthCheckTcpHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeHealthCheckTcpHealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `port_name` after provisioning.\n"]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port_specification` after provisioning.\n"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `proxy_header` after provisioning.\n"]
    pub fn proxy_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }

    #[doc = "Get a reference to the value of field `request` after provisioning.\n"]
    pub fn request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
