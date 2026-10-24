use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudfunctionsFunctionData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudfunctionsFunction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudfunctionsFunctionData>,
}

#[derive(Clone)]
pub struct DataCloudfunctionsFunction(std::rc::Rc<DataCloudfunctionsFunction_>);

impl DataCloudfunctionsFunction {
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
        "Get a reference to the value of field `build_worker_pool` after provisioning.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn build_worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_worker_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the function."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `docker_registry` after provisioning.\nDocker Registry to use for storing the function's Docker images. Allowed values are CONTAINER_REGISTRY (default) and ARTIFACT_REGISTRY."]
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
        "Get a reference to the value of field `event_trigger` after provisioning.\nA source that fires events in response to a condition in another service. Cannot be used with trigger_http."]
    pub fn event_trigger(&self) -> terrars::ListRef<DataCloudfunctionsFunctionEventTriggerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_trigger", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the function. Label keys must follow the requirements at https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
        "Get a reference to the value of field `runtime` after provisioning.\nThe runtime in which the function is going to run. Eg. \"nodejs12\", \"nodejs14\", \"python37\", \"go111\"."]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_environment_variables` after provisioning.\nSecret environment variables configuration"]
    pub fn secret_environment_variables(
        &self,
    ) -> terrars::ListRef<DataCloudfunctionsFunctionSecretEnvironmentVariablesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_volumes` after provisioning.\nSecret volumes configuration."]
    pub fn secret_volumes(&self) -> terrars::ListRef<DataCloudfunctionsFunctionSecretVolumesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_volumes", self.extract_ref()))
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
        "Get a reference to the value of field `source_repository` after provisioning.\nRepresents parameters related to source repository where a function is hosted. Cannot be set alongside source_archive_bucket or source_archive_object."]
    pub fn source_repository(&self) -> terrars::ListRef<DataCloudfunctionsFunctionSourceRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_repository", self.extract_ref()))
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
}

impl terrars::Referable for DataCloudfunctionsFunction {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudfunctionsFunction { }

impl terrars::ToListMappable for DataCloudfunctionsFunction {
    type O = terrars::ListRef<DataCloudfunctionsFunctionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudfunctionsFunction_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloudfunctions_function".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfunctionsFunction {
    pub tf_id: std::string::String,
    #[doc = "A user-defined name of the function. Function names must be unique globally."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudfunctionsFunction {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudfunctionsFunction {
        let out = DataCloudfunctionsFunction(std::rc::Rc::new(DataCloudfunctionsFunction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudfunctionsFunctionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfunctionsFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctionsFunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudfunctionsFunctionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `build_worker_pool` after provisioning.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn build_worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_worker_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the function."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `docker_registry` after provisioning.\nDocker Registry to use for storing the function's Docker images. Allowed values are CONTAINER_REGISTRY (default) and ARTIFACT_REGISTRY."]
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
        "Get a reference to the value of field `event_trigger` after provisioning.\nA source that fires events in response to a condition in another service. Cannot be used with trigger_http."]
    pub fn event_trigger(&self) -> terrars::ListRef<DataCloudfunctionsFunctionEventTriggerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_trigger", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to the function. Label keys must follow the requirements at https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
        "Get a reference to the value of field `runtime` after provisioning.\nThe runtime in which the function is going to run. Eg. \"nodejs12\", \"nodejs14\", \"python37\", \"go111\"."]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_environment_variables` after provisioning.\nSecret environment variables configuration"]
    pub fn secret_environment_variables(
        &self,
    ) -> terrars::ListRef<DataCloudfunctionsFunctionSecretEnvironmentVariablesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_environment_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_volumes` after provisioning.\nSecret volumes configuration."]
    pub fn secret_volumes(&self) -> terrars::ListRef<DataCloudfunctionsFunctionSecretVolumesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_volumes", self.extract_ref()))
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
        "Get a reference to the value of field `source_repository` after provisioning.\nRepresents parameters related to source repository where a function is hosted. Cannot be set alongside source_archive_bucket or source_archive_object."]
    pub fn source_repository(&self) -> terrars::ListRef<DataCloudfunctionsFunctionSourceRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_repository", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataCloudfunctionsFunctionEventTriggerElFailurePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudfunctionsFunctionEventTriggerElFailurePolicyEl {
    #[doc = "Set the field `retry`.\n"]
    pub fn set_retry(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.retry = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctionsFunctionEventTriggerElFailurePolicyEl {
    type O = terrars::BlockAssignable<DataCloudfunctionsFunctionEventTriggerElFailurePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctionsFunctionEventTriggerElFailurePolicyEl {}

impl BuildDataCloudfunctionsFunctionEventTriggerElFailurePolicyEl {
    pub fn build(self) -> DataCloudfunctionsFunctionEventTriggerElFailurePolicyEl {
        DataCloudfunctionsFunctionEventTriggerElFailurePolicyEl { retry: core::default::Default::default() }
    }
}

pub struct DataCloudfunctionsFunctionEventTriggerElFailurePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctionsFunctionEventTriggerElFailurePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctionsFunctionEventTriggerElFailurePolicyElRef {
        DataCloudfunctionsFunctionEventTriggerElFailurePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctionsFunctionEventTriggerElFailurePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retry` after provisioning.\n"]
    pub fn retry(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctionsFunctionEventTriggerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_policy: std::option::Option<terrars::ListField<DataCloudfunctionsFunctionEventTriggerElFailurePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctionsFunctionEventTriggerEl {
    #[doc = "Set the field `event_type`.\n"]
    pub fn set_event_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failure_policy`.\n"]
    pub fn set_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfunctionsFunctionEventTriggerElFailurePolicyEl>>,
    ) -> Self {
        self.failure_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctionsFunctionEventTriggerEl {
    type O = terrars::BlockAssignable<DataCloudfunctionsFunctionEventTriggerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctionsFunctionEventTriggerEl {}

impl BuildDataCloudfunctionsFunctionEventTriggerEl {
    pub fn build(self) -> DataCloudfunctionsFunctionEventTriggerEl {
        DataCloudfunctionsFunctionEventTriggerEl {
            event_type: core::default::Default::default(),
            failure_policy: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfunctionsFunctionEventTriggerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctionsFunctionEventTriggerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudfunctionsFunctionEventTriggerElRef {
        DataCloudfunctionsFunctionEventTriggerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctionsFunctionEventTriggerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc = "Get a reference to the value of field `failure_policy` after provisioning.\n"]
    pub fn failure_policy(&self) -> terrars::ListRef<DataCloudfunctionsFunctionEventTriggerElFailurePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctionsFunctionSecretEnvironmentVariablesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctionsFunctionSecretEnvironmentVariablesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctionsFunctionSecretEnvironmentVariablesEl {
    type O = terrars::BlockAssignable<DataCloudfunctionsFunctionSecretEnvironmentVariablesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctionsFunctionSecretEnvironmentVariablesEl {}

impl BuildDataCloudfunctionsFunctionSecretEnvironmentVariablesEl {
    pub fn build(self) -> DataCloudfunctionsFunctionSecretEnvironmentVariablesEl {
        DataCloudfunctionsFunctionSecretEnvironmentVariablesEl {
            key: core::default::Default::default(),
            project_id: core::default::Default::default(),
            secret: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfunctionsFunctionSecretEnvironmentVariablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctionsFunctionSecretEnvironmentVariablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctionsFunctionSecretEnvironmentVariablesElRef {
        DataCloudfunctionsFunctionSecretEnvironmentVariablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctionsFunctionSecretEnvironmentVariablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctionsFunctionSecretVolumesElVersionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctionsFunctionSecretVolumesElVersionsEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctionsFunctionSecretVolumesElVersionsEl {
    type O = terrars::BlockAssignable<DataCloudfunctionsFunctionSecretVolumesElVersionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctionsFunctionSecretVolumesElVersionsEl {}

impl BuildDataCloudfunctionsFunctionSecretVolumesElVersionsEl {
    pub fn build(self) -> DataCloudfunctionsFunctionSecretVolumesElVersionsEl {
        DataCloudfunctionsFunctionSecretVolumesElVersionsEl {
            path: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfunctionsFunctionSecretVolumesElVersionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctionsFunctionSecretVolumesElVersionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctionsFunctionSecretVolumesElVersionsElRef {
        DataCloudfunctionsFunctionSecretVolumesElVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctionsFunctionSecretVolumesElVersionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctionsFunctionSecretVolumesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versions: std::option::Option<terrars::ListField<DataCloudfunctionsFunctionSecretVolumesElVersionsEl>>,
}

impl DataCloudfunctionsFunctionSecretVolumesEl {
    #[doc = "Set the field `mount_path`.\n"]
    pub fn set_mount_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mount_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `versions`.\n"]
    pub fn set_versions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudfunctionsFunctionSecretVolumesElVersionsEl>>,
    ) -> Self {
        self.versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctionsFunctionSecretVolumesEl {
    type O = terrars::BlockAssignable<DataCloudfunctionsFunctionSecretVolumesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctionsFunctionSecretVolumesEl {}

impl BuildDataCloudfunctionsFunctionSecretVolumesEl {
    pub fn build(self) -> DataCloudfunctionsFunctionSecretVolumesEl {
        DataCloudfunctionsFunctionSecretVolumesEl {
            mount_path: core::default::Default::default(),
            project_id: core::default::Default::default(),
            secret: core::default::Default::default(),
            versions: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfunctionsFunctionSecretVolumesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctionsFunctionSecretVolumesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudfunctionsFunctionSecretVolumesElRef {
        DataCloudfunctionsFunctionSecretVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctionsFunctionSecretVolumesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mount_path` after provisioning.\n"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> terrars::ListRef<DataCloudfunctionsFunctionSecretVolumesElVersionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfunctionsFunctionSourceRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployed_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudfunctionsFunctionSourceRepositoryEl {
    #[doc = "Set the field `deployed_url`.\n"]
    pub fn set_deployed_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployed_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudfunctionsFunctionSourceRepositoryEl {
    type O = terrars::BlockAssignable<DataCloudfunctionsFunctionSourceRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfunctionsFunctionSourceRepositoryEl {}

impl BuildDataCloudfunctionsFunctionSourceRepositoryEl {
    pub fn build(self) -> DataCloudfunctionsFunctionSourceRepositoryEl {
        DataCloudfunctionsFunctionSourceRepositoryEl {
            deployed_url: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfunctionsFunctionSourceRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudfunctionsFunctionSourceRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudfunctionsFunctionSourceRepositoryElRef {
        DataCloudfunctionsFunctionSourceRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfunctionsFunctionSourceRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployed_url` after provisioning.\n"]
    pub fn deployed_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_url", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
