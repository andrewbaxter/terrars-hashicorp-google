use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocJobData {
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
    hadoop_config: std::option::Option<std::vec::Vec<DataprocJobHadoopConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_config: std::option::Option<std::vec::Vec<DataprocJobHiveConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pig_config: std::option::Option<std::vec::Vec<DataprocJobPigConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<std::vec::Vec<DataprocJobPlacementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    presto_config: std::option::Option<std::vec::Vec<DataprocJobPrestoConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pyspark_config: std::option::Option<std::vec::Vec<DataprocJobPysparkConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference: std::option::Option<std::vec::Vec<DataprocJobReferenceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling: std::option::Option<std::vec::Vec<DataprocJobSchedulingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_config: std::option::Option<std::vec::Vec<DataprocJobSparkConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sparksql_config: std::option::Option<std::vec::Vec<DataprocJobSparksqlConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocJobTimeoutsEl>,
    dynamic: DataprocJobDynamic,
}

struct DataprocJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocJobData>,
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

    #[doc = "Set the field `hadoop_config`.\n"]
    pub fn set_hadoop_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobHadoopConfigEl>>,
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
    pub fn set_hive_config(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobHiveConfigEl>>) -> Self {
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
    pub fn set_pig_config(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobPigConfigEl>>) -> Self {
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
    pub fn set_placement(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobPlacementEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobPrestoConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobPysparkConfigEl>>,
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
    pub fn set_reference(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobReferenceEl>>) -> Self {
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
    pub fn set_scheduling(self, v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobSchedulingEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobSparkConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobSparksqlConfigEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocJobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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
    pub fn status(&self) -> terrars::ListRef<DataprocJobStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hadoop_config` after provisioning.\n"]
    pub fn hadoop_config(&self) -> terrars::ListRef<DataprocJobHadoopConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hadoop_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hive_config` after provisioning.\n"]
    pub fn hive_config(&self) -> terrars::ListRef<DataprocJobHiveConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pig_config` after provisioning.\n"]
    pub fn pig_config(&self) -> terrars::ListRef<DataprocJobPigConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pig_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<DataprocJobPlacementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `presto_config` after provisioning.\n"]
    pub fn presto_config(&self) -> terrars::ListRef<DataprocJobPrestoConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.presto_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pyspark_config` after provisioning.\n"]
    pub fn pyspark_config(&self) -> terrars::ListRef<DataprocJobPysparkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pyspark_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference` after provisioning.\n"]
    pub fn reference(&self) -> terrars::ListRef<DataprocJobReferenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<DataprocJobSchedulingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_config` after provisioning.\n"]
    pub fn spark_config(&self) -> terrars::ListRef<DataprocJobSparkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sparksql_config` after provisioning.\n"]
    pub fn sparksql_config(&self) -> terrars::ListRef<DataprocJobSparksqlConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sparksql_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocJobTimeoutsElRef {
        DataprocJobTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataprocJobData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
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
    pub fn status(&self) -> terrars::ListRef<DataprocJobStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hadoop_config` after provisioning.\n"]
    pub fn hadoop_config(&self) -> terrars::ListRef<DataprocJobHadoopConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hadoop_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hive_config` after provisioning.\n"]
    pub fn hive_config(&self) -> terrars::ListRef<DataprocJobHiveConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pig_config` after provisioning.\n"]
    pub fn pig_config(&self) -> terrars::ListRef<DataprocJobPigConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pig_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<DataprocJobPlacementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `presto_config` after provisioning.\n"]
    pub fn presto_config(&self) -> terrars::ListRef<DataprocJobPrestoConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.presto_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pyspark_config` after provisioning.\n"]
    pub fn pyspark_config(&self) -> terrars::ListRef<DataprocJobPysparkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pyspark_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference` after provisioning.\n"]
    pub fn reference(&self) -> terrars::ListRef<DataprocJobReferenceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<DataprocJobSchedulingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_config` after provisioning.\n"]
    pub fn spark_config(&self) -> terrars::ListRef<DataprocJobSparkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sparksql_config` after provisioning.\n"]
    pub fn sparksql_config(&self) -> terrars::ListRef<DataprocJobSparksqlConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sparksql_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocJobTimeoutsElRef {
        DataprocJobTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataprocJobStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    substate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocJobStatusEl {
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

impl terrars::ToListMappable for DataprocJobStatusEl {
    type O = terrars::BlockAssignable<DataprocJobStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobStatusEl {}

impl BuildDataprocJobStatusEl {
    pub fn build(self) -> DataprocJobStatusEl {
        DataprocJobStatusEl {
            details: core::default::Default::default(),
            state: core::default::Default::default(),
            state_start_time: core::default::Default::default(),
            substate: core::default::Default::default(),
        }
    }
}

pub struct DataprocJobStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobStatusElRef {
        DataprocJobStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobStatusElRef {
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
pub struct DataprocJobHadoopConfigElLoggingConfigEl {
    driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl DataprocJobHadoopConfigElLoggingConfigEl { }

impl terrars::ToListMappable for DataprocJobHadoopConfigElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocJobHadoopConfigElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobHadoopConfigElLoggingConfigEl {
    #[doc =
        "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocJobHadoopConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobHadoopConfigElLoggingConfigEl {
        DataprocJobHadoopConfigElLoggingConfigEl { driver_log_levels: self.driver_log_levels }
    }
}

pub struct DataprocJobHadoopConfigElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobHadoopConfigElLoggingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobHadoopConfigElLoggingConfigElRef {
        DataprocJobHadoopConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobHadoopConfigElLoggingConfigElRef {
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
struct DataprocJobHadoopConfigElDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJobHadoopConfigElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocJobHadoopConfigEl {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocJobHadoopConfigElLoggingConfigEl>>,
    dynamic: DataprocJobHadoopConfigElDynamic,
}

impl DataprocJobHadoopConfigEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobHadoopConfigElLoggingConfigEl>>,
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

impl terrars::ToListMappable for DataprocJobHadoopConfigEl {
    type O = terrars::BlockAssignable<DataprocJobHadoopConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobHadoopConfigEl {}

impl BuildDataprocJobHadoopConfigEl {
    pub fn build(self) -> DataprocJobHadoopConfigEl {
        DataprocJobHadoopConfigEl {
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

pub struct DataprocJobHadoopConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobHadoopConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobHadoopConfigElRef {
        DataprocJobHadoopConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobHadoopConfigElRef {
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJobHadoopConfigElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJobHiveConfigEl {
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

impl DataprocJobHiveConfigEl {
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

impl terrars::ToListMappable for DataprocJobHiveConfigEl {
    type O = terrars::BlockAssignable<DataprocJobHiveConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobHiveConfigEl {}

impl BuildDataprocJobHiveConfigEl {
    pub fn build(self) -> DataprocJobHiveConfigEl {
        DataprocJobHiveConfigEl {
            continue_on_failure: core::default::Default::default(),
            jar_file_uris: core::default::Default::default(),
            properties: core::default::Default::default(),
            query_file_uri: core::default::Default::default(),
            query_list: core::default::Default::default(),
            script_variables: core::default::Default::default(),
        }
    }
}

pub struct DataprocJobHiveConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobHiveConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobHiveConfigElRef {
        DataprocJobHiveConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobHiveConfigElRef {
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

#[derive(Serialize)]
pub struct DataprocJobPigConfigElLoggingConfigEl {
    driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl DataprocJobPigConfigElLoggingConfigEl { }

impl terrars::ToListMappable for DataprocJobPigConfigElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocJobPigConfigElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobPigConfigElLoggingConfigEl {
    #[doc =
        "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocJobPigConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobPigConfigElLoggingConfigEl {
        DataprocJobPigConfigElLoggingConfigEl { driver_log_levels: self.driver_log_levels }
    }
}

pub struct DataprocJobPigConfigElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobPigConfigElLoggingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobPigConfigElLoggingConfigElRef {
        DataprocJobPigConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobPigConfigElLoggingConfigElRef {
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
struct DataprocJobPigConfigElDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJobPigConfigElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocJobPigConfigEl {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocJobPigConfigElLoggingConfigEl>>,
    dynamic: DataprocJobPigConfigElDynamic,
}

impl DataprocJobPigConfigEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobPigConfigElLoggingConfigEl>>,
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

impl terrars::ToListMappable for DataprocJobPigConfigEl {
    type O = terrars::BlockAssignable<DataprocJobPigConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobPigConfigEl {}

impl BuildDataprocJobPigConfigEl {
    pub fn build(self) -> DataprocJobPigConfigEl {
        DataprocJobPigConfigEl {
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

pub struct DataprocJobPigConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobPigConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobPigConfigElRef {
        DataprocJobPigConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobPigConfigElRef {
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJobPigConfigElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJobPlacementEl {
    cluster_name: terrars::PrimField<std::string::String>,
}

impl DataprocJobPlacementEl { }

impl terrars::ToListMappable for DataprocJobPlacementEl {
    type O = terrars::BlockAssignable<DataprocJobPlacementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobPlacementEl {
    #[doc = "The name of the cluster where the job will be submitted"]
    pub cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildDataprocJobPlacementEl {
    pub fn build(self) -> DataprocJobPlacementEl {
        DataprocJobPlacementEl { cluster_name: self.cluster_name }
    }
}

pub struct DataprocJobPlacementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobPlacementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobPlacementElRef {
        DataprocJobPlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobPlacementElRef {
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

#[derive(Serialize)]
pub struct DataprocJobPrestoConfigElLoggingConfigEl {
    driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl DataprocJobPrestoConfigElLoggingConfigEl { }

impl terrars::ToListMappable for DataprocJobPrestoConfigElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocJobPrestoConfigElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobPrestoConfigElLoggingConfigEl {
    #[doc =
        "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocJobPrestoConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobPrestoConfigElLoggingConfigEl {
        DataprocJobPrestoConfigElLoggingConfigEl { driver_log_levels: self.driver_log_levels }
    }
}

pub struct DataprocJobPrestoConfigElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobPrestoConfigElLoggingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobPrestoConfigElLoggingConfigElRef {
        DataprocJobPrestoConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobPrestoConfigElLoggingConfigElRef {
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
struct DataprocJobPrestoConfigElDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJobPrestoConfigElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocJobPrestoConfigEl {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocJobPrestoConfigElLoggingConfigEl>>,
    dynamic: DataprocJobPrestoConfigElDynamic,
}

impl DataprocJobPrestoConfigEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobPrestoConfigElLoggingConfigEl>>,
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

impl terrars::ToListMappable for DataprocJobPrestoConfigEl {
    type O = terrars::BlockAssignable<DataprocJobPrestoConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobPrestoConfigEl {}

impl BuildDataprocJobPrestoConfigEl {
    pub fn build(self) -> DataprocJobPrestoConfigEl {
        DataprocJobPrestoConfigEl {
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

pub struct DataprocJobPrestoConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobPrestoConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobPrestoConfigElRef {
        DataprocJobPrestoConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobPrestoConfigElRef {
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJobPrestoConfigElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJobPysparkConfigElLoggingConfigEl {
    driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl DataprocJobPysparkConfigElLoggingConfigEl { }

impl terrars::ToListMappable for DataprocJobPysparkConfigElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocJobPysparkConfigElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobPysparkConfigElLoggingConfigEl {
    #[doc =
        "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocJobPysparkConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobPysparkConfigElLoggingConfigEl {
        DataprocJobPysparkConfigElLoggingConfigEl { driver_log_levels: self.driver_log_levels }
    }
}

pub struct DataprocJobPysparkConfigElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobPysparkConfigElLoggingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobPysparkConfigElLoggingConfigElRef {
        DataprocJobPysparkConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobPysparkConfigElLoggingConfigElRef {
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
struct DataprocJobPysparkConfigElDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJobPysparkConfigElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocJobPysparkConfigEl {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocJobPysparkConfigElLoggingConfigEl>>,
    dynamic: DataprocJobPysparkConfigElDynamic,
}

impl DataprocJobPysparkConfigEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobPysparkConfigElLoggingConfigEl>>,
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

impl terrars::ToListMappable for DataprocJobPysparkConfigEl {
    type O = terrars::BlockAssignable<DataprocJobPysparkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobPysparkConfigEl {
    #[doc = "Required. The HCFS URI of the main Python file to use as the driver. Must be a .py file"]
    pub main_python_file_uri: terrars::PrimField<std::string::String>,
}

impl BuildDataprocJobPysparkConfigEl {
    pub fn build(self) -> DataprocJobPysparkConfigEl {
        DataprocJobPysparkConfigEl {
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

pub struct DataprocJobPysparkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobPysparkConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobPysparkConfigElRef {
        DataprocJobPysparkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobPysparkConfigElRef {
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJobPysparkConfigElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJobReferenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocJobReferenceEl {
    #[doc =
        "Set the field `job_id`.\nThe job ID, which must be unique within the project. The job ID is generated by the server upon job submission or provided by the user as a means to perform retries without creating duplicate jobs"]
    pub fn set_job_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.job_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocJobReferenceEl {
    type O = terrars::BlockAssignable<DataprocJobReferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobReferenceEl {}

impl BuildDataprocJobReferenceEl {
    pub fn build(self) -> DataprocJobReferenceEl {
        DataprocJobReferenceEl { job_id: core::default::Default::default() }
    }
}

pub struct DataprocJobReferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobReferenceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobReferenceElRef {
        DataprocJobReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobReferenceElRef {
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
pub struct DataprocJobSchedulingEl {
    max_failures_per_hour: terrars::PrimField<f64>,
    max_failures_total: terrars::PrimField<f64>,
}

impl DataprocJobSchedulingEl { }

impl terrars::ToListMappable for DataprocJobSchedulingEl {
    type O = terrars::BlockAssignable<DataprocJobSchedulingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobSchedulingEl {
    #[doc =
        "Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed."]
    pub max_failures_per_hour: terrars::PrimField<f64>,
    #[doc =
        "Maximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed."]
    pub max_failures_total: terrars::PrimField<f64>,
}

impl BuildDataprocJobSchedulingEl {
    pub fn build(self) -> DataprocJobSchedulingEl {
        DataprocJobSchedulingEl {
            max_failures_per_hour: self.max_failures_per_hour,
            max_failures_total: self.max_failures_total,
        }
    }
}

pub struct DataprocJobSchedulingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobSchedulingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobSchedulingElRef {
        DataprocJobSchedulingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobSchedulingElRef {
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

#[derive(Serialize)]
pub struct DataprocJobSparkConfigElLoggingConfigEl {
    driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl DataprocJobSparkConfigElLoggingConfigEl { }

impl terrars::ToListMappable for DataprocJobSparkConfigElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocJobSparkConfigElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobSparkConfigElLoggingConfigEl {
    #[doc =
        "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocJobSparkConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobSparkConfigElLoggingConfigEl {
        DataprocJobSparkConfigElLoggingConfigEl { driver_log_levels: self.driver_log_levels }
    }
}

pub struct DataprocJobSparkConfigElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobSparkConfigElLoggingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobSparkConfigElLoggingConfigElRef {
        DataprocJobSparkConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobSparkConfigElLoggingConfigElRef {
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
struct DataprocJobSparkConfigElDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJobSparkConfigElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocJobSparkConfigEl {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocJobSparkConfigElLoggingConfigEl>>,
    dynamic: DataprocJobSparkConfigElDynamic,
}

impl DataprocJobSparkConfigEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobSparkConfigElLoggingConfigEl>>,
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

impl terrars::ToListMappable for DataprocJobSparkConfigEl {
    type O = terrars::BlockAssignable<DataprocJobSparkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobSparkConfigEl {}

impl BuildDataprocJobSparkConfigEl {
    pub fn build(self) -> DataprocJobSparkConfigEl {
        DataprocJobSparkConfigEl {
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

pub struct DataprocJobSparkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobSparkConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobSparkConfigElRef {
        DataprocJobSparkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobSparkConfigElRef {
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJobSparkConfigElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJobSparksqlConfigElLoggingConfigEl {
    driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl DataprocJobSparksqlConfigElLoggingConfigEl { }

impl terrars::ToListMappable for DataprocJobSparksqlConfigElLoggingConfigEl {
    type O = terrars::BlockAssignable<DataprocJobSparksqlConfigElLoggingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobSparksqlConfigElLoggingConfigEl {
    #[doc =
        "Optional. The per-package log levels for the driver. This may include 'root' package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'."]
    pub driver_log_levels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildDataprocJobSparksqlConfigElLoggingConfigEl {
    pub fn build(self) -> DataprocJobSparksqlConfigElLoggingConfigEl {
        DataprocJobSparksqlConfigElLoggingConfigEl { driver_log_levels: self.driver_log_levels }
    }
}

pub struct DataprocJobSparksqlConfigElLoggingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobSparksqlConfigElLoggingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobSparksqlConfigElLoggingConfigElRef {
        DataprocJobSparksqlConfigElLoggingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobSparksqlConfigElLoggingConfigElRef {
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
struct DataprocJobSparksqlConfigElDynamic {
    logging_config: std::option::Option<terrars::DynamicBlock<DataprocJobSparksqlConfigElLoggingConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocJobSparksqlConfigEl {
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
    logging_config: std::option::Option<std::vec::Vec<DataprocJobSparksqlConfigElLoggingConfigEl>>,
    dynamic: DataprocJobSparksqlConfigElDynamic,
}

impl DataprocJobSparksqlConfigEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocJobSparksqlConfigElLoggingConfigEl>>,
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

impl terrars::ToListMappable for DataprocJobSparksqlConfigEl {
    type O = terrars::BlockAssignable<DataprocJobSparksqlConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobSparksqlConfigEl {}

impl BuildDataprocJobSparksqlConfigEl {
    pub fn build(self) -> DataprocJobSparksqlConfigEl {
        DataprocJobSparksqlConfigEl {
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

pub struct DataprocJobSparksqlConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobSparksqlConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobSparksqlConfigElRef {
        DataprocJobSparksqlConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobSparksqlConfigElRef {
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
    pub fn logging_config(&self) -> terrars::ListRef<DataprocJobSparksqlConfigElLoggingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocJobTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocJobTimeoutsEl {
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

impl terrars::ToListMappable for DataprocJobTimeoutsEl {
    type O = terrars::BlockAssignable<DataprocJobTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocJobTimeoutsEl {}

impl BuildDataprocJobTimeoutsEl {
    pub fn build(self) -> DataprocJobTimeoutsEl {
        DataprocJobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct DataprocJobTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocJobTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocJobTimeoutsElRef {
        DataprocJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocJobTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct DataprocJobDynamic {
    hadoop_config: std::option::Option<terrars::DynamicBlock<DataprocJobHadoopConfigEl>>,
    hive_config: std::option::Option<terrars::DynamicBlock<DataprocJobHiveConfigEl>>,
    pig_config: std::option::Option<terrars::DynamicBlock<DataprocJobPigConfigEl>>,
    placement: std::option::Option<terrars::DynamicBlock<DataprocJobPlacementEl>>,
    presto_config: std::option::Option<terrars::DynamicBlock<DataprocJobPrestoConfigEl>>,
    pyspark_config: std::option::Option<terrars::DynamicBlock<DataprocJobPysparkConfigEl>>,
    reference: std::option::Option<terrars::DynamicBlock<DataprocJobReferenceEl>>,
    scheduling: std::option::Option<terrars::DynamicBlock<DataprocJobSchedulingEl>>,
    spark_config: std::option::Option<terrars::DynamicBlock<DataprocJobSparkConfigEl>>,
    sparksql_config: std::option::Option<terrars::DynamicBlock<DataprocJobSparksqlConfigEl>>,
}
