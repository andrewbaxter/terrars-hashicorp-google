#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocJobData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_completion: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hadoop_config: std::option::Option<std::vec::Vec<DataprocJob_HadoopConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_config: std::option::Option<std::vec::Vec<DataprocJob_HiveConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pig_config: std::option::Option<std::vec::Vec<DataprocJob_PigConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<std::vec::Vec<DataprocJob_Placement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    presto_config: std::option::Option<std::vec::Vec<DataprocJob_PrestoConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pyspark_config: std::option::Option<std::vec::Vec<DataprocJob_PysparkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference: std::option::Option<std::vec::Vec<DataprocJob_Reference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling: std::option::Option<std::vec::Vec<DataprocJob_Scheduling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_config: std::option::Option<std::vec::Vec<DataprocJob_SparkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sparksql_config: std::option::Option<std::vec::Vec<DataprocJob_SparksqlConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocJob_Timeouts>,
    dynamic: DataprocJobDynamic,
}

struct DataprocJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocJobData_>,
}

#[derive(Clone)]
pub struct DataprocJob(std::rc::Rc<DataprocJob_>);

impl DataprocJob {
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
        "Set the field `force_delete`.\nBy default, you can only delete inactive jobs within Dataproc. Setting this to true, and calling destroy, will ensure that the job is first cancelled before issuing the delete."]
    pub fn set_force_delete(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. The labels to associate with this job.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe project in which the cluster can be found and jobs subsequently run against. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe Cloud Dataproc region. This essentially determines which clusters are available for this job to be submitted to. If not specified, defaults to global."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait_for_completion`.\nIf set to true, Terraform will wait for the job to reach a terminal state (DONE, ERROR, CANCELLED, ATTEMPT_FAILURE). Otherwise, Terraform will consider the job 'created' once it is in the RUNNING state."]
    pub fn set_wait_for_completion(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_completion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hadoop_config`.\n"]
    pub fn set_hadoop_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_HadoopConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hadoop_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hadoop_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hive_config`.\n"]
    pub fn set_hive_config(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_HiveConfig>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hive_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hive_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pig_config`.\n"]
    pub fn set_pig_config(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_PigConfig>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pig_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pig_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement`.\n"]
    pub fn set_placement(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_Placement>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `presto_config`.\n"]
    pub fn set_presto_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_PrestoConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().presto_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.presto_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pyspark_config`.\n"]
    pub fn set_pyspark_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_PysparkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pyspark_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pyspark_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reference`.\n"]
    pub fn set_reference(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_Reference>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduling`.\n"]
    pub fn set_scheduling(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_Scheduling>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_config`.\n"]
    pub fn set_spark_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_SparkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sparksql_config`.\n"]
    pub fn set_sparksql_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_SparksqlConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sparksql_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sparksql_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `driver_controls_files_uri` after provisioning.\nOutput-only. If present, the location of miscellaneous control files which may be used as part of job setup and handling. If not present, control files may be placed in the same location as driver_output_uri."]
    pub fn driver_controls_files_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver_controls_files_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `driver_output_resource_uri` after provisioning.\nOutput-only. A URI pointing to the location of the stdout of the job's driver program"]
    pub fn driver_output_resource_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver_output_resource_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_delete` after provisioning.\nBy default, you can only delete inactive jobs within Dataproc. Setting this to true, and calling destroy, will ensure that the job is first cancelled before issuing the delete."]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. The labels to associate with this job.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which the cluster can be found and jobs subsequently run against. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe Cloud Dataproc region. This essentially determines which clusters are available for this job to be submitted to. If not specified, defaults to global."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the job."]
    pub fn status(&self) -> terrars::ListRef<DataprocJob_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_completion` after provisioning.\nIf set to true, Terraform will wait for the job to reach a terminal state (DONE, ERROR, CANCELLED, ATTEMPT_FAILURE). Otherwise, Terraform will consider the job 'created' once it is in the RUNNING state."]
    pub fn wait_for_completion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_completion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hadoop_config` after provisioning.\n"]
    pub fn hadoop_config(&self) -> terrars::ListRef<DataprocJob_HadoopConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hadoop_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hive_config` after provisioning.\n"]
    pub fn hive_config(&self) -> terrars::ListRef<DataprocJob_HiveConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pig_config` after provisioning.\n"]
    pub fn pig_config(&self) -> terrars::ListRef<DataprocJob_PigConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pig_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<DataprocJob_PlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `presto_config` after provisioning.\n"]
    pub fn presto_config(&self) -> terrars::ListRef<DataprocJob_PrestoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.presto_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pyspark_config` after provisioning.\n"]
    pub fn pyspark_config(&self) -> terrars::ListRef<DataprocJob_PysparkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pyspark_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference` after provisioning.\n"]
    pub fn reference(&self) -> terrars::ListRef<DataprocJob_ReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<DataprocJob_SchedulingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_config` after provisioning.\n"]
    pub fn spark_config(&self) -> terrars::ListRef<DataprocJob_SparkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sparksql_config` after provisioning.\n"]
    pub fn sparksql_config(&self) -> terrars::ListRef<DataprocJob_SparksqlConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sparksql_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocJob_TimeoutsRef {
        DataprocJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataprocJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataprocJob { }

impl terrars::ToListMappable for DataprocJob {
    type O = terrars::ListRef<DataprocJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataprocJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataproc_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataprocJob {
    pub tf_id: std::string::String,
}

impl BuildDataprocJob {
    pub fn build(self, stack: &mut terrars::Stack) -> DataprocJob {
        let out = DataprocJob(std::rc::Rc::new(DataprocJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataprocJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                wait_for_completion: core::default::Default::default(),
                hadoop_config: core::default::Default::default(),
                hive_config: core::default::Default::default(),
                pig_config: core::default::Default::default(),
                placement: core::default::Default::default(),
                presto_config: core::default::Default::default(),
                pyspark_config: core::default::Default::default(),
                reference: core::default::Default::default(),
                scheduling: core::default::Default::default(),
                spark_config: core::default::Default::default(),
                sparksql_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataprocJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataprocJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `driver_controls_files_uri` after provisioning.\nOutput-only. If present, the location of miscellaneous control files which may be used as part of job setup and handling. If not present, control files may be placed in the same location as driver_output_uri."]
    pub fn driver_controls_files_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver_controls_files_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `driver_output_resource_uri` after provisioning.\nOutput-only. A URI pointing to the location of the stdout of the job's driver program"]
    pub fn driver_output_resource_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver_output_resource_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_delete` after provisioning.\nBy default, you can only delete inactive jobs within Dataproc. Setting this to true, and calling destroy, will ensure that the job is first cancelled before issuing the delete."]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. The labels to associate with this job.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which the cluster can be found and jobs subsequently run against. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe Cloud Dataproc region. This essentially determines which clusters are available for this job to be submitted to. If not specified, defaults to global."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the job."]
    pub fn status(&self) -> terrars::ListRef<DataprocJob_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_completion` after provisioning.\nIf set to true, Terraform will wait for the job to reach a terminal state (DONE, ERROR, CANCELLED, ATTEMPT_FAILURE). Otherwise, Terraform will consider the job 'created' once it is in the RUNNING state."]
    pub fn wait_for_completion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_completion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hadoop_config` after provisioning.\n"]
    pub fn hadoop_config(&self) -> terrars::ListRef<DataprocJob_HadoopConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hadoop_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hive_config` after provisioning.\n"]
    pub fn hive_config(&self) -> terrars::ListRef<DataprocJob_HiveConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pig_config` after provisioning.\n"]
    pub fn pig_config(&self) -> terrars::ListRef<DataprocJob_PigConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pig_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<DataprocJob_PlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `presto_config` after provisioning.\n"]
    pub fn presto_config(&self) -> terrars::ListRef<DataprocJob_PrestoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.presto_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pyspark_config` after provisioning.\n"]
    pub fn pyspark_config(&self) -> terrars::ListRef<DataprocJob_PysparkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pyspark_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference` after provisioning.\n"]
    pub fn reference(&self) -> terrars::ListRef<DataprocJob_ReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<DataprocJob_SchedulingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_config` after provisioning.\n"]
    pub fn spark_config(&self) -> terrars::ListRef<DataprocJob_SparkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sparksql_config` after provisioning.\n"]
    pub fn sparksql_config(&self) -> terrars::ListRef<DataprocJob_SparksqlConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sparksql_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocJob_TimeoutsRef {
        DataprocJob_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataprocJobDynamic {
    hadoop_config: std::option::Option<terrars::DynamicBlock<DataprocJob_HadoopConfig>>,
    hive_config: std::option::Option<terrars::DynamicBlock<DataprocJob_HiveConfig>>,
    pig_config: std::option::Option<terrars::DynamicBlock<DataprocJob_PigConfig>>,
    placement: std::option::Option<terrars::DynamicBlock<DataprocJob_Placement>>,
    presto_config: std::option::Option<terrars::DynamicBlock<DataprocJob_PrestoConfig>>,
    pyspark_config: std::option::Option<terrars::DynamicBlock<DataprocJob_PysparkConfig>>,
    reference: std::option::Option<terrars::DynamicBlock<DataprocJob_Reference>>,
    scheduling: std::option::Option<terrars::DynamicBlock<DataprocJob_Scheduling>>,
    spark_config: std::option::Option<terrars::DynamicBlock<DataprocJob_SparkConfig>>,
    sparksql_config: std::option::Option<terrars::DynamicBlock<DataprocJob_SparksqlConfig>>,
}

#[derive(Serialize)]
pub struct DataprocJob_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocJob_Status {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state_start_time`.\n"]
    pub fn set_state_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `substate`.\n"]
    pub fn set_substate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.substate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocJob_Status {
    type O = terrars::BlockAssignable<DataprocJob_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_Status {}

impl BuildDataprocJob_Status {
    pub fn build(self) -> DataprocJob_Status {
        DataprocJob_Status {
            details: core::default::Default::default(),
            state: core::default::Default::default(),
            state_start_time: core::default::Default::default(),
            substate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_StatusRef {
        DataprocJob_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_start_time` after provisioning.\n"]
    pub fn state_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `substate` after provisioning.\n"]
    pub fn substate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.substate", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJob_PigConfig_LoggingConfig {
    driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl DataprocJob_PigConfig_LoggingConfig { }

impl terrars::ToListMappable for DataprocJob_PigConfig_LoggingConfig {
    type O = terrars::BlockAssignable<DataprocJob_PigConfig_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_PigConfig_LoggingConfig {
    #[doc =
        "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocJob_PigConfig_LoggingConfig {
    pub fn build(self) -> DataprocJob_PigConfig_LoggingConfig {
        DataprocJob_PigConfig_LoggingConfig { driver_log_levels: self.driver_log_levels }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_PigConfig_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_PigConfig_LoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_PigConfig_LoggingConfigRef {
        DataprocJob_PigConfig_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_PigConfig_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver_log_levels` after provisioning.\nOptional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub fn driver_log_levels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_log_levels", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocJob_HadoopConfigDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJob_PigConfig_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocJob_HadoopConfig {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocJob_PigConfig_LoggingConfig>>,
    dynamic: DataprocJob_HadoopConfigDynamic,
}

impl DataprocJob_HadoopConfig {
    #[doc =
        "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `args`.\nThe arguments to pass to the driver."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nHCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_class`.\nThe class containing the main method of the driver. Must be in a provided jar or jar that is already on the classpath. Conflicts with main_jar_file_uri"]
    pub fn set_main_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_class = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_jar_file_uri`.\nThe HCFS URI of jar file containing the driver jar. Conflicts with main_class"]
    pub fn set_main_jar_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_jar_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nA mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_PigConfig_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocJob_HadoopConfig {
    type O = terrars::BlockAssignable<DataprocJob_HadoopConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_HadoopConfig {}

impl BuildDataprocJob_HadoopConfig {
    pub fn build(self) -> DataprocJob_HadoopConfig {
        DataprocJob_HadoopConfig {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
            properties: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_HadoopConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_HadoopConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_HadoopConfigRef {
        DataprocJob_HadoopConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_HadoopConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_class` after provisioning.\nThe class containing the main method of the driver. Must be in a provided jar or jar that is already on the classpath. Conflicts with main_jar_file_uri"]
    pub fn main_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_jar_file_uri` after provisioning.\nThe HCFS URI of jar file containing the driver jar. Conflicts with main_class"]
    pub fn main_jar_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_jar_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJob_PigConfig_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJob_HiveConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continue_on_failure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataprocJob_HiveConfig {
    #[doc =
        "Set the field `continue_on_failure`.\nWhether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn set_continue_on_failure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.continue_on_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nA mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/hive/conf/hive-site.xml, and classes in user code."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_file_uri`.\nHCFS URI of file containing Hive script to execute as the job. Conflicts with query_list"]
    pub fn set_query_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_list`.\nThe list of Hive queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn set_query_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_list = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `script_variables`.\nMapping of query variable names to values (equivalent to the Hive command: SET name=\"value\";)."]
    pub fn set_script_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.script_variables = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocJob_HiveConfig {
    type O = terrars::BlockAssignable<DataprocJob_HiveConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_HiveConfig {}

impl BuildDataprocJob_HiveConfig {
    pub fn build(self) -> DataprocJob_HiveConfig {
        DataprocJob_HiveConfig {
            continue_on_failure: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_list: core::default::Default::default(),
            script_variables: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_HiveConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_HiveConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_HiveConfigRef {
        DataprocJob_HiveConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_HiveConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `continue_on_failure` after provisioning.\nWhether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn continue_on_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.continue_on_failure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/hive/conf/hive-site.xml, and classes in user code."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_file_uri` after provisioning.\nHCFS URI of file containing Hive script to execute as the job. Conflicts with query_list"]
    pub fn query_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_list` after provisioning.\nThe list of Hive queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn query_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script_variables` after provisioning.\nMapping of query variable names to values (equivalent to the Hive command: SET name=\"value\";)."]
    pub fn script_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.script_variables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocJob_PigConfigDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJob_PigConfig_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocJob_PigConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continue_on_failure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocJob_PigConfig_LoggingConfig>>,
    dynamic: DataprocJob_PigConfigDynamic,
}

impl DataprocJob_PigConfig {
    #[doc =
        "Set the field `continue_on_failure`.\nWhether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn set_continue_on_failure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.continue_on_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the CLASSPATH of the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nA mapping of property names to values, used to configure Pig. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/pig/conf/pig.properties, and classes in user code."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_file_uri`.\nHCFS URI of file containing Hive script to execute as the job. Conflicts with query_list"]
    pub fn set_query_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_list`.\nThe list of Hive queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn set_query_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_list = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `script_variables`.\nMapping of query variable names to values (equivalent to the Pig command: name=[value])."]
    pub fn set_script_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.script_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_PigConfig_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocJob_PigConfig {
    type O = terrars::BlockAssignable<DataprocJob_PigConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_PigConfig {}

impl BuildDataprocJob_PigConfig {
    pub fn build(self) -> DataprocJob_PigConfig {
        DataprocJob_PigConfig {
            continue_on_failure: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_list: core::default::Default::default(),
            script_variables: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_PigConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_PigConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_PigConfigRef {
        DataprocJob_PigConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_PigConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `continue_on_failure` after provisioning.\nWhether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn continue_on_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.continue_on_failure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the CLASSPATH of the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, used to configure Pig. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/pig/conf/pig.properties, and classes in user code."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_file_uri` after provisioning.\nHCFS URI of file containing Hive script to execute as the job. Conflicts with query_list"]
    pub fn query_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_list` after provisioning.\nThe list of Hive queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn query_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script_variables` after provisioning.\nMapping of query variable names to values (equivalent to the Pig command: name=[value])."]
    pub fn script_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.script_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJob_PigConfig_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJob_Placement {
    cluster_name: terrars::PrimField<std::string::String>,
}

impl DataprocJob_Placement { }

impl terrars::ToListMappable for DataprocJob_Placement {
    type O = terrars::BlockAssignable<DataprocJob_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_Placement {
    #[doc = "The name of the cluster where the job will be submitted"]
    pub cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildDataprocJob_Placement {
    pub fn build(self) -> DataprocJob_Placement {
        DataprocJob_Placement { cluster_name: self.cluster_name }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_PlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_PlacementRef {
        DataprocJob_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_PlacementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_name` after provisioning.\nThe name of the cluster where the job will be submitted"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_uuid` after provisioning.\nOutput-only. A cluster UUID generated by the Cloud Dataproc service when the job is submitted"]
    pub fn cluster_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_uuid", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocJob_PrestoConfigDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJob_PigConfig_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocJob_PrestoConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continue_on_failure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocJob_PigConfig_LoggingConfig>>,
    dynamic: DataprocJob_PrestoConfigDynamic,
}

impl DataprocJob_PrestoConfig {
    #[doc = "Set the field `client_tags`.\nPresto client tags to attach to this query."]
    pub fn set_client_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.client_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `continue_on_failure`.\nWhether to continue executing queries if a query fails. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn set_continue_on_failure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.continue_on_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_format`.\nThe format in which query output will be displayed. See the Presto documentation for supported output formats."]
    pub fn set_output_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_format = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nA mapping of property names to values. Used to set Presto session properties Equivalent to using the --session flag in the Presto CLI."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains SQL queries. Conflicts with query_list"]
    pub fn set_query_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_list`.\nThe list of SQL queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn set_query_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_PigConfig_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocJob_PrestoConfig {
    type O = terrars::BlockAssignable<DataprocJob_PrestoConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_PrestoConfig {}

impl BuildDataprocJob_PrestoConfig {
    pub fn build(self) -> DataprocJob_PrestoConfig {
        DataprocJob_PrestoConfig {
            client_tags: core::default::Default::default(),
            continue_on_failure: core::default::Default::default(),
            output_format: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_list: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_PrestoConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_PrestoConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_PrestoConfigRef {
        DataprocJob_PrestoConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_PrestoConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_tags` after provisioning.\nPresto client tags to attach to this query."]
    pub fn client_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `continue_on_failure` after provisioning.\nWhether to continue executing queries if a query fails. Setting to true can be useful when executing independent parallel queries. Defaults to false."]
    pub fn continue_on_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.continue_on_failure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_format` after provisioning.\nThe format in which query output will be displayed. See the Presto documentation for supported output formats."]
    pub fn output_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values. Used to set Presto session properties Equivalent to using the --session flag in the Presto CLI."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains SQL queries. Conflicts with query_list"]
    pub fn query_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_list` after provisioning.\nThe list of SQL queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn query_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJob_PigConfig_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocJob_PysparkConfigDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJob_PigConfig_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocJob_PysparkConfig {
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
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocJob_PigConfig_LoggingConfig>>,
    dynamic: DataprocJob_PysparkConfigDynamic,
}

impl DataprocJob_PysparkConfig {
    #[doc =
        "Set the field `archive_uris`.\nOptional. HCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip"]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `args`.\nOptional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nOptional. HCFS URIs of files to be copied to the working directory of Python drivers and distributed tasks. Useful for naively parallel tasks"]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nOptional. HCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks"]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nOptional. A mapping of property names to values, used to configure PySpark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `python_file_uris`.\nOptional. HCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip"]
    pub fn set_python_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.python_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_PigConfig_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocJob_PysparkConfig {
    type O = terrars::BlockAssignable<DataprocJob_PysparkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_PysparkConfig {
    #[doc = "Required. The HCFS URI of the main Python file to use as the driver. Must be a .py file"]
    pub main_python_file_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocJob_PysparkConfig {
    pub fn build(self) -> DataprocJob_PysparkConfig {
        DataprocJob_PysparkConfig {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_python_file_uri: self.main_python_file_uri,
            properties: core::default::Default::default(),
            python_file_uris: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_PysparkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_PysparkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_PysparkConfigRef {
        DataprocJob_PysparkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_PysparkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nOptional. HCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip"]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nOptional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission"]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nOptional. HCFS URIs of files to be copied to the working directory of Python drivers and distributed tasks. Useful for naively parallel tasks"]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nOptional. HCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks"]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_python_file_uri` after provisioning.\nRequired. The HCFS URI of the main Python file to use as the driver. Must be a .py file"]
    pub fn main_python_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_python_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nOptional. A mapping of property names to values, used to configure PySpark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_file_uris` after provisioning.\nOptional. HCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip"]
    pub fn python_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_file_uris", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJob_PigConfig_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJob_Reference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocJob_Reference {
    #[doc =
        "Set the field `job_id`.\nThe job ID, which must be unique within the project. The job ID is generated by the server upon job submission or provided by the user as a means to perform retries without creating duplicate jobs"]
    pub fn set_job_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.job_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocJob_Reference {
    type O = terrars::BlockAssignable<DataprocJob_Reference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_Reference {}

impl BuildDataprocJob_Reference {
    pub fn build(self) -> DataprocJob_Reference {
        DataprocJob_Reference { job_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_ReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_ReferenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_ReferenceRef {
        DataprocJob_ReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_ReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `job_id` after provisioning.\nThe job ID, which must be unique within the project. The job ID is generated by the server upon job submission or provided by the user as a means to perform retries without creating duplicate jobs"]
    pub fn job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJob_Scheduling {
    max_failures_per_hour: terrars::PrimField<f64>,
    max_failures_total: terrars::PrimField<f64>,
}

impl DataprocJob_Scheduling { }

impl terrars::ToListMappable for DataprocJob_Scheduling {
    type O = terrars::BlockAssignable<DataprocJob_Scheduling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_Scheduling {
    #[doc =
        "Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed."]
    pub max_failures_per_hour: terrars::PrimField<f64>,
    #[doc =
        "Maximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed."]
    pub max_failures_total: terrars::PrimField<f64>,
}

impl BuildDataprocJob_Scheduling {
    pub fn build(self) -> DataprocJob_Scheduling {
        DataprocJob_Scheduling {
            max_failures_per_hour: self.max_failures_per_hour,
            max_failures_total: self.max_failures_total,
        }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_SchedulingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_SchedulingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_SchedulingRef {
        DataprocJob_SchedulingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_SchedulingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_failures_per_hour` after provisioning.\nMaximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed."]
    pub fn max_failures_per_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_failures_per_hour", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_failures_total` after provisioning.\nMaximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed."]
    pub fn max_failures_total(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_failures_total", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocJob_SparkConfigDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJob_PigConfig_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocJob_SparkConfig {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocJob_PigConfig_LoggingConfig>>,
    dynamic: DataprocJob_SparkConfigDynamic,
}

impl DataprocJob_SparkConfig {
    #[doc =
        "Set the field `archive_uris`.\nHCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `args`.\nThe arguments to pass to the driver."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nHCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_file_uris`.\nHCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_class`.\nThe class containing the main method of the driver. Must be in a provided jar or jar that is already on the classpath. Conflicts with main_jar_file_uri"]
    pub fn set_main_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_class = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_jar_file_uri`.\nThe HCFS URI of jar file containing the driver jar. Conflicts with main_class"]
    pub fn set_main_jar_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_jar_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nA mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_PigConfig_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocJob_SparkConfig {
    type O = terrars::BlockAssignable<DataprocJob_SparkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_SparkConfig {}

impl BuildDataprocJob_SparkConfig {
    pub fn build(self) -> DataprocJob_SparkConfig {
        DataprocJob_SparkConfig {
            archive_uris: core::default::Default::default(),
            args: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_jar_file_uri: core::default::Default::default(),
            properties: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_SparkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_SparkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_SparkConfigRef {
        DataprocJob_SparkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_SparkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nHCFS URIs of archives to be extracted in the working directory of .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\nThe arguments to pass to the driver."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nHCFS URIs of files to be copied to the working directory of Spark drivers and distributed tasks. Useful for naively parallel tasks."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_file_uris` after provisioning.\nHCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub fn jar_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_class` after provisioning.\nThe class containing the main method of the driver. Must be in a provided jar or jar that is already on the classpath. Conflicts with main_jar_file_uri"]
    pub fn main_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_jar_file_uri` after provisioning.\nThe HCFS URI of jar file containing the driver jar. Conflicts with main_class"]
    pub fn main_jar_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_jar_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Cloud Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJob_PigConfig_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocJob_SparksqlConfigDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJob_PigConfig_LoggingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocJob_SparksqlConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_list: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<std::vec::Vec<DataprocJob_PigConfig_LoggingConfig>>,
    dynamic: DataprocJob_SparksqlConfigDynamic,
}

impl DataprocJob_SparksqlConfig {
    #[doc = "Set the field `jar_file_uris`.\nHCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub fn set_jar_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nA mapping of property names to values, used to configure Spark SQL's SparkConf. Properties that conflict with values set by the Cloud Dataproc API may be overwritten."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_file_uri`.\nThe HCFS URI of the script that contains SQL queries. Conflicts with query_list"]
    pub fn set_query_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_list`.\nThe list of SQL queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn set_query_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_list = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `script_variables`.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET name=\"value\";)."]
    pub fn set_script_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.script_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJob_PigConfig_LoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logging_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocJob_SparksqlConfig {
    type O = terrars::BlockAssignable<DataprocJob_SparksqlConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_SparksqlConfig {}

impl BuildDataprocJob_SparksqlConfig {
    pub fn build(self) -> DataprocJob_SparksqlConfig {
        DataprocJob_SparksqlConfig {
            jar_file_uris: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_list: core::default::Default::default(),
            script_variables: core::default::Default::default(),
            logging_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_SparksqlConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_SparksqlConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_SparksqlConfigRef {
        DataprocJob_SparksqlConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_SparksqlConfigRef {
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
        "Get a reference to the value of field `properties` after provisioning.\nA mapping of property names to values, used to configure Spark SQL's SparkConf. Properties that conflict with values set by the Cloud Dataproc API may be overwritten."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_file_uri` after provisioning.\nThe HCFS URI of the script that contains SQL queries. Conflicts with query_list"]
    pub fn query_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_list` after provisioning.\nThe list of SQL queries or statements to execute as part of the job. Conflicts with query_file_uri"]
    pub fn query_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_list", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script_variables` after provisioning.\nMapping of query variable names to values (equivalent to the Spark SQL command: SET name=\"value\";)."]
    pub fn script_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.script_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJob_PigConfig_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocJob_Timeouts {
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
}

impl terrars::ToListMappable for DataprocJob_Timeouts {
    type O = terrars::BlockAssignable<DataprocJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJob_Timeouts {}

impl BuildDataprocJob_Timeouts {
    pub fn build(self) -> DataprocJob_Timeouts {
        DataprocJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJob_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJob_TimeoutsRef {
        DataprocJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJob_TimeoutsRef {
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
}
