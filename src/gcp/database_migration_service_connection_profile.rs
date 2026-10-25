#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DatabaseMigrationServiceConnectionProfileData_ {
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
    connection_profile_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alloydb: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Alloydb>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudsql: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Cloudsql>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Mysql>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Oracle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Postgresql>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DatabaseMigrationServiceConnectionProfile_Timeouts>,
    dynamic: DatabaseMigrationServiceConnectionProfileDynamic,
}

struct DatabaseMigrationServiceConnectionProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabaseMigrationServiceConnectionProfileData_>,
}

#[derive(Clone)]
pub struct DatabaseMigrationServiceConnectionProfile(std::rc::Rc<DatabaseMigrationServiceConnectionProfile_>);

impl DatabaseMigrationServiceConnectionProfile {
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

    #[doc = "Set the field `display_name`.\nThe connection profile display name."]
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
        "Set the field `labels`.\nThe resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location where the connection profile should reside."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role`.\nThe connection profile role. Possible values: [\"SOURCE\", \"DESTINATION\"]"]
    pub fn set_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alloydb`.\n"]
    pub fn set_alloydb(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Alloydb>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alloydb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alloydb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloudsql`.\n"]
    pub fn set_cloudsql(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Cloudsql>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudsql = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudsql = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mysql`.\n"]
    pub fn set_mysql(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Mysql>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mysql = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mysql = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle`.\n"]
    pub fn set_oracle(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Oracle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oracle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oracle = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postgresql`.\n"]
    pub fn set_postgresql(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Postgresql>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().postgresql = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.postgresql = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DatabaseMigrationServiceConnectionProfile_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `connection_profile_id` after provisioning.\nThe ID of the connection profile."]
    pub fn connection_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_profile_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dbprovider` after provisioning.\nThe database provider."]
    pub fn dbprovider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dbprovider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe connection profile display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error` after provisioning.\nOutput only. The error details in case of state FAILED."]
    pub fn error(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the connection profile should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nThe connection profile role. Possible values: [\"SOURCE\", \"DESTINATION\"]"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current connection profile state."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alloydb` after provisioning.\n"]
    pub fn alloydb(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_AlloydbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alloydb", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudsql` after provisioning.\n"]
    pub fn cloudsql(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_CloudsqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudsql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql` after provisioning.\n"]
    pub fn mysql(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_MysqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oracle` after provisioning.\n"]
    pub fn oracle(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_OracleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgresql` after provisioning.\n"]
    pub fn postgresql(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_PostgresqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServiceConnectionProfile_TimeoutsRef {
        DatabaseMigrationServiceConnectionProfile_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DatabaseMigrationServiceConnectionProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabaseMigrationServiceConnectionProfile { }

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile {
    type O = terrars::ListRef<DatabaseMigrationServiceConnectionProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabaseMigrationServiceConnectionProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_database_migration_service_connection_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile {
    pub tf_id: std::string::String,
    #[doc = "The ID of the connection profile."]
    pub connection_profile_id: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceConnectionProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabaseMigrationServiceConnectionProfile {
        let out =
            DatabaseMigrationServiceConnectionProfile(std::rc::Rc::new(DatabaseMigrationServiceConnectionProfile_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DatabaseMigrationServiceConnectionProfileData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    connection_profile_id: self.connection_profile_id,
                    deletion_policy: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: core::default::Default::default(),
                    project: core::default::Default::default(),
                    role: core::default::Default::default(),
                    alloydb: core::default::Default::default(),
                    cloudsql: core::default::Default::default(),
                    mysql: core::default::Default::default(),
                    oracle: core::default::Default::default(),
                    postgresql: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseMigrationServiceConnectionProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseMigrationServiceConnectionProfileRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_profile_id` after provisioning.\nThe ID of the connection profile."]
    pub fn connection_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_profile_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dbprovider` after provisioning.\nThe database provider."]
    pub fn dbprovider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dbprovider", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe connection profile display name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error` after provisioning.\nOutput only. The error details in case of state FAILED."]
    pub fn error(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the connection profile should reside."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nThe connection profile role. Possible values: [\"SOURCE\", \"DESTINATION\"]"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current connection profile state."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alloydb` after provisioning.\n"]
    pub fn alloydb(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_AlloydbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alloydb", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudsql` after provisioning.\n"]
    pub fn cloudsql(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_CloudsqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudsql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql` after provisioning.\n"]
    pub fn mysql(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_MysqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oracle` after provisioning.\n"]
    pub fn oracle(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_OracleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgresql` after provisioning.\n"]
    pub fn postgresql(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_PostgresqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServiceConnectionProfile_TimeoutsRef {
        DatabaseMigrationServiceConnectionProfile_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfileDynamic {
    alloydb: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Alloydb>>,
    cloudsql: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Cloudsql>>,
    mysql: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Mysql>>,
    oracle: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Oracle>>,
    postgresql: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Postgresql>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServiceConnectionProfile_Error {
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

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Error {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Error {}

impl BuildDatabaseMigrationServiceConnectionProfile_Error {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Error {
        DatabaseMigrationServiceConnectionProfile_Error {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_ErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_ErrorRef {
        DatabaseMigrationServiceConnectionProfile_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_ErrorRef {
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
pub struct DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser {
    password: terrars::PrimField<std::string::String>,
    user: terrars::PrimField<std::string::String>,
}

impl DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser { }

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser {
    #[doc = "The initial password for the user."]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = "The database username."]
    pub user: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser {
        DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser {
            password: self.password,
            user: self.user,
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUserRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUserRef {
        DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe initial password for the user."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_set` after provisioning.\nOutput only. Indicates if the initialUser.password field has been set."]
    pub fn password_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_set", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nThe database username."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig {
    cpu_count: terrars::PrimField<f64>,
}

impl DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig { }

impl terrars
::ToListMappable for DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig {
    type O =
        terrars
        ::BlockAssignable<
            DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig {
    #[doc = "The number of CPU's in the VM instance."]
    pub cpu_count: terrars::PrimField<f64>,
}

impl BuildDatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig {
    pub fn build(
        self,
    ) -> DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig {
        DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig {
            cpu_count: self.cpu_count,
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfigRef {
        DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu_count` after provisioning.\nThe number of CPU's in the VM instance."]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettingsDynamic {
    machine_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_flags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig>,
    >,
    dynamic: DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettingsDynamic,
}

impl DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings {
    #[doc =
        "Set the field `database_flags`.\nDatabase flags to pass to AlloyDB when DMS is creating the AlloyDB cluster and instances. See the AlloyDB documentation for how these can be used."]
    pub fn set_database_flags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.database_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nLabels for the AlloyDB primary instance created by DMS."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_config`.\n"]
    pub fn set_machine_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.machine_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings {
    type O =
        terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings {
    #[doc = "The database username."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings {
        DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings {
            database_flags: core::default::Default::default(),
            id: self.id,
            labels: core::default::Default::default(),
            machine_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettingsRef {
        DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags to pass to AlloyDB when DMS is creating the AlloyDB cluster and instances. See the AlloyDB documentation for how these can be used."]
    pub fn database_flags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.database_flags", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe database username."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels for the AlloyDB primary instance created by DMS."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_ip` after provisioning.\nOutput only. The private IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_config` after provisioning.\n"]
    pub fn machine_config(
        &self,
    ) -> terrars
    ::ListRef<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings_MachineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfile_Alloydb_SettingsDynamic {
    initial_user: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser>>,
    primary_instance_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings>,
    >,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Alloydb_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_network: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_user: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_instance_settings: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings>>,
    dynamic: DatabaseMigrationServiceConnectionProfile_Alloydb_SettingsDynamic,
}

impl DatabaseMigrationServiceConnectionProfile_Alloydb_Settings {
    #[doc = "Set the field `labels`.\nLabels for the AlloyDB cluster created by DMS."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_user`.\n"]
    pub fn set_initial_user(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUser>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.initial_user = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.initial_user = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `primary_instance_settings`.\n"]
    pub fn set_primary_instance_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.primary_instance_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.primary_instance_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Alloydb_Settings {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Alloydb_Settings {
    #[doc =
        "Required. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster.\nIt is specified in the form: 'projects/{project_number}/global/networks/{network_id}'. This is required to create a cluster."]
    pub vpc_network: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceConnectionProfile_Alloydb_Settings {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Alloydb_Settings {
        DatabaseMigrationServiceConnectionProfile_Alloydb_Settings {
            labels: core::default::Default::default(),
            vpc_network: self.vpc_network,
            initial_user: core::default::Default::default(),
            primary_instance_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Alloydb_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Alloydb_SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Alloydb_SettingsRef {
        DatabaseMigrationServiceConnectionProfile_Alloydb_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Alloydb_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels for the AlloyDB cluster created by DMS."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vpc_network` after provisioning.\nRequired. The resource link for the VPC network in which cluster resources are created and from which they are accessible via Private IP. The network must belong to the same project as the cluster.\nIt is specified in the form: 'projects/{project_number}/global/networks/{network_id}'. This is required to create a cluster."]
    pub fn vpc_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_network", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_user` after provisioning.\n"]
    pub fn initial_user(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_InitialUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_user", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_instance_settings` after provisioning.\n"]
    pub fn primary_instance_settings(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings_PrimaryInstanceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_instance_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfile_AlloydbDynamic {
    settings: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Alloydb {
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings>>,
    dynamic: DatabaseMigrationServiceConnectionProfile_AlloydbDynamic,
}

impl DatabaseMigrationServiceConnectionProfile_Alloydb {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Alloydb_Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Alloydb {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Alloydb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Alloydb {
    #[doc = "Required. The AlloyDB cluster ID that this connection profile is associated with."]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceConnectionProfile_Alloydb {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Alloydb {
        DatabaseMigrationServiceConnectionProfile_Alloydb {
            cluster_id: self.cluster_id,
            settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_AlloydbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_AlloydbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_AlloydbRef {
        DatabaseMigrationServiceConnectionProfile_AlloydbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_AlloydbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nRequired. The AlloyDB cluster ID that this connection profile is associated with."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Alloydb_SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks {
    #[doc = "Set the field `expire_time`.\nThe time when this access control entry expires in RFC 3339 format."]
    pub fn set_expire_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expire_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `label`.\nA label to identify this entry."]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\nInput only. The time-to-leave of this access control entry."]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks {
    type O =
        terrars
        ::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks {
    #[doc = "The allowlisted value for the access control list."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks {
        DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks {
            expire_time: core::default::Default::default(),
            label: core::default::Default::default(),
            ttl: core::default::Default::default(),
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworksRef {
        DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nThe time when this access control entry expires in RFC 3339 format."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\nA label to identify this entry."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nInput only. The time-to-leave of this access control entry."]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe allowlisted value for the access control list."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfigDynamic {
    authorized_networks: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks>,
    >,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ipv4: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_networks: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks>>,
    dynamic: DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfigDynamic,
}

impl DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig {
    #[doc = "Set the field `enable_ipv4`.\nWhether the instance should be assigned an IPv4 address or not."]
    pub fn set_enable_ipv4(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ipv4 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_network`.\nThe resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default.\nThis setting can be updated, but it cannot be removed after it is set."]
    pub fn set_private_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_ssl`.\nWhether SSL connections over IP should be enforced or not."]
    pub fn set_require_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorized_networks`.\n"]
    pub fn set_authorized_networks(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworks,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorized_networks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorized_networks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig {}

impl BuildDatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig {
        DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig {
            enable_ipv4: core::default::Default::default(),
            private_network: core::default::Default::default(),
            require_ssl: core::default::Default::default(),
            authorized_networks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfigRef {
        DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_ipv4` after provisioning.\nWhether the instance should be assigned an IPv4 address or not."]
    pub fn enable_ipv4(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ipv4", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_network` after provisioning.\nThe resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default.\nThis setting can be updated, but it cannot be removed after it is set."]
    pub fn private_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_ssl` after provisioning.\nWhether SSL connections over IP should be enforced or not."]
    pub fn require_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `authorized_networks` after provisioning.\n"]
    pub fn authorized_networks(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig_AuthorizedNetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_networks", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfile_Cloudsql_SettingsDynamic {
    ip_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activation_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_storage_increase: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cmek_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_disk_size_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_flags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_password: std::option::Option<terrars::PrimField<std::string::String>>,
    source_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_auto_resize_limit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_config: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig>>,
    dynamic: DatabaseMigrationServiceConnectionProfile_Cloudsql_SettingsDynamic,
}

impl DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings {
    #[doc =
        "Set the field `activation_policy`.\nThe activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'. Possible values: [\"ALWAYS\", \"NEVER\"]"]
    pub fn set_activation_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.activation_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_storage_increase`.\nIf you enable this setting, Cloud SQL checks your available storage every 30 seconds. If the available storage falls below a threshold size, Cloud SQL automatically adds additional storage capacity.\nIf the available storage repeatedly falls below the threshold size, Cloud SQL continues to add storage until it reaches the maximum of 30 TB."]
    pub fn set_auto_storage_increase(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_storage_increase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cmek_key_name`.\nThe KMS key name used for the csql instance."]
    pub fn set_cmek_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cmek_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collation`.\nThe Cloud SQL default instance level collation."]
    pub fn set_collation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_disk_size_gb`.\nThe storage capacity available to the database, in GB. The minimum (and default) size is 10GB."]
    pub fn set_data_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_disk_type`.\nThe type of storage. Possible values: [\"PD_SSD\", \"PD_HDD\"]"]
    pub fn set_data_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_flags`.\nThe database flags passed to the Cloud SQL instance at startup."]
    pub fn set_database_flags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.database_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `database_version`.\nThe database engine type and version.\nCurrently supported values located at https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.connectionProfiles#sqldatabaseversion"]
    pub fn set_database_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `edition`.\nThe edition of the given Cloud SQL instance. Possible values: [\"ENTERPRISE\", \"ENTERPRISE_PLUS\"]"]
    pub fn set_edition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.edition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_password`.\nInput only. Initial root password."]
    pub fn set_root_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_auto_resize_limit`.\nThe maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit."]
    pub fn set_storage_auto_resize_limit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.storage_auto_resize_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tier`.\nThe tier (or machine type) for this instance, for example: db-n1-standard-1 (MySQL instances) or db-custom-1-3840 (PostgreSQL instances).\nFor more information, see https://cloud.google.com/sql/docs/mysql/instance-settings"]
    pub fn set_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tier = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_labels`.\nThe resource labels for a Cloud SQL instance to use to annotate any related underlying resources such as Compute Engine VMs."]
    pub fn set_user_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nThe Google Cloud Platform zone where your Cloud SQL datdabse instance is located."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_config`.\n"]
    pub fn set_ip_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Cloudsql_Settings {
    #[doc =
        "The Database Migration Service source connection profile ID, in the format: projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID"]
    pub source_id: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceConnectionProfile_Cloudsql_Settings {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings {
        DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings {
            activation_policy: core::default::Default::default(),
            auto_storage_increase: core::default::Default::default(),
            cmek_key_name: core::default::Default::default(),
            collation: core::default::Default::default(),
            data_disk_size_gb: core::default::Default::default(),
            data_disk_type: core::default::Default::default(),
            database_flags: core::default::Default::default(),
            database_version: core::default::Default::default(),
            edition: core::default::Default::default(),
            root_password: core::default::Default::default(),
            source_id: self.source_id,
            storage_auto_resize_limit: core::default::Default::default(),
            tier: core::default::Default::default(),
            user_labels: core::default::Default::default(),
            zone: core::default::Default::default(),
            ip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Cloudsql_SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Cloudsql_SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Cloudsql_SettingsRef {
        DatabaseMigrationServiceConnectionProfile_Cloudsql_SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Cloudsql_SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `activation_policy` after provisioning.\nThe activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'. Possible values: [\"ALWAYS\", \"NEVER\"]"]
    pub fn activation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auto_storage_increase` after provisioning.\nIf you enable this setting, Cloud SQL checks your available storage every 30 seconds. If the available storage falls below a threshold size, Cloud SQL automatically adds additional storage capacity.\nIf the available storage repeatedly falls below the threshold size, Cloud SQL continues to add storage until it reaches the maximum of 30 TB."]
    pub fn auto_storage_increase(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_storage_increase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cmek_key_name` after provisioning.\nThe KMS key name used for the csql instance."]
    pub fn cmek_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cmek_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `collation` after provisioning.\nThe Cloud SQL default instance level collation."]
    pub fn collation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_disk_size_gb` after provisioning.\nThe storage capacity available to the database, in GB. The minimum (and default) size is 10GB."]
    pub fn data_disk_size_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_disk_type` after provisioning.\nThe type of storage. Possible values: [\"PD_SSD\", \"PD_HDD\"]"]
    pub fn data_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_flags` after provisioning.\nThe database flags passed to the Cloud SQL instance at startup."]
    pub fn database_flags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.database_flags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_version` after provisioning.\nThe database engine type and version.\nCurrently supported values located at https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.connectionProfiles#sqldatabaseversion"]
    pub fn database_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `edition` after provisioning.\nThe edition of the given Cloud SQL instance. Possible values: [\"ENTERPRISE\", \"ENTERPRISE_PLUS\"]"]
    pub fn edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `root_password` after provisioning.\nInput only. Initial root password."]
    pub fn root_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `root_password_set` after provisioning.\nOutput only. Indicates If this connection profile root password is stored."]
    pub fn root_password_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_password_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_id` after provisioning.\nThe Database Migration Service source connection profile ID, in the format: projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID"]
    pub fn source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_auto_resize_limit` after provisioning.\nThe maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit."]
    pub fn storage_auto_resize_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_auto_resize_limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nThe tier (or machine type) for this instance, for example: db-n1-standard-1 (MySQL instances) or db-custom-1-3840 (PostgreSQL instances).\nFor more information, see https://cloud.google.com/sql/docs/mysql/instance-settings"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_labels` after provisioning.\nThe resource labels for a Cloud SQL instance to use to annotate any related underlying resources such as Compute Engine VMs."]
    pub fn user_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe Google Cloud Platform zone where your Cloud SQL datdabse instance is located."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_config` after provisioning.\n"]
    pub fn ip_config(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings_IpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfile_CloudsqlDynamic {
    settings: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Cloudsql {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings>>,
    dynamic: DatabaseMigrationServiceConnectionProfile_CloudsqlDynamic,
}

impl DatabaseMigrationServiceConnectionProfile_Cloudsql {
    #[doc = "Set the field `settings`.\n"]
    pub fn set_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Cloudsql_Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Cloudsql {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Cloudsql>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Cloudsql {}

impl BuildDatabaseMigrationServiceConnectionProfile_Cloudsql {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Cloudsql {
        DatabaseMigrationServiceConnectionProfile_Cloudsql {
            settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_CloudsqlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_CloudsqlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_CloudsqlRef {
        DatabaseMigrationServiceConnectionProfile_CloudsqlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_CloudsqlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloud_sql_id` after provisioning.\nOutput only. The Cloud SQL instance ID that this connection profile is associated with."]
    pub fn cloud_sql_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_ip` after provisioning.\nOutput only. The Cloud SQL database instance's private IP."]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_ip` after provisioning.\nOutput only. The Cloud SQL database instance's public IP."]
    pub fn public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Cloudsql_SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Mysql_Ssl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServiceConnectionProfile_Mysql_Ssl {
    #[doc =
        "Set the field `ca_certificate`.\nInput only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.\nThe replica will use this certificate to verify it's connecting to the right host."]
    pub fn set_ca_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_certificate`.\nInput only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.\nIf this field is used then the 'clientKey' field is mandatory"]
    pub fn set_client_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_key`.\nInput only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'clientCertificate' field is mandatory."]
    pub fn set_client_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe current connection profile state. Possible values: [\"SERVER_ONLY\", \"SERVER_CLIENT\", \"REQUIRED\", \"NONE\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Mysql_Ssl {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Mysql_Ssl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Mysql_Ssl {}

impl BuildDatabaseMigrationServiceConnectionProfile_Mysql_Ssl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Mysql_Ssl {
        DatabaseMigrationServiceConnectionProfile_Mysql_Ssl {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Mysql_SslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Mysql_SslRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Mysql_SslRef {
        DatabaseMigrationServiceConnectionProfile_Mysql_SslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Mysql_SslRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate` after provisioning.\nInput only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.\nThe replica will use this certificate to verify it's connecting to the right host."]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_certificate` after provisioning.\nInput only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.\nIf this field is used then the 'clientKey' field is mandatory"]
    pub fn client_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_key` after provisioning.\nInput only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'clientCertificate' field is mandatory."]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe current connection profile state. Possible values: [\"SERVER_ONLY\", \"SERVER_CLIENT\", \"REQUIRED\", \"NONE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfile_MysqlDynamic {
    ssl: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Mysql_Ssl>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Mysql {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Mysql_Ssl>>,
    dynamic: DatabaseMigrationServiceConnectionProfile_MysqlDynamic,
}

impl DatabaseMigrationServiceConnectionProfile_Mysql {
    #[doc =
        "Set the field `cloud_sql_id`.\nIf the connection profile is a Cloud SQL database, use this field to provide the Cloud SQL instance ID."]
    pub fn set_cloud_sql_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cloud_sql_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nThe IP or hostname of the source MySQL database."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe network port of the source MySQL database."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nThe username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Mysql_Ssl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Mysql {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Mysql>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Mysql {}

impl BuildDatabaseMigrationServiceConnectionProfile_Mysql {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Mysql {
        DatabaseMigrationServiceConnectionProfile_Mysql {
            cloud_sql_id: core::default::Default::default(),
            host: core::default::Default::default(),
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            username: core::default::Default::default(),
            ssl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_MysqlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_MysqlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_MysqlRef {
        DatabaseMigrationServiceConnectionProfile_MysqlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_MysqlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloud_sql_id` after provisioning.\nIf the connection profile is a Cloud SQL database, use this field to provide the Cloud SQL instance ID."]
    pub fn cloud_sql_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe IP or hostname of the source MySQL database."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_set` after provisioning.\nOutput only. Indicates If this connection profile password is stored."]
    pub fn password_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe network port of the source MySQL database."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Mysql_SslRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity {
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
}

impl DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity {
    #[doc =
        "Set the field `password`.\nInput only. SSH password. Only one of 'password' and 'private_key' can be configured."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_key`.\nInput only. SSH private key. Only one of 'password' and 'private_key' can be configured."]
    pub fn set_private_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity {
    #[doc = "Required. Hostname for the SSH tunnel."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Port for the SSH tunnel, default value is 22."]
    pub port: terrars::PrimField<f64>,
    #[doc = "Required. Username for the SSH tunnel."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity {
        DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity {
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: self.port,
            private_key: core::default::Default::default(),
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivityRef {
        DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nRequired. Hostname for the SSH tunnel."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. SSH password. Only one of 'password' and 'private_key' can be configured."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort for the SSH tunnel, default value is 22."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key` after provisioning.\nInput only. SSH private key. Only one of 'password' and 'private_key' can be configured."]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nRequired. Username for the SSH tunnel."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity {
    private_connection: terrars::PrimField<std::string::String>,
}

impl DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity { }

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity {
    #[doc = "Required. The resource name (URI) of the private connection."]
    pub private_connection: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity {
        DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity {
            private_connection: self.private_connection,
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivityRef {
        DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `private_connection` after provisioning.\nRequired. The resource name (URI) of the private connection."]
    pub fn private_connection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_connection", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Oracle_Ssl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServiceConnectionProfile_Oracle_Ssl {
    #[doc =
        "Set the field `ca_certificate`.\nInput only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.\nThe replica will use this certificate to verify it's connecting to the right host."]
    pub fn set_ca_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_certificate`.\nInput only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.\nIf this field is used then the 'clientKey' field is mandatory"]
    pub fn set_client_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_key`.\nInput only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'clientCertificate' field is mandatory."]
    pub fn set_client_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Oracle_Ssl {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Oracle_Ssl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Oracle_Ssl {}

impl BuildDatabaseMigrationServiceConnectionProfile_Oracle_Ssl {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Oracle_Ssl {
        DatabaseMigrationServiceConnectionProfile_Oracle_Ssl {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Oracle_SslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Oracle_SslRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Oracle_SslRef {
        DatabaseMigrationServiceConnectionProfile_Oracle_SslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Oracle_SslRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate` after provisioning.\nInput only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate.\nThe replica will use this certificate to verify it's connecting to the right host."]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_certificate` after provisioning.\nInput only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.\nIf this field is used then the 'clientKey' field is mandatory"]
    pub fn client_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_key` after provisioning.\nInput only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'clientCertificate' field is mandatory."]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe current connection profile state."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity {}

impl DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity { }

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity {}

impl BuildDatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity {
        DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity {}
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivityRef {
        DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfile_OracleDynamic {
    forward_ssh_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity>>,
    private_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity>>,
    ssl: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Oracle_Ssl>>,
    static_service_ip_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Oracle {
    database_service: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    password: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_ssh_connectivity: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_connectivity: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Oracle_Ssl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_service_ip_connectivity: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity>>,
    dynamic: DatabaseMigrationServiceConnectionProfile_OracleDynamic,
}

impl DatabaseMigrationServiceConnectionProfile_Oracle {
    #[doc = "Set the field `forward_ssh_connectivity`.\n"]
    pub fn set_forward_ssh_connectivity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivity>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forward_ssh_connectivity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forward_ssh_connectivity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_connectivity`.\n"]
    pub fn set_private_connectivity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_connectivity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_connectivity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Oracle_Ssl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `static_service_ip_connectivity`.\n"]
    pub fn set_static_service_ip_connectivity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivity,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.static_service_ip_connectivity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.static_service_ip_connectivity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Oracle {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Oracle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Oracle {
    #[doc = "Required. Database service for the Oracle connection."]
    pub database_service: terrars::PrimField<std::string::String>,
    #[doc = "Required. The IP or hostname of the source Oracle database."]
    pub host: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. Input only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = "Required. The network port of the source Oracle database."]
    pub port: terrars::PrimField<f64>,
    #[doc =
        "Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServiceConnectionProfile_Oracle {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Oracle {
        DatabaseMigrationServiceConnectionProfile_Oracle {
            database_service: self.database_service,
            host: self.host,
            password: self.password,
            port: self.port,
            username: self.username,
            forward_ssh_connectivity: core::default::Default::default(),
            private_connectivity: core::default::Default::default(),
            ssl: core::default::Default::default(),
            static_service_ip_connectivity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_OracleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_OracleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_OracleRef {
        DatabaseMigrationServiceConnectionProfile_OracleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_OracleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database_service` after provisioning.\nRequired. Database service for the Oracle connection."]
    pub fn database_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nRequired. The IP or hostname of the source Oracle database."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nRequired. Input only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_set` after provisioning.\nOutput only. Indicates If this connection profile password is stored."]
    pub fn password_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nRequired. The network port of the source Oracle database."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nRequired. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `forward_ssh_connectivity` after provisioning.\n"]
    pub fn forward_ssh_connectivity(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Oracle_ForwardSshConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_ssh_connectivity", self.base))
    }

    #[doc = "Get a reference to the value of field `private_connectivity` after provisioning.\n"]
    pub fn private_connectivity(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_connectivity", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Oracle_SslRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `static_service_ip_connectivity` after provisioning.\n"]
    pub fn static_service_ip_connectivity(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Oracle_StaticServiceIpConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_service_ip_connectivity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServiceConnectionProfile_PostgresqlDynamic {
    private_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity>>,
    ssl: std::option::Option<terrars::DynamicBlock<DatabaseMigrationServiceConnectionProfile_Mysql_Ssl>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Postgresql {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alloydb_cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_connectivity: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<std::vec::Vec<DatabaseMigrationServiceConnectionProfile_Mysql_Ssl>>,
    dynamic: DatabaseMigrationServiceConnectionProfile_PostgresqlDynamic,
}

impl DatabaseMigrationServiceConnectionProfile_Postgresql {
    #[doc =
        "Set the field `alloydb_cluster_id`.\nIf the connection profile is an AlloyDB instance, use this field to provide the AlloyDB cluster ID."]
    pub fn set_alloydb_cluster_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.alloydb_cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cloud_sql_id`.\nIf the connection profile is a Cloud SQL database, use this field to provide the Cloud SQL instance ID."]
    pub fn set_cloud_sql_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cloud_sql_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database`.\nThe name of the specific database within the host."]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nThe IP or hostname of the source MySQL database."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe network port of the source MySQL database."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nThe username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_connectivity`.\n"]
    pub fn set_private_connectivity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivity>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_connectivity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_connectivity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Mysql_Ssl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Postgresql {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Postgresql>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Postgresql {}

impl BuildDatabaseMigrationServiceConnectionProfile_Postgresql {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Postgresql {
        DatabaseMigrationServiceConnectionProfile_Postgresql {
            alloydb_cluster_id: core::default::Default::default(),
            cloud_sql_id: core::default::Default::default(),
            database: core::default::Default::default(),
            host: core::default::Default::default(),
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            username: core::default::Default::default(),
            private_connectivity: core::default::Default::default(),
            ssl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_PostgresqlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_PostgresqlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_PostgresqlRef {
        DatabaseMigrationServiceConnectionProfile_PostgresqlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_PostgresqlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alloydb_cluster_id` after provisioning.\nIf the connection profile is an AlloyDB instance, use this field to provide the AlloyDB cluster ID."]
    pub fn alloydb_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alloydb_cluster_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cloud_sql_id` after provisioning.\nIf the connection profile is a Cloud SQL database, use this field to provide the Cloud SQL instance ID."]
    pub fn cloud_sql_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe name of the specific database within the host."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe IP or hostname of the source MySQL database."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_architecture` after provisioning.\nOutput only. If the source is a Cloud SQL database, this field indicates the network architecture it's associated with."]
    pub fn network_architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_architecture", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password for the user that Database Migration Service will be using to connect to the database.\nThis field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_set` after provisioning.\nOutput only. Indicates If this connection profile password is stored."]
    pub fn password_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe network port of the source MySQL database."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `private_connectivity` after provisioning.\n"]
    pub fn private_connectivity(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Oracle_PrivateConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_connectivity", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> terrars::ListRef<DatabaseMigrationServiceConnectionProfile_Mysql_SslRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServiceConnectionProfile_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServiceConnectionProfile_Timeouts {
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

impl terrars::ToListMappable for DatabaseMigrationServiceConnectionProfile_Timeouts {
    type O = terrars::BlockAssignable<DatabaseMigrationServiceConnectionProfile_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServiceConnectionProfile_Timeouts {}

impl BuildDatabaseMigrationServiceConnectionProfile_Timeouts {
    pub fn build(self) -> DatabaseMigrationServiceConnectionProfile_Timeouts {
        DatabaseMigrationServiceConnectionProfile_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServiceConnectionProfile_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServiceConnectionProfile_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServiceConnectionProfile_TimeoutsRef {
        DatabaseMigrationServiceConnectionProfile_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServiceConnectionProfile_TimeoutsRef {
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
