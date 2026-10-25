#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DatabaseMigrationServiceMigrationJobData_ {
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
    desired_state: std::option::Option<terrars::PrimField<std::string::String>>,
    destination: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dump_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dump_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    migration_job_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_on_warnings: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dump_flags: std::option::Option<std::vec::Vec<DatabaseMigrationServiceMigrationJob_DumpFlags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objects_config: std::option::Option<std::vec::Vec<DatabaseMigrationServiceMigrationJob_ObjectsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_config: std::option::Option<std::vec::Vec<DatabaseMigrationServiceMigrationJob_PerformanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgres_homogeneous_config: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reverse_ssh_connectivity: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceMigrationJob_ReverseSshConnectivity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_ip_connectivity: std::option::Option<std::vec::Vec<DatabaseMigrationServiceMigrationJob_StaticIpConnectivity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DatabaseMigrationServiceMigrationJob_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_peering_connectivity: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity>>,
    dynamic: DatabaseMigrationServiceMigrationJobDynamic,
}

struct DatabaseMigrationServiceMigrationJob_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabaseMigrationServiceMigrationJobData_>,
}

#[derive(Clone)]
pub struct DatabaseMigrationServiceMigrationJob(std::rc::Rc<DatabaseMigrationServiceMigrationJob_>);

impl DatabaseMigrationServiceMigrationJob {
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
        "Set the field `desired_state`.\nThe desired state of the migration job. If set to 'RUNNING', the migration job will be started. Possible values: [\"NOT_STARTED\", \"RUNNING\"]"]
    pub fn set_desired_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().desired_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe migration job display name."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dump_path`.\nThe path to the dump file in Google Cloud Storage,\nin the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]).\nThis field and the \"dump_flags\" field are mutually exclusive."]
    pub fn set_dump_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dump_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dump_type`.\nThe type of the data dump. Supported for MySQL to CloudSQL for MySQL\nmigrations only. Possible values: [\"LOGICAL\", \"PHYSICAL\"]"]
    pub fn set_dump_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dump_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location where the migration job should reside."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stop_on_warnings`.\nIf set to true, will stop the Terraform apply if there are validation warnings."]
    pub fn set_stop_on_warnings(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().stop_on_warnings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dump_flags`.\n"]
    pub fn set_dump_flags(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_DumpFlags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dump_flags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dump_flags = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `objects_config`.\n"]
    pub fn set_objects_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_ObjectsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().objects_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.objects_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `performance_config`.\n"]
    pub fn set_performance_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_PerformanceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().performance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.performance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postgres_homogeneous_config`.\n"]
    pub fn set_postgres_homogeneous_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().postgres_homogeneous_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.postgres_homogeneous_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reverse_ssh_connectivity`.\n"]
    pub fn set_reverse_ssh_connectivity(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_ReverseSshConnectivity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reverse_ssh_connectivity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reverse_ssh_connectivity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `static_ip_connectivity`.\n"]
    pub fn set_static_ip_connectivity(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_StaticIpConnectivity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().static_ip_connectivity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.static_ip_connectivity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DatabaseMigrationServiceMigrationJob_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_connectivity`.\n"]
    pub fn set_vpc_peering_connectivity(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_peering_connectivity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_peering_connectivity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nThe desired state of the migration job. If set to 'RUNNING', the migration job will be started. Possible values: [\"NOT_STARTED\", \"RUNNING\"]"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destination` after provisioning.\nThe name of the destination connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{destinationConnectionProfile}."]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe migration job display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dump_path` after provisioning.\nThe path to the dump file in Google Cloud Storage,\nin the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]).\nThis field and the \"dump_flags\" field are mutually exclusive."]
    pub fn dump_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dump_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dump_type` after provisioning.\nThe type of the data dump. Supported for MySQL to CloudSQL for MySQL\nmigrations only. Possible values: [\"LOGICAL\", \"PHYSICAL\"]"]
    pub fn dump_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dump_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error` after provisioning.\nOutput only. The error details in case of state FAILED."]
    pub fn error(&self) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the migration job should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migration_job_id` after provisioning.\nThe ID of the migration job."]
    pub fn migration_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.migration_job_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this migration job resource in the form of projects/{project}/locations/{location}/migrationJobs/{migrationJob}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phase` after provisioning.\nThe current migration job phase."]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nThe name of the source connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{sourceConnectionProfile}."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current migration job state."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stop_on_warnings` after provisioning.\nIf set to true, will stop the Terraform apply if there are validation warnings."]
    pub fn stop_on_warnings(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_on_warnings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the migration job. Possible values: [\"ONE_TIME\", \"CONTINUOUS\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dump_flags` after provisioning.\n"]
    pub fn dump_flags(&self) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_DumpFlagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dump_flags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `objects_config` after provisioning.\n"]
    pub fn objects_config(&self) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_ObjectsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\n"]
    pub fn performance_config(&self) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_PerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.performance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgres_homogeneous_config` after provisioning.\n"]
    pub fn postgres_homogeneous_config(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgres_homogeneous_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reverse_ssh_connectivity` after provisioning.\n"]
    pub fn reverse_ssh_connectivity(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_ReverseSshConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reverse_ssh_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `static_ip_connectivity` after provisioning.\n"]
    pub fn static_ip_connectivity(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_StaticIpConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_ip_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServiceMigrationJob_TimeoutsRef {
        DatabaseMigrationServiceMigrationJob_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connectivity` after provisioning.\n"]
    pub fn vpc_peering_connectivity(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_peering_connectivity", self.extract_ref()))
    }
}

impl terrars::Referable for DatabaseMigrationServiceMigrationJob {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabaseMigrationServiceMigrationJob { }

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob {
    type O = terrars::ListRef<DatabaseMigrationServiceMigrationJobRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabaseMigrationServiceMigrationJob_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_database_migration_service_migration_job".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the destination connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{destinationConnectionProfile}."]
    pub destination: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the migration job."]
    pub migration_job_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the source connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{sourceConnectionProfile}."]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = "The type of the migration job. Possible values: [\"ONE_TIME\", \"CONTINUOUS\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceMigrationJob {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabaseMigrationServiceMigrationJob {
        let out = DatabaseMigrationServiceMigrationJob(std::rc::Rc::new(DatabaseMigrationServiceMigrationJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatabaseMigrationServiceMigrationJobData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                destination: self.destination,
                display_name: core::default::Default::default(),
                dump_path: core::default::Default::default(),
                dump_type: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                migration_job_id: self.migration_job_id,
                project: core::default::Default::default(),
                source: self.source,
                stop_on_warnings: core::default::Default::default(),
                type_: self.type_,
                dump_flags: core::default::Default::default(),
                objects_config: core::default::Default::default(),
                performance_config: core::default::Default::default(),
                postgres_homogeneous_config: core::default::Default::default(),
                reverse_ssh_connectivity: core::default::Default::default(),
                static_ip_connectivity: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_peering_connectivity: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseMigrationServiceMigrationJobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseMigrationServiceMigrationJobRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nThe desired state of the migration job. If set to 'RUNNING', the migration job will be started. Possible values: [\"NOT_STARTED\", \"RUNNING\"]"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destination` after provisioning.\nThe name of the destination connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{destinationConnectionProfile}."]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe migration job display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dump_path` after provisioning.\nThe path to the dump file in Google Cloud Storage,\nin the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]).\nThis field and the \"dump_flags\" field are mutually exclusive."]
    pub fn dump_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dump_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dump_type` after provisioning.\nThe type of the data dump. Supported for MySQL to CloudSQL for MySQL\nmigrations only. Possible values: [\"LOGICAL\", \"PHYSICAL\"]"]
    pub fn dump_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dump_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error` after provisioning.\nOutput only. The error details in case of state FAILED."]
    pub fn error(&self) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the migration job should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `migration_job_id` after provisioning.\nThe ID of the migration job."]
    pub fn migration_job_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.migration_job_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this migration job resource in the form of projects/{project}/locations/{location}/migrationJobs/{migrationJob}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phase` after provisioning.\nThe current migration job phase."]
    pub fn phase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phase", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nThe name of the source connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{sourceConnectionProfile}."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current migration job state."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stop_on_warnings` after provisioning.\nIf set to true, will stop the Terraform apply if there are validation warnings."]
    pub fn stop_on_warnings(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_on_warnings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the migration job. Possible values: [\"ONE_TIME\", \"CONTINUOUS\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dump_flags` after provisioning.\n"]
    pub fn dump_flags(&self) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_DumpFlagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dump_flags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `objects_config` after provisioning.\n"]
    pub fn objects_config(&self) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_ObjectsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `performance_config` after provisioning.\n"]
    pub fn performance_config(&self) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_PerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.performance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgres_homogeneous_config` after provisioning.\n"]
    pub fn postgres_homogeneous_config(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgres_homogeneous_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reverse_ssh_connectivity` after provisioning.\n"]
    pub fn reverse_ssh_connectivity(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_ReverseSshConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reverse_ssh_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `static_ip_connectivity` after provisioning.\n"]
    pub fn static_ip_connectivity(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_StaticIpConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_ip_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServiceMigrationJob_TimeoutsRef {
        DatabaseMigrationServiceMigrationJob_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connectivity` after provisioning.\n"]
    pub fn vpc_peering_connectivity(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_peering_connectivity", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceMigrationJobDynamic {
    dump_flags: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceMigrationJob_DumpFlags>>,
    objects_config: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceMigrationJob_ObjectsConfig>>,
    performance_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceMigrationJob_PerformanceConfig>>,
    postgres_homogeneous_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig>>,
    reverse_ssh_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceMigrationJob_ReverseSshConnectivity>>,
    static_ip_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceMigrationJob_StaticIpConnectivity>>,
    vpc_peering_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServiceMigrationJob_Error {
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

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_Error {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_Error {}

impl BuildDatabaseMigrationServiceMigrationJob_Error {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_Error {
        DatabaseMigrationServiceMigrationJob_Error {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_ErrorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatabaseMigrationServiceMigrationJob_ErrorRef {
        DatabaseMigrationServiceMigrationJob_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_ErrorRef {
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
pub struct DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags {
    #[doc = "Set the field `name`.\nThe name of the flag"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe vale of the flag"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags {}

impl BuildDatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags {
        DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlagsRef {
        DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the flag"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe vale of the flag"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceMigrationJob_DumpFlagsDynamic {
    dump_flags: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_DumpFlags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dump_flags: std::option::Option<std::vec::Vec<DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags>>,
    dynamic: DatabaseMigrationServiceMigrationJob_DumpFlagsDynamic,
}

impl DatabaseMigrationServiceMigrationJob_DumpFlags {
    #[doc = "Set the field `dump_flags`.\n"]
    pub fn set_dump_flags(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dump_flags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dump_flags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_DumpFlags {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_DumpFlags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_DumpFlags {}

impl BuildDatabaseMigrationServiceMigrationJob_DumpFlags {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_DumpFlags {
        DatabaseMigrationServiceMigrationJob_DumpFlags {
            dump_flags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_DumpFlagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_DumpFlagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_DumpFlagsRef {
        DatabaseMigrationServiceMigrationJob_DumpFlagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_DumpFlagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dump_flags` after provisioning.\n"]
    pub fn dump_flags(&self) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_DumpFlags_DumpFlagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dump_flags", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier {
    #[doc =
        "Set the field `database`.\nThe database name. Required only if the object uses\na database name as part of its unique identifier."]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schema`.\nThe schema name. Required only if the object uses\na schema name as part of its unique identifier."]
    pub fn set_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table`.\nThe table name. Required only if the object is a level\nbelow database or schema."]
    pub fn set_table(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier {
    type O =
        terrars
        ::BlockAssignable<
            DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier {
    #[doc =
        "The category of the migration job object: 'DATABASE',\n'SCHEMA', or 'TABLE'. Possible values: [\"DATABASE\", \"SCHEMA\", \"TABLE\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier {
    pub fn build(
        self,
    ) -> DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier {
        DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier {
            database: core::default::Default::default(),
            schema: core::default::Default::default(),
            table: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifierRef {
        DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe database name. Required only if the object uses\na database name as part of its unique identifier."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nThe schema name. Required only if the object uses\na schema name as part of its unique identifier."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table` after provisioning.\nThe table name. Required only if the object is a level\nbelow database or schema."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe category of the migration job object: 'DATABASE',\n'SCHEMA', or 'TABLE'. Possible values: [\"DATABASE\", \"SCHEMA\", \"TABLE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigsDynamic {
    object_identifier: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_identifier: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier>,
    >,
    dynamic: DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigsDynamic,
}

impl DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs {
    #[doc = "Set the field `object_identifier`.\n"]
    pub fn set_object_identifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifier,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.object_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.object_identifier = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs {
    type O =
        terrars
        ::BlockAssignable<DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs {}

impl BuildDatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs {
        DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs {
            object_identifier: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigsRef {
        DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_identifier` after provisioning.\n"]
    pub fn object_identifier(
        &self,
    ) -> terrars
    ::ListRef<
        DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs_ObjectIdentifierRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfigDynamic {
    object_configs: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs>,
    >,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objects_selection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_configs: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs>>,
    dynamic: DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfigDynamic,
}

impl DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig {
    #[doc =
        "Set the field `objects_selection_type`.\nThe objects selection type of the migration job. When set to\n'SPECIFIED_OBJECTS', only the objects listed in 'objectConfigs' are\nmigrated. When set to 'ALL_OBJECTS', all objects available on the\nsource are migrated. Possible values: [\"ALL_OBJECTS\", \"SPECIFIED_OBJECTS\"]"]
    pub fn set_objects_selection_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.objects_selection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_configs`.\n"]
    pub fn set_object_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.object_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.object_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig {}

impl BuildDatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig {
        DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig {
            objects_selection_type: core::default::Default::default(),
            object_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfigRef {
        DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `objects_selection_type` after provisioning.\nThe objects selection type of the migration job. When set to\n'SPECIFIED_OBJECTS', only the objects listed in 'objectConfigs' are\nmigrated. When set to 'ALL_OBJECTS', all objects available on the\nsource are migrated. Possible values: [\"ALL_OBJECTS\", \"SPECIFIED_OBJECTS\"]"]
    pub fn objects_selection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.objects_selection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `object_configs` after provisioning.\n"]
    pub fn object_configs(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig_ObjectConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_configs", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceMigrationJob_ObjectsConfigDynamic {
    source_objects_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_ObjectsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_objects_config: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig>>,
    dynamic: DatabaseMigrationServiceMigrationJob_ObjectsConfigDynamic,
}

impl DatabaseMigrationServiceMigrationJob_ObjectsConfig {
    #[doc = "Set the field `source_objects_config`.\n"]
    pub fn set_source_objects_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_objects_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_objects_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_ObjectsConfig {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_ObjectsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_ObjectsConfig {}

impl BuildDatabaseMigrationServiceMigrationJob_ObjectsConfig {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_ObjectsConfig {
        DatabaseMigrationServiceMigrationJob_ObjectsConfig {
            source_objects_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_ObjectsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_ObjectsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_ObjectsConfigRef {
        DatabaseMigrationServiceMigrationJob_ObjectsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_ObjectsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_objects_config` after provisioning.\n"]
    pub fn source_objects_config(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceMigrationJob_ObjectsConfig_SourceObjectsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_objects_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_PerformanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dump_parallel_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServiceMigrationJob_PerformanceConfig {
    #[doc =
        "Set the field `dump_parallel_level`.\nInitial dump parallelism level. Possible values: [\"MIN\", \"OPTIMAL\", \"MAX\"]"]
    pub fn set_dump_parallel_level(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.dump_parallel_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_PerformanceConfig {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_PerformanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_PerformanceConfig {}

impl BuildDatabaseMigrationServiceMigrationJob_PerformanceConfig {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_PerformanceConfig {
        DatabaseMigrationServiceMigrationJob_PerformanceConfig {
            dump_parallel_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_PerformanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_PerformanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_PerformanceConfigRef {
        DatabaseMigrationServiceMigrationJob_PerformanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_PerformanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dump_parallel_level` after provisioning.\nInitial dump parallelism level. Possible values: [\"MIN\", \"OPTIMAL\", \"MAX\"]"]
    pub fn dump_parallel_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dump_parallel_level", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig {
    is_native_logical: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_additional_subscriptions: std::option::Option<terrars::PrimField<f64>>,
}

impl DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig {
    #[doc =
        "Set the field `max_additional_subscriptions`.\nMaximum number of additional subscriptions to use for the migration job."]
    pub fn set_max_additional_subscriptions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_additional_subscriptions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig {
    #[doc = "Whether the migration uses native logical replication."]
    pub is_native_logical: terrars::PrimField<bool>,
}

impl BuildDatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig {
        DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfig {
            is_native_logical: self.is_native_logical,
            max_additional_subscriptions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfigRef {
        DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_PostgresHomogeneousConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_native_logical` after provisioning.\nWhether the migration uses native logical replication."]
    pub fn is_native_logical(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_native_logical", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_additional_subscriptions` after provisioning.\nMaximum number of additional subscriptions to use for the migration job."]
    pub fn max_additional_subscriptions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_additional_subscriptions", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_ReverseSshConnectivity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServiceMigrationJob_ReverseSshConnectivity {
    #[doc =
        "Set the field `vm`.\nThe name of the virtual machine (Compute Engine) used as the bastion server\nfor the SSH tunnel."]
    pub fn set_vm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vm = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vm_ip`.\nThe IP of the virtual machine (Compute Engine) used as the bastion server\nfor the SSH tunnel."]
    pub fn set_vm_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vm_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vm_port`.\nThe forwarding port of the virtual machine (Compute Engine) used as the\nbastion server for the SSH tunnel."]
    pub fn set_vm_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vm_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\nThe name of the VPC to peer with the Cloud SQL private network."]
    pub fn set_vpc(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_ReverseSshConnectivity {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_ReverseSshConnectivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_ReverseSshConnectivity {}

impl BuildDatabaseMigrationServiceMigrationJob_ReverseSshConnectivity {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_ReverseSshConnectivity {
        DatabaseMigrationServiceMigrationJob_ReverseSshConnectivity {
            vm: core::default::Default::default(),
            vm_ip: core::default::Default::default(),
            vm_port: core::default::Default::default(),
            vpc: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_ReverseSshConnectivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_ReverseSshConnectivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_ReverseSshConnectivityRef {
        DatabaseMigrationServiceMigrationJob_ReverseSshConnectivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_ReverseSshConnectivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `vm` after provisioning.\nThe name of the virtual machine (Compute Engine) used as the bastion server\nfor the SSH tunnel."]
    pub fn vm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vm_ip` after provisioning.\nThe IP of the virtual machine (Compute Engine) used as the bastion server\nfor the SSH tunnel."]
    pub fn vm_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vm_port` after provisioning.\nThe forwarding port of the virtual machine (Compute Engine) used as the\nbastion server for the SSH tunnel."]
    pub fn vm_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vpc` after provisioning.\nThe name of the VPC to peer with the Cloud SQL private network."]
    pub fn vpc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_StaticIpConnectivity {}

impl DatabaseMigrationServiceMigrationJob_StaticIpConnectivity { }

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_StaticIpConnectivity {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_StaticIpConnectivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_StaticIpConnectivity {}

impl BuildDatabaseMigrationServiceMigrationJob_StaticIpConnectivity {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_StaticIpConnectivity {
        DatabaseMigrationServiceMigrationJob_StaticIpConnectivity {}
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_StaticIpConnectivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_StaticIpConnectivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_StaticIpConnectivityRef {
        DatabaseMigrationServiceMigrationJob_StaticIpConnectivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_StaticIpConnectivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServiceMigrationJob_Timeouts {
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

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_Timeouts {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_Timeouts {}

impl BuildDatabaseMigrationServiceMigrationJob_Timeouts {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_Timeouts {
        DatabaseMigrationServiceMigrationJob_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_TimeoutsRef {
        DatabaseMigrationServiceMigrationJob_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_TimeoutsRef {
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

#[derive(Serialize)]
pub struct DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity {
    #[doc = "Set the field `vpc`.\nThe name of the VPC network to peer with the Cloud SQL private network."]
    pub fn set_vpc(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity {}

impl BuildDatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity {
    pub fn build(self) -> DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity {
        DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivity { vpc: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivityRef {
        DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceMigrationJob_VpcPeeringConnectivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `vpc` after provisioning.\nThe name of the VPC network to peer with the Cloud SQL private network."]
    pub fn vpc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}
