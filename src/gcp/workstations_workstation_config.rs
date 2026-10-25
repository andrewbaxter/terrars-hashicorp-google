#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct WorkstationsWorkstationConfigData_ {
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
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_tcp_connections: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_audit_agent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_usable_workstations: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_zones: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    running_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    workstation_cluster_id: terrars::PrimField<std::string::String>,
    workstation_config_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_ports: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_AllowedPorts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_Container>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_EncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_directories: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_EphemeralDirectories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_Host>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistent_directories: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_PersistentDirectories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    readiness_checks: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_ReadinessChecks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<WorkstationsWorkstationConfig_Timeouts>,
    dynamic: WorkstationsWorkstationConfigDynamic,
}

struct WorkstationsWorkstationConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkstationsWorkstationConfigData_>,
}

#[derive(Clone)]
pub struct WorkstationsWorkstationConfig(std::rc::Rc<WorkstationsWorkstationConfig_>);

impl WorkstationsWorkstationConfig {
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
        "Set the field `annotations`.\nClient-specified annotations. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_tcp_connections`.\nDisables support for plain TCP connections in the workstation. By default the service supports TCP connections via a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain tcp connections, such as ssh. When enabled, all communication must occur over https or wss."]
    pub fn set_disable_tcp_connections(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_tcp_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nHuman-readable name for this resource."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_audit_agent`.\nWhether to enable Linux 'auditd' logging on the workstation. When enabled, a service account must also be specified that has 'logging.buckets.write' permission on the project. Operating system audit logging is distinct from Cloud Audit Logs."]
    pub fn set_enable_audit_agent(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_audit_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `idle_timeout`.\nHow long to wait before automatically stopping an instance that hasn't recently received any user traffic. A value of 0 indicates that this instance should never time out from idleness. Defaults to 20 minutes.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_idle_timeout(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().idle_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nClient-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_usable_workstations`.\nMaximum number of workstations under this configuration a user can have workstations.workstation.use permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request."]
    pub fn set_max_usable_workstations(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_usable_workstations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `replica_zones`.\nSpecifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, '['us-central1-a', 'us-central1-f']'.\nIf this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created."]
    pub fn set_replica_zones(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().replica_zones = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `running_timeout`.\nHow long to wait before automatically stopping a workstation after it was started. A value of 0 indicates that workstations using this configuration should never time out from running duration. Must be greater than 0 and less than 24 hours if 'encryption_key' is set. Defaults to 12 hours.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_running_timeout(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().running_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_ports`.\n"]
    pub fn set_allowed_ports(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_AllowedPorts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allowed_ports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allowed_ports = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_Container>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_EncryptionKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ephemeral_directories`.\n"]
    pub fn set_ephemeral_directories(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_EphemeralDirectories>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_directories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_directories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `host`.\n"]
    pub fn set_host(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_Host>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().host = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.host = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `persistent_directories`.\n"]
    pub fn set_persistent_directories(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_PersistentDirectories>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().persistent_directories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.persistent_directories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `readiness_checks`.\n"]
    pub fn set_readiness_checks(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_ReadinessChecks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().readiness_checks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.readiness_checks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<WorkstationsWorkstationConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nClient-specified annotations. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conditions` after provisioning.\nStatus conditions describing the current resource state."]
    pub fn conditions(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime when this resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `degraded` after provisioning.\nWhether this resource is in degraded mode, in which case it may require user action to restore full functionality. Details can be found in the conditions field."]
    pub fn degraded(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.degraded", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_tcp_connections` after provisioning.\nDisables support for plain TCP connections in the workstation. By default the service supports TCP connections via a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain tcp connections, such as ssh. When enabled, all communication must occur over https or wss."]
    pub fn disable_tcp_connections(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_tcp_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name for this resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_audit_agent` after provisioning.\nWhether to enable Linux 'auditd' logging on the workstation. When enabled, a service account must also be specified that has 'logging.buckets.write' permission on the project. Operating system audit logging is distinct from Cloud Audit Logs."]
    pub fn enable_audit_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_audit_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nChecksum computed by the server.\nMay be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `idle_timeout` after provisioning.\nHow long to wait before automatically stopping an instance that hasn't recently received any user traffic. A value of 0 indicates that this instance should never time out from idleness. Defaults to 20 minutes.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nClient-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the workstation cluster config should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_usable_workstations` after provisioning.\nMaximum number of workstations under this configuration a user can have workstations.workstation.use permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request."]
    pub fn max_usable_workstations(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_usable_workstations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull name of this resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_zones` after provisioning.\nSpecifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, '['us-central1-a', 'us-central1-f']'.\nIf this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created."]
    pub fn replica_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_zones", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `running_timeout` after provisioning.\nHow long to wait before automatically stopping a workstation after it was started. A value of 0 indicates that workstations using this configuration should never time out from running duration. Must be greater than 0 and less than 24 hours if 'encryption_key' is set. Defaults to 12 hours.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn running_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.running_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workstation_cluster_id` after provisioning.\nThe ID of the parent workstation cluster."]
    pub fn workstation_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workstation_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workstation_config_id` after provisioning.\nThe ID to be assigned to the workstation cluster config."]
    pub fn workstation_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workstation_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_ports` after provisioning.\n"]
    pub fn allowed_ports(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_AllowedPortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_ports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_EncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_directories` after provisioning.\n"]
    pub fn ephemeral_directories(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_EphemeralDirectoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_directories", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_HostRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persistent_directories` after provisioning.\n"]
    pub fn persistent_directories(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_PersistentDirectoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistent_directories", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `readiness_checks` after provisioning.\n"]
    pub fn readiness_checks(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_ReadinessChecksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.readiness_checks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkstationsWorkstationConfig_TimeoutsRef {
        WorkstationsWorkstationConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for WorkstationsWorkstationConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkstationsWorkstationConfig { }

impl terrars::ToListMappable for WorkstationsWorkstationConfig {
    type O = terrars::ListRef<WorkstationsWorkstationConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkstationsWorkstationConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_workstations_workstation_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkstationsWorkstationConfig {
    pub tf_id: std::string::String,
    #[doc = "The location where the workstation cluster config should reside."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the parent workstation cluster."]
    pub workstation_cluster_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID to be assigned to the workstation cluster config."]
    pub workstation_config_id: terrars::PrimField<std::string::String>,
}

impl BuildWorkstationsWorkstationConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkstationsWorkstationConfig {
        let out = WorkstationsWorkstationConfig(std::rc::Rc::new(WorkstationsWorkstationConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkstationsWorkstationConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                disable_tcp_connections: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enable_audit_agent: core::default::Default::default(),
                id: core::default::Default::default(),
                idle_timeout: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                max_usable_workstations: core::default::Default::default(),
                project: core::default::Default::default(),
                replica_zones: core::default::Default::default(),
                running_timeout: core::default::Default::default(),
                workstation_cluster_id: self.workstation_cluster_id,
                workstation_config_id: self.workstation_config_id,
                allowed_ports: core::default::Default::default(),
                container: core::default::Default::default(),
                encryption_key: core::default::Default::default(),
                ephemeral_directories: core::default::Default::default(),
                host: core::default::Default::default(),
                persistent_directories: core::default::Default::default(),
                readiness_checks: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkstationsWorkstationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkstationsWorkstationConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nClient-specified annotations. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `conditions` after provisioning.\nStatus conditions describing the current resource state."]
    pub fn conditions(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime when this resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `degraded` after provisioning.\nWhether this resource is in degraded mode, in which case it may require user action to restore full functionality. Details can be found in the conditions field."]
    pub fn degraded(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.degraded", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_tcp_connections` after provisioning.\nDisables support for plain TCP connections in the workstation. By default the service supports TCP connections via a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain tcp connections, such as ssh. When enabled, all communication must occur over https or wss."]
    pub fn disable_tcp_connections(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_tcp_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable name for this resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_audit_agent` after provisioning.\nWhether to enable Linux 'auditd' logging on the workstation. When enabled, a service account must also be specified that has 'logging.buckets.write' permission on the project. Operating system audit logging is distinct from Cloud Audit Logs."]
    pub fn enable_audit_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_audit_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nChecksum computed by the server.\nMay be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `idle_timeout` after provisioning.\nHow long to wait before automatically stopping an instance that hasn't recently received any user traffic. A value of 0 indicates that this instance should never time out from idleness. Defaults to 20 minutes.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nClient-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the workstation cluster config should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_usable_workstations` after provisioning.\nMaximum number of workstations under this configuration a user can have workstations.workstation.use permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request."]
    pub fn max_usable_workstations(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_usable_workstations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull name of this resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replica_zones` after provisioning.\nSpecifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, '['us-central1-a', 'us-central1-f']'.\nIf this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created."]
    pub fn replica_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_zones", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `running_timeout` after provisioning.\nHow long to wait before automatically stopping a workstation after it was started. A value of 0 indicates that workstations using this configuration should never time out from running duration. Must be greater than 0 and less than 24 hours if 'encryption_key' is set. Defaults to 12 hours.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn running_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.running_timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workstation_cluster_id` after provisioning.\nThe ID of the parent workstation cluster."]
    pub fn workstation_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workstation_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workstation_config_id` after provisioning.\nThe ID to be assigned to the workstation cluster config."]
    pub fn workstation_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workstation_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allowed_ports` after provisioning.\n"]
    pub fn allowed_ports(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_AllowedPortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_ports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_EncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_directories` after provisioning.\n"]
    pub fn ephemeral_directories(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_EphemeralDirectoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_directories", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_HostRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `persistent_directories` after provisioning.\n"]
    pub fn persistent_directories(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_PersistentDirectoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persistent_directories", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `readiness_checks` after provisioning.\n"]
    pub fn readiness_checks(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_ReadinessChecksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.readiness_checks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkstationsWorkstationConfig_TimeoutsRef {
        WorkstationsWorkstationConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfigDynamic {
    allowed_ports: std::option::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_AllowedPorts>>,
    container: std::option::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_Container>>,
    encryption_key: std::option::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_EncryptionKey>>,
    ephemeral_directories: std
    ::option
    ::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_EphemeralDirectories>>,
    host: std::option::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_Host>>,
    persistent_directories: std
    ::option
    ::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_PersistentDirectories>>,
    readiness_checks: std::option::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_ReadinessChecks>>,
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkstationsWorkstationConfig_Conditions {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_Conditions {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_Conditions {}

impl BuildWorkstationsWorkstationConfig_Conditions {
    pub fn build(self) -> WorkstationsWorkstationConfig_Conditions {
        WorkstationsWorkstationConfig_Conditions {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkstationsWorkstationConfig_ConditionsRef {
        WorkstationsWorkstationConfig_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_AllowedPorts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last: std::option::Option<terrars::PrimField<f64>>,
}

impl WorkstationsWorkstationConfig_AllowedPorts {
    #[doc =
        "Set the field `first`.\nStarting port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535."]
    pub fn set_first(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.first = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `last`.\nEnding port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535."]
    pub fn set_last(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.last = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_AllowedPorts {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_AllowedPorts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_AllowedPorts {}

impl BuildWorkstationsWorkstationConfig_AllowedPorts {
    pub fn build(self) -> WorkstationsWorkstationConfig_AllowedPorts {
        WorkstationsWorkstationConfig_AllowedPorts {
            first: core::default::Default::default(),
            last: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_AllowedPortsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_AllowedPortsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkstationsWorkstationConfig_AllowedPortsRef {
        WorkstationsWorkstationConfig_AllowedPortsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_AllowedPortsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `first` after provisioning.\nStarting port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535."]
    pub fn first(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last` after provisioning.\nEnding port number for the current range of ports. Valid ports are 22, 80, and ports within the range 1024-65535."]
    pub fn last(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_Container {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_as_user: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_dir: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkstationsWorkstationConfig_Container {
    #[doc = "Set the field `args`.\nArguments passed to the entrypoint."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `command`.\nIf set, overrides the default ENTRYPOINT specified by the image."]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `env`.\nEnvironment variables passed to the container.\nThe elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image`.\nDocker image defining the container. This image must be accessible by the config's service account."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_as_user`.\nIf set, overrides the USER specified in the image with the given uid."]
    pub fn set_run_as_user(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.run_as_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `working_dir`.\nIf set, overrides the default DIR specified by the image."]
    pub fn set_working_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.working_dir = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_Container {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_Container>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_Container {}

impl BuildWorkstationsWorkstationConfig_Container {
    pub fn build(self) -> WorkstationsWorkstationConfig_Container {
        WorkstationsWorkstationConfig_Container {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: core::default::Default::default(),
            run_as_user: core::default::Default::default(),
            working_dir: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_ContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_ContainerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkstationsWorkstationConfig_ContainerRef {
        WorkstationsWorkstationConfig_ContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_ContainerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\nArguments passed to the entrypoint."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nIf set, overrides the default ENTRYPOINT specified by the image."]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nEnvironment variables passed to the container.\nThe elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
    pub fn env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nDocker image defining the container. This image must be accessible by the config's service account."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `run_as_user` after provisioning.\nIf set, overrides the USER specified in the image with the given uid."]
    pub fn run_as_user(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_as_user", self.base))
    }

    #[doc =
        "Get a reference to the value of field `working_dir` after provisioning.\nIf set, overrides the default DIR specified by the image."]
    pub fn working_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_EncryptionKey {
    kms_key: terrars::PrimField<std::string::String>,
    kms_key_service_account: terrars::PrimField<std::string::String>,
}

impl WorkstationsWorkstationConfig_EncryptionKey { }

impl terrars::ToListMappable for WorkstationsWorkstationConfig_EncryptionKey {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_EncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_EncryptionKey {
    #[doc = "The name of the Google Cloud KMS encryption key."]
    pub kms_key: terrars::PrimField<std::string::String>,
    #[doc = "The service account to use with the specified KMS key."]
    pub kms_key_service_account: terrars::PrimField<std::string::String>,
}

impl BuildWorkstationsWorkstationConfig_EncryptionKey {
    pub fn build(self) -> WorkstationsWorkstationConfig_EncryptionKey {
        WorkstationsWorkstationConfig_EncryptionKey {
            kms_key: self.kms_key,
            kms_key_service_account: self.kms_key_service_account,
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_EncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_EncryptionKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_EncryptionKeyRef {
        WorkstationsWorkstationConfig_EncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_EncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe name of the Google Cloud KMS encryption key."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account to use with the specified KMS key."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_EphemeralDirectories_GcePd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_snapshot: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkstationsWorkstationConfig_EphemeralDirectories_GcePd {
    #[doc = "Set the field `disk_type`.\nType of the disk to use. Defaults to '\"pd-standard\"'."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `read_only`.\nWhether the disk is read only. If true, the disk may be shared by multiple VMs and 'sourceSnapshot' must be set."]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_image`.\nName of the disk image to use as the source for the disk.\n\nMust be empty 'sourceSnapshot' is set.\nUpdating 'sourceImage' will update content in the ephemeral directory after the workstation is restarted."]
    pub fn set_source_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_snapshot`.\nName of the snapshot to use as the source for the disk.\n\nMust be empty if 'sourceImage' is set.\nMust be empty if 'read_only' is false.\nUpdating 'source_snapshot' will update content in the ephemeral directory after the workstation is restarted."]
    pub fn set_source_snapshot(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_snapshot = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_EphemeralDirectories_GcePd {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_EphemeralDirectories_GcePd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_EphemeralDirectories_GcePd {}

impl BuildWorkstationsWorkstationConfig_EphemeralDirectories_GcePd {
    pub fn build(self) -> WorkstationsWorkstationConfig_EphemeralDirectories_GcePd {
        WorkstationsWorkstationConfig_EphemeralDirectories_GcePd {
            disk_type: core::default::Default::default(),
            read_only: core::default::Default::default(),
            source_image: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_EphemeralDirectories_GcePdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_EphemeralDirectories_GcePdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_EphemeralDirectories_GcePdRef {
        WorkstationsWorkstationConfig_EphemeralDirectories_GcePdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_EphemeralDirectories_GcePdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nType of the disk to use. Defaults to '\"pd-standard\"'."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nWhether the disk is read only. If true, the disk may be shared by multiple VMs and 'sourceSnapshot' must be set."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_image` after provisioning.\nName of the disk image to use as the source for the disk.\n\nMust be empty 'sourceSnapshot' is set.\nUpdating 'sourceImage' will update content in the ephemeral directory after the workstation is restarted."]
    pub fn source_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_snapshot` after provisioning.\nName of the snapshot to use as the source for the disk.\n\nMust be empty if 'sourceImage' is set.\nMust be empty if 'read_only' is false.\nUpdating 'source_snapshot' will update content in the ephemeral directory after the workstation is restarted."]
    pub fn source_snapshot(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfig_EphemeralDirectoriesDynamic {
    gce_pd: std::option::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_EphemeralDirectories_GcePd>>,
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_EphemeralDirectories {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_pd: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_EphemeralDirectories_GcePd>>,
    dynamic: WorkstationsWorkstationConfig_EphemeralDirectoriesDynamic,
}

impl WorkstationsWorkstationConfig_EphemeralDirectories {
    #[doc = "Set the field `mount_path`.\nLocation of this directory in the running workstation."]
    pub fn set_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gce_pd`.\n"]
    pub fn set_gce_pd(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_EphemeralDirectories_GcePd>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gce_pd = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_pd = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_EphemeralDirectories {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_EphemeralDirectories>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_EphemeralDirectories {}

impl BuildWorkstationsWorkstationConfig_EphemeralDirectories {
    pub fn build(self) -> WorkstationsWorkstationConfig_EphemeralDirectories {
        WorkstationsWorkstationConfig_EphemeralDirectories {
            mount_path: core::default::Default::default(),
            gce_pd: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_EphemeralDirectoriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_EphemeralDirectoriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_EphemeralDirectoriesRef {
        WorkstationsWorkstationConfig_EphemeralDirectoriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_EphemeralDirectoriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mount_path` after provisioning.\nLocation of this directory in the running workstation."]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gce_pd` after provisioning.\n"]
    pub fn gce_pd(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_EphemeralDirectories_GcePdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_pd", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_Host_GceInstance_Accelerators {
    count: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WorkstationsWorkstationConfig_Host_GceInstance_Accelerators { }

impl terrars::ToListMappable for WorkstationsWorkstationConfig_Host_GceInstance_Accelerators {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance_Accelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_Host_GceInstance_Accelerators {
    #[doc = "Number of accelerator cards exposed to the instance."]
    pub count: terrars::PrimField<f64>,
    #[doc = "Type of accelerator resource to attach to the instance, for example, \"nvidia-tesla-p100\"."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWorkstationsWorkstationConfig_Host_GceInstance_Accelerators {
    pub fn build(self) -> WorkstationsWorkstationConfig_Host_GceInstance_Accelerators {
        WorkstationsWorkstationConfig_Host_GceInstance_Accelerators {
            count: self.count,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_Host_GceInstance_AcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_Host_GceInstance_AcceleratorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_Host_GceInstance_AcceleratorsRef {
        WorkstationsWorkstationConfig_Host_GceInstance_AcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_Host_GceInstance_AcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nNumber of accelerator cards exposed to the instance."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of accelerator resource to attach to the instance, for example, \"nvidia-tesla-p100\"."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigsDynamic {
    accelerators: std
    ::option
    ::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_Host_GceInstance_Accelerators>>,
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_nested_virtualization: std::option::Option<terrars::PrimField<bool>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_Host_GceInstance_Accelerators>>,
    dynamic: WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigsDynamic,
}

impl WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs {
    #[doc =
        "Set the field `boot_disk_size_gb`.\nSize of the boot disk in GB. The minimum boot disk size is '30' GB. Defaults to '50' GB."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_nested_virtualization`.\nWhether to enable nested virtualization on the Compute Engine VMs backing boosted Workstations.\n\nSee https://cloud.google.com/workstations/docs/reference/rest/v1/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization"]
    pub fn set_enable_nested_virtualization(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nThe type of machine that boosted VM instances will use—for example, e2-standard-4. For more information about machine types that Cloud Workstations supports, see the list of available machine types https://cloud.google.com/workstations/docs/available-machine-types. Defaults to e2-standard-4."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_size`.\nNumber of instances to pool for faster workstation boosting."]
    pub fn set_pool_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pool_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance_Accelerators>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs {
    #[doc = "The id to be used for the boost config."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildWorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs {
    pub fn build(self) -> WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs {
        WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs {
            boot_disk_size_gb: core::default::Default::default(),
            enable_nested_virtualization: core::default::Default::default(),
            id: self.id,
            machine_type: core::default::Default::default(),
            pool_size: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigsRef {
        WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the boot disk in GB. The minimum boot disk size is '30' GB. Defaults to '50' GB."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nWhether to enable nested virtualization on the Compute Engine VMs backing boosted Workstations.\n\nSee https://cloud.google.com/workstations/docs/reference/rest/v1/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization"]
    pub fn enable_nested_virtualization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id to be used for the boost config."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe type of machine that boosted VM instances will use—for example, e2-standard-4. For more information about machine types that Cloud Workstations supports, see the list of available machine types https://cloud.google.com/workstations/docs/available-machine-types. Defaults to e2-standard-4."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pool_size` after provisioning.\nNumber of instances to pool for faster workstation boosting."]
    pub fn pool_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_size", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_Host_GceInstance_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_confidential_compute: std::option::Option<terrars::PrimField<bool>>,
}

impl WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig {
    #[doc = "Set the field `enable_confidential_compute`.\nWhether the instance has confidential compute enabled."]
    pub fn set_enable_confidential_compute(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_confidential_compute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig {}

impl BuildWorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig {
    pub fn build(self) -> WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig {
        WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig {
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfigRef {
        WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nWhether the instance has confidential compute enabled."]
    pub fn enable_confidential_compute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_compute", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vtpm: std::option::Option<terrars::PrimField<bool>>,
}

impl WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig {
    #[doc = "Set the field `enable_integrity_monitoring`.\nWhether the instance has integrity monitoring enabled."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_secure_boot`.\nWhether the instance has Secure Boot enabled."]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_vtpm`.\nWhether the instance has the vTPM enabled."]
    pub fn set_enable_vtpm(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_vtpm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig {}

impl BuildWorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig {
    pub fn build(self) -> WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig {
        WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfigRef {
        WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nWhether the instance has integrity monitoring enabled."]
    pub fn enable_integrity_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_secure_boot` after provisioning.\nWhether the instance has Secure Boot enabled."]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_vtpm` after provisioning.\nWhether the instance has the vTPM enabled."]
    pub fn enable_vtpm(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfig_Host_GceInstanceDynamic {
    accelerators: std
    ::option
    ::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_Host_GceInstance_Accelerators>>,
    boost_configs: std
    ::option
    ::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs>>,
    confidential_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig>>,
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_Host_GceInstance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_public_ip_addresses: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_ssh: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_nested_virtualization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pool_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_Host_GceInstance_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost_configs: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_config: std
    ::option
    ::Option<std::vec::Vec<WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std
    ::option
    ::Option<std::vec::Vec<WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig>>,
    dynamic: WorkstationsWorkstationConfig_Host_GceInstanceDynamic,
}

impl WorkstationsWorkstationConfig_Host_GceInstance {
    #[doc = "Set the field `boot_disk_size_gb`.\nSize of the boot disk in GB."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_public_ip_addresses`.\nWhether instances have no public IP address."]
    pub fn set_disable_public_ip_addresses(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_public_ip_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_ssh`.\nWhether to disable SSH access to the VM."]
    pub fn set_disable_ssh(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_ssh = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_nested_virtualization`.\nWhether to enable nested virtualization on the Compute Engine VMs backing the Workstations.\n\nSee https://cloud.google.com/workstations/docs/reference/rest/v1/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization"]
    pub fn set_enable_nested_virtualization(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instance_metadata`.\nClient-specified metadata key-value pairs, to be passed to the start-up script in the VM."]
    pub fn set_instance_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\nThe name of a Compute Engine machine type."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pool_size`.\nNumber of instances to pool for faster workstation startup."]
    pub fn set_pool_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pool_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nEmail address of the service account that will be used on VM instances used to support this config. This service account must have permission to pull the specified container image. If not set, VMs will run without a service account, in which case the image must be publicly accessible."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_scopes`.\nScopes to grant to the service_account. Various scopes are automatically added based on feature usage. When specified, users of workstations under this configuration must have 'iam.serviceAccounts.actAs' on the service account."]
    pub fn set_service_account_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.service_account_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nNetwork tags to add to the Compute Engine machines backing the Workstations."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vm_tags`.\nResource manager tags to be bound to the VM instances backing the Workstations.\nTag keys and values have the same definition as\nhttps://docs.cloud.google.com/resource-manager/docs/tags/tags-overview\nKeys must be in the format 'tagKeys/{tag_key_id}', and\nvalues are in the format 'tagValues/456'."]
    pub fn set_vm_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.vm_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance_Accelerators>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `boost_configs`.\n"]
    pub fn set_boost_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boost_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boost_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confidential_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.shielded_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_Host_GceInstance {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_Host_GceInstance {}

impl BuildWorkstationsWorkstationConfig_Host_GceInstance {
    pub fn build(self) -> WorkstationsWorkstationConfig_Host_GceInstance {
        WorkstationsWorkstationConfig_Host_GceInstance {
            boot_disk_size_gb: core::default::Default::default(),
            disable_public_ip_addresses: core::default::Default::default(),
            disable_ssh: core::default::Default::default(),
            enable_nested_virtualization: core::default::Default::default(),
            instance_metadata: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            pool_size: core::default::Default::default(),
            service_account: core::default::Default::default(),
            service_account_scopes: core::default::Default::default(),
            tags: core::default::Default::default(),
            vm_tags: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            boost_configs: core::default::Default::default(),
            confidential_instance_config: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_Host_GceInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_Host_GceInstanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_Host_GceInstanceRef {
        WorkstationsWorkstationConfig_Host_GceInstanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_Host_GceInstanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize of the boot disk in GB."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_public_ip_addresses` after provisioning.\nWhether instances have no public IP address."]
    pub fn disable_public_ip_addresses(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_public_ip_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_ssh` after provisioning.\nWhether to disable SSH access to the VM."]
    pub fn disable_ssh(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_ssh", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nWhether to enable nested virtualization on the Compute Engine VMs backing the Workstations.\n\nSee https://cloud.google.com/workstations/docs/reference/rest/v1/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization"]
    pub fn enable_nested_virtualization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_metadata` after provisioning.\nClient-specified metadata key-value pairs, to be passed to the start-up script in the VM."]
    pub fn instance_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.instance_metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe name of a Compute Engine machine type."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pool_size` after provisioning.\nNumber of instances to pool for faster workstation startup."]
    pub fn pool_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nEmail address of the service account that will be used on VM instances used to support this config. This service account must have permission to pull the specified container image. If not set, VMs will run without a service account, in which case the image must be publicly accessible."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_scopes` after provisioning.\nScopes to grant to the service_account. Various scopes are automatically added based on feature usage. When specified, users of workstations under this configuration must have 'iam.serviceAccounts.actAs' on the service account."]
    pub fn service_account_scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_scopes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nNetwork tags to add to the Compute Engine machines backing the Workstations."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vm_tags` after provisioning.\nResource manager tags to be bound to the VM instances backing the Workstations.\nTag keys and values have the same definition as\nhttps://docs.cloud.google.com/resource-manager/docs/tags/tags-overview\nKeys must be in the format 'tagKeys/{tag_key_id}', and\nvalues are in the format 'tagValues/456'."]
    pub fn vm_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.vm_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_Host_GceInstance_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `boost_configs` after provisioning.\n"]
    pub fn boost_configs(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_Host_GceInstance_BoostConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boost_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> terrars::ListRef<WorkstationsWorkstationConfig_Host_GceInstance_ConfidentialInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> terrars::ListRef<WorkstationsWorkstationConfig_Host_GceInstance_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfig_HostDynamic {
    gce_instance: std::option::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_Host_GceInstance>>,
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_Host {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_instance: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_Host_GceInstance>>,
    dynamic: WorkstationsWorkstationConfig_HostDynamic,
}

impl WorkstationsWorkstationConfig_Host {
    #[doc = "Set the field `gce_instance`.\n"]
    pub fn set_gce_instance(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_Host_GceInstance>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gce_instance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_instance = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_Host {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_Host>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_Host {}

impl BuildWorkstationsWorkstationConfig_Host {
    pub fn build(self) -> WorkstationsWorkstationConfig_Host {
        WorkstationsWorkstationConfig_Host {
            gce_instance: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_HostRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_HostRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkstationsWorkstationConfig_HostRef {
        WorkstationsWorkstationConfig_HostRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_HostRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gce_instance` after provisioning.\n"]
    pub fn gce_instance(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_Host_GceInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_PersistentDirectories_GceHd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reclaim_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_snapshot: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkstationsWorkstationConfig_PersistentDirectories_GceHd {
    #[doc = "Set the field `archive_timeout`.\nHow long to wait before converting the disk into a snapshot."]
    pub fn set_archive_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.archive_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reclaim_policy`.\nWhether the persistent disk should be deleted when the workstation is deleted. Possible values: [\"DELETE\", \"RETAIN\"]"]
    pub fn set_reclaim_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reclaim_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_gb`.\nThe GB capacity of a persistent home directory. Defaults to '200'."]
    pub fn set_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_snapshot`.\nName of the snapshot to use as the source for the disk."]
    pub fn set_source_snapshot(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_snapshot = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_PersistentDirectories_GceHd {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_PersistentDirectories_GceHd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_PersistentDirectories_GceHd {}

impl BuildWorkstationsWorkstationConfig_PersistentDirectories_GceHd {
    pub fn build(self) -> WorkstationsWorkstationConfig_PersistentDirectories_GceHd {
        WorkstationsWorkstationConfig_PersistentDirectories_GceHd {
            archive_timeout: core::default::Default::default(),
            reclaim_policy: core::default::Default::default(),
            size_gb: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_PersistentDirectories_GceHdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_PersistentDirectories_GceHdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_PersistentDirectories_GceHdRef {
        WorkstationsWorkstationConfig_PersistentDirectories_GceHdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_PersistentDirectories_GceHdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_timeout` after provisioning.\nHow long to wait before converting the disk into a snapshot."]
    pub fn archive_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archive_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reclaim_policy` after provisioning.\nWhether the persistent disk should be deleted when the workstation is deleted. Possible values: [\"DELETE\", \"RETAIN\"]"]
    pub fn reclaim_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reclaim_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_gb` after provisioning.\nThe GB capacity of a persistent home directory. Defaults to '200'."]
    pub fn size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_snapshot` after provisioning.\nName of the snapshot to use as the source for the disk."]
    pub fn source_snapshot(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_PersistentDirectories_GcePd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fs_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reclaim_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_snapshot: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkstationsWorkstationConfig_PersistentDirectories_GcePd {
    #[doc =
        "Set the field `disk_type`.\nThe type of the persistent disk for the home directory. Defaults to 'pd-standard'."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fs_type`.\nType of file system that the disk should be formatted with. The workstation image must support this file system type. Must be empty if 'sourceSnapshot' is set. Defaults to 'ext4'."]
    pub fn set_fs_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fs_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reclaim_policy`.\nWhether the persistent disk should be deleted when the workstation is deleted. Valid values are 'DELETE' and 'RETAIN'. Defaults to 'DELETE'. Possible values: [\"DELETE\", \"RETAIN\"]"]
    pub fn set_reclaim_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reclaim_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `size_gb`.\nThe GB capacity of a persistent home directory for each workstation created with this configuration. Must be empty if 'sourceSnapshot' is set.\nValid values are '10', '50', '100', '200', '500', or '1000'. Defaults to '200'. If less than '200' GB, the 'diskType' must be 'pd-balanced' or 'pd-ssd'."]
    pub fn set_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_snapshot`.\nName of the snapshot to use as the source for the disk. This can be the snapshot's 'self_link', 'id', or a string in the format of 'projects/{project}/global/snapshots/{snapshot}'. If set, 'sizeGb' and 'fsType' must be empty. Can only be updated if it has an existing value."]
    pub fn set_source_snapshot(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_snapshot = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_PersistentDirectories_GcePd {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_PersistentDirectories_GcePd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_PersistentDirectories_GcePd {}

impl BuildWorkstationsWorkstationConfig_PersistentDirectories_GcePd {
    pub fn build(self) -> WorkstationsWorkstationConfig_PersistentDirectories_GcePd {
        WorkstationsWorkstationConfig_PersistentDirectories_GcePd {
            disk_type: core::default::Default::default(),
            fs_type: core::default::Default::default(),
            reclaim_policy: core::default::Default::default(),
            size_gb: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_PersistentDirectories_GcePdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_PersistentDirectories_GcePdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_PersistentDirectories_GcePdRef {
        WorkstationsWorkstationConfig_PersistentDirectories_GcePdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_PersistentDirectories_GcePdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nThe type of the persistent disk for the home directory. Defaults to 'pd-standard'."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fs_type` after provisioning.\nType of file system that the disk should be formatted with. The workstation image must support this file system type. Must be empty if 'sourceSnapshot' is set. Defaults to 'ext4'."]
    pub fn fs_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fs_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reclaim_policy` after provisioning.\nWhether the persistent disk should be deleted when the workstation is deleted. Valid values are 'DELETE' and 'RETAIN'. Defaults to 'DELETE'. Possible values: [\"DELETE\", \"RETAIN\"]"]
    pub fn reclaim_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reclaim_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_gb` after provisioning.\nThe GB capacity of a persistent home directory for each workstation created with this configuration. Must be empty if 'sourceSnapshot' is set.\nValid values are '10', '50', '100', '200', '500', or '1000'. Defaults to '200'. If less than '200' GB, the 'diskType' must be 'pd-balanced' or 'pd-ssd'."]
    pub fn size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_snapshot` after provisioning.\nName of the snapshot to use as the source for the disk. This can be the snapshot's 'self_link', 'id', or a string in the format of 'projects/{project}/global/snapshots/{snapshot}'. If set, 'sizeGb' and 'fsType' must be empty. Can only be updated if it has an existing value."]
    pub fn source_snapshot(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkstationsWorkstationConfig_PersistentDirectoriesDynamic {
    gce_hd: std::option::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_PersistentDirectories_GceHd>>,
    gce_pd: std::option::Option<terrars::DynamicBlock<WorkstationsWorkstationConfig_PersistentDirectories_GcePd>>,
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_PersistentDirectories {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_hd: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_PersistentDirectories_GceHd>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_pd: std::option::Option<std::vec::Vec<WorkstationsWorkstationConfig_PersistentDirectories_GcePd>>,
    dynamic: WorkstationsWorkstationConfig_PersistentDirectoriesDynamic,
}

impl WorkstationsWorkstationConfig_PersistentDirectories {
    #[doc = "Set the field `mount_path`.\nLocation of this directory in the running workstation."]
    pub fn set_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gce_hd`.\n"]
    pub fn set_gce_hd(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_PersistentDirectories_GceHd>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gce_hd = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_hd = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gce_pd`.\n"]
    pub fn set_gce_pd(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WorkstationsWorkstationConfig_PersistentDirectories_GcePd>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gce_pd = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gce_pd = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WorkstationsWorkstationConfig_PersistentDirectories {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_PersistentDirectories>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_PersistentDirectories {}

impl BuildWorkstationsWorkstationConfig_PersistentDirectories {
    pub fn build(self) -> WorkstationsWorkstationConfig_PersistentDirectories {
        WorkstationsWorkstationConfig_PersistentDirectories {
            mount_path: core::default::Default::default(),
            gce_hd: core::default::Default::default(),
            gce_pd: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_PersistentDirectoriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_PersistentDirectoriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_PersistentDirectoriesRef {
        WorkstationsWorkstationConfig_PersistentDirectoriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_PersistentDirectoriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mount_path` after provisioning.\nLocation of this directory in the running workstation."]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gce_hd` after provisioning.\n"]
    pub fn gce_hd(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_PersistentDirectories_GceHdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_hd", self.base))
    }

    #[doc = "Get a reference to the value of field `gce_pd` after provisioning.\n"]
    pub fn gce_pd(&self) -> terrars::ListRef<WorkstationsWorkstationConfig_PersistentDirectories_GcePdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_pd", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_ReadinessChecks {
    path: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl WorkstationsWorkstationConfig_ReadinessChecks { }

impl terrars::ToListMappable for WorkstationsWorkstationConfig_ReadinessChecks {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_ReadinessChecks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_ReadinessChecks {
    #[doc = "Path to which the request should be sent."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "Port to which the request should be sent."]
    pub port: terrars::PrimField<f64>,
}

impl BuildWorkstationsWorkstationConfig_ReadinessChecks {
    pub fn build(self) -> WorkstationsWorkstationConfig_ReadinessChecks {
        WorkstationsWorkstationConfig_ReadinessChecks {
            path: self.path,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_ReadinessChecksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_ReadinessChecksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkstationsWorkstationConfig_ReadinessChecksRef {
        WorkstationsWorkstationConfig_ReadinessChecksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_ReadinessChecksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath to which the request should be sent."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort to which the request should be sent."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkstationsWorkstationConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkstationsWorkstationConfig_Timeouts {
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

impl terrars::ToListMappable for WorkstationsWorkstationConfig_Timeouts {
    type O = terrars::BlockAssignable<WorkstationsWorkstationConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkstationsWorkstationConfig_Timeouts {}

impl BuildWorkstationsWorkstationConfig_Timeouts {
    pub fn build(self) -> WorkstationsWorkstationConfig_Timeouts {
        WorkstationsWorkstationConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkstationsWorkstationConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkstationsWorkstationConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkstationsWorkstationConfig_TimeoutsRef {
        WorkstationsWorkstationConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkstationsWorkstationConfig_TimeoutsRef {
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
