#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocBatchData_ {
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
    batch_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_config: std::option::Option<std::vec::Vec<DataprocBatch_EnvironmentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pyspark_batch: std::option::Option<std::vec::Vec<DataprocBatch_PysparkBatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_config: std::option::Option<std::vec::Vec<DataprocBatch_RuntimeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_batch: std::option::Option<std::vec::Vec<DataprocBatch_SparkBatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_r_batch: std::option::Option<std::vec::Vec<DataprocBatch_SparkRBatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_sql_batch: std::option::Option<std::vec::Vec<DataprocBatch_SparkSqlBatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocBatch_Timeouts>,
    dynamic: DataprocBatchDynamic,
}

struct DataprocBatch_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocBatchData_>,
}

#[derive(Clone)]
pub struct DataprocBatch(std::rc::Rc<DataprocBatch_>);

impl DataprocBatch {
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
        "Set the field `batch_id`.\nThe ID to use for the batch, which will become the final component of the batch's resource name.\nThis value must be 4-63 characters. Valid characters are /[a-z][0-9]-/."]
    pub fn set_batch_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().batch_id = std::option::Option::Some(v.into());
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
        "Set the field `labels`.\nThe labels to associate with this batch.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location in which the batch will be created in."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_config`.\n"]
    pub fn set_environment_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocBatch_EnvironmentConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().environment_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.environment_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pyspark_batch`.\n"]
    pub fn set_pyspark_batch(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocBatch_PysparkBatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pyspark_batch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pyspark_batch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `runtime_config`.\n"]
    pub fn set_runtime_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocBatch_RuntimeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_batch`.\n"]
    pub fn set_spark_batch(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocBatch_SparkBatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_batch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_batch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_r_batch`.\n"]
    pub fn set_spark_r_batch(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocBatch_SparkRBatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_r_batch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_r_batch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_sql_batch`.\n"]
    pub fn set_spark_sql_batch(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocBatch_SparkSqlBatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_sql_batch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_sql_batch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocBatch_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `batch_id` after provisioning.\nThe ID to use for the batch, which will become the final component of the batch's resource name.\nThis value must be 4-63 characters. Valid characters are /[a-z][0-9]-/."]
    pub fn batch_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the batch was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nThe email address of the user who created the batch."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this batch.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location in which the batch will be created in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the batch."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe resource name of the operation associated with this batch."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_info` after provisioning.\nRuntime information about batch execution."]
    pub fn runtime_info(&self) -> terrars::ListRef<DataprocBatch_RuntimeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the batch. For possible values, see the [API documentation](https://cloud.google.com/dataproc-serverless/docs/reference/rest/v1/projects.locations.batches#State)."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_history` after provisioning.\nHistorical state information for the batch."]
    pub fn state_history(&self) -> terrars::ListRef<DataprocBatch_StateHistoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_history", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_message` after provisioning.\nBatch state details, such as a failure description if the state is FAILED."]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_time` after provisioning.\nBatch state details, such as a failure description if the state is FAILED."]
    pub fn state_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uuid` after provisioning.\nA batch UUID (Unique Universal Identifier). The service generates this value when it creates the batch."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_config` after provisioning.\n"]
    pub fn environment_config(&self) -> terrars::ListRef<DataprocBatch_EnvironmentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pyspark_batch` after provisioning.\n"]
    pub fn pyspark_batch(&self) -> terrars::ListRef<DataprocBatch_PysparkBatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pyspark_batch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(&self) -> terrars::ListRef<DataprocBatch_RuntimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_batch` after provisioning.\n"]
    pub fn spark_batch(&self) -> terrars::ListRef<DataprocBatch_SparkBatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_batch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_r_batch` after provisioning.\n"]
    pub fn spark_r_batch(&self) -> terrars::ListRef<DataprocBatch_SparkRBatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_r_batch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_sql_batch` after provisioning.\n"]
    pub fn spark_sql_batch(&self) -> terrars::ListRef<DataprocBatch_SparkSqlBatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_sql_batch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocBatch_TimeoutsRef {
        DataprocBatch_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataprocBatch {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataprocBatch { }

impl terrars::ToListMappable for DataprocBatch {
    type O = terrars::ListRef<DataprocBatchRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataprocBatch_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataproc_batch".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataprocBatch {
    pub tf_id: std::string::String,
}

impl BuildDataprocBatch {
    pub fn build(self, stack: &mut terrars::Stack) -> DataprocBatch {
        let out = DataprocBatch(std::rc::Rc::new(DataprocBatch_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataprocBatchData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                batch_id: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                environment_config: core::default::Default::default(),
                pyspark_batch: core::default::Default::default(),
                runtime_config: core::default::Default::default(),
                spark_batch: core::default::Default::default(),
                spark_r_batch: core::default::Default::default(),
                spark_sql_batch: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataprocBatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataprocBatchRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `batch_id` after provisioning.\nThe ID to use for the batch, which will become the final component of the batch's resource name.\nThis value must be 4-63 characters. Valid characters are /[a-z][0-9]-/."]
    pub fn batch_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the batch was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nThe email address of the user who created the batch."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this batch.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location in which the batch will be created in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the batch."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nThe resource name of the operation associated with this batch."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_info` after provisioning.\nRuntime information about batch execution."]
    pub fn runtime_info(&self) -> terrars::ListRef<DataprocBatch_RuntimeInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the batch. For possible values, see the [API documentation](https://cloud.google.com/dataproc-serverless/docs/reference/rest/v1/projects.locations.batches#State)."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_history` after provisioning.\nHistorical state information for the batch."]
    pub fn state_history(&self) -> terrars::ListRef<DataprocBatch_StateHistoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state_history", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_message` after provisioning.\nBatch state details, such as a failure description if the state is FAILED."]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_time` after provisioning.\nBatch state details, such as a failure description if the state is FAILED."]
    pub fn state_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uuid` after provisioning.\nA batch UUID (Unique Universal Identifier). The service generates this value when it creates the batch."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_config` after provisioning.\n"]
    pub fn environment_config(&self) -> terrars::ListRef<DataprocBatch_EnvironmentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pyspark_batch` after provisioning.\n"]
    pub fn pyspark_batch(&self) -> terrars::ListRef<DataprocBatch_PysparkBatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pyspark_batch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(&self) -> terrars::ListRef<DataprocBatch_RuntimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_batch` after provisioning.\n"]
    pub fn spark_batch(&self) -> terrars::ListRef<DataprocBatch_SparkBatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_batch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_r_batch` after provisioning.\n"]
    pub fn spark_r_batch(&self) -> terrars::ListRef<DataprocBatch_SparkRBatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_r_batch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_sql_batch` after provisioning.\n"]
    pub fn spark_sql_batch(&self) -> terrars::ListRef<DataprocBatch_SparkSqlBatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_sql_batch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocBatch_TimeoutsRef {
        DataprocBatch_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataprocBatchDynamic {
    environment_config: std::option::Option<terrars::DynamicBlock<DataprocBatch_EnvironmentConfig>>,
    pyspark_batch: std::option::Option<terrars::DynamicBlock<DataprocBatch_PysparkBatch>>,
    runtime_config: std::option::Option<terrars::DynamicBlock<DataprocBatch_RuntimeConfig>>,
    spark_batch: std::option::Option<terrars::DynamicBlock<DataprocBatch_SparkBatch>>,
    spark_r_batch: std::option::Option<terrars::DynamicBlock<DataprocBatch_SparkRBatch>>,
    spark_sql_batch: std::option::Option<terrars::DynamicBlock<DataprocBatch_SparkSqlBatch>>,
}

#[derive(Serialize)]
pub struct DataprocBatch_RuntimeInfo_ApproximateUsage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    milli_accelerator_seconds: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    milli_dcu_seconds: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shuffle_storage_gb_seconds: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocBatch_RuntimeInfo_ApproximateUsage {
    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `milli_accelerator_seconds`.\n"]
    pub fn set_milli_accelerator_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.milli_accelerator_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `milli_dcu_seconds`.\n"]
    pub fn set_milli_dcu_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.milli_dcu_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shuffle_storage_gb_seconds`.\n"]
    pub fn set_shuffle_storage_gb_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.shuffle_storage_gb_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_RuntimeInfo_ApproximateUsage {
    type O = terrars::BlockAssignable<DataprocBatch_RuntimeInfo_ApproximateUsage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_RuntimeInfo_ApproximateUsage {}

impl BuildDataprocBatch_RuntimeInfo_ApproximateUsage {
    pub fn build(self) -> DataprocBatch_RuntimeInfo_ApproximateUsage {
        DataprocBatch_RuntimeInfo_ApproximateUsage {
            accelerator_type: core::default::Default::default(),
            milli_accelerator_seconds: core::default::Default::default(),
            milli_dcu_seconds: core::default::Default::default(),
            shuffle_storage_gb_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_RuntimeInfo_ApproximateUsageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_RuntimeInfo_ApproximateUsageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_RuntimeInfo_ApproximateUsageRef {
        DataprocBatch_RuntimeInfo_ApproximateUsageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_RuntimeInfo_ApproximateUsageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc = "Get a reference to the value of field `milli_accelerator_seconds` after provisioning.\n"]
    pub fn milli_accelerator_seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.milli_accelerator_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `milli_dcu_seconds` after provisioning.\n"]
    pub fn milli_dcu_seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.milli_dcu_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `shuffle_storage_gb_seconds` after provisioning.\n"]
    pub fn shuffle_storage_gb_seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shuffle_storage_gb_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocBatch_RuntimeInfo_CurrentUsage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    milli_accelerator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    milli_dcu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    milli_dcu_premium: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shuffle_storage_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shuffle_storage_gb_premium: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocBatch_RuntimeInfo_CurrentUsage {
    #[doc = "Set the field `accelerator_type`.\n"]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `milli_accelerator`.\n"]
    pub fn set_milli_accelerator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.milli_accelerator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `milli_dcu`.\n"]
    pub fn set_milli_dcu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.milli_dcu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `milli_dcu_premium`.\n"]
    pub fn set_milli_dcu_premium(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.milli_dcu_premium = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shuffle_storage_gb`.\n"]
    pub fn set_shuffle_storage_gb(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.shuffle_storage_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shuffle_storage_gb_premium`.\n"]
    pub fn set_shuffle_storage_gb_premium(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.shuffle_storage_gb_premium = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_time`.\n"]
    pub fn set_snapshot_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_RuntimeInfo_CurrentUsage {
    type O = terrars::BlockAssignable<DataprocBatch_RuntimeInfo_CurrentUsage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_RuntimeInfo_CurrentUsage {}

impl BuildDataprocBatch_RuntimeInfo_CurrentUsage {
    pub fn build(self) -> DataprocBatch_RuntimeInfo_CurrentUsage {
        DataprocBatch_RuntimeInfo_CurrentUsage {
            accelerator_type: core::default::Default::default(),
            milli_accelerator: core::default::Default::default(),
            milli_dcu: core::default::Default::default(),
            milli_dcu_premium: core::default::Default::default(),
            shuffle_storage_gb: core::default::Default::default(),
            shuffle_storage_gb_premium: core::default::Default::default(),
            snapshot_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_RuntimeInfo_CurrentUsageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_RuntimeInfo_CurrentUsageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_RuntimeInfo_CurrentUsageRef {
        DataprocBatch_RuntimeInfo_CurrentUsageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_RuntimeInfo_CurrentUsageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\n"]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc = "Get a reference to the value of field `milli_accelerator` after provisioning.\n"]
    pub fn milli_accelerator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.milli_accelerator", self.base))
    }

    #[doc = "Get a reference to the value of field `milli_dcu` after provisioning.\n"]
    pub fn milli_dcu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.milli_dcu", self.base))
    }

    #[doc = "Get a reference to the value of field `milli_dcu_premium` after provisioning.\n"]
    pub fn milli_dcu_premium(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.milli_dcu_premium", self.base))
    }

    #[doc = "Get a reference to the value of field `shuffle_storage_gb` after provisioning.\n"]
    pub fn shuffle_storage_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shuffle_storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `shuffle_storage_gb_premium` after provisioning.\n"]
    pub fn shuffle_storage_gb_premium(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shuffle_storage_gb_premium", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_time` after provisioning.\n"]
    pub fn snapshot_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocBatch_RuntimeInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approximate_usage: std::option::Option<terrars::ListField<DataprocBatch_RuntimeInfo_ApproximateUsage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_usage: std::option::Option<terrars::ListField<DataprocBatch_RuntimeInfo_CurrentUsage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    diagnostic_output_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocBatch_RuntimeInfo {
    #[doc = "Set the field `approximate_usage`.\n"]
    pub fn set_approximate_usage(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataprocBatch_RuntimeInfo_ApproximateUsage>>,
    ) -> Self {
        self.approximate_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `current_usage`.\n"]
    pub fn set_current_usage(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataprocBatch_RuntimeInfo_CurrentUsage>>,
    ) -> Self {
        self.current_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `diagnostic_output_uri`.\n"]
    pub fn set_diagnostic_output_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.diagnostic_output_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_uri`.\n"]
    pub fn set_output_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_RuntimeInfo {
    type O = terrars::BlockAssignable<DataprocBatch_RuntimeInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_RuntimeInfo {}

impl BuildDataprocBatch_RuntimeInfo {
    pub fn build(self) -> DataprocBatch_RuntimeInfo {
        DataprocBatch_RuntimeInfo {
            approximate_usage: core::default::Default::default(),
            current_usage: core::default::Default::default(),
            diagnostic_output_uri: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            output_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_RuntimeInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_RuntimeInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_RuntimeInfoRef {
        DataprocBatch_RuntimeInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_RuntimeInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approximate_usage` after provisioning.\n"]
    pub fn approximate_usage(&self) -> terrars::ListRef<DataprocBatch_RuntimeInfo_ApproximateUsageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approximate_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `current_usage` after provisioning.\n"]
    pub fn current_usage(&self) -> terrars::ListRef<DataprocBatch_RuntimeInfo_CurrentUsageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `diagnostic_output_uri` after provisioning.\n"]
    pub fn diagnostic_output_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.diagnostic_output_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `output_uri` after provisioning.\n"]
    pub fn output_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocBatch_StateHistory {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocBatch_StateHistory {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_start_time`.\n"]
    pub fn set_state_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_StateHistory {
    type O = terrars::BlockAssignable<DataprocBatch_StateHistory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_StateHistory {}

impl BuildDataprocBatch_StateHistory {
    pub fn build(self) -> DataprocBatch_StateHistory {
        DataprocBatch_StateHistory {
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
            state_start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_StateHistoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_StateHistoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_StateHistoryRef {
        DataprocBatch_StateHistoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_StateHistoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.base))
    }

    #[doc = "Get a reference to the value of field `state_start_time` after provisioning.\n"]
    pub fn state_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_workload_authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
    #[doc =
        "Set the field `user_workload_authentication_type`.\nAuthentication type for the user workload running in containers. Possible values: [\"SERVICE_ACCOUNT\", \"END_USER_CREDENTIALS\"]"]
    pub fn set_user_workload_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_workload_authentication_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
    type O = terrars::BlockAssignable<DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {}

impl BuildDataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
    pub fn build(self) -> DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
        DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig {
            user_workload_authentication_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef {
        DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `user_workload_authentication_type` after provisioning.\nAuthentication type for the user workload running in containers. Possible values: [\"SERVICE_ACCOUNT\", \"END_USER_CREDENTIALS\"]"]
    pub fn user_workload_authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_workload_authentication_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocBatch_EnvironmentConfig_ExecutionConfigDynamic {
    authentication_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig>>,
}

#[derive(Serialize)]
pub struct DataprocBatch_EnvironmentConfig_ExecutionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    staging_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_config: std
    ::option
    ::Option<std::vec::Vec<DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig>>,
    dynamic: DataprocBatch_EnvironmentConfig_ExecutionConfigDynamic,
}

impl DataprocBatch_EnvironmentConfig_ExecutionConfig {
    #[doc = "Set the field `kms_key`.\nThe Cloud KMS key to use for encryption."]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_tags`.\nTags used for network traffic control."]
    pub fn set_network_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.network_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_uri`.\nNetwork configuration for workload execution."]
    pub fn set_network_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\nService account that used to execute workload."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `staging_bucket`.\nA Cloud Storage bucket used to stage workload dependencies, config files, and store\nworkload output and other ephemeral data, such as Spark history files. If you do not specify a staging bucket,\nCloud Dataproc will determine a Cloud Storage location according to the region where your workload is running,\nand then create and manage project-level, per-location staging and temporary buckets.\nThis field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket."]
    pub fn set_staging_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.staging_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork_uri`.\nSubnetwork configuration for workload execution."]
    pub fn set_subnetwork_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ttl`.\nThe duration after which the workload will be terminated.\nWhen the workload exceeds this duration, it will be unconditionally terminated without waiting for ongoing\nwork to finish. If ttl is not specified for a batch workload, the workload will be allowed to run until it\nexits naturally (or run forever without exiting). If ttl is not specified for an interactive session,\nit defaults to 24 hours. If ttl is not specified for a batch that uses 2.1+ runtime version, it defaults to 4 hours.\nMinimum value is 10 minutes; maximum value is 14 days. If both ttl and idleTtl are specified (for an interactive session),\nthe conditions are treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or\nwhen ttl has been exceeded, whichever occurs first."]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_config`.\n"]
    pub fn set_authentication_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_EnvironmentConfig_ExecutionConfig {
    type O = terrars::BlockAssignable<DataprocBatch_EnvironmentConfig_ExecutionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_EnvironmentConfig_ExecutionConfig {}

impl BuildDataprocBatch_EnvironmentConfig_ExecutionConfig {
    pub fn build(self) -> DataprocBatch_EnvironmentConfig_ExecutionConfig {
        DataprocBatch_EnvironmentConfig_ExecutionConfig {
            kms_key: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            network_uri: core::default::Default::default(),
            service_account: core::default::Default::default(),
            staging_bucket: core::default::Default::default(),
            subnetwork_uri: core::default::Default::default(),
            ttl: core::default::Default::default(),
            authentication_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_EnvironmentConfig_ExecutionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_EnvironmentConfig_ExecutionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocBatch_EnvironmentConfig_ExecutionConfigRef {
        DataprocBatch_EnvironmentConfig_ExecutionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_EnvironmentConfig_ExecutionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key to use for encryption."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_tags` after provisioning.\nTags used for network traffic control."]
    pub fn network_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_uri` after provisioning.\nNetwork configuration for workload execution."]
    pub fn network_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nService account that used to execute workload."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `staging_bucket` after provisioning.\nA Cloud Storage bucket used to stage workload dependencies, config files, and store\nworkload output and other ephemeral data, such as Spark history files. If you do not specify a staging bucket,\nCloud Dataproc will determine a Cloud Storage location according to the region where your workload is running,\nand then create and manage project-level, per-location staging and temporary buckets.\nThis field requires a Cloud Storage bucket name, not a gs://... URI to a Cloud Storage bucket."]
    pub fn staging_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.staging_bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork_uri` after provisioning.\nSubnetwork configuration for workload execution."]
    pub fn subnetwork_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe duration after which the workload will be terminated.\nWhen the workload exceeds this duration, it will be unconditionally terminated without waiting for ongoing\nwork to finish. If ttl is not specified for a batch workload, the workload will be allowed to run until it\nexits naturally (or run forever without exiting). If ttl is not specified for an interactive session,\nit defaults to 24 hours. If ttl is not specified for a batch that uses 2.1+ runtime version, it defaults to 4 hours.\nMinimum value is 10 minutes; maximum value is 14 days. If both ttl and idleTtl are specified (for an interactive session),\nthe conditions are treated as OR conditions: the workload will be terminated when it has been idle for idleTtl or\nwhen ttl has been exceeded, whichever occurs first."]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication_config` after provisioning.\n"]
    pub fn authentication_config(
        &self,
    ) -> terrars::ListRef<DataprocBatch_EnvironmentConfig_ExecutionConfig_AuthenticationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataproc_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
    #[doc =
        "Set the field `dataproc_cluster`.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn set_dataproc_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataproc_cluster = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
    type O = terrars::BlockAssignable<DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {}

impl BuildDataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
    pub fn build(self) -> DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
        DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig {
            dataproc_cluster: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef {
        DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataproc_cluster` after provisioning.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the workload."]
    pub fn dataproc_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataproc_cluster", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocBatch_EnvironmentConfig_PeripheralsConfigDynamic {
    spark_history_server_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig>>,
}

#[derive(Serialize)]
pub struct DataprocBatch_EnvironmentConfig_PeripheralsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metastore_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_history_server_config: std
    ::option
    ::Option<std::vec::Vec<DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig>>,
    dynamic: DataprocBatch_EnvironmentConfig_PeripheralsConfigDynamic,
}

impl DataprocBatch_EnvironmentConfig_PeripheralsConfig {
    #[doc = "Set the field `metastore_service`.\nResource name of an existing Dataproc Metastore service."]
    pub fn set_metastore_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metastore_service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spark_history_server_config`.\n"]
    pub fn set_spark_history_server_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spark_history_server_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_history_server_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_EnvironmentConfig_PeripheralsConfig {
    type O = terrars::BlockAssignable<DataprocBatch_EnvironmentConfig_PeripheralsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_EnvironmentConfig_PeripheralsConfig {}

impl BuildDataprocBatch_EnvironmentConfig_PeripheralsConfig {
    pub fn build(self) -> DataprocBatch_EnvironmentConfig_PeripheralsConfig {
        DataprocBatch_EnvironmentConfig_PeripheralsConfig {
            metastore_service: core::default::Default::default(),
            spark_history_server_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_EnvironmentConfig_PeripheralsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_EnvironmentConfig_PeripheralsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocBatch_EnvironmentConfig_PeripheralsConfigRef {
        DataprocBatch_EnvironmentConfig_PeripheralsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_EnvironmentConfig_PeripheralsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metastore_service` after provisioning.\nResource name of an existing Dataproc Metastore service."]
    pub fn metastore_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metastore_service", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_history_server_config` after provisioning.\n"]
    pub fn spark_history_server_config(
        &self,
    ) -> terrars::ListRef<DataprocBatch_EnvironmentConfig_PeripheralsConfig_SparkHistoryServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_history_server_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocBatch_EnvironmentConfigDynamic {
    execution_config: std::option::Option<terrars::DynamicBlock<DataprocBatch_EnvironmentConfig_ExecutionConfig>>,
    peripherals_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocBatch_EnvironmentConfig_PeripheralsConfig>>,
}

#[derive(Serialize)]
pub struct DataprocBatch_EnvironmentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_config: std::option::Option<std::vec::Vec<DataprocBatch_EnvironmentConfig_ExecutionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peripherals_config: std::option::Option<std::vec::Vec<DataprocBatch_EnvironmentConfig_PeripheralsConfig>>,
    dynamic: DataprocBatch_EnvironmentConfigDynamic,
}

impl DataprocBatch_EnvironmentConfig {
    #[doc = "Set the field `execution_config`.\n"]
    pub fn set_execution_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocBatch_EnvironmentConfig_ExecutionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.execution_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.execution_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `peripherals_config`.\n"]
    pub fn set_peripherals_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocBatch_EnvironmentConfig_PeripheralsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.peripherals_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.peripherals_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_EnvironmentConfig {
    type O = terrars::BlockAssignable<DataprocBatch_EnvironmentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_EnvironmentConfig {}

impl BuildDataprocBatch_EnvironmentConfig {
    pub fn build(self) -> DataprocBatch_EnvironmentConfig {
        DataprocBatch_EnvironmentConfig {
            execution_config: core::default::Default::default(),
            peripherals_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_EnvironmentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_EnvironmentConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_EnvironmentConfigRef {
        DataprocBatch_EnvironmentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_EnvironmentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execution_config` after provisioning.\n"]
    pub fn execution_config(&self) -> terrars::ListRef<DataprocBatch_EnvironmentConfig_ExecutionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_config", self.base))
    }

    #[doc = "Get a reference to the value of field `peripherals_config` after provisioning.\n"]
    pub fn peripherals_config(&self) -> terrars::ListRef<DataprocBatch_EnvironmentConfig_PeripheralsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peripherals_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocBatch_PysparkBatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_python_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataprocBatch_PysparkBatch {
    #[doc =
        "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
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
        "Set the field `main_python_file_uri`.\nThe HCFS URI of the main Python file to use as the Spark driver. Must be a .py file."]
    pub fn set_main_python_file_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.main_python_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `python_file_uris`.\nHCFS file URIs of Python files to pass to the PySpark framework.\nSupported file types: .py, .egg, and .zip."]
    pub fn set_python_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.python_file_uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_PysparkBatch {
    type O = terrars::BlockAssignable<DataprocBatch_PysparkBatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_PysparkBatch {}

impl BuildDataprocBatch_PysparkBatch {
    pub fn build(self) -> DataprocBatch_PysparkBatch {
        DataprocBatch_PysparkBatch {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_python_file_uri: core::default::Default::default(),
            python_file_uris: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_PysparkBatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_PysparkBatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_PysparkBatchRef {
        DataprocBatch_PysparkBatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_PysparkBatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
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
        "Get a reference to the value of field `main_python_file_uri` after provisioning.\nThe HCFS URI of the main Python file to use as the Spark driver. Must be a .py file."]
    pub fn main_python_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_python_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_file_uris` after provisioning.\nHCFS file URIs of Python files to pass to the PySpark framework.\nSupported file types: .py, .egg, and .zip."]
    pub fn python_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_file_uris", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocBatch_RuntimeConfig_AutotuningConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scenarios: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataprocBatch_RuntimeConfig_AutotuningConfig {
    #[doc =
        "Set the field `scenarios`.\nOptional. Scenarios for which tunings are applied. Possible values: [\"AUTO\", \"SCALING\", \"BROADCAST_HASH_JOIN\", \"MEMORY\"]"]
    pub fn set_scenarios(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scenarios = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_RuntimeConfig_AutotuningConfig {
    type O = terrars::BlockAssignable<DataprocBatch_RuntimeConfig_AutotuningConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_RuntimeConfig_AutotuningConfig {}

impl BuildDataprocBatch_RuntimeConfig_AutotuningConfig {
    pub fn build(self) -> DataprocBatch_RuntimeConfig_AutotuningConfig {
        DataprocBatch_RuntimeConfig_AutotuningConfig { scenarios: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_RuntimeConfig_AutotuningConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_RuntimeConfig_AutotuningConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocBatch_RuntimeConfig_AutotuningConfigRef {
        DataprocBatch_RuntimeConfig_AutotuningConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_RuntimeConfig_AutotuningConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `scenarios` after provisioning.\nOptional. Scenarios for which tunings are applied. Possible values: [\"AUTO\", \"SCALING\", \"BROADCAST_HASH_JOIN\", \"MEMORY\"]"]
    pub fn scenarios(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scenarios", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocBatch_RuntimeConfigDynamic {
    autotuning_config: std::option::Option<terrars::DynamicBlock<DataprocBatch_RuntimeConfig_AutotuningConfig>>,
}

#[derive(Serialize)]
pub struct DataprocBatch_RuntimeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cohort: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autotuning_config: std::option::Option<std::vec::Vec<DataprocBatch_RuntimeConfig_AutotuningConfig>>,
    dynamic: DataprocBatch_RuntimeConfigDynamic,
}

impl DataprocBatch_RuntimeConfig {
    #[doc =
        "Set the field `cohort`.\nOptional. Cohort identifier. Identifies families of the workloads having the same shape, e.g. daily ETL jobs."]
    pub fn set_cohort(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cohort = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `container_image`.\nOptional custom container image for the job runtime environment. If not specified, a default container image will be used."]
    pub fn set_container_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nVersion of the batch runtime."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autotuning_config`.\n"]
    pub fn set_autotuning_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocBatch_RuntimeConfig_AutotuningConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autotuning_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autotuning_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_RuntimeConfig {
    type O = terrars::BlockAssignable<DataprocBatch_RuntimeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_RuntimeConfig {}

impl BuildDataprocBatch_RuntimeConfig {
    pub fn build(self) -> DataprocBatch_RuntimeConfig {
        DataprocBatch_RuntimeConfig {
            cohort: core::default::Default::default(),
            container_image: core::default::Default::default(),
            properties: core::default::Default::default(),
            version: core::default::Default::default(),
            autotuning_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_RuntimeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_RuntimeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_RuntimeConfigRef {
        DataprocBatch_RuntimeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_RuntimeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cohort` after provisioning.\nOptional. Cohort identifier. Identifies families of the workloads having the same shape, e.g. daily ETL jobs."]
    pub fn cohort(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cohort", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container_image` after provisioning.\nOptional custom container image for the job runtime environment. If not specified, a default container image will be used."]
    pub fn container_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `effective_properties` after provisioning.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn effective_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, which are used to configure workload execution."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the batch runtime."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `autotuning_config` after provisioning.\n"]
    pub fn autotuning_config(&self) -> terrars::ListRef<DataprocBatch_RuntimeConfig_AutotuningConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autotuning_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocBatch_SparkBatch {
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

impl DataprocBatch_SparkBatch {
    #[doc =
        "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
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
        "Set the field `main_class`.\nThe name of the driver main class. The jar file that contains the class must be in the\nclasspath or specified in jarFileUris."]
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

impl terrars::ToListMappable for DataprocBatch_SparkBatch {
    type O = terrars::BlockAssignable<DataprocBatch_SparkBatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_SparkBatch {}

impl BuildDataprocBatch_SparkBatch {
    pub fn build(self) -> DataprocBatch_SparkBatch {
        DataprocBatch_SparkBatch {
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
pub struct DataprocBatch_SparkBatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_SparkBatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_SparkBatchRef {
        DataprocBatch_SparkBatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_SparkBatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
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
        "Get a reference to the value of field `main_class` after provisioning.\nThe name of the driver main class. The jar file that contains the class must be in the\nclasspath or specified in jarFileUris."]
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
pub struct DataprocBatch_SparkRBatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_r_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocBatch_SparkRBatch {
    #[doc =
        "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
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
        "Set the field `main_r_file_uri`.\nThe HCFS URI of the main R file to use as the driver. Must be a .R or .r file."]
    pub fn set_main_r_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_r_file_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_SparkRBatch {
    type O = terrars::BlockAssignable<DataprocBatch_SparkRBatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_SparkRBatch {}

impl BuildDataprocBatch_SparkRBatch {
    pub fn build(self) -> DataprocBatch_SparkRBatch {
        DataprocBatch_SparkRBatch {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            main_r_file_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_SparkRBatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_SparkRBatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_SparkRBatchRef {
        DataprocBatch_SparkRBatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_SparkRBatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted into the working directory of each executor.\nSupported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver. Do not include arguments that can be set as batch\nproperties, such as --conf, since a collision can occur that causes an incorrect batch submission."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be placed in the working directory of each executor."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_r_file_uri` after provisioning.\nThe HCFS URI of the main R file to use as the driver. Must be a .R or .r file."]
    pub fn main_r_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_r_file_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocBatch_SparkSqlBatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocBatch_SparkSqlBatch {
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains Spark SQL queries to execute."]
    pub fn set_query_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_variables`.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET name=\"value\";)."]
    pub fn set_query_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_variables = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocBatch_SparkSqlBatch {
    type O = terrars::BlockAssignable<DataprocBatch_SparkSqlBatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_SparkSqlBatch {}

impl BuildDataprocBatch_SparkSqlBatch {
    pub fn build(self) -> DataprocBatch_SparkSqlBatch {
        DataprocBatch_SparkSqlBatch {
            jar_file_uris: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_variables: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_SparkSqlBatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_SparkSqlBatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_SparkSqlBatchRef {
        DataprocBatch_SparkSqlBatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_SparkSqlBatchRef {
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
        "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains Spark SQL queries to execute."]
    pub fn query_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_variables` after provisioning.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET name=\"value\";)."]
    pub fn query_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.query_variables", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocBatch_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocBatch_Timeouts {
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

impl terrars::ToListMappable for DataprocBatch_Timeouts {
    type O = terrars::BlockAssignable<DataprocBatch_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocBatch_Timeouts {}

impl BuildDataprocBatch_Timeouts {
    pub fn build(self) -> DataprocBatch_Timeouts {
        DataprocBatch_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocBatch_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocBatch_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocBatch_TimeoutsRef {
        DataprocBatch_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocBatch_TimeoutsRef {
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
