use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DatastreamConnectionProfileData {
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
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfileBigqueryProfileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_ssh_connectivity: std::option::Option<std::vec::Vec<DatastreamConnectionProfileForwardSshConnectivityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfileGcsProfileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfileMysqlProfileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfileOracleProfileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_profile: std::option::Option<std::vec::Vec<DatastreamConnectionProfilePostgresqlProfileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_connectivity: std::option::Option<std::vec::Vec<DatastreamConnectionProfilePrivateConnectivityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DatastreamConnectionProfileTimeoutsEl>,
    dynamic: DatastreamConnectionProfileDynamic,
}

struct DatastreamConnectionProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatastreamConnectionProfileData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfileBigqueryProfileEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfileForwardSshConnectivityEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfileGcsProfileEl>>,
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

    #[doc = "Set the field `mysql_profile`.\n"]
    pub fn set_mysql_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfileMysqlProfileEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfileOracleProfileEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfilePostgresqlProfileEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfilePrivateConnectivityEl>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DatastreamConnectionProfileTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `connection_profile_id` after provisioning.\nThe connection profile identifier."]
    pub fn connection_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_profile_id", self.extract_ref()))
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
    pub fn bigquery_profile(&self) -> terrars::ListRef<DatastreamConnectionProfileBigqueryProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_ssh_connectivity` after provisioning.\n"]
    pub fn forward_ssh_connectivity(&self) -> terrars::ListRef<DatastreamConnectionProfileForwardSshConnectivityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_ssh_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcs_profile` after provisioning.\n"]
    pub fn gcs_profile(&self) -> terrars::ListRef<DatastreamConnectionProfileGcsProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql_profile` after provisioning.\n"]
    pub fn mysql_profile(&self) -> terrars::ListRef<DatastreamConnectionProfileMysqlProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oracle_profile` after provisioning.\n"]
    pub fn oracle_profile(&self) -> terrars::ListRef<DatastreamConnectionProfileOracleProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgresql_profile` after provisioning.\n"]
    pub fn postgresql_profile(&self) -> terrars::ListRef<DatastreamConnectionProfilePostgresqlProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_connectivity` after provisioning.\n"]
    pub fn private_connectivity(&self) -> terrars::ListRef<DatastreamConnectionProfilePrivateConnectivityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamConnectionProfileTimeoutsElRef {
        DatastreamConnectionProfileTimeoutsElRef::new(
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
            data: std::cell::RefCell::new(DatastreamConnectionProfileData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connection_profile_id: self.connection_profile_id,
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                bigquery_profile: core::default::Default::default(),
                forward_ssh_connectivity: core::default::Default::default(),
                gcs_profile: core::default::Default::default(),
                mysql_profile: core::default::Default::default(),
                oracle_profile: core::default::Default::default(),
                postgresql_profile: core::default::Default::default(),
                private_connectivity: core::default::Default::default(),
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
    pub fn bigquery_profile(&self) -> terrars::ListRef<DatastreamConnectionProfileBigqueryProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_ssh_connectivity` after provisioning.\n"]
    pub fn forward_ssh_connectivity(&self) -> terrars::ListRef<DatastreamConnectionProfileForwardSshConnectivityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_ssh_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gcs_profile` after provisioning.\n"]
    pub fn gcs_profile(&self) -> terrars::ListRef<DatastreamConnectionProfileGcsProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql_profile` after provisioning.\n"]
    pub fn mysql_profile(&self) -> terrars::ListRef<DatastreamConnectionProfileMysqlProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oracle_profile` after provisioning.\n"]
    pub fn oracle_profile(&self) -> terrars::ListRef<DatastreamConnectionProfileOracleProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postgresql_profile` after provisioning.\n"]
    pub fn postgresql_profile(&self) -> terrars::ListRef<DatastreamConnectionProfilePostgresqlProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_connectivity` after provisioning.\n"]
    pub fn private_connectivity(&self) -> terrars::ListRef<DatastreamConnectionProfilePrivateConnectivityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamConnectionProfileTimeoutsElRef {
        DatastreamConnectionProfileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfileBigqueryProfileEl {}

impl DatastreamConnectionProfileBigqueryProfileEl { }

impl terrars::ToListMappable for DatastreamConnectionProfileBigqueryProfileEl {
    type O = terrars::BlockAssignable<DatastreamConnectionProfileBigqueryProfileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfileBigqueryProfileEl {}

impl BuildDatastreamConnectionProfileBigqueryProfileEl {
    pub fn build(self) -> DatastreamConnectionProfileBigqueryProfileEl {
        DatastreamConnectionProfileBigqueryProfileEl {}
    }
}

pub struct DatastreamConnectionProfileBigqueryProfileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfileBigqueryProfileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfileBigqueryProfileElRef {
        DatastreamConnectionProfileBigqueryProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfileBigqueryProfileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfileForwardSshConnectivityEl {
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
}

impl DatastreamConnectionProfileForwardSshConnectivityEl {
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

impl terrars::ToListMappable for DatastreamConnectionProfileForwardSshConnectivityEl {
    type O = terrars::BlockAssignable<DatastreamConnectionProfileForwardSshConnectivityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfileForwardSshConnectivityEl {
    #[doc = "Hostname for the SSH tunnel."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Username for the SSH tunnel."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfileForwardSshConnectivityEl {
    pub fn build(self) -> DatastreamConnectionProfileForwardSshConnectivityEl {
        DatastreamConnectionProfileForwardSshConnectivityEl {
            hostname: self.hostname,
            password: core::default::Default::default(),
            port: core::default::Default::default(),
            private_key: core::default::Default::default(),
            username: self.username,
        }
    }
}

pub struct DatastreamConnectionProfileForwardSshConnectivityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfileForwardSshConnectivityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfileForwardSshConnectivityElRef {
        DatastreamConnectionProfileForwardSshConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfileForwardSshConnectivityElRef {
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
pub struct DatastreamConnectionProfileGcsProfileEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamConnectionProfileGcsProfileEl {
    #[doc = "Set the field `root_path`.\nThe root path inside the Cloud Storage bucket."]
    pub fn set_root_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfileGcsProfileEl {
    type O = terrars::BlockAssignable<DatastreamConnectionProfileGcsProfileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfileGcsProfileEl {
    #[doc = "The Cloud Storage bucket name."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfileGcsProfileEl {
    pub fn build(self) -> DatastreamConnectionProfileGcsProfileEl {
        DatastreamConnectionProfileGcsProfileEl {
            bucket: self.bucket,
            root_path: core::default::Default::default(),
        }
    }
}

pub struct DatastreamConnectionProfileGcsProfileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfileGcsProfileElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamConnectionProfileGcsProfileElRef {
        DatastreamConnectionProfileGcsProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfileGcsProfileElRef {
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
pub struct DatastreamConnectionProfileMysqlProfileElSslConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamConnectionProfileMysqlProfileElSslConfigEl {
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

impl terrars::ToListMappable for DatastreamConnectionProfileMysqlProfileElSslConfigEl {
    type O = terrars::BlockAssignable<DatastreamConnectionProfileMysqlProfileElSslConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfileMysqlProfileElSslConfigEl {}

impl BuildDatastreamConnectionProfileMysqlProfileElSslConfigEl {
    pub fn build(self) -> DatastreamConnectionProfileMysqlProfileElSslConfigEl {
        DatastreamConnectionProfileMysqlProfileElSslConfigEl {
            ca_certificate: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_key: core::default::Default::default(),
        }
    }
}

pub struct DatastreamConnectionProfileMysqlProfileElSslConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfileMysqlProfileElSslConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfileMysqlProfileElSslConfigElRef {
        DatastreamConnectionProfileMysqlProfileElSslConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfileMysqlProfileElSslConfigElRef {
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
struct DatastreamConnectionProfileMysqlProfileElDynamic {
    ssl_config: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfileMysqlProfileElSslConfigEl>>,
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfileMysqlProfileEl {
    hostname: terrars::PrimField<std::string::String>,
    password: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_config: std::option::Option<std::vec::Vec<DatastreamConnectionProfileMysqlProfileElSslConfigEl>>,
    dynamic: DatastreamConnectionProfileMysqlProfileElDynamic,
}

impl DatastreamConnectionProfileMysqlProfileEl {
    #[doc = "Set the field `port`.\nPort for the MySQL connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamConnectionProfileMysqlProfileElSslConfigEl>>,
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

impl terrars::ToListMappable for DatastreamConnectionProfileMysqlProfileEl {
    type O = terrars::BlockAssignable<DatastreamConnectionProfileMysqlProfileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfileMysqlProfileEl {
    #[doc = "Hostname for the MySQL connection."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Password for the MySQL connection."]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = "Username for the MySQL connection."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfileMysqlProfileEl {
    pub fn build(self) -> DatastreamConnectionProfileMysqlProfileEl {
        DatastreamConnectionProfileMysqlProfileEl {
            hostname: self.hostname,
            password: self.password,
            port: core::default::Default::default(),
            username: self.username,
            ssl_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamConnectionProfileMysqlProfileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfileMysqlProfileElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamConnectionProfileMysqlProfileElRef {
        DatastreamConnectionProfileMysqlProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfileMysqlProfileElRef {
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

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the MySQL connection."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<DatastreamConnectionProfileMysqlProfileElSslConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfileOracleProfileEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    database_service: terrars::PrimField<std::string::String>,
    hostname: terrars::PrimField<std::string::String>,
    password: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    username: terrars::PrimField<std::string::String>,
}

impl DatastreamConnectionProfileOracleProfileEl {
    #[doc = "Set the field `connection_attributes`.\nConnection string attributes"]
    pub fn set_connection_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.connection_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nPort for the Oracle connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfileOracleProfileEl {
    type O = terrars::BlockAssignable<DatastreamConnectionProfileOracleProfileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfileOracleProfileEl {
    #[doc = "Database for the Oracle connection."]
    pub database_service: terrars::PrimField<std::string::String>,
    #[doc = "Hostname for the Oracle connection."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Password for the Oracle connection."]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = "Username for the Oracle connection."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfileOracleProfileEl {
    pub fn build(self) -> DatastreamConnectionProfileOracleProfileEl {
        DatastreamConnectionProfileOracleProfileEl {
            connection_attributes: core::default::Default::default(),
            database_service: self.database_service,
            hostname: self.hostname,
            password: self.password,
            port: core::default::Default::default(),
            username: self.username,
        }
    }
}

pub struct DatastreamConnectionProfileOracleProfileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfileOracleProfileElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamConnectionProfileOracleProfileElRef {
        DatastreamConnectionProfileOracleProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfileOracleProfileElRef {
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

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the Oracle connection."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfilePostgresqlProfileEl {
    database: terrars::PrimField<std::string::String>,
    hostname: terrars::PrimField<std::string::String>,
    password: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    username: terrars::PrimField<std::string::String>,
}

impl DatastreamConnectionProfilePostgresqlProfileEl {
    #[doc = "Set the field `port`.\nPort for the PostgreSQL connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamConnectionProfilePostgresqlProfileEl {
    type O = terrars::BlockAssignable<DatastreamConnectionProfilePostgresqlProfileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfilePostgresqlProfileEl {
    #[doc = "Database for the PostgreSQL connection."]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = "Hostname for the PostgreSQL connection."]
    pub hostname: terrars::PrimField<std::string::String>,
    #[doc = "Password for the PostgreSQL connection."]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = "Username for the PostgreSQL connection."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfilePostgresqlProfileEl {
    pub fn build(self) -> DatastreamConnectionProfilePostgresqlProfileEl {
        DatastreamConnectionProfilePostgresqlProfileEl {
            database: self.database,
            hostname: self.hostname,
            password: self.password,
            port: core::default::Default::default(),
            username: self.username,
        }
    }
}

pub struct DatastreamConnectionProfilePostgresqlProfileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfilePostgresqlProfileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfilePostgresqlProfileElRef {
        DatastreamConnectionProfilePostgresqlProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfilePostgresqlProfileElRef {
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
        "Get a reference to the value of field `username` after provisioning.\nUsername for the PostgreSQL connection."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamConnectionProfilePrivateConnectivityEl {
    private_connection: terrars::PrimField<std::string::String>,
}

impl DatastreamConnectionProfilePrivateConnectivityEl { }

impl terrars::ToListMappable for DatastreamConnectionProfilePrivateConnectivityEl {
    type O = terrars::BlockAssignable<DatastreamConnectionProfilePrivateConnectivityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfilePrivateConnectivityEl {
    #[doc =
        "A reference to a private connection resource. Format: 'projects/{project}/locations/{location}/privateConnections/{name}'"]
    pub private_connection: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamConnectionProfilePrivateConnectivityEl {
    pub fn build(self) -> DatastreamConnectionProfilePrivateConnectivityEl {
        DatastreamConnectionProfilePrivateConnectivityEl { private_connection: self.private_connection }
    }
}

pub struct DatastreamConnectionProfilePrivateConnectivityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfilePrivateConnectivityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamConnectionProfilePrivateConnectivityElRef {
        DatastreamConnectionProfilePrivateConnectivityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfilePrivateConnectivityElRef {
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
pub struct DatastreamConnectionProfileTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamConnectionProfileTimeoutsEl {
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

impl terrars::ToListMappable for DatastreamConnectionProfileTimeoutsEl {
    type O = terrars::BlockAssignable<DatastreamConnectionProfileTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamConnectionProfileTimeoutsEl {}

impl BuildDatastreamConnectionProfileTimeoutsEl {
    pub fn build(self) -> DatastreamConnectionProfileTimeoutsEl {
        DatastreamConnectionProfileTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DatastreamConnectionProfileTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamConnectionProfileTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamConnectionProfileTimeoutsElRef {
        DatastreamConnectionProfileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamConnectionProfileTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct DatastreamConnectionProfileDynamic {
    bigquery_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfileBigqueryProfileEl>>,
    forward_ssh_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamConnectionProfileForwardSshConnectivityEl>>,
    gcs_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfileGcsProfileEl>>,
    mysql_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfileMysqlProfileEl>>,
    oracle_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfileOracleProfileEl>>,
    postgresql_profile: std::option::Option<terrars::DynamicBlock<DatastreamConnectionProfilePostgresqlProfileEl>>,
    private_connectivity: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamConnectionProfilePrivateConnectivityEl>>,
}
