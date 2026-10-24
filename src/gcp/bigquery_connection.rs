use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryConnectionData {
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
    connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    friendly_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws: std::option::Option<std::vec::Vec<BigqueryConnectionAwsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure: std::option::Option<std::vec::Vec<BigqueryConnectionAzureEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_resource: std::option::Option<std::vec::Vec<BigqueryConnectionCloudResourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_spanner: std::option::Option<std::vec::Vec<BigqueryConnectionCloudSpannerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql: std::option::Option<std::vec::Vec<BigqueryConnectionCloudSqlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark: std::option::Option<std::vec::Vec<BigqueryConnectionSparkEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryConnectionTimeoutsEl>,
    dynamic: BigqueryConnectionDynamic,
}

struct BigqueryConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryConnectionData>,
}

#[derive(Clone)]
pub struct BigqueryConnection(std::rc::Rc<BigqueryConnection_>);

impl BigqueryConnection {
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

    #[doc = "Set the field `connection_id`.\nOptional connection id that should be assigned to the created connection."]
    pub fn set_connection_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA descriptive description for the connection"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `friendly_name`.\nA descriptive name for the connection"]
    pub fn set_friendly_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().friendly_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe geographic location where the connection should reside.\nCloud SQL instance must be in the same location as the connection\nwith following exceptions: Cloud SQL us-central1 maps to BigQuery US, Cloud SQL europe-west1 maps to BigQuery EU.\nExamples: US, EU, asia-northeast1, us-central1, europe-west1.\nSpanner Connections same as spanner region\nAWS allowed regions are aws-us-east-1\nAzure allowed regions are azure-eastus2"]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws`.\n"]
    pub fn set_aws(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnectionAwsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aws = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aws = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure`.\n"]
    pub fn set_azure(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnectionAzureEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().azure = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.azure = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_resource`.\n"]
    pub fn set_cloud_resource(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnectionCloudResourceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_spanner`.\n"]
    pub fn set_cloud_spanner(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnectionCloudSpannerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_spanner = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_spanner = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_sql`.\n"]
    pub fn set_cloud_sql(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnectionCloudSqlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_sql = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_sql = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark`.\n"]
    pub fn set_spark(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnectionSparkEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `connection_id` after provisioning.\nOptional connection id that should be assigned to the created connection."]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA descriptive description for the connection"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the connection"]
    pub fn friendly_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_credential` after provisioning.\nTrue if the connection has credential assigned."]
    pub fn has_credential(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the connection should reside.\nCloud SQL instance must be in the same location as the connection\nwith following exceptions: Cloud SQL us-central1 maps to BigQuery US, Cloud SQL europe-west1 maps to BigQuery EU.\nExamples: US, EU, asia-northeast1, us-central1, europe-west1.\nSpanner Connections same as spanner region\nAWS allowed regions are aws-us-east-1\nAzure allowed regions are azure-eastus2"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the connection in the form of:\n\"projects/{project_id}/locations/{location_id}/connections/{connectionId}\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws` after provisioning.\n"]
    pub fn aws(&self) -> terrars::ListRef<BigqueryConnectionAwsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> terrars::ListRef<BigqueryConnectionAzureElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_resource` after provisioning.\n"]
    pub fn cloud_resource(&self) -> terrars::ListRef<BigqueryConnectionCloudResourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_spanner` after provisioning.\n"]
    pub fn cloud_spanner(&self) -> terrars::ListRef<BigqueryConnectionCloudSpannerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_spanner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_sql` after provisioning.\n"]
    pub fn cloud_sql(&self) -> terrars::ListRef<BigqueryConnectionCloudSqlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_sql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> terrars::ListRef<BigqueryConnectionSparkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryConnectionTimeoutsElRef {
        BigqueryConnectionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BigqueryConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryConnection { }

impl terrars::ToListMappable for BigqueryConnection {
    type O = terrars::ListRef<BigqueryConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryConnection {
    pub tf_id: std::string::String,
}

impl BuildBigqueryConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryConnection {
        let out = BigqueryConnection(std::rc::Rc::new(BigqueryConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryConnectionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connection_id: core::default::Default::default(),
                description: core::default::Default::default(),
                friendly_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                aws: core::default::Default::default(),
                azure: core::default::Default::default(),
                cloud_resource: core::default::Default::default(),
                cloud_spanner: core::default::Default::default(),
                cloud_sql: core::default::Default::default(),
                spark: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryConnectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_id` after provisioning.\nOptional connection id that should be assigned to the created connection."]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA descriptive description for the connection"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the connection"]
    pub fn friendly_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `has_credential` after provisioning.\nTrue if the connection has credential assigned."]
    pub fn has_credential(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_credential", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the connection should reside.\nCloud SQL instance must be in the same location as the connection\nwith following exceptions: Cloud SQL us-central1 maps to BigQuery US, Cloud SQL europe-west1 maps to BigQuery EU.\nExamples: US, EU, asia-northeast1, us-central1, europe-west1.\nSpanner Connections same as spanner region\nAWS allowed regions are aws-us-east-1\nAzure allowed regions are azure-eastus2"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the connection in the form of:\n\"projects/{project_id}/locations/{location_id}/connections/{connectionId}\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws` after provisioning.\n"]
    pub fn aws(&self) -> terrars::ListRef<BigqueryConnectionAwsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> terrars::ListRef<BigqueryConnectionAzureElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_resource` after provisioning.\n"]
    pub fn cloud_resource(&self) -> terrars::ListRef<BigqueryConnectionCloudResourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_spanner` after provisioning.\n"]
    pub fn cloud_spanner(&self) -> terrars::ListRef<BigqueryConnectionCloudSpannerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_spanner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_sql` after provisioning.\n"]
    pub fn cloud_sql(&self) -> terrars::ListRef<BigqueryConnectionCloudSqlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_sql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> terrars::ListRef<BigqueryConnectionSparkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryConnectionTimeoutsElRef {
        BigqueryConnectionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionAwsElAccessRoleEl {
    iam_role_id: terrars::PrimField<std::string::String>,
}

impl BigqueryConnectionAwsElAccessRoleEl { }

impl terrars::ToListMappable for BigqueryConnectionAwsElAccessRoleEl {
    type O = terrars::BlockAssignable<BigqueryConnectionAwsElAccessRoleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionAwsElAccessRoleEl {
    #[doc = "The user’s AWS IAM Role that trusts the Google-owned AWS IAM user Connection."]
    pub iam_role_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnectionAwsElAccessRoleEl {
    pub fn build(self) -> BigqueryConnectionAwsElAccessRoleEl {
        BigqueryConnectionAwsElAccessRoleEl { iam_role_id: self.iam_role_id }
    }
}

pub struct BigqueryConnectionAwsElAccessRoleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionAwsElAccessRoleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnectionAwsElAccessRoleElRef {
        BigqueryConnectionAwsElAccessRoleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionAwsElAccessRoleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `iam_role_id` after provisioning.\nThe user’s AWS IAM Role that trusts the Google-owned AWS IAM user Connection."]
    pub fn iam_role_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity` after provisioning.\nA unique Google-owned and Google-generated identity for the Connection. This identity will be used to access the user's AWS IAM Role."]
    pub fn identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnectionAwsElDynamic {
    access_role: std::option::Option<terrars::DynamicBlock<BigqueryConnectionAwsElAccessRoleEl>>,
}

#[derive(Serialize)]
pub struct BigqueryConnectionAwsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_role: std::option::Option<std::vec::Vec<BigqueryConnectionAwsElAccessRoleEl>>,
    dynamic: BigqueryConnectionAwsElDynamic,
}

impl BigqueryConnectionAwsEl {
    #[doc = "Set the field `access_role`.\n"]
    pub fn set_access_role(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnectionAwsElAccessRoleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_role = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_role = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryConnectionAwsEl {
    type O = terrars::BlockAssignable<BigqueryConnectionAwsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionAwsEl {}

impl BuildBigqueryConnectionAwsEl {
    pub fn build(self) -> BigqueryConnectionAwsEl {
        BigqueryConnectionAwsEl {
            access_role: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BigqueryConnectionAwsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionAwsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnectionAwsElRef {
        BigqueryConnectionAwsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionAwsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_role` after provisioning.\n"]
    pub fn access_role(&self) -> terrars::ListRef<BigqueryConnectionAwsElAccessRoleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_role", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionAzureEl {
    customer_tenant_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    federated_application_client_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryConnectionAzureEl {
    #[doc =
        "Set the field `federated_application_client_id`.\nThe Azure Application (client) ID where the federated credentials will be hosted."]
    pub fn set_federated_application_client_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.federated_application_client_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryConnectionAzureEl {
    type O = terrars::BlockAssignable<BigqueryConnectionAzureEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionAzureEl {
    #[doc = "The id of customer's directory that host the data."]
    pub customer_tenant_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnectionAzureEl {
    pub fn build(self) -> BigqueryConnectionAzureEl {
        BigqueryConnectionAzureEl {
            customer_tenant_id: self.customer_tenant_id,
            federated_application_client_id: core::default::Default::default(),
        }
    }
}

pub struct BigqueryConnectionAzureElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionAzureElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnectionAzureElRef {
        BigqueryConnectionAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionAzureElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application` after provisioning.\nThe name of the Azure Active Directory Application."]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe client id of the Azure Active Directory Application."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `customer_tenant_id` after provisioning.\nThe id of customer's directory that host the data."]
    pub fn customer_tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_tenant_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `federated_application_client_id` after provisioning.\nThe Azure Application (client) ID where the federated credentials will be hosted."]
    pub fn federated_application_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.federated_application_client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity` after provisioning.\nA unique Google-owned and Google-generated identity for the Connection. This identity will be used to access the user's Azure Active Directory Application."]
    pub fn identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `object_id` after provisioning.\nThe object id of the Azure Active Directory Application."]
    pub fn object_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirect_uri` after provisioning.\nThe URL user will be redirected to after granting consent during connection setup."]
    pub fn redirect_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionCloudResourceEl {}

impl BigqueryConnectionCloudResourceEl { }

impl terrars::ToListMappable for BigqueryConnectionCloudResourceEl {
    type O = terrars::BlockAssignable<BigqueryConnectionCloudResourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionCloudResourceEl {}

impl BuildBigqueryConnectionCloudResourceEl {
    pub fn build(self) -> BigqueryConnectionCloudResourceEl {
        BigqueryConnectionCloudResourceEl {}
    }
}

pub struct BigqueryConnectionCloudResourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionCloudResourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnectionCloudResourceElRef {
        BigqueryConnectionCloudResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionCloudResourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account_id` after provisioning.\nThe account ID of the service created for the purpose of this connection."]
    pub fn service_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionCloudSpannerEl {
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_parallelism: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_data_boost: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_parallelism: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_serverless_analytics: std::option::Option<terrars::PrimField<bool>>,
}

impl BigqueryConnectionCloudSpannerEl {
    #[doc =
        "Set the field `database_role`.\nCloud Spanner database role for fine-grained access control. The Cloud Spanner admin should have provisioned the database role with appropriate permissions, such as 'SELECT' and 'INSERT'. Other users should only use roles provided by their Cloud Spanner admins. The database role name must start with a letter, and can only contain letters, numbers, and underscores. For more details, see https://cloud.google.com/spanner/docs/fgac-about."]
    pub fn set_database_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_role = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_parallelism`.\nAllows setting max parallelism per query when executing on Spanner independent compute resources. If unspecified, default values of parallelism are chosen that are dependent on the Cloud Spanner instance configuration. 'useParallelism' and 'useDataBoost' must be set when setting max parallelism."]
    pub fn set_max_parallelism(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_parallelism = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_data_boost`.\nIf set, the request will be executed via Spanner independent compute resources. 'use_parallelism' must be set when using data boost."]
    pub fn set_use_data_boost(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_data_boost = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_parallelism`.\nIf parallelism should be used when reading from Cloud Spanner."]
    pub fn set_use_parallelism(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_parallelism = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_serverless_analytics`.\nIf the serverless analytics service should be used to read data from Cloud Spanner. 'useParallelism' must be set when using serverless analytics."]
    pub fn set_use_serverless_analytics(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_serverless_analytics = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryConnectionCloudSpannerEl {
    type O = terrars::BlockAssignable<BigqueryConnectionCloudSpannerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionCloudSpannerEl {
    #[doc = "Cloud Spanner database in the form 'project/instance/database'."]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnectionCloudSpannerEl {
    pub fn build(self) -> BigqueryConnectionCloudSpannerEl {
        BigqueryConnectionCloudSpannerEl {
            database: self.database,
            database_role: core::default::Default::default(),
            max_parallelism: core::default::Default::default(),
            use_data_boost: core::default::Default::default(),
            use_parallelism: core::default::Default::default(),
            use_serverless_analytics: core::default::Default::default(),
        }
    }
}

pub struct BigqueryConnectionCloudSpannerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionCloudSpannerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnectionCloudSpannerElRef {
        BigqueryConnectionCloudSpannerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionCloudSpannerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nCloud Spanner database in the form 'project/instance/database'."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_role` after provisioning.\nCloud Spanner database role for fine-grained access control. The Cloud Spanner admin should have provisioned the database role with appropriate permissions, such as 'SELECT' and 'INSERT'. Other users should only use roles provided by their Cloud Spanner admins. The database role name must start with a letter, and can only contain letters, numbers, and underscores. For more details, see https://cloud.google.com/spanner/docs/fgac-about."]
    pub fn database_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_role", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_parallelism` after provisioning.\nAllows setting max parallelism per query when executing on Spanner independent compute resources. If unspecified, default values of parallelism are chosen that are dependent on the Cloud Spanner instance configuration. 'useParallelism' and 'useDataBoost' must be set when setting max parallelism."]
    pub fn max_parallelism(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_parallelism", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_data_boost` after provisioning.\nIf set, the request will be executed via Spanner independent compute resources. 'use_parallelism' must be set when using data boost."]
    pub fn use_data_boost(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_data_boost", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_parallelism` after provisioning.\nIf parallelism should be used when reading from Cloud Spanner."]
    pub fn use_parallelism(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_parallelism", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_serverless_analytics` after provisioning.\nIf the serverless analytics service should be used to read data from Cloud Spanner. 'useParallelism' must be set when using serverless analytics."]
    pub fn use_serverless_analytics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_serverless_analytics", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionCloudSqlElCredentialEl {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl BigqueryConnectionCloudSqlElCredentialEl { }

impl terrars::ToListMappable for BigqueryConnectionCloudSqlElCredentialEl {
    type O = terrars::BlockAssignable<BigqueryConnectionCloudSqlElCredentialEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionCloudSqlElCredentialEl {
    #[doc = "Password for database."]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = "Username for database."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnectionCloudSqlElCredentialEl {
    pub fn build(self) -> BigqueryConnectionCloudSqlElCredentialEl {
        BigqueryConnectionCloudSqlElCredentialEl {
            password: self.password,
            username: self.username,
        }
    }
}

pub struct BigqueryConnectionCloudSqlElCredentialElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionCloudSqlElCredentialElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnectionCloudSqlElCredentialElRef {
        BigqueryConnectionCloudSqlElCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionCloudSqlElCredentialElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for database."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for database."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnectionCloudSqlElDynamic {
    credential: std::option::Option<terrars::DynamicBlock<BigqueryConnectionCloudSqlElCredentialEl>>,
}

#[derive(Serialize)]
pub struct BigqueryConnectionCloudSqlEl {
    database: terrars::PrimField<std::string::String>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential: std::option::Option<std::vec::Vec<BigqueryConnectionCloudSqlElCredentialEl>>,
    dynamic: BigqueryConnectionCloudSqlElDynamic,
}

impl BigqueryConnectionCloudSqlEl {
    #[doc = "Set the field `credential`.\n"]
    pub fn set_credential(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnectionCloudSqlElCredentialEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.credential = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryConnectionCloudSqlEl {
    type O = terrars::BlockAssignable<BigqueryConnectionCloudSqlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionCloudSqlEl {
    #[doc = "Database name."]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = "Cloud SQL instance ID in the form project:location:instance."]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Type of the Cloud SQL database. Possible values: [\"DATABASE_TYPE_UNSPECIFIED\", \"POSTGRES\", \"MYSQL\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnectionCloudSqlEl {
    pub fn build(self) -> BigqueryConnectionCloudSqlEl {
        BigqueryConnectionCloudSqlEl {
            database: self.database,
            instance_id: self.instance_id,
            type_: self.type_,
            credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BigqueryConnectionCloudSqlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionCloudSqlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnectionCloudSqlElRef {
        BigqueryConnectionCloudSqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionCloudSqlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nCloud SQL instance ID in the form project:location:instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_id` after provisioning.\nWhen the connection is used in the context of an operation in BigQuery, this service account will serve as the identity being used for connecting to the CloudSQL instance specified in this connection."]
    pub fn service_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of the Cloud SQL database. Possible values: [\"DATABASE_TYPE_UNSPECIFIED\", \"POSTGRES\", \"MYSQL\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `credential` after provisioning.\n"]
    pub fn credential(&self) -> terrars::ListRef<BigqueryConnectionCloudSqlElCredentialElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credential", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionSparkElMetastoreServiceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metastore_service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryConnectionSparkElMetastoreServiceConfigEl {
    #[doc =
        "Set the field `metastore_service`.\nResource name of an existing Dataproc Metastore service in the form of projects/[projectId]/locations/[region]/services/[serviceId]."]
    pub fn set_metastore_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metastore_service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryConnectionSparkElMetastoreServiceConfigEl {
    type O = terrars::BlockAssignable<BigqueryConnectionSparkElMetastoreServiceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionSparkElMetastoreServiceConfigEl {}

impl BuildBigqueryConnectionSparkElMetastoreServiceConfigEl {
    pub fn build(self) -> BigqueryConnectionSparkElMetastoreServiceConfigEl {
        BigqueryConnectionSparkElMetastoreServiceConfigEl { metastore_service: core::default::Default::default() }
    }
}

pub struct BigqueryConnectionSparkElMetastoreServiceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionSparkElMetastoreServiceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryConnectionSparkElMetastoreServiceConfigElRef {
        BigqueryConnectionSparkElMetastoreServiceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionSparkElMetastoreServiceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metastore_service` after provisioning.\nResource name of an existing Dataproc Metastore service in the form of projects/[projectId]/locations/[region]/services/[serviceId]."]
    pub fn metastore_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metastore_service", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionSparkElSparkHistoryServerConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataproc_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryConnectionSparkElSparkHistoryServerConfigEl {
    #[doc =
        "Set the field `dataproc_cluster`.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the connection if the form of projects/[projectId]/regions/[region]/clusters/[cluster_name]."]
    pub fn set_dataproc_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataproc_cluster = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryConnectionSparkElSparkHistoryServerConfigEl {
    type O = terrars::BlockAssignable<BigqueryConnectionSparkElSparkHistoryServerConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionSparkElSparkHistoryServerConfigEl {}

impl BuildBigqueryConnectionSparkElSparkHistoryServerConfigEl {
    pub fn build(self) -> BigqueryConnectionSparkElSparkHistoryServerConfigEl {
        BigqueryConnectionSparkElSparkHistoryServerConfigEl { dataproc_cluster: core::default::Default::default() }
    }
}

pub struct BigqueryConnectionSparkElSparkHistoryServerConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionSparkElSparkHistoryServerConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryConnectionSparkElSparkHistoryServerConfigElRef {
        BigqueryConnectionSparkElSparkHistoryServerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionSparkElSparkHistoryServerConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataproc_cluster` after provisioning.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the connection if the form of projects/[projectId]/regions/[region]/clusters/[cluster_name]."]
    pub fn dataproc_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataproc_cluster", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnectionSparkElDynamic {
    metastore_service_config: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryConnectionSparkElMetastoreServiceConfigEl>>,
    spark_history_server_config: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryConnectionSparkElSparkHistoryServerConfigEl>>,
}

#[derive(Serialize)]
pub struct BigqueryConnectionSparkEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metastore_service_config: std::option::Option<std::vec::Vec<BigqueryConnectionSparkElMetastoreServiceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_history_server_config: std::option::Option<std::vec::Vec<BigqueryConnectionSparkElSparkHistoryServerConfigEl>>,
    dynamic: BigqueryConnectionSparkElDynamic,
}

impl BigqueryConnectionSparkEl {
    #[doc = "Set the field `metastore_service_config`.\n"]
    pub fn set_metastore_service_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnectionSparkElMetastoreServiceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metastore_service_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metastore_service_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_history_server_config`.\n"]
    pub fn set_spark_history_server_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnectionSparkElSparkHistoryServerConfigEl>>,
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

impl terrars::ToListMappable for BigqueryConnectionSparkEl {
    type O = terrars::BlockAssignable<BigqueryConnectionSparkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionSparkEl {}

impl BuildBigqueryConnectionSparkEl {
    pub fn build(self) -> BigqueryConnectionSparkEl {
        BigqueryConnectionSparkEl {
            metastore_service_config: core::default::Default::default(),
            spark_history_server_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BigqueryConnectionSparkElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionSparkElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnectionSparkElRef {
        BigqueryConnectionSparkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionSparkElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account_id` after provisioning.\nThe account ID of the service created for the purpose of this connection."]
    pub fn service_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `metastore_service_config` after provisioning.\n"]
    pub fn metastore_service_config(&self) -> terrars::ListRef<BigqueryConnectionSparkElMetastoreServiceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metastore_service_config", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_history_server_config` after provisioning.\n"]
    pub fn spark_history_server_config(
        &self,
    ) -> terrars::ListRef<BigqueryConnectionSparkElSparkHistoryServerConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_history_server_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryConnectionTimeoutsEl {
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

impl terrars::ToListMappable for BigqueryConnectionTimeoutsEl {
    type O = terrars::BlockAssignable<BigqueryConnectionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionTimeoutsEl {}

impl BuildBigqueryConnectionTimeoutsEl {
    pub fn build(self) -> BigqueryConnectionTimeoutsEl {
        BigqueryConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BigqueryConnectionTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnectionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnectionTimeoutsElRef {
        BigqueryConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionTimeoutsElRef {
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
struct BigqueryConnectionDynamic {
    aws: std::option::Option<terrars::DynamicBlock<BigqueryConnectionAwsEl>>,
    azure: std::option::Option<terrars::DynamicBlock<BigqueryConnectionAzureEl>>,
    cloud_resource: std::option::Option<terrars::DynamicBlock<BigqueryConnectionCloudResourceEl>>,
    cloud_spanner: std::option::Option<terrars::DynamicBlock<BigqueryConnectionCloudSpannerEl>>,
    cloud_sql: std::option::Option<terrars::DynamicBlock<BigqueryConnectionCloudSqlEl>>,
    spark: std::option::Option<terrars::DynamicBlock<BigqueryConnectionSparkEl>>,
}
