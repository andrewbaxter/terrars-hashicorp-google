#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DatastreamConnectionProfileData_ {
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
    create_without_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_BigqueryProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_ssh_connectivity: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_ForwardSshConnectivity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_GcsProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mongodb_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_MongodbProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_MysqlProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_OracleProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_PostgresqlProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_connectivity: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_PrivateConnectivity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_server_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_SqlServerProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DatastreamConnectionProfile_Timeouts>,
    dynamic: DatastreamConnectionProfileDynamic,
}

struct DatastreamConnectionProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatastreamConnectionProfileData_>,
}

#[derive(Clone)]
pub struct DatastreamConnectionProfile(std::rc::Rc<DatastreamConnectionProfile_>);

impl DatastreamConnectionProfile {
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

    #[doc = "Set the field `create_without_validation`.\nCreate the connection profile without validating it."]
    pub fn set_create_without_validation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_without_validation = std::option::Option::Some(v.into());
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
        "Set the field `labels`.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_profile`.\n"]
    pub fn set_bigquery_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_BigqueryProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigquery_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigquery_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forward_ssh_connectivity`.\n"]
    pub fn set_forward_ssh_connectivity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_ForwardSshConnectivity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().forward_ssh_connectivity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.forward_ssh_connectivity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_profile`.\n"]
    pub fn set_gcs_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_GcsProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gcs_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gcs_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mongodb_profile`.\n"]
    pub fn set_mongodb_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_MongodbProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mongodb_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mongodb_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mysql_profile`.\n"]
    pub fn set_mysql_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_MysqlProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mysql_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mysql_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle_profile`.\n"]
    pub fn set_oracle_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_OracleProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oracle_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oracle_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postgresql_profile`.\n"]
    pub fn set_postgresql_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_PostgresqlProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().postgresql_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.postgresql_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_connectivity`.\n"]
    pub fn set_private_connectivity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_PrivateConnectivity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_connectivity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_connectivity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_server_profile`.\n"]
    pub fn set_sql_server_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_SqlServerProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sql_server_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sql_server_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DatastreamConnectionProfile_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `connection_profile_id` after provisioning.\nThe connection profile identifier."]
    pub fn connection_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_profile_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_without_validation` after provisioning.\nCreate the connection profile without validating it."]
    pub fn create_without_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_without_validation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this connection profile is located in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigquery_profile` after provisioning.\n"]
    pub fn bigquery_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_BigqueryProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_ssh_connectivity` after provisioning.\n"]
    pub fn forward_ssh_connectivity(&self) -> terrars::ListRef<DatastreamConnectionProfile_ForwardSshConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_ssh_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcs_profile` after provisioning.\n"]
    pub fn gcs_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_GcsProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mongodb_profile` after provisioning.\n"]
    pub fn mongodb_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_MongodbProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql_profile` after provisioning.\n"]
    pub fn mysql_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_MysqlProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oracle_profile` after provisioning.\n"]
    pub fn oracle_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_OracleProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgresql_profile` after provisioning.\n"]
    pub fn postgresql_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_PostgresqlProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_connectivity` after provisioning.\n"]
    pub fn private_connectivity(&self) -> terrars::ListRef<DatastreamConnectionProfile_PrivateConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sql_server_profile` after provisioning.\n"]
    pub fn sql_server_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_SqlServerProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamConnectionProfile_TimeoutsRef {
        DatastreamConnectionProfile_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DatastreamConnectionProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatastreamConnectionProfile { }

impl terrars::ToListMappable for DatastreamConnectionProfile {
    type O = terrars::ListRef<DatastreamConnectionProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatastreamConnectionProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_datastream_connection_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatastreamConnectionProfile {
    pub tf_id: std::string::String,
    #[doc = "The connection profile identifier."]
    pub connection_profile_id: terrars::PrimField<std::string::String>,
    #[doc = "Display name."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the location this connection profile is located in."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> DatastreamConnectionProfile {
        let out = DatastreamConnectionProfile(std::rc::Rc::new(DatastreamConnectionProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatastreamConnectionProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connection_profile_id: self.connection_profile_id,
                create_without_validation: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                bigquery_profile: core::default::Default::default(),
                forward_ssh_connectivity: core::default::Default::default(),
                gcs_profile: core::default::Default::default(),
                mongodb_profile: core::default::Default::default(),
                mysql_profile: core::default::Default::default(),
                oracle_profile: core::default::Default::default(),
                postgresql_profile: core::default::Default::default(),
                private_connectivity: core::default::Default::default(),
                sql_server_profile: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatastreamConnectionProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatastreamConnectionProfileRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_profile_id` after provisioning.\nThe connection profile identifier."]
    pub fn connection_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_profile_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_without_validation` after provisioning.\nCreate the connection profile without validating it."]
    pub fn create_without_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_without_validation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this connection profile is located in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigquery_profile` after provisioning.\n"]
    pub fn bigquery_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_BigqueryProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_ssh_connectivity` after provisioning.\n"]
    pub fn forward_ssh_connectivity(&self) -> terrars::ListRef<DatastreamConnectionProfile_ForwardSshConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_ssh_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcs_profile` after provisioning.\n"]
    pub fn gcs_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_GcsProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mongodb_profile` after provisioning.\n"]
    pub fn mongodb_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_MongodbProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql_profile` after provisioning.\n"]
    pub fn mysql_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_MysqlProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oracle_profile` after provisioning.\n"]
    pub fn oracle_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_OracleProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgresql_profile` after provisioning.\n"]
    pub fn postgresql_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_PostgresqlProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_connectivity` after provisioning.\n"]
    pub fn private_connectivity(&self) -> terrars::ListRef<DatastreamConnectionProfile_PrivateConnectivityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sql_server_profile` after provisioning.\n"]
    pub fn sql_server_profile(&self) -> terrars::ListRef<DatastreamConnectionProfile_SqlServerProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamConnectionProfile_TimeoutsRef {
        DatastreamConnectionProfile_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DatastreamConnectionProfileDynamic {
    bigquery_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfile_BigqueryProfile>>,
    forward_ssh_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamConnectionProfile_ForwardSshConnectivity>>,
    gcs_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfile_GcsProfile>>,
    mongodb_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfile_MongodbProfile>>,
    mysql_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfile_MysqlProfile>>,
    oracle_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfile_OracleProfile>>,
    postgresql_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfile_PostgresqlProfile>>,
    private_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamConnectionProfile_PrivateConnectivity>>,
    sql_server_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfile_SqlServerProfile>>,
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_BigqueryProfile {}

impl DatastreamConnectionProfile_BigqueryProfile { }

impl terrars::ToListMappable for DatastreamConnectionProfile_BigqueryProfile {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_BigqueryProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_BigqueryProfile {}

impl BuildDatastreamConnectionProfile_BigqueryProfile {
    pub fn build(self) -> DatastreamConnectionProfile_BigqueryProfile {
        DatastreamConnectionProfile_BigqueryProfile {}
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_BigqueryProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_BigqueryProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_BigqueryProfileRef {
        DatastreamConnectionProfile_BigqueryProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_BigqueryProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_ForwardSshConnectivity {
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
}

impl DatastreamConnectionProfile_ForwardSshConnectivity {
    #[doc = "Set the field `password`.\nSSH password."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nPort for the SSH tunnel."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\nSSH private key."]
    pub fn set_private_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_ForwardSshConnectivity {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_ForwardSshConnectivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_ForwardSshConnectivity {
    #[doc = "Hostname for the SSH tunnel."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Username for the SSH tunnel."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_ForwardSshConnectivity {
    pub fn build(self) -> DatastreamConnectionProfile_ForwardSshConnectivity {
        DatastreamConnectionProfile_ForwardSshConnectivity {
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            private_key: core::default::Default::default(),
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_ForwardSshConnectivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_ForwardSshConnectivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_ForwardSshConnectivityRef {
        DatastreamConnectionProfile_ForwardSshConnectivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_ForwardSshConnectivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname for the SSH tunnel."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nSSH password."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the SSH tunnel."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\nSSH private key."]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the SSH tunnel."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_GcsProfile {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamConnectionProfile_GcsProfile {
    #[doc = "Set the field `root_path`.\nThe root path inside the Cloud Storage bucket."]
    pub fn set_root_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_GcsProfile {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_GcsProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_GcsProfile {
    #[doc = "The Cloud Storage bucket name."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_GcsProfile {
    pub fn build(self) -> DatastreamConnectionProfile_GcsProfile {
        DatastreamConnectionProfile_GcsProfile {
            bucket: self.bucket,
            root_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_GcsProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_GcsProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamConnectionProfile_GcsProfileRef {
        DatastreamConnectionProfile_GcsProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_GcsProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe Cloud Storage bucket name."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `root_path` after provisioning.\nThe root path inside the Cloud Storage bucket."]
    pub fn root_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_path", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_MongodbProfile_HostAddresses {
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DatastreamConnectionProfile_MongodbProfile_HostAddresses {
    #[doc = "Set the field `port`.\nPort for the connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_MongodbProfile_HostAddresses {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_MongodbProfile_HostAddresses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_MongodbProfile_HostAddresses {
    #[doc = "Hostname for the connection."]
    pub hostname: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_MongodbProfile_HostAddresses {
    pub fn build(self) -> DatastreamConnectionProfile_MongodbProfile_HostAddresses {
        DatastreamConnectionProfile_MongodbProfile_HostAddresses {
            hostname: self.hostname,
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_MongodbProfile_HostAddressesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_MongodbProfile_HostAddressesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_MongodbProfile_HostAddressesRef {
        DatastreamConnectionProfile_MongodbProfile_HostAddressesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_MongodbProfile_HostAddressesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname for the connection."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the connection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat {}

impl DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat { }

impl terrars::ToListMappable for DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat {}

impl BuildDatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat {
    pub fn build(self) -> DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat {
        DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat {}
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormatRef {
        DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_MongodbProfile_SslConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager_stored_client_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamConnectionProfile_MongodbProfile_SslConfig {
    #[doc =
        "Set the field `ca_certificate`.\nPEM-encoded certificate of the CA that signed the source database\nserver's certificate."]
    pub fn set_ca_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_certificate`.\nPEM-encoded certificate that will be used by the replica to\nauthenticate against the source database server. If this field\nis used then the 'clientKey' and the 'caCertificate' fields are\nmandatory."]
    pub fn set_client_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_key`.\nPEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'client_certificate' and the\n'ca_certificate' fields are mandatory."]
    pub fn set_client_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_manager_stored_client_key`.\nA reference to a Secret Manager resource name storing the\nPEM-encoded private key. Mutually exclusive with clientKey."]
    pub fn set_secret_manager_stored_client_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_manager_stored_client_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_MongodbProfile_SslConfig {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_MongodbProfile_SslConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_MongodbProfile_SslConfig {}

impl BuildDatastreamConnectionProfile_MongodbProfile_SslConfig {
    pub fn build(self) -> DatastreamConnectionProfile_MongodbProfile_SslConfig {
        DatastreamConnectionProfile_MongodbProfile_SslConfig {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
            secret_manager_stored_client_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_MongodbProfile_SslConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_MongodbProfile_SslConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_MongodbProfile_SslConfigRef {
        DatastreamConnectionProfile_MongodbProfile_SslConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_MongodbProfile_SslConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM-encoded certificate of the CA that signed the source database\nserver's certificate."]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate_set` after provisioning.\nIndicates whether the clientKey field is set."]
    pub fn ca_certificate_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_certificate` after provisioning.\nPEM-encoded certificate that will be used by the replica to\nauthenticate against the source database server. If this field\nis used then the 'clientKey' and the 'caCertificate' fields are\nmandatory."]
    pub fn client_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_certificate_set` after provisioning.\nIndicates whether the clientCertificate field is set."]
    pub fn client_certificate_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_key` after provisioning.\nPEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'client_certificate' and the\n'ca_certificate' fields are mandatory."]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_key_set` after provisioning.\nIndicates whether the clientKey field is set."]
    pub fn client_key_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_manager_stored_client_key` after provisioning.\nA reference to a Secret Manager resource name storing the\nPEM-encoded private key. Mutually exclusive with clientKey."]
    pub fn secret_manager_stored_client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_manager_stored_client_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_connection: std::option::Option<terrars::PrimField<bool>>,
}

impl DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat {
    #[doc =
        "Set the field `direct_connection`.\nSpecifies whether the client connects directly to the\nhost[:port] in the connection URI."]
    pub fn set_direct_connection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.direct_connection = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat {}

impl BuildDatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat {
    pub fn build(self) -> DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat {
        DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat {
            direct_connection: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormatRef {
        DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `direct_connection` after provisioning.\nSpecifies whether the client connects directly to the\nhost[:port] in the connection URI."]
    pub fn direct_connection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direct_connection", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamConnectionProfile_MongodbProfileDynamic {
    host_addresses: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamConnectionProfile_MongodbProfile_HostAddresses>>,
    srv_connection_format: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat>>,
    ssl_config: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfile_MongodbProfile_SslConfig>>,
    standard_connection_format: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat>>,
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_MongodbProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_set: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager_stored_password: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_addresses: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_MongodbProfile_HostAddresses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    srv_connection_format: std
    ::option
    ::Option<std::vec::Vec<DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_config: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_MongodbProfile_SslConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_connection_format: std
    ::option
    ::Option<std::vec::Vec<DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat>>,
    dynamic: DatastreamConnectionProfile_MongodbProfileDynamic,
}

impl DatastreamConnectionProfile_MongodbProfile {
    #[doc =
        "Set the field `additional_options`.\nA map of additional options for the MongoDB connection.\nKeys are case-sensitive and should match the official\nMongoDB connection string options: https://www.mongodb.com/docs/manual/reference/connection-string-options/"]
    pub fn set_additional_options(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.additional_options = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nPassword for the MongoDB connection. Mutually exclusive with\nsecretManagerStoredPassword."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_set`.\nName of the replica set."]
    pub fn set_replica_set(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.replica_set = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_manager_stored_password`.\nA reference to a Secret Manager resource name storing the MongoDB\nconnection password. Mutually exclusive with password."]
    pub fn set_secret_manager_stored_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_manager_stored_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_addresses`.\n"]
    pub fn set_host_addresses(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamConnectionProfile_MongodbProfile_HostAddresses>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.host_addresses = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.host_addresses = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `srv_connection_format`.\n"]
    pub fn set_srv_connection_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormat>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.srv_connection_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.srv_connection_format = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_MongodbProfile_SslConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssl_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `standard_connection_format`.\n"]
    pub fn set_standard_connection_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormat>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.standard_connection_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.standard_connection_format = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_MongodbProfile {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_MongodbProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_MongodbProfile {
    #[doc = "Username for the MongoDB connection."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_MongodbProfile {
    pub fn build(self) -> DatastreamConnectionProfile_MongodbProfile {
        DatastreamConnectionProfile_MongodbProfile {
            additional_options: core::default::Default::default(),
            password: core::default::Default::default(),
            replica_set: core::default::Default::default(),
            secret_manager_stored_password: core::default::Default::default(),
            username: self.username,
            host_addresses: core::default::Default::default(),
            srv_connection_format: core::default::Default::default(),
            ssl_config: core::default::Default::default(),
            standard_connection_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_MongodbProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_MongodbProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamConnectionProfile_MongodbProfileRef {
        DatastreamConnectionProfile_MongodbProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_MongodbProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_options` after provisioning.\nA map of additional options for the MongoDB connection.\nKeys are case-sensitive and should match the official\nMongoDB connection string options: https://www.mongodb.com/docs/manual/reference/connection-string-options/"]
    pub fn additional_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.additional_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword for the MongoDB connection. Mutually exclusive with\nsecretManagerStoredPassword."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_set` after provisioning.\nName of the replica set."]
    pub fn replica_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_manager_stored_password` after provisioning.\nA reference to a Secret Manager resource name storing the MongoDB\nconnection password. Mutually exclusive with password."]
    pub fn secret_manager_stored_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_manager_stored_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nUsername for the MongoDB connection."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `host_addresses` after provisioning.\n"]
    pub fn host_addresses(&self) -> terrars::ListRef<DatastreamConnectionProfile_MongodbProfile_HostAddressesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `srv_connection_format` after provisioning.\n"]
    pub fn srv_connection_format(
        &self,
    ) -> terrars::ListRef<DatastreamConnectionProfile_MongodbProfile_SrvConnectionFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.srv_connection_format", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<DatastreamConnectionProfile_MongodbProfile_SslConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }

    #[doc = "Get a reference to the value of field `standard_connection_format` after provisioning.\n"]
    pub fn standard_connection_format(
        &self,
    ) -> terrars::ListRef<DatastreamConnectionProfile_MongodbProfile_StandardConnectionFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_connection_format", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_MysqlProfile_SslConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamConnectionProfile_MysqlProfile_SslConfig {
    #[doc =
        "Set the field `ca_certificate`.\nPEM-encoded certificate of the CA that signed the source database\nserver's certificate."]
    pub fn set_ca_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_certificate`.\nPEM-encoded certificate that will be used by the replica to\nauthenticate against the source database server. If this field\nis used then the 'clientKey' and the 'caCertificate' fields are\nmandatory."]
    pub fn set_client_certificate(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_key`.\nPEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'client_certificate' and the\n'ca_certificate' fields are mandatory."]
    pub fn set_client_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_MysqlProfile_SslConfig {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_MysqlProfile_SslConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_MysqlProfile_SslConfig {}

impl BuildDatastreamConnectionProfile_MysqlProfile_SslConfig {
    pub fn build(self) -> DatastreamConnectionProfile_MysqlProfile_SslConfig {
        DatastreamConnectionProfile_MysqlProfile_SslConfig {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_MysqlProfile_SslConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_MysqlProfile_SslConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_MysqlProfile_SslConfigRef {
        DatastreamConnectionProfile_MysqlProfile_SslConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_MysqlProfile_SslConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM-encoded certificate of the CA that signed the source database\nserver's certificate."]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate_set` after provisioning.\nIndicates whether the clientKey field is set."]
    pub fn ca_certificate_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_certificate` after provisioning.\nPEM-encoded certificate that will be used by the replica to\nauthenticate against the source database server. If this field\nis used then the 'clientKey' and the 'caCertificate' fields are\nmandatory."]
    pub fn client_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_certificate_set` after provisioning.\nIndicates whether the clientCertificate field is set."]
    pub fn client_certificate_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_key` after provisioning.\nPEM-encoded private key associated with the Client Certificate.\nIf this field is used then the 'client_certificate' and the\n'ca_certificate' fields are mandatory."]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_key_set` after provisioning.\nIndicates whether the clientKey field is set."]
    pub fn client_key_set(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key_set", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamConnectionProfile_MysqlProfileDynamic {
    ssl_config: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfile_MysqlProfile_SslConfig>>,
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_MysqlProfile {
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager_stored_password: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_config: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_MysqlProfile_SslConfig>>,
    dynamic: DatastreamConnectionProfile_MysqlProfileDynamic,
}

impl DatastreamConnectionProfile_MysqlProfile {
    #[doc = "Set the field `password`.\nPassword for the MySQL connection."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nPort for the MySQL connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_manager_stored_password`.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn set_secret_manager_stored_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_manager_stored_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfile_MysqlProfile_SslConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssl_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_MysqlProfile {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_MysqlProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_MysqlProfile {
    #[doc = "Hostname for the MySQL connection."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Username for the MySQL connection."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_MysqlProfile {
    pub fn build(self) -> DatastreamConnectionProfile_MysqlProfile {
        DatastreamConnectionProfile_MysqlProfile {
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            secret_manager_stored_password: core::default::Default::default(),
            username: self.username,
            ssl_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_MysqlProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_MysqlProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamConnectionProfile_MysqlProfileRef {
        DatastreamConnectionProfile_MysqlProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_MysqlProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname for the MySQL connection."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for the MySQL connection."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the MySQL connection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_manager_stored_password` after provisioning.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn secret_manager_stored_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_manager_stored_password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the MySQL connection."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<DatastreamConnectionProfile_MysqlProfile_SslConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_OracleProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    database_service: terrars::PrimField<std::string::String>,
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager_stored_password: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
}

impl DatastreamConnectionProfile_OracleProfile {
    #[doc = "Set the field `connection_attributes`.\nConnection string attributes"]
    pub fn set_connection_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.connection_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\nPassword for the Oracle connection."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nPort for the Oracle connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_manager_stored_password`.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn set_secret_manager_stored_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_manager_stored_password = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_OracleProfile {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_OracleProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_OracleProfile {
    #[doc = "Database for the Oracle connection."]
    pub database_service: terrars::PrimField<std::string::String>,
    #[doc = "Hostname for the Oracle connection."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Username for the Oracle connection."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_OracleProfile {
    pub fn build(self) -> DatastreamConnectionProfile_OracleProfile {
        DatastreamConnectionProfile_OracleProfile {
            connection_attributes: core::default::Default::default(),
            database_service: self.database_service,
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            secret_manager_stored_password: core::default::Default::default(),
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_OracleProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_OracleProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamConnectionProfile_OracleProfileRef {
        DatastreamConnectionProfile_OracleProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_OracleProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_attributes` after provisioning.\nConnection string attributes"]
    pub fn connection_attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.connection_attributes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_service` after provisioning.\nDatabase for the Oracle connection."]
    pub fn database_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_service", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname for the Oracle connection."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for the Oracle connection."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the Oracle connection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_manager_stored_password` after provisioning.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn secret_manager_stored_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_manager_stored_password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the Oracle connection."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification {
    ca_certificate: terrars::PrimField<std::string::String>,
    client_certificate: terrars::PrimField<std::string::String>,
    client_key: terrars::PrimField<std::string::String>,
}

impl DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification { }

impl terrars::ToListMappable for DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification {
    type O =
        terrars
        ::BlockAssignable<DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification {
    #[doc = "PEM-encoded server root CA certificate."]
    pub ca_certificate: terrars::PrimField<std::string::String>,
    #[doc =
        "PEM-encoded certificate used by the source database to authenticate the\nclient identity (i.e., the Datastream's identity). This certificate is\nsigned by either a root certificate trusted by the server or one or more\nintermediate certificates (which is stored with the leaf certificate) to\nlink to this certificate to the trusted root certificate."]
    pub client_certificate: terrars::PrimField<std::string::String>,
    #[doc =
        "PEM-encoded private key associated with the client certificate.\nThis value will be used during the SSL/TLS handshake, allowing\nthe PostgreSQL server to authenticate the client's identity,\ni.e. identity of the stream."]
    pub client_key: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification {
    pub fn build(self) -> DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification {
        DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification {
            ca_certificate: self.ca_certificate,
            client_certificate: self.client_certificate,
            client_key: self.client_key,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerificationRef {
        DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM-encoded server root CA certificate."]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_certificate` after provisioning.\nPEM-encoded certificate used by the source database to authenticate the\nclient identity (i.e., the Datastream's identity). This certificate is\nsigned by either a root certificate trusted by the server or one or more\nintermediate certificates (which is stored with the leaf certificate) to\nlink to this certificate to the trusted root certificate."]
    pub fn client_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_key` after provisioning.\nPEM-encoded private key associated with the client certificate.\nThis value will be used during the SSL/TLS handshake, allowing\nthe PostgreSQL server to authenticate the client's identity,\ni.e. identity of the stream."]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification {
    ca_certificate: terrars::PrimField<std::string::String>,
}

impl DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification { }

impl terrars::ToListMappable for DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification {
    #[doc = "PEM-encoded server root CA certificate."]
    pub ca_certificate: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification {
    pub fn build(self) -> DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification {
        DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification {
            ca_certificate: self.ca_certificate,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerificationRef {
        DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate` after provisioning.\nPEM-encoded server root CA certificate."]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamConnectionProfile_PostgresqlProfile_SslConfigDynamic {
    server_and_client_verification: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification>,
    >,
    server_verification: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification>>,
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_PostgresqlProfile_SslConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_and_client_verification: std
    ::option
    ::Option<std::vec::Vec<DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_verification: std
    ::option
    ::Option<std::vec::Vec<DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification>>,
    dynamic: DatastreamConnectionProfile_PostgresqlProfile_SslConfigDynamic,
}

impl DatastreamConnectionProfile_PostgresqlProfile_SslConfig {
    #[doc = "Set the field `server_and_client_verification`.\n"]
    pub fn set_server_and_client_verification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerification,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.server_and_client_verification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.server_and_client_verification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_verification`.\n"]
    pub fn set_server_verification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.server_verification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.server_verification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_PostgresqlProfile_SslConfig {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_PostgresqlProfile_SslConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_PostgresqlProfile_SslConfig {}

impl BuildDatastreamConnectionProfile_PostgresqlProfile_SslConfig {
    pub fn build(self) -> DatastreamConnectionProfile_PostgresqlProfile_SslConfig {
        DatastreamConnectionProfile_PostgresqlProfile_SslConfig {
            server_and_client_verification: core::default::Default::default(),
            server_verification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_PostgresqlProfile_SslConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_PostgresqlProfile_SslConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_PostgresqlProfile_SslConfigRef {
        DatastreamConnectionProfile_PostgresqlProfile_SslConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_PostgresqlProfile_SslConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `server_and_client_verification` after provisioning.\n"]
    pub fn server_and_client_verification(
        &self,
    ) -> terrars::ListRef<DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerAndClientVerificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_and_client_verification", self.base))
    }

    #[doc = "Get a reference to the value of field `server_verification` after provisioning.\n"]
    pub fn server_verification(
        &self,
    ) -> terrars::ListRef<DatastreamConnectionProfile_PostgresqlProfile_SslConfig_ServerVerificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_verification", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamConnectionProfile_PostgresqlProfileDynamic {
    ssl_config: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfile_PostgresqlProfile_SslConfig>>,
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_PostgresqlProfile {
    database: terrars::PrimField<std::string::String>,
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager_stored_password: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_config: std::option::Option<std::vec::Vec<DatastreamConnectionProfile_PostgresqlProfile_SslConfig>>,
    dynamic: DatastreamConnectionProfile_PostgresqlProfileDynamic,
}

impl DatastreamConnectionProfile_PostgresqlProfile {
    #[doc = "Set the field `password`.\nPassword for the PostgreSQL connection."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nPort for the PostgreSQL connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_manager_stored_password`.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn set_secret_manager_stored_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_manager_stored_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamConnectionProfile_PostgresqlProfile_SslConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssl_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_PostgresqlProfile {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_PostgresqlProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_PostgresqlProfile {
    #[doc = "Database for the PostgreSQL connection."]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = "Hostname for the PostgreSQL connection."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Username for the PostgreSQL connection."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_PostgresqlProfile {
    pub fn build(self) -> DatastreamConnectionProfile_PostgresqlProfile {
        DatastreamConnectionProfile_PostgresqlProfile {
            database: self.database,
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            secret_manager_stored_password: core::default::Default::default(),
            username: self.username,
            ssl_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_PostgresqlProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_PostgresqlProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_PostgresqlProfileRef {
        DatastreamConnectionProfile_PostgresqlProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_PostgresqlProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nDatabase for the PostgreSQL connection."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostname for the PostgreSQL connection."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword for the PostgreSQL connection."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the PostgreSQL connection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_manager_stored_password` after provisioning.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn secret_manager_stored_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_manager_stored_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nUsername for the PostgreSQL connection."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<DatastreamConnectionProfile_PostgresqlProfile_SslConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_PrivateConnectivity {
    private_connection: terrars::PrimField<std::string::String>,
}

impl DatastreamConnectionProfile_PrivateConnectivity { }

impl terrars::ToListMappable for DatastreamConnectionProfile_PrivateConnectivity {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_PrivateConnectivity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_PrivateConnectivity {
    #[doc =
        "A reference to a private connection resource. Format: 'projects/{project}/locations/{location}/privateConnections/{name}'"]
    pub private_connection: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_PrivateConnectivity {
    pub fn build(self) -> DatastreamConnectionProfile_PrivateConnectivity {
        DatastreamConnectionProfile_PrivateConnectivity { private_connection: self.private_connection }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_PrivateConnectivityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_PrivateConnectivityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_PrivateConnectivityRef {
        DatastreamConnectionProfile_PrivateConnectivityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_PrivateConnectivityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `private_connection` after provisioning.\nA reference to a private connection resource. Format: 'projects/{project}/locations/{location}/privateConnections/{name}'"]
    pub fn private_connection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_connection", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_SqlServerProfile {
    database: terrars::PrimField<std::string::String>,
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_manager_stored_password: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
}

impl DatastreamConnectionProfile_SqlServerProfile {
    #[doc = "Set the field `password`.\nPassword for the SQL Server connection."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nPort for the SQL Server connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_manager_stored_password`.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn set_secret_manager_stored_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_manager_stored_password = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfile_SqlServerProfile {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_SqlServerProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_SqlServerProfile {
    #[doc = "Database for the SQL Server connection."]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = "Hostname for the SQL Server connection."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Username for the SQL Server connection."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfile_SqlServerProfile {
    pub fn build(self) -> DatastreamConnectionProfile_SqlServerProfile {
        DatastreamConnectionProfile_SqlServerProfile {
            database: self.database,
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            secret_manager_stored_password: core::default::Default::default(),
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_SqlServerProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_SqlServerProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfile_SqlServerProfileRef {
        DatastreamConnectionProfile_SqlServerProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_SqlServerProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nDatabase for the SQL Server connection."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostname for the SQL Server connection."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword for the SQL Server connection."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort for the SQL Server connection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_manager_stored_password` after provisioning.\nA reference to a Secret Manager resource name storing the user's password."]
    pub fn secret_manager_stored_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_manager_stored_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nUsername for the SQL Server connection."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfile_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamConnectionProfile_Timeouts {
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

impl terrars::ToListMappable for DatastreamConnectionProfile_Timeouts {
    type O = terrars::BlockAssignable<DatastreamConnectionProfile_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfile_Timeouts {}

impl BuildDatastreamConnectionProfile_Timeouts {
    pub fn build(self) -> DatastreamConnectionProfile_Timeouts {
        DatastreamConnectionProfile_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamConnectionProfile_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfile_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamConnectionProfile_TimeoutsRef {
        DatastreamConnectionProfile_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfile_TimeoutsRef {
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
