#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudfunctionsFunctionData_ {
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
    available_memory_mb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_worker_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_registry: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_point: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_trigger_security_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_trigger_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_settings: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instances: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    runtime: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_archive_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_archive_object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_http: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connector_egress_settings: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_update_policy: std::option::Option<std::vec::Vec<CloudfunctionsFunction_AutomaticUpdatePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_trigger: std::option::Option<std::vec::Vec<CloudfunctionsFunction_EventTrigger>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_deploy_update_policy: std::option::Option<std::vec::Vec<CloudfunctionsFunction_OnDeployUpdatePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_environment_variables: std::option::Option<std::vec::Vec<CloudfunctionsFunction_SecretEnvironmentVariables>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_volumes: std::option::Option<std::vec::Vec<CloudfunctionsFunction_SecretVolumes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_repository: std::option::Option<std::vec::Vec<CloudfunctionsFunction_SourceRepository>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudfunctionsFunction_Timeouts>,
    dynamic: CloudfunctionsFunctionDynamic,
}

struct CloudfunctionsFunction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfunctionsFunctionData_>,
}

#[derive(Clone)]
pub struct CloudfunctionsFunction(std::rc::Rc<CloudfunctionsFunction_>);

impl CloudfunctionsFunction {
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
        "Set the field `available_memory_mb`.\nMemory (in MB), available to the function. Default value is 256. Possible values include 128, 256, 512, 1024, etc."]
    pub fn set_available_memory_mb(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().available_memory_mb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `build_environment_variables`.\n A set of key/value environment variable pairs available during build time."]
    pub fn set_build_environment_variables(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().build_environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `build_service_account`.\nThe fully-qualified name of the service account to be used for the build step of deploying this function"]
    pub fn set_build_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().build_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `build_worker_pool`.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn set_build_worker_pool(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().build_worker_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the function."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `docker_registry`.\nDocker Registry to use for storing the function's Docker images. Allowed values are ARTIFACT_REGISTRY (default) and CONTAINER_REGISTRY."]
    pub fn set_docker_registry(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().docker_registry = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `docker_repository`.\nUser managed repository created in Artifact Registry optionally with a customer managed encryption key. If specified, deployments will use Artifact Registry for storing images built with Cloud Build."]
    pub fn set_docker_repository(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().docker_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `entry_point`.\nName of the function that will be executed when the Google Cloud Function is triggered."]
    pub fn set_entry_point(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().entry_point = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `environment_variables`.\nA set of key/value environment variable pairs to assign to the function."]
    pub fn set_environment_variables(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `https_trigger_security_level`.\nThe security level for the function. Defaults to SECURE_OPTIONAL. Valid only if trigger_http is used."]
    pub fn set_https_trigger_security_level(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().https_trigger_security_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `https_trigger_url`.\nURL which triggers function execution. Returned only if trigger_http is used."]
    pub fn set_https_trigger_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().https_trigger_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ingress_settings`.\nString value that controls what traffic can reach the function. Allowed values are ALLOW_ALL and ALLOW_INTERNAL_ONLY. Changes to this field will recreate the cloud function."]
    pub fn set_ingress_settings(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ingress_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_name`.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources."]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA set of key/value label pairs to assign to the function. Label keys must follow the requirements at https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_instances`.\nThe limit on the maximum number of function instances that may coexist at a given time."]
    pub fn set_max_instances(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_instances`.\nThe limit on the minimum number of function instances that may coexist at a given time."]
    pub fn set_min_instances(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nProject of the function. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion of function. If it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_email`.\n If provided, the self-provided service account to run the function with."]
    pub fn set_service_account_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_archive_bucket`.\nThe GCS bucket containing the zip archive which contains the function."]
    pub fn set_source_archive_bucket(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_archive_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_archive_object`.\nThe source archive object (file) in archive bucket."]
    pub fn set_source_archive_object(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_archive_object = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout`.\nTimeout (in seconds) for the function. Default value is 60 seconds. Cannot be more than 540 seconds."]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trigger_http`.\nBoolean variable. Any HTTP request (of a supported type) to the endpoint will trigger function execution. Supported HTTP request types are: POST, PUT, GET, DELETE, and OPTIONS. Endpoint is returned as https_trigger_url. Cannot be used with trigger_bucket and trigger_topic."]
    pub fn set_trigger_http(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().trigger_http = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vpc_connector`.\nThe VPC Network Connector that this cloud function can connect to. It can be either the fully-qualified URI, or the short name of the network connector resource. The format of this field is projects/*/locations/*/connectors/*."]
    pub fn set_vpc_connector(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_connector = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vpc_connector_egress_settings`.\nThe egress settings for the connector, controlling what traffic is diverted through it. Allowed values are ALL_TRAFFIC and PRIVATE_RANGES_ONLY. Defaults to PRIVATE_RANGES_ONLY. If unset, this field preserves the previously set value."]
    pub fn set_vpc_connector_egress_settings(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().vpc_connector_egress_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_update_policy`.\n"]
    pub fn set_automatic_update_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfunctionsFunction_AutomaticUpdatePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automatic_update_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automatic_update_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_trigger`.\n"]
    pub fn set_event_trigger(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfunctionsFunction_EventTrigger>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_trigger = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_deploy_update_policy`.\n"]
    pub fn set_on_deploy_update_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfunctionsFunction_OnDeployUpdatePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_deploy_update_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.on_deploy_update_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_environment_variables`.\n"]
    pub fn set_secret_environment_variables(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfunctionsFunction_SecretEnvironmentVariables>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secret_environment_variables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secret_environment_variables = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_volumes`.\n"]
    pub fn set_secret_volumes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfunctionsFunction_SecretVolumes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secret_volumes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secret_volumes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_repository`.\n"]
    pub fn set_source_repository(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfunctionsFunction_SourceRepository>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_repository = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudfunctionsFunction_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `available_memory_mb` after provisioning.\nMemory (in MB), available to the function. Default value is 256. Possible values include 128, 256, 512, 1024, etc."]
    pub fn available_memory_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_memory_mb", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_environment_variables` after provisioning.\n A set of key/value environment variable pairs available during build time."]
    pub fn build_environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.build_environment_variables", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_service_account` after provisioning.\nThe fully-qualified name of the service account to be used for the build step of deploying this function"]
    pub fn build_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_worker_pool` after provisioning.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn build_worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_worker_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the function."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `docker_registry` after provisioning.\nDocker Registry to use for storing the function's Docker images. Allowed values are ARTIFACT_REGISTRY (default) and CONTAINER_REGISTRY."]
    pub fn docker_registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.docker_registry", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `docker_repository` after provisioning.\nUser managed repository created in Artifact Registry optionally with a customer managed encryption key. If specified, deployments will use Artifact Registry for storing images built with Cloud Build."]
    pub fn docker_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.docker_repository", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_point` after provisioning.\nName of the function that will be executed when the Google Cloud Function is triggered."]
    pub fn entry_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_point", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environment_variables` after provisioning.\nA set of key/value environment variable pairs to assign to the function."]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `https_trigger_security_level` after provisioning.\nThe security level for the function. Defaults to SECURE_OPTIONAL. Valid only if trigger_http is used."]
    pub fn https_trigger_security_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_trigger_security_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `https_trigger_url` after provisioning.\nURL which triggers function execution. Returned only if trigger_http is used."]
    pub fn https_trigger_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_trigger_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingress_settings` after provisioning.\nString value that controls what traffic can reach the function. Allowed values are ALLOW_ALL and ALLOW_INTERNAL_ONLY. Changes to this field will recreate the cloud function."]
    pub fn ingress_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the function. Label keys must follow the requirements at https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_instances` after provisioning.\nThe limit on the maximum number of function instances that may coexist at a given time."]
    pub fn max_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_instances` after provisioning.\nThe limit on the minimum number of function instances that may coexist at a given time."]
    pub fn min_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instances", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the function. Function names must be unique globally."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nProject of the function. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion of function. If it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime` after provisioning.\nThe runtime in which the function is going to run. Eg. \"nodejs20\", \"python37\", \"go111\"."]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\n If provided, the self-provided service account to run the function with."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_archive_bucket` after provisioning.\nThe GCS bucket containing the zip archive which contains the function."]
    pub fn source_archive_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_archive_bucket", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_archive_object` after provisioning.\nThe source archive object (file) in archive bucket."]
    pub fn source_archive_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_archive_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nDescribes the current stage of a deployment."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nTimeout (in seconds) for the function. Default value is 60 seconds. Cannot be more than 540 seconds."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trigger_http` after provisioning.\nBoolean variable. Any HTTP request (of a supported type) to the endpoint will trigger function execution. Supported HTTP request types are: POST, PUT, GET, DELETE, and OPTIONS. Endpoint is returned as https_trigger_url. Cannot be used with trigger_bucket and trigger_topic."]
    pub fn trigger_http(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_http", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_id` after provisioning.\nThe version identifier of the Cloud Function. Each deployment attempt results in a new version of a function being created."]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_connector` after provisioning.\nThe VPC Network Connector that this cloud function can connect to. It can be either the fully-qualified URI, or the short name of the network connector resource. The format of this field is projects/*/locations/*/connectors/*."]
    pub fn vpc_connector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_connector", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_connector_egress_settings` after provisioning.\nThe egress settings for the connector, controlling what traffic is diverted through it. Allowed values are ALL_TRAFFIC and PRIVATE_RANGES_ONLY. Defaults to PRIVATE_RANGES_ONLY. If unset, this field preserves the previously set value."]
    pub fn vpc_connector_egress_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector_egress_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `automatic_update_policy` after provisioning.\n"]
    pub fn automatic_update_policy(&self) -> terrars::ListRef<CloudfunctionsFunction_AutomaticUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_trigger` after provisioning.\n"]
    pub fn event_trigger(&self) -> terrars::ListRef<CloudfunctionsFunction_EventTriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_trigger", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_deploy_update_policy` after provisioning.\n"]
    pub fn on_deploy_update_policy(&self) -> terrars::ListRef<CloudfunctionsFunction_OnDeployUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_deploy_update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_environment_variables` after provisioning.\n"]
    pub fn secret_environment_variables(&self) -> terrars::ListRef<CloudfunctionsFunction_SecretEnvironmentVariablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_volumes` after provisioning.\n"]
    pub fn secret_volumes(&self) -> terrars::ListRef<CloudfunctionsFunction_SecretVolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_volumes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_repository` after provisioning.\n"]
    pub fn source_repository(&self) -> terrars::ListRef<CloudfunctionsFunction_SourceRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudfunctionsFunction_TimeoutsRef {
        CloudfunctionsFunction_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfunctionsFunction {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfunctionsFunction { }

impl terrars::ToListMappable for CloudfunctionsFunction {
    type O = terrars::ListRef<CloudfunctionsFunctionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfunctionsFunction_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloudfunctions_function".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfunctionsFunction {
    pub tf_id: std::string::String,
    #[doc = "A user-defined name of the function. Function names must be unique globally."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The runtime in which the function is going to run. Eg. \"nodejs20\", \"python37\", \"go111\"."]
    pub runtime: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctionsFunction {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfunctionsFunction {
        let out = CloudfunctionsFunction(std::rc::Rc::new(CloudfunctionsFunction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfunctionsFunctionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                available_memory_mb: core::default::Default::default(),
                build_environment_variables: core::default::Default::default(),
                build_service_account: core::default::Default::default(),
                build_worker_pool: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                docker_registry: core::default::Default::default(),
                docker_repository: core::default::Default::default(),
                entry_point: core::default::Default::default(),
                environment_variables: core::default::Default::default(),
                https_trigger_security_level: core::default::Default::default(),
                https_trigger_url: core::default::Default::default(),
                id: core::default::Default::default(),
                ingress_settings: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                max_instances: core::default::Default::default(),
                min_instances: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                runtime: self.runtime,
                service_account_email: core::default::Default::default(),
                source_archive_bucket: core::default::Default::default(),
                source_archive_object: core::default::Default::default(),
                timeout: core::default::Default::default(),
                trigger_http: core::default::Default::default(),
                vpc_connector: core::default::Default::default(),
                vpc_connector_egress_settings: core::default::Default::default(),
                automatic_update_policy: core::default::Default::default(),
                event_trigger: core::default::Default::default(),
                on_deploy_update_policy: core::default::Default::default(),
                secret_environment_variables: core::default::Default::default(),
                secret_volumes: core::default::Default::default(),
                source_repository: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfunctionsFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfunctionsFunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfunctionsFunctionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `available_memory_mb` after provisioning.\nMemory (in MB), available to the function. Default value is 256. Possible values include 128, 256, 512, 1024, etc."]
    pub fn available_memory_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_memory_mb", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_environment_variables` after provisioning.\n A set of key/value environment variable pairs available during build time."]
    pub fn build_environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.build_environment_variables", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_service_account` after provisioning.\nThe fully-qualified name of the service account to be used for the build step of deploying this function"]
    pub fn build_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `build_worker_pool` after provisioning.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn build_worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_worker_pool", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the function."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `docker_registry` after provisioning.\nDocker Registry to use for storing the function's Docker images. Allowed values are ARTIFACT_REGISTRY (default) and CONTAINER_REGISTRY."]
    pub fn docker_registry(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.docker_registry", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `docker_repository` after provisioning.\nUser managed repository created in Artifact Registry optionally with a customer managed encryption key. If specified, deployments will use Artifact Registry for storing images built with Cloud Build."]
    pub fn docker_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.docker_repository", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_point` after provisioning.\nName of the function that will be executed when the Google Cloud Function is triggered."]
    pub fn entry_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_point", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environment_variables` after provisioning.\nA set of key/value environment variable pairs to assign to the function."]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `https_trigger_security_level` after provisioning.\nThe security level for the function. Defaults to SECURE_OPTIONAL. Valid only if trigger_http is used."]
    pub fn https_trigger_security_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_trigger_security_level", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `https_trigger_url` after provisioning.\nURL which triggers function execution. Returned only if trigger_http is used."]
    pub fn https_trigger_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_trigger_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingress_settings` after provisioning.\nString value that controls what traffic can reach the function. Allowed values are ALLOW_ALL and ALLOW_INTERNAL_ONLY. Changes to this field will recreate the cloud function."]
    pub fn ingress_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the function. Label keys must follow the requirements at https://docs.cloud.google.com/resource-manager/docs/creating-managing-labels#requirements.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_instances` after provisioning.\nThe limit on the maximum number of function instances that may coexist at a given time."]
    pub fn max_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_instances` after provisioning.\nThe limit on the minimum number of function instances that may coexist at a given time."]
    pub fn min_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instances", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the function. Function names must be unique globally."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nProject of the function. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion of function. If it is not provided, the provider region is used."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime` after provisioning.\nThe runtime in which the function is going to run. Eg. \"nodejs20\", \"python37\", \"go111\"."]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\n If provided, the self-provided service account to run the function with."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_archive_bucket` after provisioning.\nThe GCS bucket containing the zip archive which contains the function."]
    pub fn source_archive_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_archive_bucket", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_archive_object` after provisioning.\nThe source archive object (file) in archive bucket."]
    pub fn source_archive_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_archive_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nDescribes the current stage of a deployment."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nTimeout (in seconds) for the function. Default value is 60 seconds. Cannot be more than 540 seconds."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trigger_http` after provisioning.\nBoolean variable. Any HTTP request (of a supported type) to the endpoint will trigger function execution. Supported HTTP request types are: POST, PUT, GET, DELETE, and OPTIONS. Endpoint is returned as https_trigger_url. Cannot be used with trigger_bucket and trigger_topic."]
    pub fn trigger_http(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_http", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_id` after provisioning.\nThe version identifier of the Cloud Function. Each deployment attempt results in a new version of a function being created."]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_connector` after provisioning.\nThe VPC Network Connector that this cloud function can connect to. It can be either the fully-qualified URI, or the short name of the network connector resource. The format of this field is projects/*/locations/*/connectors/*."]
    pub fn vpc_connector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_connector", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_connector_egress_settings` after provisioning.\nThe egress settings for the connector, controlling what traffic is diverted through it. Allowed values are ALL_TRAFFIC and PRIVATE_RANGES_ONLY. Defaults to PRIVATE_RANGES_ONLY. If unset, this field preserves the previously set value."]
    pub fn vpc_connector_egress_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector_egress_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `automatic_update_policy` after provisioning.\n"]
    pub fn automatic_update_policy(&self) -> terrars::ListRef<CloudfunctionsFunction_AutomaticUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_trigger` after provisioning.\n"]
    pub fn event_trigger(&self) -> terrars::ListRef<CloudfunctionsFunction_EventTriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_trigger", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `on_deploy_update_policy` after provisioning.\n"]
    pub fn on_deploy_update_policy(&self) -> terrars::ListRef<CloudfunctionsFunction_OnDeployUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_deploy_update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_environment_variables` after provisioning.\n"]
    pub fn secret_environment_variables(&self) -> terrars::ListRef<CloudfunctionsFunction_SecretEnvironmentVariablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_volumes` after provisioning.\n"]
    pub fn secret_volumes(&self) -> terrars::ListRef<CloudfunctionsFunction_SecretVolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_volumes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_repository` after provisioning.\n"]
    pub fn source_repository(&self) -> terrars::ListRef<CloudfunctionsFunction_SourceRepositoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_repository", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudfunctionsFunction_TimeoutsRef {
        CloudfunctionsFunction_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudfunctionsFunctionDynamic {
    automatic_update_policy: std::option::Option<terrars::DynamicBlock<CloudfunctionsFunction_AutomaticUpdatePolicy>>,
    event_trigger: std::option::Option<terrars::DynamicBlock<CloudfunctionsFunction_EventTrigger>>,
    on_deploy_update_policy: std::option::Option<terrars::DynamicBlock<CloudfunctionsFunction_OnDeployUpdatePolicy>>,
    secret_environment_variables: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfunctionsFunction_SecretEnvironmentVariables>>,
    secret_volumes: std::option::Option<terrars::DynamicBlock<CloudfunctionsFunction_SecretVolumes>>,
    source_repository: std::option::Option<terrars::DynamicBlock<CloudfunctionsFunction_SourceRepository>>,
}

#[derive(Serialize)]
pub struct CloudfunctionsFunction_AutomaticUpdatePolicy {}

impl CloudfunctionsFunction_AutomaticUpdatePolicy { }

impl terrars::ToListMappable for CloudfunctionsFunction_AutomaticUpdatePolicy {
    type O = terrars::BlockAssignable<CloudfunctionsFunction_AutomaticUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctionsFunction_AutomaticUpdatePolicy {}

impl BuildCloudfunctionsFunction_AutomaticUpdatePolicy {
    pub fn build(self) -> CloudfunctionsFunction_AutomaticUpdatePolicy {
        CloudfunctionsFunction_AutomaticUpdatePolicy {}
    }
}

#[allow(dead_code)]
pub struct CloudfunctionsFunction_AutomaticUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfunctionsFunction_AutomaticUpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfunctionsFunction_AutomaticUpdatePolicyRef {
        CloudfunctionsFunction_AutomaticUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfunctionsFunction_AutomaticUpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CloudfunctionsFunction_EventTrigger_FailurePolicy {
    retry: terrars::PrimField<bool>,
}

impl CloudfunctionsFunction_EventTrigger_FailurePolicy { }

impl terrars::ToListMappable for CloudfunctionsFunction_EventTrigger_FailurePolicy {
    type O = terrars::BlockAssignable<CloudfunctionsFunction_EventTrigger_FailurePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctionsFunction_EventTrigger_FailurePolicy {
    #[doc = "Whether the function should be retried on failure. Defaults to false."]
    pub retry: terrars::PrimField<bool>,
}

impl BuildCloudfunctionsFunction_EventTrigger_FailurePolicy {
    pub fn build(self) -> CloudfunctionsFunction_EventTrigger_FailurePolicy {
        CloudfunctionsFunction_EventTrigger_FailurePolicy { retry: self.retry }
    }
}

#[allow(dead_code)]
pub struct CloudfunctionsFunction_EventTrigger_FailurePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfunctionsFunction_EventTrigger_FailurePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfunctionsFunction_EventTrigger_FailurePolicyRef {
        CloudfunctionsFunction_EventTrigger_FailurePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfunctionsFunction_EventTrigger_FailurePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `retry` after provisioning.\nWhether the function should be retried on failure. Defaults to false."]
    pub fn retry(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfunctionsFunction_EventTriggerDynamic {
    failure_policy: std::option::Option<terrars::DynamicBlock<CloudfunctionsFunction_EventTrigger_FailurePolicy>>,
}

#[derive(Serialize)]
pub struct CloudfunctionsFunction_EventTrigger {
    event_type: terrars::PrimField<std::string::String>,
    resource: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_policy: std::option::Option<std::vec::Vec<CloudfunctionsFunction_EventTrigger_FailurePolicy>>,
    dynamic: CloudfunctionsFunction_EventTriggerDynamic,
}

impl CloudfunctionsFunction_EventTrigger {
    #[doc = "Set the field `failure_policy`.\n"]
    pub fn set_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfunctionsFunction_EventTrigger_FailurePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.failure_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.failure_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfunctionsFunction_EventTrigger {
    type O = terrars::BlockAssignable<CloudfunctionsFunction_EventTrigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctionsFunction_EventTrigger {
    #[doc =
        "The type of event to observe. For example: \"google.storage.object.finalize\". See the documentation on calling Cloud Functions for a full reference of accepted triggers."]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc =
        "The name or partial URI of the resource from which to observe events. For example, \"myBucket\" or \"projects/my-project/topics/my-topic\""]
    pub resource: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctionsFunction_EventTrigger {
    pub fn build(self) -> CloudfunctionsFunction_EventTrigger {
        CloudfunctionsFunction_EventTrigger {
            event_type: self.event_type,
            resource: self.resource,
            failure_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfunctionsFunction_EventTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfunctionsFunction_EventTriggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfunctionsFunction_EventTriggerRef {
        CloudfunctionsFunction_EventTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfunctionsFunction_EventTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `event_type` after provisioning.\nThe type of event to observe. For example: \"google.storage.object.finalize\". See the documentation on calling Cloud Functions for a full reference of accepted triggers."]
    pub fn event_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nThe name or partial URI of the resource from which to observe events. For example, \"myBucket\" or \"projects/my-project/topics/my-topic\""]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_policy` after provisioning.\n"]
    pub fn failure_policy(&self) -> terrars::ListRef<CloudfunctionsFunction_EventTrigger_FailurePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfunctionsFunction_OnDeployUpdatePolicy {}

impl CloudfunctionsFunction_OnDeployUpdatePolicy { }

impl terrars::ToListMappable for CloudfunctionsFunction_OnDeployUpdatePolicy {
    type O = terrars::BlockAssignable<CloudfunctionsFunction_OnDeployUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctionsFunction_OnDeployUpdatePolicy {}

impl BuildCloudfunctionsFunction_OnDeployUpdatePolicy {
    pub fn build(self) -> CloudfunctionsFunction_OnDeployUpdatePolicy {
        CloudfunctionsFunction_OnDeployUpdatePolicy {}
    }
}

#[allow(dead_code)]
pub struct CloudfunctionsFunction_OnDeployUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfunctionsFunction_OnDeployUpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfunctionsFunction_OnDeployUpdatePolicyRef {
        CloudfunctionsFunction_OnDeployUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfunctionsFunction_OnDeployUpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `runtime_version` after provisioning.\nThe runtime version which was used during latest function deployment."]
    pub fn runtime_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_version", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfunctionsFunction_SecretEnvironmentVariables {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    secret: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl CloudfunctionsFunction_SecretEnvironmentVariables {
    #[doc =
        "Set the field `project_id`.\nProject identifier (due to a known limitation, only project number is supported by this field) of the project that contains the secret. If not set, it will be populated with the function's project, assuming that the secret exists in the same project as of the function."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfunctionsFunction_SecretEnvironmentVariables {
    type O = terrars::BlockAssignable<CloudfunctionsFunction_SecretEnvironmentVariables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctionsFunction_SecretEnvironmentVariables {
    #[doc = "Name of the environment variable."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "ID of the secret in secret manager (not the full resource name)."]
    pub secret: terrars::PrimField<std::string::String>,
    #[doc =
        "Version of the secret (version number or the string \"latest\"). It is recommended to use a numeric version for secret environment variables as any updates to the secret value is not reflected until new clones start."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctionsFunction_SecretEnvironmentVariables {
    pub fn build(self) -> CloudfunctionsFunction_SecretEnvironmentVariables {
        CloudfunctionsFunction_SecretEnvironmentVariables {
            key: self.key,
            project_id: core::default::Default::default(),
            secret: self.secret,
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfunctionsFunction_SecretEnvironmentVariablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfunctionsFunction_SecretEnvironmentVariablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfunctionsFunction_SecretEnvironmentVariablesRef {
        CloudfunctionsFunction_SecretEnvironmentVariablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfunctionsFunction_SecretEnvironmentVariablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nName of the environment variable."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nProject identifier (due to a known limitation, only project number is supported by this field) of the project that contains the secret. If not set, it will be populated with the function's project, assuming that the secret exists in the same project as of the function."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nID of the secret in secret manager (not the full resource name)."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the secret (version number or the string \"latest\"). It is recommended to use a numeric version for secret environment variables as any updates to the secret value is not reflected until new clones start."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfunctionsFunction_SecretVolumes_Versions {
    path: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl CloudfunctionsFunction_SecretVolumes_Versions { }

impl terrars::ToListMappable for CloudfunctionsFunction_SecretVolumes_Versions {
    type O = terrars::BlockAssignable<CloudfunctionsFunction_SecretVolumes_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctionsFunction_SecretVolumes_Versions {
    #[doc =
        "Relative path of the file under the mount path where the secret value for this version will be fetched and made available. For example, setting the mount_path as \"/etc/secrets\" and path as \"/secret_foo\" would mount the secret value file at \"/etc/secrets/secret_foo\"."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc =
        "Version of the secret (version number or the string \"latest\"). It is preferable to use \"latest\" version with secret volumes as secret value changes are reflected immediately."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctionsFunction_SecretVolumes_Versions {
    pub fn build(self) -> CloudfunctionsFunction_SecretVolumes_Versions {
        CloudfunctionsFunction_SecretVolumes_Versions {
            path: self.path,
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfunctionsFunction_SecretVolumes_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfunctionsFunction_SecretVolumes_VersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfunctionsFunction_SecretVolumes_VersionsRef {
        CloudfunctionsFunction_SecretVolumes_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfunctionsFunction_SecretVolumes_VersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nRelative path of the file under the mount path where the secret value for this version will be fetched and made available. For example, setting the mount_path as \"/etc/secrets\" and path as \"/secret_foo\" would mount the secret value file at \"/etc/secrets/secret_foo\"."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the secret (version number or the string \"latest\"). It is preferable to use \"latest\" version with secret volumes as secret value changes are reflected immediately."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfunctionsFunction_SecretVolumesDynamic {
    versions: std::option::Option<terrars::DynamicBlock<CloudfunctionsFunction_SecretVolumes_Versions>>,
}

#[derive(Serialize)]
pub struct CloudfunctionsFunction_SecretVolumes {
    mount_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versions: std::option::Option<std::vec::Vec<CloudfunctionsFunction_SecretVolumes_Versions>>,
    dynamic: CloudfunctionsFunction_SecretVolumesDynamic,
}

impl CloudfunctionsFunction_SecretVolumes {
    #[doc =
        "Set the field `project_id`.\nProject identifier (due to a known limitation, only project number is supported by this field) of the project that contains the secret. If not set, it will be populated with the function's project, assuming that the secret exists in the same project as of the function."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `versions`.\n"]
    pub fn set_versions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfunctionsFunction_SecretVolumes_Versions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.versions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.versions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfunctionsFunction_SecretVolumes {
    type O = terrars::BlockAssignable<CloudfunctionsFunction_SecretVolumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctionsFunction_SecretVolumes {
    #[doc =
        "The path within the container to mount the secret volume. For example, setting the mount_path as \"/etc/secrets\" would mount the secret value files under the \"/etc/secrets\" directory. This directory will also be completely shadowed and unavailable to mount any other secrets. Recommended mount paths: \"/etc/secrets\" Restricted mount paths: \"/cloudsql\", \"/dev/log\", \"/pod\", \"/proc\", \"/var/log\"."]
    pub mount_path: terrars::PrimField<std::string::String>,
    #[doc = "ID of the secret in secret manager (not the full resource name)."]
    pub secret: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctionsFunction_SecretVolumes {
    pub fn build(self) -> CloudfunctionsFunction_SecretVolumes {
        CloudfunctionsFunction_SecretVolumes {
            mount_path: self.mount_path,
            project_id: core::default::Default::default(),
            secret: self.secret,
            versions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfunctionsFunction_SecretVolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfunctionsFunction_SecretVolumesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfunctionsFunction_SecretVolumesRef {
        CloudfunctionsFunction_SecretVolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfunctionsFunction_SecretVolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mount_path` after provisioning.\nThe path within the container to mount the secret volume. For example, setting the mount_path as \"/etc/secrets\" would mount the secret value files under the \"/etc/secrets\" directory. This directory will also be completely shadowed and unavailable to mount any other secrets. Recommended mount paths: \"/etc/secrets\" Restricted mount paths: \"/cloudsql\", \"/dev/log\", \"/pod\", \"/proc\", \"/var/log\"."]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nProject identifier (due to a known limitation, only project number is supported by this field) of the project that contains the secret. If not set, it will be populated with the function's project, assuming that the secret exists in the same project as of the function."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nID of the secret in secret manager (not the full resource name)."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> terrars::ListRef<CloudfunctionsFunction_SecretVolumes_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfunctionsFunction_SourceRepository {
    url: terrars::PrimField<std::string::String>,
}

impl CloudfunctionsFunction_SourceRepository { }

impl terrars::ToListMappable for CloudfunctionsFunction_SourceRepository {
    type O = terrars::BlockAssignable<CloudfunctionsFunction_SourceRepository>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctionsFunction_SourceRepository {
    #[doc = "The URL pointing to the hosted repository where the function is defined."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctionsFunction_SourceRepository {
    pub fn build(self) -> CloudfunctionsFunction_SourceRepository {
        CloudfunctionsFunction_SourceRepository { url: self.url }
    }
}

#[allow(dead_code)]
pub struct CloudfunctionsFunction_SourceRepositoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfunctionsFunction_SourceRepositoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfunctionsFunction_SourceRepositoryRef {
        CloudfunctionsFunction_SourceRepositoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfunctionsFunction_SourceRepositoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deployed_url` after provisioning.\nThe URL pointing to the hosted repository where the function was defined at the time of deployment."]
    pub fn deployed_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe URL pointing to the hosted repository where the function is defined."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfunctionsFunction_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudfunctionsFunction_Timeouts {
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

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfunctionsFunction_Timeouts {
    type O = terrars::BlockAssignable<CloudfunctionsFunction_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctionsFunction_Timeouts {}

impl BuildCloudfunctionsFunction_Timeouts {
    pub fn build(self) -> CloudfunctionsFunction_Timeouts {
        CloudfunctionsFunction_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfunctionsFunction_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfunctionsFunction_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudfunctionsFunction_TimeoutsRef {
        CloudfunctionsFunction_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfunctionsFunction_TimeoutsRef {
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

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
