#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionHealthCheckData_ {
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
    check_interval_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unhealthy_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_health_check: std::option::Option<std::vec::Vec<ComputeRegionHealthCheck_GrpcHealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_tls_health_check: std::option::Option<std::vec::Vec<ComputeRegionHealthCheck_GrpcTlsHealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2_health_check: std::option::Option<std::vec::Vec<ComputeRegionHealthCheck_Http2HealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_health_check: std::option::Option<std::vec::Vec<ComputeRegionHealthCheck_HttpHealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_health_check: std::option::Option<std::vec::Vec<ComputeRegionHealthCheck_HttpsHealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_config: std::option::Option<std::vec::Vec<ComputeRegionHealthCheck_LogConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_health_check: std::option::Option<std::vec::Vec<ComputeRegionHealthCheck_SslHealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_health_check: std::option::Option<std::vec::Vec<ComputeRegionHealthCheck_TcpHealthCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRegionHealthCheck_Timeouts>,
    dynamic: ComputeRegionHealthCheckDynamic,
}

struct ComputeRegionHealthCheck_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRegionHealthCheckData_>,
}

#[derive(Clone)]
pub struct ComputeRegionHealthCheck(std::rc::Rc<ComputeRegionHealthCheck_>);

impl ComputeRegionHealthCheck {
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
        "Set the field `check_interval_sec`.\nHow often (in seconds) to send a health check. The default value is 5\nseconds."]
    pub fn set_check_interval_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().check_interval_sec = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `healthy_threshold`.\nA so-far unhealthy instance will be marked healthy after this many\nconsecutive successes. The default value is 2."]
    pub fn set_healthy_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().healthy_threshold = std::option::Option::Some(v.into());
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
        "Set the field `region`.\nThe Region in which the created health check should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_sec`.\nHow long (in seconds) to wait before claiming failure.\nThe default value is 5 seconds.  It is invalid for timeoutSec to have\ngreater value than checkIntervalSec."]
    pub fn set_timeout_sec(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unhealthy_threshold`.\nA so-far healthy instance will be marked unhealthy after this many\nconsecutive failures. The default value is 2."]
    pub fn set_unhealthy_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().unhealthy_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc_health_check`.\n"]
    pub fn set_grpc_health_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionHealthCheck_GrpcHealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grpc_health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grpc_health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `grpc_tls_health_check`.\n"]
    pub fn set_grpc_tls_health_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionHealthCheck_GrpcTlsHealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grpc_tls_health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grpc_tls_health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http2_health_check`.\n"]
    pub fn set_http2_health_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionHealthCheck_Http2HealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http2_health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http2_health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_health_check`.\n"]
    pub fn set_http_health_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionHealthCheck_HttpHealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `https_health_check`.\n"]
    pub fn set_https_health_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionHealthCheck_HttpsHealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().https_health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.https_health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionHealthCheck_LogConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssl_health_check`.\n"]
    pub fn set_ssl_health_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionHealthCheck_SslHealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl_health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl_health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp_health_check`.\n"]
    pub fn set_tcp_health_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionHealthCheck_TcpHealthCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tcp_health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tcp_health_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRegionHealthCheck_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `health_check_id` after provisioning.\nThe unique identifier number for the resource. This identifier is defined by the server."]
    pub fn health_check_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `healthy_threshold` after provisioning.\nA so-far unhealthy instance will be marked healthy after this many\nconsecutive successes. The default value is 2."]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created health check should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout_sec` after provisioning.\nHow long (in seconds) to wait before claiming failure.\nThe default value is 5 seconds.  It is invalid for timeoutSec to have\ngreater value than checkIntervalSec."]
    pub fn timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the health check. One of HTTP, HTTP2, HTTPS, TCP, or SSL."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unhealthy_threshold` after provisioning.\nA so-far healthy instance will be marked unhealthy after this many\nconsecutive failures. The default value is 2."]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grpc_health_check` after provisioning.\n"]
    pub fn grpc_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_GrpcHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grpc_tls_health_check` after provisioning.\n"]
    pub fn grpc_tls_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_GrpcTlsHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_tls_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http2_health_check` after provisioning.\n"]
    pub fn http2_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_Http2HealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_health_check` after provisioning.\n"]
    pub fn http_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_HttpHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `https_health_check` after provisioning.\n"]
    pub fn https_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_HttpsHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.https_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<ComputeRegionHealthCheck_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_health_check` after provisioning.\n"]
    pub fn ssl_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_SslHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_health_check` after provisioning.\n"]
    pub fn tcp_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_TcpHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionHealthCheck_TimeoutsRef {
        ComputeRegionHealthCheck_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeRegionHealthCheck {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeRegionHealthCheck { }

impl terrars::ToListMappable for ComputeRegionHealthCheck {
    type O = terrars::ListRef<ComputeRegionHealthCheckRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeRegionHealthCheck_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_region_health_check".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRegionHealthCheck {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionHealthCheck {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeRegionHealthCheck {
        let out = ComputeRegionHealthCheck(std::rc::Rc::new(ComputeRegionHealthCheck_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeRegionHealthCheckData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                check_interval_sec: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                healthy_threshold: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                timeout_sec: core::default::Default::default(),
                unhealthy_threshold: core::default::Default::default(),
                grpc_health_check: core::default::Default::default(),
                grpc_tls_health_check: core::default::Default::default(),
                http2_health_check: core::default::Default::default(),
                http_health_check: core::default::Default::default(),
                https_health_check: core::default::Default::default(),
                log_config: core::default::Default::default(),
                ssl_health_check: core::default::Default::default(),
                tcp_health_check: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRegionHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionHealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRegionHealthCheckRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `health_check_id` after provisioning.\nThe unique identifier number for the resource. This identifier is defined by the server."]
    pub fn health_check_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `healthy_threshold` after provisioning.\nA so-far unhealthy instance will be marked healthy after this many\nconsecutive successes. The default value is 2."]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created health check should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout_sec` after provisioning.\nHow long (in seconds) to wait before claiming failure.\nThe default value is 5 seconds.  It is invalid for timeoutSec to have\ngreater value than checkIntervalSec."]
    pub fn timeout_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_sec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the health check. One of HTTP, HTTP2, HTTPS, TCP, or SSL."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unhealthy_threshold` after provisioning.\nA so-far healthy instance will be marked unhealthy after this many\nconsecutive failures. The default value is 2."]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grpc_health_check` after provisioning.\n"]
    pub fn grpc_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_GrpcHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grpc_tls_health_check` after provisioning.\n"]
    pub fn grpc_tls_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_GrpcTlsHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_tls_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http2_health_check` after provisioning.\n"]
    pub fn http2_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_Http2HealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_health_check` after provisioning.\n"]
    pub fn http_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_HttpHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `https_health_check` after provisioning.\n"]
    pub fn https_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_HttpsHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.https_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<ComputeRegionHealthCheck_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_health_check` after provisioning.\n"]
    pub fn ssl_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_SslHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_health_check` after provisioning.\n"]
    pub fn tcp_health_check(&self) -> terrars::ListRef<ComputeRegionHealthCheck_TcpHealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionHealthCheck_TimeoutsRef {
        ComputeRegionHealthCheck_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionHealthCheckDynamic {
    grpc_health_check: std::option::Option<terrars::DynamicBlock<ComputeRegionHealthCheck_GrpcHealthCheck>>,
    grpc_tls_health_check: std::option::Option<terrars::DynamicBlock<ComputeRegionHealthCheck_GrpcTlsHealthCheck>>,
    http2_health_check: std::option::Option<terrars::DynamicBlock<ComputeRegionHealthCheck_Http2HealthCheck>>,
    http_health_check: std::option::Option<terrars::DynamicBlock<ComputeRegionHealthCheck_HttpHealthCheck>>,
    https_health_check: std::option::Option<terrars::DynamicBlock<ComputeRegionHealthCheck_HttpsHealthCheck>>,
    log_config: std::option::Option<terrars::DynamicBlock<ComputeRegionHealthCheck_LogConfig>>,
    ssl_health_check: std::option::Option<terrars::DynamicBlock<ComputeRegionHealthCheck_SslHealthCheck>>,
    tcp_health_check: std::option::Option<terrars::DynamicBlock<ComputeRegionHealthCheck_TcpHealthCheck>>,
}

#[derive(Serialize)]
pub struct ComputeRegionHealthCheck_GrpcHealthCheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_specification: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionHealthCheck_GrpcHealthCheck {
    #[doc =
        "Set the field `grpc_service_name`.\nThe gRPC service name for the health check.\nThe value of grpcServiceName has the following meanings by convention:\n\n* Empty serviceName means the overall status of all services at the backend.\n* Non-empty serviceName means the health of that gRPC service, as defined by the owner of the service.\n\nThe grpcServiceName can only be ASCII."]
    pub fn set_grpc_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grpc_service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nThe port number for the health check request.\nMust be specified if portName and portSpecification are not set\nor if port_specification is USE_FIXED_PORT. Valid values are 1 through 65535."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, gRPC health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionHealthCheck_GrpcHealthCheck {
    type O = terrars::BlockAssignable<ComputeRegionHealthCheck_GrpcHealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionHealthCheck_GrpcHealthCheck {}

impl BuildComputeRegionHealthCheck_GrpcHealthCheck {
    pub fn build(self) -> ComputeRegionHealthCheck_GrpcHealthCheck {
        ComputeRegionHealthCheck_GrpcHealthCheck {
            grpc_service_name: core::default::Default::default(),
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionHealthCheck_GrpcHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionHealthCheck_GrpcHealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionHealthCheck_GrpcHealthCheckRef {
        ComputeRegionHealthCheck_GrpcHealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionHealthCheck_GrpcHealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `grpc_service_name` after provisioning.\nThe gRPC service name for the health check.\nThe value of grpcServiceName has the following meanings by convention:\n\n* Empty serviceName means the overall status of all services at the backend.\n* Non-empty serviceName means the health of that gRPC service, as defined by the owner of the service.\n\nThe grpcServiceName can only be ASCII."]
    pub fn grpc_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grpc_service_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number for the health check request.\nMust be specified if portName and portSpecification are not set\nor if port_specification is USE_FIXED_PORT. Valid values are 1 through 65535."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, gRPC health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionHealthCheck_GrpcTlsHealthCheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_specification: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionHealthCheck_GrpcTlsHealthCheck {
    #[doc =
        "Set the field `grpc_service_name`.\nThe gRPC service name for the health check.\nThe value of grpcServiceName has the following meanings by convention:\n\n* Empty serviceName means the overall status of all services at the backend.\n* Non-empty serviceName means the health of that gRPC service, as defined by the owner of the service.\n\nThe grpcServiceName can only be ASCII."]
    pub fn set_grpc_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grpc_service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nThe port number for the health check request.\nMust be specified if port_specification is USE_FIXED_PORT. Valid values are 1 through 65535."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': Not supported for GRPC with TLS health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, gRPC health check follows behavior specified in the 'port' field. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionHealthCheck_GrpcTlsHealthCheck {
    type O = terrars::BlockAssignable<ComputeRegionHealthCheck_GrpcTlsHealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionHealthCheck_GrpcTlsHealthCheck {}

impl BuildComputeRegionHealthCheck_GrpcTlsHealthCheck {
    pub fn build(self) -> ComputeRegionHealthCheck_GrpcTlsHealthCheck {
        ComputeRegionHealthCheck_GrpcTlsHealthCheck {
            grpc_service_name: core::default::Default::default(),
            port: core::default::Default::default(),
            port_specification: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionHealthCheck_GrpcTlsHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionHealthCheck_GrpcTlsHealthCheckRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionHealthCheck_GrpcTlsHealthCheckRef {
        ComputeRegionHealthCheck_GrpcTlsHealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionHealthCheck_GrpcTlsHealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `grpc_service_name` after provisioning.\nThe gRPC service name for the health check.\nThe value of grpcServiceName has the following meanings by convention:\n\n* Empty serviceName means the overall status of all services at the backend.\n* Non-empty serviceName means the health of that gRPC service, as defined by the owner of the service.\n\nThe grpcServiceName can only be ASCII."]
    pub fn grpc_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grpc_service_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port number for the health check request.\nMust be specified if port_specification is USE_FIXED_PORT. Valid values are 1 through 65535."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': Not supported for GRPC with TLS health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, gRPC health check follows behavior specified in the 'port' field. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionHealthCheck_Http2HealthCheck {
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

impl ComputeRegionHealthCheck_Http2HealthCheck {
    #[doc =
        "Set the field `host`.\nThe value of the host header in the HTTP2 health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe TCP port number for the HTTP2 health check request.\nThe default value is 443."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTP2 health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_header`.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn set_proxy_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_path`.\nThe request path of the HTTP2 health check request.\nThe default value is /."]
    pub fn set_request_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response`.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionHealthCheck_Http2HealthCheck {
    type O = terrars::BlockAssignable<ComputeRegionHealthCheck_Http2HealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionHealthCheck_Http2HealthCheck {}

impl BuildComputeRegionHealthCheck_Http2HealthCheck {
    pub fn build(self) -> ComputeRegionHealthCheck_Http2HealthCheck {
        ComputeRegionHealthCheck_Http2HealthCheck {
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

#[allow(dead_code)]
pub struct ComputeRegionHealthCheck_Http2HealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionHealthCheck_Http2HealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionHealthCheck_Http2HealthCheckRef {
        ComputeRegionHealthCheck_Http2HealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionHealthCheck_Http2HealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe value of the host header in the HTTP2 health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe TCP port number for the HTTP2 health check request.\nThe default value is 443."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTP2 health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_header` after provisioning.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn proxy_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_path` after provisioning.\nThe request path of the HTTP2 health check request.\nThe default value is /."]
    pub fn request_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response` after provisioning.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionHealthCheck_HttpHealthCheck {
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

impl ComputeRegionHealthCheck_HttpHealthCheck {
    #[doc =
        "Set the field `host`.\nThe value of the host header in the HTTP health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe TCP port number for the HTTP health check request.\nThe default value is 80."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTP health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_header`.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn set_proxy_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_path`.\nThe request path of the HTTP health check request.\nThe default value is /."]
    pub fn set_request_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response`.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionHealthCheck_HttpHealthCheck {
    type O = terrars::BlockAssignable<ComputeRegionHealthCheck_HttpHealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionHealthCheck_HttpHealthCheck {}

impl BuildComputeRegionHealthCheck_HttpHealthCheck {
    pub fn build(self) -> ComputeRegionHealthCheck_HttpHealthCheck {
        ComputeRegionHealthCheck_HttpHealthCheck {
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

#[allow(dead_code)]
pub struct ComputeRegionHealthCheck_HttpHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionHealthCheck_HttpHealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionHealthCheck_HttpHealthCheckRef {
        ComputeRegionHealthCheck_HttpHealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionHealthCheck_HttpHealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe value of the host header in the HTTP health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe TCP port number for the HTTP health check request.\nThe default value is 80."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTP health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_header` after provisioning.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn proxy_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_path` after provisioning.\nThe request path of the HTTP health check request.\nThe default value is /."]
    pub fn request_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response` after provisioning.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionHealthCheck_HttpsHealthCheck {
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

impl ComputeRegionHealthCheck_HttpsHealthCheck {
    #[doc =
        "Set the field `host`.\nThe value of the host header in the HTTPS health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe TCP port number for the HTTPS health check request.\nThe default value is 443."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTPS health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_header`.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn set_proxy_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_path`.\nThe request path of the HTTPS health check request.\nThe default value is /."]
    pub fn set_request_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response`.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionHealthCheck_HttpsHealthCheck {
    type O = terrars::BlockAssignable<ComputeRegionHealthCheck_HttpsHealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionHealthCheck_HttpsHealthCheck {}

impl BuildComputeRegionHealthCheck_HttpsHealthCheck {
    pub fn build(self) -> ComputeRegionHealthCheck_HttpsHealthCheck {
        ComputeRegionHealthCheck_HttpsHealthCheck {
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

#[allow(dead_code)]
pub struct ComputeRegionHealthCheck_HttpsHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionHealthCheck_HttpsHealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionHealthCheck_HttpsHealthCheckRef {
        ComputeRegionHealthCheck_HttpsHealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionHealthCheck_HttpsHealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe value of the host header in the HTTPS health check request.\nIf left empty (default value), the public IP on behalf of which this health\ncheck is performed will be used."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe TCP port number for the HTTPS health check request.\nThe default value is 443."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, HTTPS health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_header` after provisioning.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn proxy_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_path` after provisioning.\nThe request path of the HTTPS health check request.\nThe default value is /."]
    pub fn request_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response` after provisioning.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionHealthCheck_LogConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionHealthCheck_LogConfig {
    #[doc =
        "Set the field `enable`.\nIndicates whether or not to export logs. This is false by default,\nwhich means no health check logging will be done."]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionHealthCheck_LogConfig {
    type O = terrars::BlockAssignable<ComputeRegionHealthCheck_LogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionHealthCheck_LogConfig {}

impl BuildComputeRegionHealthCheck_LogConfig {
    pub fn build(self) -> ComputeRegionHealthCheck_LogConfig {
        ComputeRegionHealthCheck_LogConfig { enable: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionHealthCheck_LogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionHealthCheck_LogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionHealthCheck_LogConfigRef {
        ComputeRegionHealthCheck_LogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionHealthCheck_LogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nIndicates whether or not to export logs. This is false by default,\nwhich means no health check logging will be done."]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionHealthCheck_SslHealthCheck {
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

impl ComputeRegionHealthCheck_SslHealthCheck {
    #[doc = "Set the field `port`.\nThe TCP port number for the SSL health check request.\nThe default value is 443."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, SSL health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_header`.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn set_proxy_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request`.\nThe application data to send once the SSL connection has been\nestablished (default value is empty). If both request and response are\nempty, the connection establishment alone will indicate health. The request\ndata can only be ASCII."]
    pub fn set_request(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response`.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionHealthCheck_SslHealthCheck {
    type O = terrars::BlockAssignable<ComputeRegionHealthCheck_SslHealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionHealthCheck_SslHealthCheck {}

impl BuildComputeRegionHealthCheck_SslHealthCheck {
    pub fn build(self) -> ComputeRegionHealthCheck_SslHealthCheck {
        ComputeRegionHealthCheck_SslHealthCheck {
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionHealthCheck_SslHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionHealthCheck_SslHealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionHealthCheck_SslHealthCheckRef {
        ComputeRegionHealthCheck_SslHealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionHealthCheck_SslHealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe TCP port number for the SSL health check request.\nThe default value is 443."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, SSL health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_header` after provisioning.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn proxy_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request` after provisioning.\nThe application data to send once the SSL connection has been\nestablished (default value is empty). If both request and response are\nempty, the connection establishment alone will indicate health. The request\ndata can only be ASCII."]
    pub fn request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response` after provisioning.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionHealthCheck_TcpHealthCheck {
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

impl ComputeRegionHealthCheck_TcpHealthCheck {
    #[doc = "Set the field `port`.\nThe TCP port number for the TCP health check request.\nThe default value is 80."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_name`.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn set_port_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port_specification`.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, TCP health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn set_port_specification(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.port_specification = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxy_header`.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn set_proxy_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.proxy_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request`.\nThe application data to send once the TCP connection has been\nestablished (default value is empty). If both request and response are\nempty, the connection establishment alone will indicate health. The request\ndata can only be ASCII."]
    pub fn set_request(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response`.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionHealthCheck_TcpHealthCheck {
    type O = terrars::BlockAssignable<ComputeRegionHealthCheck_TcpHealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionHealthCheck_TcpHealthCheck {}

impl BuildComputeRegionHealthCheck_TcpHealthCheck {
    pub fn build(self) -> ComputeRegionHealthCheck_TcpHealthCheck {
        ComputeRegionHealthCheck_TcpHealthCheck {
            port: core::default::Default::default(),
            port_name: core::default::Default::default(),
            port_specification: core::default::Default::default(),
            proxy_header: core::default::Default::default(),
            request: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionHealthCheck_TcpHealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionHealthCheck_TcpHealthCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionHealthCheck_TcpHealthCheckRef {
        ComputeRegionHealthCheck_TcpHealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionHealthCheck_TcpHealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe TCP port number for the TCP health check request.\nThe default value is 80."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_name` after provisioning.\nPort name as defined in InstanceGroup#NamedPort#name. If both port and\nport_name are defined, port takes precedence."]
    pub fn port_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port_specification` after provisioning.\nSpecifies how port is selected for health checking, can be one of the\nfollowing values:\n\n  * 'USE_FIXED_PORT': The port number in 'port' is used for health checking.\n\n  * 'USE_NAMED_PORT': The 'portName' is used for health checking.\n\n  * 'USE_SERVING_PORT': For NetworkEndpointGroup, the port specified for each\n  network endpoint is used for health checking. For other backends, the\n  port or named port specified in the Backend Service is used for health\n  checking.\n\nIf not specified, TCP health check follows behavior specified in 'port' and\n'portName' fields. Possible values: [\"USE_FIXED_PORT\", \"USE_NAMED_PORT\", \"USE_SERVING_PORT\"]"]
    pub fn port_specification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_specification", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proxy_header` after provisioning.\nSpecifies the type of proxy header to append before sending data to the\nbackend. Default value: \"NONE\" Possible values: [\"NONE\", \"PROXY_V1\"]"]
    pub fn proxy_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request` after provisioning.\nThe application data to send once the TCP connection has been\nestablished (default value is empty). If both request and response are\nempty, the connection establishment alone will indicate health. The request\ndata can only be ASCII."]
    pub fn request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response` after provisioning.\nThe bytes to match against the beginning of the response data. If left empty\n(the default value), any response will indicate health. The response data\ncan only be ASCII."]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionHealthCheck_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionHealthCheck_Timeouts {
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

impl terrars::ToListMappable for ComputeRegionHealthCheck_Timeouts {
    type O = terrars::BlockAssignable<ComputeRegionHealthCheck_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionHealthCheck_Timeouts {}

impl BuildComputeRegionHealthCheck_Timeouts {
    pub fn build(self) -> ComputeRegionHealthCheck_Timeouts {
        ComputeRegionHealthCheck_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionHealthCheck_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionHealthCheck_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionHealthCheck_TimeoutsRef {
        ComputeRegionHealthCheck_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionHealthCheck_TimeoutsRef {
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
