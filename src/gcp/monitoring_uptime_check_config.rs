#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MonitoringUptimeCheckConfigData_ {
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
    checker_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_check_failures: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    timeout: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_matchers: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_ContentMatchers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_check: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_HttpCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitored_resource: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_MonitoredResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_group: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_ResourceGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    synthetic_monitor: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_SyntheticMonitor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_check: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_TcpCheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MonitoringUptimeCheckConfig_Timeouts>,
    dynamic: MonitoringUptimeCheckConfigDynamic,
}

struct MonitoringUptimeCheckConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MonitoringUptimeCheckConfigData_>,
}

#[derive(Clone)]
pub struct MonitoringUptimeCheckConfig(std::rc::Rc<MonitoringUptimeCheckConfig_>);

impl MonitoringUptimeCheckConfig {
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
        "Set the field `checker_type`.\nThe checker type to use for the check. If the monitored resource type is 'servicedirectory_service', 'checker_type' must be set to 'VPC_CHECKERS'. Possible values: [\"STATIC_IP_CHECKERS\", \"VPC_CHECKERS\"]"]
    pub fn set_checker_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().checker_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `log_check_failures`.\nSpecifies whether to log the results of failed probes to Cloud Logging."]
    pub fn set_log_check_failures(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().log_check_failures = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period`.\nHow often, in seconds, the uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 300s."]
    pub fn set_period(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `selected_regions`.\nThe list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions to include a minimum of 3 locations must be provided, or an error message is returned. Not specifying this field will result in uptime checks running from all regions."]
    pub fn set_selected_regions(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().selected_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_labels`.\nUser-supplied key/value data to be used for organizing and identifying the 'UptimeCheckConfig' objects. The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
    pub fn set_user_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().user_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_matchers`.\n"]
    pub fn set_content_matchers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_ContentMatchers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_matchers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_matchers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_check`.\n"]
    pub fn set_http_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_HttpCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().http_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.http_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitored_resource`.\n"]
    pub fn set_monitored_resource(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_MonitoredResource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitored_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitored_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_group`.\n"]
    pub fn set_resource_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_ResourceGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `synthetic_monitor`.\n"]
    pub fn set_synthetic_monitor(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_SyntheticMonitor>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().synthetic_monitor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.synthetic_monitor = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp_check`.\n"]
    pub fn set_tcp_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_TcpCheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tcp_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tcp_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MonitoringUptimeCheckConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `checker_type` after provisioning.\nThe checker type to use for the check. If the monitored resource type is 'servicedirectory_service', 'checker_type' must be set to 'VPC_CHECKERS'. Possible values: [\"STATIC_IP_CHECKERS\", \"VPC_CHECKERS\"]"]
    pub fn checker_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checker_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA human-friendly name for the uptime check configuration. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_check_failures` after provisioning.\nSpecifies whether to log the results of failed probes to Cloud Logging."]
    pub fn log_check_failures(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_check_failures", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique resource name for this UptimeCheckConfig. The format is 'projects/[PROJECT_ID]/uptimeCheckConfigs/[UPTIME_CHECK_ID]'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nHow often, in seconds, the uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 300s."]
    pub fn period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `selected_regions` after provisioning.\nThe list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions to include a minimum of 3 locations must be provided, or an error message is returned. Not specifying this field will result in uptime checks running from all regions."]
    pub fn selected_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selected_regions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). [See the accepted formats]( https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)"]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uptime_check_id` after provisioning.\nThe id of the uptime check"]
    pub fn uptime_check_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uptime_check_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_labels` after provisioning.\nUser-supplied key/value data to be used for organizing and identifying the 'UptimeCheckConfig' objects. The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
    pub fn user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_matchers` after provisioning.\n"]
    pub fn content_matchers(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_ContentMatchersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_matchers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_check` after provisioning.\n"]
    pub fn http_check(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_HttpCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitored_resource` after provisioning.\n"]
    pub fn monitored_resource(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_MonitoredResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitored_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\n"]
    pub fn resource_group(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_ResourceGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `synthetic_monitor` after provisioning.\n"]
    pub fn synthetic_monitor(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_SyntheticMonitorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.synthetic_monitor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_check` after provisioning.\n"]
    pub fn tcp_check(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_TcpCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringUptimeCheckConfig_TimeoutsRef {
        MonitoringUptimeCheckConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for MonitoringUptimeCheckConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MonitoringUptimeCheckConfig { }

impl terrars::ToListMappable for MonitoringUptimeCheckConfig {
    type O = terrars::ListRef<MonitoringUptimeCheckConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MonitoringUptimeCheckConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_monitoring_uptime_check_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMonitoringUptimeCheckConfig {
    pub tf_id: std::string::String,
    #[doc =
        "A human-friendly name for the uptime check configuration. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). [See the accepted formats]( https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)"]
    pub timeout: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringUptimeCheckConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> MonitoringUptimeCheckConfig {
        let out = MonitoringUptimeCheckConfig(std::rc::Rc::new(MonitoringUptimeCheckConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MonitoringUptimeCheckConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                checker_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                log_check_failures: core::default::Default::default(),
                period: core::default::Default::default(),
                project: core::default::Default::default(),
                selected_regions: core::default::Default::default(),
                timeout: self.timeout,
                user_labels: core::default::Default::default(),
                content_matchers: core::default::Default::default(),
                http_check: core::default::Default::default(),
                monitored_resource: core::default::Default::default(),
                resource_group: core::default::Default::default(),
                synthetic_monitor: core::default::Default::default(),
                tcp_check: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MonitoringUptimeCheckConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MonitoringUptimeCheckConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `checker_type` after provisioning.\nThe checker type to use for the check. If the monitored resource type is 'servicedirectory_service', 'checker_type' must be set to 'VPC_CHECKERS'. Possible values: [\"STATIC_IP_CHECKERS\", \"VPC_CHECKERS\"]"]
    pub fn checker_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checker_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA human-friendly name for the uptime check configuration. The display name should be unique within a Stackdriver Workspace in order to make it easier to identify; however, uniqueness is not enforced."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_check_failures` after provisioning.\nSpecifies whether to log the results of failed probes to Cloud Logging."]
    pub fn log_check_failures(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_check_failures", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique resource name for this UptimeCheckConfig. The format is 'projects/[PROJECT_ID]/uptimeCheckConfigs/[UPTIME_CHECK_ID]'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `period` after provisioning.\nHow often, in seconds, the uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 300s."]
    pub fn period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `selected_regions` after provisioning.\nThe list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions to include a minimum of 3 locations must be provided, or an error message is returned. Not specifying this field will result in uptime checks running from all regions."]
    pub fn selected_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selected_regions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). [See the accepted formats]( https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Duration)"]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uptime_check_id` after provisioning.\nThe id of the uptime check"]
    pub fn uptime_check_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uptime_check_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user_labels` after provisioning.\nUser-supplied key/value data to be used for organizing and identifying the 'UptimeCheckConfig' objects. The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter."]
    pub fn user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_matchers` after provisioning.\n"]
    pub fn content_matchers(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_ContentMatchersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_matchers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_check` after provisioning.\n"]
    pub fn http_check(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_HttpCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitored_resource` after provisioning.\n"]
    pub fn monitored_resource(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_MonitoredResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitored_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\n"]
    pub fn resource_group(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_ResourceGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `synthetic_monitor` after provisioning.\n"]
    pub fn synthetic_monitor(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_SyntheticMonitorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.synthetic_monitor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tcp_check` after provisioning.\n"]
    pub fn tcp_check(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_TcpCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MonitoringUptimeCheckConfig_TimeoutsRef {
        MonitoringUptimeCheckConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct MonitoringUptimeCheckConfigDynamic {
    content_matchers: std::option::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_ContentMatchers>>,
    http_check: std::option::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_HttpCheck>>,
    monitored_resource: std::option::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_MonitoredResource>>,
    resource_group: std::option::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_ResourceGroup>>,
    synthetic_monitor: std::option::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_SyntheticMonitor>>,
    tcp_check: std::option::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_TcpCheck>>,
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_matcher: std::option::Option<terrars::PrimField<std::string::String>>,
    json_path: terrars::PrimField<std::string::String>,
}

impl MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher {
    #[doc =
        "Set the field `json_matcher`.\nOptions to perform JSONPath content matching. Default value: \"EXACT_MATCH\" Possible values: [\"EXACT_MATCH\", \"REGEX_MATCH\"]"]
    pub fn set_json_matcher(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json_matcher = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher {
    #[doc = "JSONPath within the response output pointing to the expected 'ContentMatcher::content' to match against."]
    pub json_path: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher {
    pub fn build(self) -> MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher {
        MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher {
            json_matcher: core::default::Default::default(),
            json_path: self.json_path,
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcherRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcherRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcherRef {
        MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcherRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcherRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `json_matcher` after provisioning.\nOptions to perform JSONPath content matching. Default value: \"EXACT_MATCH\" Possible values: [\"EXACT_MATCH\", \"REGEX_MATCH\"]"]
    pub fn json_matcher(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_matcher", self.base))
    }

    #[doc =
        "Get a reference to the value of field `json_path` after provisioning.\nJSONPath within the response output pointing to the expected 'ContentMatcher::content' to match against."]
    pub fn json_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringUptimeCheckConfig_ContentMatchersDynamic {
    json_path_matcher: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher>>,
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_ContentMatchers {
    content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matcher: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_path_matcher: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher>>,
    dynamic: MonitoringUptimeCheckConfig_ContentMatchersDynamic,
}

impl MonitoringUptimeCheckConfig_ContentMatchers {
    #[doc =
        "Set the field `matcher`.\nThe type of content matcher that will be applied to the server output, compared to the content string when the check is run. Default value: \"CONTAINS_STRING\" Possible values: [\"CONTAINS_STRING\", \"NOT_CONTAINS_STRING\", \"MATCHES_REGEX\", \"NOT_MATCHES_REGEX\", \"MATCHES_JSON_PATH\", \"NOT_MATCHES_JSON_PATH\"]"]
    pub fn set_matcher(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.matcher = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `json_path_matcher`.\n"]
    pub fn set_json_path_matcher(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcher>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json_path_matcher = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json_path_matcher = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_ContentMatchers {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_ContentMatchers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_ContentMatchers {
    #[doc = "String or regex content to match (max 1024 bytes)"]
    pub content: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringUptimeCheckConfig_ContentMatchers {
    pub fn build(self) -> MonitoringUptimeCheckConfig_ContentMatchers {
        MonitoringUptimeCheckConfig_ContentMatchers {
            content: self.content,
            matcher: core::default::Default::default(),
            json_path_matcher: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_ContentMatchersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_ContentMatchersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringUptimeCheckConfig_ContentMatchersRef {
        MonitoringUptimeCheckConfig_ContentMatchersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_ContentMatchersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nString or regex content to match (max 1024 bytes)"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `matcher` after provisioning.\nThe type of content matcher that will be applied to the server output, compared to the content string when the check is run. Default value: \"CONTAINS_STRING\" Possible values: [\"CONTAINS_STRING\", \"NOT_CONTAINS_STRING\", \"MATCHES_REGEX\", \"NOT_MATCHES_REGEX\", \"MATCHES_JSON_PATH\", \"NOT_MATCHES_JSON_PATH\"]"]
    pub fn matcher(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.matcher", self.base))
    }

    #[doc = "Get a reference to the value of field `json_path_matcher` after provisioning.\n"]
    pub fn json_path_matcher(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_ContentMatchers_JsonPathMatcherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_path_matcher", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_value: std::option::Option<terrars::PrimField<f64>>,
}

impl MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes {
    #[doc =
        "Set the field `status_class`.\nA class of status codes to accept. Possible values: [\"STATUS_CLASS_1XX\", \"STATUS_CLASS_2XX\", \"STATUS_CLASS_3XX\", \"STATUS_CLASS_4XX\", \"STATUS_CLASS_5XX\", \"STATUS_CLASS_ANY\"]"]
    pub fn set_status_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_value`.\nA status code to accept."]
    pub fn set_status_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes {}

impl BuildMonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes {
    pub fn build(self) -> MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes {
        MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes {
            status_class: core::default::Default::default(),
            status_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodesRef {
        MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `status_class` after provisioning.\nA class of status codes to accept. Possible values: [\"STATUS_CLASS_1XX\", \"STATUS_CLASS_2XX\", \"STATUS_CLASS_3XX\", \"STATUS_CLASS_4XX\", \"STATUS_CLASS_5XX\", \"STATUS_CLASS_ANY\"]"]
    pub fn status_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_class", self.base))
    }

    #[doc = "Get a reference to the value of field `status_value` after provisioning.\nA status code to accept."]
    pub fn status_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_value", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_HttpCheck_AuthInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
}

impl MonitoringUptimeCheckConfig_HttpCheck_AuthInfo {
    #[doc = "Set the field `password`.\nThe password to authenticate."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_wo`.\nThe password to authenticate."]
    pub fn set_password_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_wo_version`.\nTriggers update of 'password_wo' write-only. Increment this value when an update to 'password_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_password_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_wo_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_HttpCheck_AuthInfo {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_HttpCheck_AuthInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_HttpCheck_AuthInfo {
    #[doc = "The username to authenticate."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringUptimeCheckConfig_HttpCheck_AuthInfo {
    pub fn build(self) -> MonitoringUptimeCheckConfig_HttpCheck_AuthInfo {
        MonitoringUptimeCheckConfig_HttpCheck_AuthInfo {
            password: core::default::Default::default(),
            password_wo: core::default::Default::default(),
            password_wo_version: core::default::Default::default(),
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_HttpCheck_AuthInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_HttpCheck_AuthInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringUptimeCheckConfig_HttpCheck_AuthInfoRef {
        MonitoringUptimeCheckConfig_HttpCheck_AuthInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_HttpCheck_AuthInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe password to authenticate."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `password_wo` after provisioning.\nThe password to authenticate."]
    pub fn password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_wo_version` after provisioning.\nTriggers update of 'password_wo' write-only. Increment this value when an update to 'password_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn password_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe username to authenticate."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_TcpCheck_PingConfig {
    pings_count: terrars::PrimField<f64>,
}

impl MonitoringUptimeCheckConfig_TcpCheck_PingConfig { }

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_TcpCheck_PingConfig {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_TcpCheck_PingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_TcpCheck_PingConfig {
    #[doc = "Number of ICMP pings. A maximum of 3 ICMP pings is currently supported."]
    pub pings_count: terrars::PrimField<f64>,
}

impl BuildMonitoringUptimeCheckConfig_TcpCheck_PingConfig {
    pub fn build(self) -> MonitoringUptimeCheckConfig_TcpCheck_PingConfig {
        MonitoringUptimeCheckConfig_TcpCheck_PingConfig { pings_count: self.pings_count }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_TcpCheck_PingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_TcpCheck_PingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringUptimeCheckConfig_TcpCheck_PingConfigRef {
        MonitoringUptimeCheckConfig_TcpCheck_PingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_TcpCheck_PingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pings_count` after provisioning.\nNumber of ICMP pings. A maximum of 3 ICMP pings is currently supported."]
    pub fn pings_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pings_count", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication {
    #[doc =
        "Set the field `type_`.\nThe type of authentication to use. Possible values: [\"SERVICE_AGENT_AUTHENTICATION_TYPE_UNSPECIFIED\", \"OIDC_TOKEN\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication {}

impl BuildMonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication {
    pub fn build(self) -> MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication {
        MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthenticationRef {
        MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of authentication to use. Possible values: [\"SERVICE_AGENT_AUTHENTICATION_TYPE_UNSPECIFIED\", \"OIDC_TOKEN\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringUptimeCheckConfig_HttpCheckDynamic {
    accepted_response_status_codes: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes>>,
    auth_info: std::option::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_HttpCheck_AuthInfo>>,
    ping_config: std::option::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_TcpCheck_PingConfig>>,
    service_agent_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication>>,
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_HttpCheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mask_headers: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validate_ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accepted_response_status_codes: std
    ::option
    ::Option<std::vec::Vec<MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_info: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_HttpCheck_AuthInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ping_config: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_TcpCheck_PingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_authentication: std
    ::option
    ::Option<std::vec::Vec<MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication>>,
    dynamic: MonitoringUptimeCheckConfig_HttpCheckDynamic,
}

impl MonitoringUptimeCheckConfig_HttpCheck {
    #[doc =
        "Set the field `body`.\nThe request body associated with the HTTP POST request. If 'content_type' is 'URL_ENCODED', the body passed in must be URL-encoded. Users can provide a 'Content-Length' header via the 'headers' field or the API will do so. If the 'request_method' is 'GET' and 'body' is not empty, the API will return an error. The maximum byte size is 1 megabyte. Note - As with all bytes fields JSON representations are base64 encoded. e.g. 'foo=bar' in URL-encoded form is 'foo%3Dbar' and in base64 encoding is 'Zm9vJTI1M0RiYXI='."]
    pub fn set_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.body = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_type`.\nThe content type to use for the check. Possible values: [\"TYPE_UNSPECIFIED\", \"URL_ENCODED\", \"USER_PROVIDED\"]"]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_content_type`.\nA user provided content type header to use for the check. The invalid configurations outlined in the 'content_type' field apply to custom_content_type', as well as the following 1. 'content_type' is 'URL_ENCODED' and 'custom_content_type' is set. 2. 'content_type' is 'USER_PROVIDED' and 'custom_content_type' is not set."]
    pub fn set_custom_content_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `headers`.\nThe list of headers to send as part of the uptime check request. If two headers have the same key and different values, they should be entered as a single header, with the value being a comma-separated list of all the desired values as described in [RFC 2616 (page 31)](https://www.w3.org/Protocols/rfc2616/rfc2616.txt). Entering two separate headers with the same key in a Create call will cause the first to be overwritten by the second. The maximum number of headers allowed is 100."]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mask_headers`.\nBoolean specifying whether to encrypt the header information. Encryption should be specified for any headers related to authentication that you do not wish to be seen when retrieving the configuration. The server will be responsible for encrypting the headers. On Get/List calls, if 'mask_headers' is set to 'true' then the headers will be obscured with '******'."]
    pub fn set_mask_headers(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mask_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nThe path to the page to run the check against. Will be combined with the host (specified within the MonitoredResource) and port to construct the full URL. If the provided path does not begin with '/', a '/' will be prepended automatically. Optional (defaults to '/')."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nThe port to the page to run the check against. Will be combined with 'host' (specified within the ['monitored_resource'](#nested_monitored_resource)) and path to construct the full URL. Optional (defaults to 80 without SSL, or 443 with SSL)."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_method`.\nThe HTTP request method to use for the check. If set to 'METHOD_UNSPECIFIED' then 'request_method' defaults to 'GET'. Default value: \"GET\" Possible values: [\"METHOD_UNSPECIFIED\", \"GET\", \"POST\"]"]
    pub fn set_request_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_ssl`.\nIf true, use HTTPS instead of HTTP to run the check."]
    pub fn set_use_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `validate_ssl`.\nBoolean specifying whether to include SSL certificate validation as a part of the Uptime check. Only applies to checks where 'monitored_resource' is set to 'uptime_url'. If 'use_ssl' is 'false', setting 'validate_ssl' to 'true' has no effect."]
    pub fn set_validate_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.validate_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accepted_response_status_codes`.\n"]
    pub fn set_accepted_response_status_codes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodes>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accepted_response_status_codes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accepted_response_status_codes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auth_info`.\n"]
    pub fn set_auth_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_HttpCheck_AuthInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ping_config`.\n"]
    pub fn set_ping_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_TcpCheck_PingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ping_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ping_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_agent_authentication`.\n"]
    pub fn set_service_agent_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthentication>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_agent_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_agent_authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_HttpCheck {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_HttpCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_HttpCheck {}

impl BuildMonitoringUptimeCheckConfig_HttpCheck {
    pub fn build(self) -> MonitoringUptimeCheckConfig_HttpCheck {
        MonitoringUptimeCheckConfig_HttpCheck {
            body: core::default::Default::default(),
            content_type: core::default::Default::default(),
            custom_content_type: core::default::Default::default(),
            headers: core::default::Default::default(),
            mask_headers: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            request_method: core::default::Default::default(),
            use_ssl: core::default::Default::default(),
            validate_ssl: core::default::Default::default(),
            accepted_response_status_codes: core::default::Default::default(),
            auth_info: core::default::Default::default(),
            ping_config: core::default::Default::default(),
            service_agent_authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_HttpCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_HttpCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringUptimeCheckConfig_HttpCheckRef {
        MonitoringUptimeCheckConfig_HttpCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_HttpCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `body` after provisioning.\nThe request body associated with the HTTP POST request. If 'content_type' is 'URL_ENCODED', the body passed in must be URL-encoded. Users can provide a 'Content-Length' header via the 'headers' field or the API will do so. If the 'request_method' is 'GET' and 'body' is not empty, the API will return an error. The maximum byte size is 1 megabyte. Note - As with all bytes fields JSON representations are base64 encoded. e.g. 'foo=bar' in URL-encoded form is 'foo%3Dbar' and in base64 encoding is 'Zm9vJTI1M0RiYXI='."]
    pub fn body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nThe content type to use for the check. Possible values: [\"TYPE_UNSPECIFIED\", \"URL_ENCODED\", \"USER_PROVIDED\"]"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_content_type` after provisioning.\nA user provided content type header to use for the check. The invalid configurations outlined in the 'content_type' field apply to custom_content_type', as well as the following 1. 'content_type' is 'URL_ENCODED' and 'custom_content_type' is set. 2. 'content_type' is 'USER_PROVIDED' and 'custom_content_type' is not set."]
    pub fn custom_content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_content_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `headers` after provisioning.\nThe list of headers to send as part of the uptime check request. If two headers have the same key and different values, they should be entered as a single header, with the value being a comma-separated list of all the desired values as described in [RFC 2616 (page 31)](https://www.w3.org/Protocols/rfc2616/rfc2616.txt). Entering two separate headers with the same key in a Create call will cause the first to be overwritten by the second. The maximum number of headers allowed is 100."]
    pub fn headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mask_headers` after provisioning.\nBoolean specifying whether to encrypt the header information. Encryption should be specified for any headers related to authentication that you do not wish to be seen when retrieving the configuration. The server will be responsible for encrypting the headers. On Get/List calls, if 'mask_headers' is set to 'true' then the headers will be obscured with '******'."]
    pub fn mask_headers(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mask_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe path to the page to run the check against. Will be combined with the host (specified within the MonitoredResource) and port to construct the full URL. If the provided path does not begin with '/', a '/' will be prepended automatically. Optional (defaults to '/')."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port to the page to run the check against. Will be combined with 'host' (specified within the ['monitored_resource'](#nested_monitored_resource)) and path to construct the full URL. Optional (defaults to 80 without SSL, or 443 with SSL)."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_method` after provisioning.\nThe HTTP request method to use for the check. If set to 'METHOD_UNSPECIFIED' then 'request_method' defaults to 'GET'. Default value: \"GET\" Possible values: [\"METHOD_UNSPECIFIED\", \"GET\", \"POST\"]"]
    pub fn request_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_ssl` after provisioning.\nIf true, use HTTPS instead of HTTP to run the check."]
    pub fn use_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_ssl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `validate_ssl` after provisioning.\nBoolean specifying whether to include SSL certificate validation as a part of the Uptime check. Only applies to checks where 'monitored_resource' is set to 'uptime_url'. If 'use_ssl' is 'false', setting 'validate_ssl' to 'true' has no effect."]
    pub fn validate_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validate_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `accepted_response_status_codes` after provisioning.\n"]
    pub fn accepted_response_status_codes(
        &self,
    ) -> terrars::ListRef<MonitoringUptimeCheckConfig_HttpCheck_AcceptedResponseStatusCodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accepted_response_status_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_info` after provisioning.\n"]
    pub fn auth_info(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_HttpCheck_AuthInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_info", self.base))
    }

    #[doc = "Get a reference to the value of field `ping_config` after provisioning.\n"]
    pub fn ping_config(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_TcpCheck_PingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ping_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_agent_authentication` after provisioning.\n"]
    pub fn service_agent_authentication(
        &self,
    ) -> terrars::ListRef<MonitoringUptimeCheckConfig_HttpCheck_ServiceAgentAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_agent_authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_MonitoredResource {
    labels: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl MonitoringUptimeCheckConfig_MonitoredResource { }

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_MonitoredResource {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_MonitoredResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_MonitoredResource {
    #[doc =
        "Values for all of the labels listed in the associated monitored resource descriptor. For example, Compute Engine VM instances use the labels 'project_id', 'instance_id', and 'zone'."]
    pub labels: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The monitored resource type. This field must match the type field of a ['MonitoredResourceDescriptor'](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.monitoredResourceDescriptors#MonitoredResourceDescriptor) object. For example, the type of a Compute Engine VM instance is 'gce_instance'. For a list of types, see [Monitoring resource types](https://cloud.google.com/monitoring/api/resources) and [Logging resource types](https://cloud.google.com/logging/docs/api/v2/resource-list)."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringUptimeCheckConfig_MonitoredResource {
    pub fn build(self) -> MonitoringUptimeCheckConfig_MonitoredResource {
        MonitoringUptimeCheckConfig_MonitoredResource {
            labels: self.labels,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_MonitoredResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_MonitoredResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringUptimeCheckConfig_MonitoredResourceRef {
        MonitoringUptimeCheckConfig_MonitoredResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_MonitoredResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nValues for all of the labels listed in the associated monitored resource descriptor. For example, Compute Engine VM instances use the labels 'project_id', 'instance_id', and 'zone'."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe monitored resource type. This field must match the type field of a ['MonitoredResourceDescriptor'](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.monitoredResourceDescriptors#MonitoredResourceDescriptor) object. For example, the type of a Compute Engine VM instance is 'gce_instance'. For a list of types, see [Monitoring resource types](https://cloud.google.com/monitoring/api/resources) and [Logging resource types](https://cloud.google.com/logging/docs/api/v2/resource-list)."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_ResourceGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringUptimeCheckConfig_ResourceGroup {
    #[doc = "Set the field `group_id`.\nThe group of resources being monitored. Should be the 'name' of a group"]
    pub fn set_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_type`.\nThe resource type of the group members. Possible values: [\"RESOURCE_TYPE_UNSPECIFIED\", \"INSTANCE\", \"AWS_ELB_LOAD_BALANCER\"]"]
    pub fn set_resource_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_ResourceGroup {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_ResourceGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_ResourceGroup {}

impl BuildMonitoringUptimeCheckConfig_ResourceGroup {
    pub fn build(self) -> MonitoringUptimeCheckConfig_ResourceGroup {
        MonitoringUptimeCheckConfig_ResourceGroup {
            group_id: core::default::Default::default(),
            resource_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_ResourceGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_ResourceGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringUptimeCheckConfig_ResourceGroupRef {
        MonitoringUptimeCheckConfig_ResourceGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_ResourceGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group_id` after provisioning.\nThe group of resources being monitored. Should be the 'name' of a group"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_type` after provisioning.\nThe resource type of the group members. Possible values: [\"RESOURCE_TYPE_UNSPECIFIED\", \"INSTANCE\", \"AWS_ELB_LOAD_BALANCER\"]"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2 {
    name: terrars::PrimField<std::string::String>,
}

impl MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2 { }

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2 {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2 {
    #[doc = "The fully qualified name of the cloud function resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2 {
    pub fn build(self) -> MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2 {
        MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2 { name: self.name }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2Ref {
        MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe fully qualified name of the cloud function resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringUptimeCheckConfig_SyntheticMonitorDynamic {
    cloud_function_v2: std
    ::option
    ::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2>>,
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_SyntheticMonitor {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_function_v2: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2>>,
    dynamic: MonitoringUptimeCheckConfig_SyntheticMonitorDynamic,
}

impl MonitoringUptimeCheckConfig_SyntheticMonitor {
    #[doc = "Set the field `cloud_function_v2`.\n"]
    pub fn set_cloud_function_v2(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_function_v2 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_function_v2 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_SyntheticMonitor {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_SyntheticMonitor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_SyntheticMonitor {}

impl BuildMonitoringUptimeCheckConfig_SyntheticMonitor {
    pub fn build(self) -> MonitoringUptimeCheckConfig_SyntheticMonitor {
        MonitoringUptimeCheckConfig_SyntheticMonitor {
            cloud_function_v2: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_SyntheticMonitorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_SyntheticMonitorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MonitoringUptimeCheckConfig_SyntheticMonitorRef {
        MonitoringUptimeCheckConfig_SyntheticMonitorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_SyntheticMonitorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_function_v2` after provisioning.\n"]
    pub fn cloud_function_v2(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_SyntheticMonitor_CloudFunctionV2Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_function_v2", self.base))
    }
}

#[derive(Serialize, Default)]
struct MonitoringUptimeCheckConfig_TcpCheckDynamic {
    ping_config: std::option::Option<terrars::DynamicBlock<MonitoringUptimeCheckConfig_TcpCheck_PingConfig>>,
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_TcpCheck {
    port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ping_config: std::option::Option<std::vec::Vec<MonitoringUptimeCheckConfig_TcpCheck_PingConfig>>,
    dynamic: MonitoringUptimeCheckConfig_TcpCheckDynamic,
}

impl MonitoringUptimeCheckConfig_TcpCheck {
    #[doc = "Set the field `ping_config`.\n"]
    pub fn set_ping_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MonitoringUptimeCheckConfig_TcpCheck_PingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ping_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ping_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_TcpCheck {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_TcpCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_TcpCheck {
    #[doc =
        "The port to the page to run the check against. Will be combined with host (specified within the 'monitored_resource') to construct the full URL."]
    pub port: terrars::PrimField<f64>,
}

impl BuildMonitoringUptimeCheckConfig_TcpCheck {
    pub fn build(self) -> MonitoringUptimeCheckConfig_TcpCheck {
        MonitoringUptimeCheckConfig_TcpCheck {
            port: self.port,
            ping_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_TcpCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_TcpCheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringUptimeCheckConfig_TcpCheckRef {
        MonitoringUptimeCheckConfig_TcpCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_TcpCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port to the page to run the check against. Will be combined with host (specified within the 'monitored_resource') to construct the full URL."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `ping_config` after provisioning.\n"]
    pub fn ping_config(&self) -> terrars::ListRef<MonitoringUptimeCheckConfig_TcpCheck_PingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ping_config", self.base))
    }
}

#[derive(Serialize)]
pub struct MonitoringUptimeCheckConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MonitoringUptimeCheckConfig_Timeouts {
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

impl terrars::ToListMappable for MonitoringUptimeCheckConfig_Timeouts {
    type O = terrars::BlockAssignable<MonitoringUptimeCheckConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMonitoringUptimeCheckConfig_Timeouts {}

impl BuildMonitoringUptimeCheckConfig_Timeouts {
    pub fn build(self) -> MonitoringUptimeCheckConfig_Timeouts {
        MonitoringUptimeCheckConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MonitoringUptimeCheckConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MonitoringUptimeCheckConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MonitoringUptimeCheckConfig_TimeoutsRef {
        MonitoringUptimeCheckConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MonitoringUptimeCheckConfig_TimeoutsRef {
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
