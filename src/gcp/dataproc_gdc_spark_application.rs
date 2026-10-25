#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocGdcSparkApplicationData_ {
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
    application_environment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dependency_images: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    serviceinstance: terrars::PrimField<std::string::String>,
    spark_application_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pyspark_application_config: std::option::Option<std::vec::Vec<DataprocGdcSparkApplication_PysparkApplicationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_application_config: std::option::Option<std::vec::Vec<DataprocGdcSparkApplication_SparkApplicationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_r_application_config: std::option::Option<std::vec::Vec<DataprocGdcSparkApplication_SparkRApplicationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_sql_application_config: std
    ::option
    ::Option<std::vec::Vec<DataprocGdcSparkApplication_SparkSqlApplicationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocGdcSparkApplication_Timeouts>,
    dynamic: DataprocGdcSparkApplicationDynamic,
}

struct DataprocGdcSparkApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocGdcSparkApplicationData_>,
}

#[derive(Clone)]
pub struct DataprocGdcSparkApplication(std::rc::Rc<DataprocGdcSparkApplication_>);

impl DataprocGdcSparkApplication {
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
        "Set the field `annotations`.\nThe annotations to associate with this application. Annotations may be used to store client information, but are not used by the server. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `application_environment`.\nAn ApplicationEnvironment from which to inherit configuration properties."]
    pub fn set_application_environment(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().application_environment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dependency_images`.\nList of container image uris for additional file dependencies. Dependent files are sequentially copied from each image. If a file with the same name exists in 2 images then the file from later image is used."]
    pub fn set_dependency_images(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().dependency_images = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser-provided human-readable name to be used in user interfaces."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels to associate with this application. Labels may be used for filtering and billing tracking. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `namespace`.\nThe Kubernetes namespace in which to create the application. This namespace must already exist on the cluster."]
    pub fn set_namespace(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\napplication-specific properties."]
    pub fn set_properties(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nThe Dataproc version of this application."]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pyspark_application_config`.\n"]
    pub fn set_pyspark_application_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocGdcSparkApplication_PysparkApplicationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pyspark_application_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pyspark_application_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_application_config`.\n"]
    pub fn set_spark_application_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocGdcSparkApplication_SparkApplicationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_application_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_application_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_r_application_config`.\n"]
    pub fn set_spark_r_application_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocGdcSparkApplication_SparkRApplicationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_r_application_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_r_application_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_sql_application_config`.\n"]
    pub fn set_spark_sql_application_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocGdcSparkApplication_SparkSqlApplicationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_sql_application_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_sql_application_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocGdcSparkApplication_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nThe annotations to associate with this application. Annotations may be used to store client information, but are not used by the server. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_environment` after provisioning.\nAn ApplicationEnvironment from which to inherit configuration properties."]
    pub fn application_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_environment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dependency_images` after provisioning.\nList of container image uris for additional file dependencies. Dependent files are sequentially copied from each image. If a file with the same name exists in 2 images then the file from later image is used."]
    pub fn dependency_images(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dependency_images", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-provided human-readable name to be used in user interfaces."]
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this application. Labels may be used for filtering and billing tracking. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the spark application."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitoring_endpoint` after provisioning.\nURL for a monitoring UI for this application (for eventual Spark PHS/UI support) Out of scope for private GA"]
    pub fn monitoring_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the application. Format: projects/{project}/locations/{location}/serviceInstances/{service_instance}/sparkApplications/{application}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nThe Kubernetes namespace in which to create the application. This namespace must already exist on the cluster."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `output_uri` after provisioning.\nAn HCFS URI pointing to the location of stdout and stdout of the application Mainly useful for Pantheon and gcloud Not in scope for private GA"]
    pub fn output_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\napplication-specific properties."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nWhether the application is currently reconciling. True if the current state of the resource does not match the intended state, and the system is working to reconcile them, whether or not the change was user initiated."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `serviceinstance` after provisioning.\nThe id of the service instance to which this spark application belongs."]
    pub fn serviceinstance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serviceinstance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `spark_application_id` after provisioning.\nThe id of the application"]
    pub fn spark_application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spark_application_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state.\nPossible values:\n* 'STATE_UNSPECIFIED'\n* 'PENDING'\n* 'RUNNING'\n* 'CANCELLING'\n* 'CANCELLED'\n* 'SUCCEEDED'\n* 'FAILED'"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_message` after provisioning.\nA message explaining the current state."]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for this application, formatted as UUID4."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was most recently updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Dataproc version of this application."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pyspark_application_config` after provisioning.\n"]
    pub fn pyspark_application_config(
        &self,
    ) -> terrars::ListRef<DataprocGdcSparkApplication_PysparkApplicationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pyspark_application_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_application_config` after provisioning.\n"]
    pub fn spark_application_config(&self) -> terrars::ListRef<DataprocGdcSparkApplication_SparkApplicationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_application_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_r_application_config` after provisioning.\n"]
    pub fn spark_r_application_config(&self) -> terrars::ListRef<DataprocGdcSparkApplication_SparkRApplicationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_r_application_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_sql_application_config` after provisioning.\n"]
    pub fn spark_sql_application_config(
        &self,
    ) -> terrars::ListRef<DataprocGdcSparkApplication_SparkSqlApplicationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_sql_application_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocGdcSparkApplication_TimeoutsRef {
        DataprocGdcSparkApplication_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataprocGdcSparkApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataprocGdcSparkApplication { }

impl terrars::ToListMappable for DataprocGdcSparkApplication {
    type O = terrars::ListRef<DataprocGdcSparkApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataprocGdcSparkApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataproc_gdc_spark_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataprocGdcSparkApplication {
    pub tf_id: std::string::String,
    #[doc = "The location of the spark application."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The id of the service instance to which this spark application belongs."]
    pub serviceinstance: terrars::PrimField<std::string::String>,
    #[doc = "The id of the application"]
    pub spark_application_id: terrars::PrimField<std::string::String>,
}

impl BuildDataprocGdcSparkApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> DataprocGdcSparkApplication {
        let out = DataprocGdcSparkApplication(std::rc::Rc::new(DataprocGdcSparkApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataprocGdcSparkApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                application_environment: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                dependency_images: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                namespace: core::default::Default::default(),
                project: core::default::Default::default(),
                properties: core::default::Default::default(),
                serviceinstance: self.serviceinstance,
                spark_application_id: self.spark_application_id,
                version: core::default::Default::default(),
                pyspark_application_config: core::default::Default::default(),
                spark_application_config: core::default::Default::default(),
                spark_r_application_config: core::default::Default::default(),
                spark_sql_application_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataprocGdcSparkApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocGdcSparkApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataprocGdcSparkApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nThe annotations to associate with this application. Annotations may be used to store client information, but are not used by the server. \n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_environment` after provisioning.\nAn ApplicationEnvironment from which to inherit configuration properties."]
    pub fn application_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_environment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dependency_images` after provisioning.\nList of container image uris for additional file dependencies. Dependent files are sequentially copied from each image. If a file with the same name exists in 2 images then the file from later image is used."]
    pub fn dependency_images(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dependency_images", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-provided human-readable name to be used in user interfaces."]
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this application. Labels may be used for filtering and billing tracking. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the spark application."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `monitoring_endpoint` after provisioning.\nURL for a monitoring UI for this application (for eventual Spark PHS/UI support) Out of scope for private GA"]
    pub fn monitoring_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the application. Format: projects/{project}/locations/{location}/serviceInstances/{service_instance}/sparkApplications/{application}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nThe Kubernetes namespace in which to create the application. This namespace must already exist on the cluster."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `output_uri` after provisioning.\nAn HCFS URI pointing to the location of stdout and stdout of the application Mainly useful for Pantheon and gcloud Not in scope for private GA"]
    pub fn output_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\napplication-specific properties."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nWhether the application is currently reconciling. True if the current state of the resource does not match the intended state, and the system is working to reconcile them, whether or not the change was user initiated."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `serviceinstance` after provisioning.\nThe id of the service instance to which this spark application belongs."]
    pub fn serviceinstance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serviceinstance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `spark_application_id` after provisioning.\nThe id of the application"]
    pub fn spark_application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spark_application_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state.\nPossible values:\n* 'STATE_UNSPECIFIED'\n* 'PENDING'\n* 'RUNNING'\n* 'CANCELLING'\n* 'CANCELLED'\n* 'SUCCEEDED'\n* 'FAILED'"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_message` after provisioning.\nA message explaining the current state."]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem generated unique identifier for this application, formatted as UUID4."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was most recently updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Dataproc version of this application."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pyspark_application_config` after provisioning.\n"]
    pub fn pyspark_application_config(
        &self,
    ) -> terrars::ListRef<DataprocGdcSparkApplication_PysparkApplicationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pyspark_application_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_application_config` after provisioning.\n"]
    pub fn spark_application_config(&self) -> terrars::ListRef<DataprocGdcSparkApplication_SparkApplicationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_application_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_r_application_config` after provisioning.\n"]
    pub fn spark_r_application_config(&self) -> terrars::ListRef<DataprocGdcSparkApplication_SparkRApplicationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_r_application_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_sql_application_config` after provisioning.\n"]
    pub fn spark_sql_application_config(
        &self,
    ) -> terrars::ListRef<DataprocGdcSparkApplication_SparkSqlApplicationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_sql_application_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocGdcSparkApplication_TimeoutsRef {
        DataprocGdcSparkApplication_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DataprocGdcSparkApplicationDynamic {
    pyspark_application_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocGdcSparkApplication_PysparkApplicationConfig>>,
    spark_application_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocGdcSparkApplication_SparkApplicationConfig>>,
    spark_r_application_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocGdcSparkApplication_SparkRApplicationConfig>>,
    spark_sql_application_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocGdcSparkApplication_SparkSqlApplicationConfig>>,
}

#[derive(Serialize)]
pub struct DataprocGdcSparkApplication_PysparkApplicationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    main_python_file_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataprocGdcSparkApplication_PysparkApplicationConfig {
    #[doc =
        "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nThe arguments to pass to the driver.  Do not include arguments, such as '--conf', that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nHCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `python_file_uris`.\nHCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip."]
    pub fn set_python_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.python_file_uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocGdcSparkApplication_PysparkApplicationConfig {
    type O = terrars::BlockAssignable<DataprocGdcSparkApplication_PysparkApplicationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocGdcSparkApplication_PysparkApplicationConfig {
    #[doc = "The HCFS URI of the main Python file to use as the driver. Must be a .py file."]
    pub main_python_file_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocGdcSparkApplication_PysparkApplicationConfig {
    pub fn build(self) -> DataprocGdcSparkApplication_PysparkApplicationConfig {
        DataprocGdcSparkApplication_PysparkApplicationConfig {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_python_file_uri: self.main_python_file_uri,
            python_file_uris: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocGdcSparkApplication_PysparkApplicationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocGdcSparkApplication_PysparkApplicationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocGdcSparkApplication_PysparkApplicationConfigRef {
        DataprocGdcSparkApplication_PysparkApplicationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocGdcSparkApplication_PysparkApplicationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver.  Do not include arguments, such as '--conf', that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_python_file_uri` after provisioning.\nThe HCFS URI of the main Python file to use as the driver. Must be a .py file."]
    pub fn main_python_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_python_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_file_uris` after provisioning.\nHCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip."]
    pub fn python_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_file_uris", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocGdcSparkApplication_SparkApplicationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_jar_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocGdcSparkApplication_SparkApplicationConfig {
    #[doc =
        "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: '.jar', '.tar', '.tar.gz', '.tgz', and '.zip'."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nThe arguments to pass to the driver. Do not include arguments that can be set as application properties, such as '--conf', since a collision can occur that causes an incorrect application submission."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_uris`.\nHCFS URIs of files to be placed in the working directory of each executor."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the classpath of the Spark driver and tasks."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_class`.\nThe name of the driver main class. The jar file that contains the class must be in the classpath or specified in 'jar_file_uris'."]
    pub fn set_main_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `main_jar_file_uri`.\nThe HCFS URI of the jar file that contains the main class."]
    pub fn set_main_jar_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_jar_file_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocGdcSparkApplication_SparkApplicationConfig {
    type O = terrars::BlockAssignable<DataprocGdcSparkApplication_SparkApplicationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocGdcSparkApplication_SparkApplicationConfig {}

impl BuildDataprocGdcSparkApplication_SparkApplicationConfig {
    pub fn build(self) -> DataprocGdcSparkApplication_SparkApplicationConfig {
        DataprocGdcSparkApplication_SparkApplicationConfig {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocGdcSparkApplication_SparkApplicationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocGdcSparkApplication_SparkApplicationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocGdcSparkApplication_SparkApplicationConfigRef {
        DataprocGdcSparkApplication_SparkApplicationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocGdcSparkApplication_SparkApplicationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: '.jar', '.tar', '.tar.gz', '.tgz', and '.zip'."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver. Do not include arguments that can be set as application properties, such as '--conf', since a collision can occur that causes an incorrect application submission."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be placed in the working directory of each executor."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the classpath of the Spark driver and tasks."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_class` after provisioning.\nThe name of the driver main class. The jar file that contains the class must be in the classpath or specified in 'jar_file_uris'."]
    pub fn main_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_jar_file_uri` after provisioning.\nThe HCFS URI of the jar file that contains the main class."]
    pub fn main_jar_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_jar_file_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocGdcSparkApplication_SparkRApplicationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    main_r_file_uri: terrars::PrimField<std::string::String>,
}

impl DataprocGdcSparkApplication_SparkRApplicationConfig {
    #[doc =
        "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nThe arguments to pass to the driver.  Do not include arguments, such as '--conf', that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nHCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocGdcSparkApplication_SparkRApplicationConfig {
    type O = terrars::BlockAssignable<DataprocGdcSparkApplication_SparkRApplicationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocGdcSparkApplication_SparkRApplicationConfig {
    #[doc = "The HCFS URI of the main R file to use as the driver. Must be a .R file."]
    pub main_r_file_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocGdcSparkApplication_SparkRApplicationConfig {
    pub fn build(self) -> DataprocGdcSparkApplication_SparkRApplicationConfig {
        DataprocGdcSparkApplication_SparkRApplicationConfig {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            main_r_file_uri: self.main_r_file_uri,
        }
    }
}

#[allow(dead_code)]
pub struct DataprocGdcSparkApplication_SparkRApplicationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocGdcSparkApplication_SparkRApplicationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocGdcSparkApplication_SparkRApplicationConfigRef {
        DataprocGdcSparkApplication_SparkRApplicationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocGdcSparkApplication_SparkRApplicationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver.  Do not include arguments, such as '--conf', that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_r_file_uri` after provisioning.\nThe HCFS URI of the main R file to use as the driver. Must be a .R file."]
    pub fn main_r_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_r_file_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList {
    queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList { }

impl terrars::ToListMappable for DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList {
    type O = terrars::BlockAssignable<DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList {
    #[doc = "The queries to run."]
    pub queries: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList {
    pub fn build(self) -> DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList {
        DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList { queries: self.queries }
    }
}

#[allow(dead_code)]
pub struct DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryListRef {
        DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `queries` after provisioning.\nThe queries to run."]
    pub fn queries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.queries", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocGdcSparkApplication_SparkSqlApplicationConfigDynamic {
    query_list: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList>>,
}

#[derive(Serialize)]
pub struct DataprocGdcSparkApplication_SparkSqlApplicationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<std::vec::Vec<DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList>>,
    dynamic: DataprocGdcSparkApplication_SparkSqlApplicationConfigDynamic,
}

impl DataprocGdcSparkApplication_SparkSqlApplicationConfig {
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains SQL queries."]
    pub fn set_query_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `script_variables`.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET 'name=\"value\";')."]
    pub fn set_script_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.script_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_list`.\n"]
    pub fn set_query_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryList>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocGdcSparkApplication_SparkSqlApplicationConfig {
    type O = terrars::BlockAssignable<DataprocGdcSparkApplication_SparkSqlApplicationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocGdcSparkApplication_SparkSqlApplicationConfig {}

impl BuildDataprocGdcSparkApplication_SparkSqlApplicationConfig {
    pub fn build(self) -> DataprocGdcSparkApplication_SparkSqlApplicationConfig {
        DataprocGdcSparkApplication_SparkSqlApplicationConfig {
            jar_file_uris: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            script_variables: core::default::Default::default(),
            query_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocGdcSparkApplication_SparkSqlApplicationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocGdcSparkApplication_SparkSqlApplicationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocGdcSparkApplication_SparkSqlApplicationConfigRef {
        DataprocGdcSparkApplication_SparkSqlApplicationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocGdcSparkApplication_SparkSqlApplicationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains SQL queries."]
    pub fn query_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script_variables` after provisioning.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET 'name=\"value\";')."]
    pub fn script_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.script_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `query_list` after provisioning.\n"]
    pub fn query_list(&self) -> terrars::ListRef<DataprocGdcSparkApplication_SparkSqlApplicationConfig_QueryListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocGdcSparkApplication_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocGdcSparkApplication_Timeouts {
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

impl terrars::ToListMappable for DataprocGdcSparkApplication_Timeouts {
    type O = terrars::BlockAssignable<DataprocGdcSparkApplication_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocGdcSparkApplication_Timeouts {}

impl BuildDataprocGdcSparkApplication_Timeouts {
    pub fn build(self) -> DataprocGdcSparkApplication_Timeouts {
        DataprocGdcSparkApplication_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocGdcSparkApplication_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocGdcSparkApplication_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocGdcSparkApplication_TimeoutsRef {
        DataprocGdcSparkApplication_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocGdcSparkApplication_TimeoutsRef {
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
