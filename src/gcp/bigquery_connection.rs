#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryConnectionData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    friendly_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws: std::option::Option<std::vec::Vec<BigqueryConnection_Aws>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure: std::option::Option<std::vec::Vec<BigqueryConnection_Azure>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_resource: std::option::Option<std::vec::Vec<BigqueryConnection_CloudResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_spanner: std::option::Option<std::vec::Vec<BigqueryConnection_CloudSpanner>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql: std::option::Option<std::vec::Vec<BigqueryConnection_CloudSql>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<BigqueryConnection_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark: std::option::Option<std::vec::Vec<BigqueryConnection_Spark>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryConnection_Timeouts>,
    dynamic: BigqueryConnectionDynamic,
}

struct BigqueryConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryConnectionData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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
        "Set the field `kms_key_name`.\nOptional. The Cloud KMS key that is used for encryption.\n\nExample: projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]"]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
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
    pub fn set_aws(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Aws>>) -> Self {
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
    pub fn set_azure(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Azure>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_CloudResource>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_CloudSpanner>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_CloudSql>>,
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark`.\n"]
    pub fn set_spark(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Spark>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryConnection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `connection_id` after provisioning.\nOptional connection id that should be assigned to the created connection."]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `kms_key_name` after provisioning.\nOptional. The Cloud KMS key that is used for encryption.\n\nExample: projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
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
    pub fn aws(&self) -> terrars::ListRef<BigqueryConnection_AwsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> terrars::ListRef<BigqueryConnection_AzureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_resource` after provisioning.\n"]
    pub fn cloud_resource(&self) -> terrars::ListRef<BigqueryConnection_CloudResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_spanner` after provisioning.\n"]
    pub fn cloud_spanner(&self) -> terrars::ListRef<BigqueryConnection_CloudSpannerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_spanner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_sql` after provisioning.\n"]
    pub fn cloud_sql(&self) -> terrars::ListRef<BigqueryConnection_CloudSqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_sql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<BigqueryConnection_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> terrars::ListRef<BigqueryConnection_SparkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryConnection_TimeoutsRef {
        BigqueryConnection_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(BigqueryConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connection_id: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                friendly_name: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                aws: core::default::Default::default(),
                azure: core::default::Default::default(),
                cloud_resource: core::default::Default::default(),
                cloud_spanner: core::default::Default::default(),
                cloud_sql: core::default::Default::default(),
                configuration: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `kms_key_name` after provisioning.\nOptional. The Cloud KMS key that is used for encryption.\n\nExample: projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
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
    pub fn aws(&self) -> terrars::ListRef<BigqueryConnection_AwsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> terrars::ListRef<BigqueryConnection_AzureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_resource` after provisioning.\n"]
    pub fn cloud_resource(&self) -> terrars::ListRef<BigqueryConnection_CloudResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_resource", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_spanner` after provisioning.\n"]
    pub fn cloud_spanner(&self) -> terrars::ListRef<BigqueryConnection_CloudSpannerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_spanner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_sql` after provisioning.\n"]
    pub fn cloud_sql(&self) -> terrars::ListRef<BigqueryConnection_CloudSqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_sql", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<BigqueryConnection_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> terrars::ListRef<BigqueryConnection_SparkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryConnection_TimeoutsRef {
        BigqueryConnection_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnectionDynamic {
    aws: std::option::Option<terrars::DynamicBlock<BigqueryConnection_Aws>>,
    azure: std::option::Option<terrars::DynamicBlock<BigqueryConnection_Azure>>,
    cloud_resource: std::option::Option<terrars::DynamicBlock<BigqueryConnection_CloudResource>>,
    cloud_spanner: std::option::Option<terrars::DynamicBlock<BigqueryConnection_CloudSpanner>>,
    cloud_sql: std::option::Option<terrars::DynamicBlock<BigqueryConnection_CloudSql>>,
    configuration: std::option::Option<terrars::DynamicBlock<BigqueryConnection_Configuration>>,
    spark: std::option::Option<terrars::DynamicBlock<BigqueryConnection_Spark>>,
}

#[derive(Serialize)]
pub struct BigqueryConnection_Aws_AccessRole {
    iam_role_id: terrars::PrimField<std::string::String>,
}

impl BigqueryConnection_Aws_AccessRole { }

impl terrars::ToListMappable for BigqueryConnection_Aws_AccessRole {
    type O = terrars::BlockAssignable<BigqueryConnection_Aws_AccessRole>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Aws_AccessRole {
    #[doc = "The user’s AWS IAM Role that trusts the Google-owned AWS IAM user Connection."]
    pub iam_role_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnection_Aws_AccessRole {
    pub fn build(self) -> BigqueryConnection_Aws_AccessRole {
        BigqueryConnection_Aws_AccessRole { iam_role_id: self.iam_role_id }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_Aws_AccessRoleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_Aws_AccessRoleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_Aws_AccessRoleRef {
        BigqueryConnection_Aws_AccessRoleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_Aws_AccessRoleRef {
    #[allow(dead_code)]
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
struct BigqueryConnection_AwsDynamic {
    access_role: std::option::Option<terrars::DynamicBlock<BigqueryConnection_Aws_AccessRole>>,
}

#[derive(Serialize)]
pub struct BigqueryConnection_Aws {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_role: std::option::Option<std::vec::Vec<BigqueryConnection_Aws_AccessRole>>,
    dynamic: BigqueryConnection_AwsDynamic,
}

impl BigqueryConnection_Aws {
    #[doc = "Set the field `access_role`.\n"]
    pub fn set_access_role(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Aws_AccessRole>>,
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

impl terrars::ToListMappable for BigqueryConnection_Aws {
    type O = terrars::BlockAssignable<BigqueryConnection_Aws>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Aws {}

impl BuildBigqueryConnection_Aws {
    pub fn build(self) -> BigqueryConnection_Aws {
        BigqueryConnection_Aws {
            access_role: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_AwsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_AwsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_AwsRef {
        BigqueryConnection_AwsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_AwsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_role` after provisioning.\n"]
    pub fn access_role(&self) -> terrars::ListRef<BigqueryConnection_Aws_AccessRoleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_role", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnection_Azure {
    customer_tenant_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    federated_application_client_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryConnection_Azure {
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

impl terrars::ToListMappable for BigqueryConnection_Azure {
    type O = terrars::BlockAssignable<BigqueryConnection_Azure>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Azure {
    #[doc = "The id of customer's directory that host the data."]
    pub customer_tenant_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnection_Azure {
    pub fn build(self) -> BigqueryConnection_Azure {
        BigqueryConnection_Azure {
            customer_tenant_id: self.customer_tenant_id,
            federated_application_client_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_AzureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_AzureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_AzureRef {
        BigqueryConnection_AzureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_AzureRef {
    #[allow(dead_code)]
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
pub struct BigqueryConnection_CloudResource {}

impl BigqueryConnection_CloudResource { }

impl terrars::ToListMappable for BigqueryConnection_CloudResource {
    type O = terrars::BlockAssignable<BigqueryConnection_CloudResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_CloudResource {}

impl BuildBigqueryConnection_CloudResource {
    pub fn build(self) -> BigqueryConnection_CloudResource {
        BigqueryConnection_CloudResource {}
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_CloudResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_CloudResourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_CloudResourceRef {
        BigqueryConnection_CloudResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_CloudResourceRef {
    #[allow(dead_code)]
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
pub struct BigqueryConnection_CloudSpanner {
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

impl BigqueryConnection_CloudSpanner {
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

impl terrars::ToListMappable for BigqueryConnection_CloudSpanner {
    type O = terrars::BlockAssignable<BigqueryConnection_CloudSpanner>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_CloudSpanner {
    #[doc = "Cloud Spanner database in the form 'project/instance/database'."]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnection_CloudSpanner {
    pub fn build(self) -> BigqueryConnection_CloudSpanner {
        BigqueryConnection_CloudSpanner {
            database: self.database,
            database_role: core::default::Default::default(),
            max_parallelism: core::default::Default::default(),
            use_data_boost: core::default::Default::default(),
            use_parallelism: core::default::Default::default(),
            use_serverless_analytics: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_CloudSpannerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_CloudSpannerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_CloudSpannerRef {
        BigqueryConnection_CloudSpannerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_CloudSpannerRef {
    #[allow(dead_code)]
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
pub struct BigqueryConnection_CloudSql_Credential {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
}

impl BigqueryConnection_CloudSql_Credential {
    #[doc = "Set the field `password`.\nPassword for database."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_wo`.\nPassword for database."]
    pub fn set_password_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_wo_version`.\nTriggers update of 'password_wo' write-only. Increment this value when an update to 'password_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_password_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_wo_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryConnection_CloudSql_Credential {
    type O = terrars::BlockAssignable<BigqueryConnection_CloudSql_Credential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_CloudSql_Credential {
    #[doc = "Username for database."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnection_CloudSql_Credential {
    pub fn build(self) -> BigqueryConnection_CloudSql_Credential {
        BigqueryConnection_CloudSql_Credential {
            password: core::default::Default::default(),
            password_wo: core::default::Default::default(),
            password_wo_version: core::default::Default::default(),
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_CloudSql_CredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_CloudSql_CredentialRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_CloudSql_CredentialRef {
        BigqueryConnection_CloudSql_CredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_CloudSql_CredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword for database."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `password_wo` after provisioning.\nPassword for database."]
    pub fn password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_wo_version` after provisioning.\nTriggers update of 'password_wo' write-only. Increment this value when an update to 'password_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn password_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for database."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnection_CloudSqlDynamic {
    credential: std::option::Option<terrars::DynamicBlock<BigqueryConnection_CloudSql_Credential>>,
}

#[derive(Serialize)]
pub struct BigqueryConnection_CloudSql {
    database: terrars::PrimField<std::string::String>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential: std::option::Option<std::vec::Vec<BigqueryConnection_CloudSql_Credential>>,
    dynamic: BigqueryConnection_CloudSqlDynamic,
}

impl BigqueryConnection_CloudSql {
    #[doc = "Set the field `credential`.\n"]
    pub fn set_credential(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_CloudSql_Credential>>,
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

impl terrars::ToListMappable for BigqueryConnection_CloudSql {
    type O = terrars::BlockAssignable<BigqueryConnection_CloudSql>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_CloudSql {
    #[doc = "Database name."]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = "Cloud SQL instance ID in the form project:location:instance."]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Type of the Cloud SQL database. Possible values: [\"DATABASE_TYPE_UNSPECIFIED\", \"POSTGRES\", \"MYSQL\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnection_CloudSql {
    pub fn build(self) -> BigqueryConnection_CloudSql {
        BigqueryConnection_CloudSql {
            database: self.database,
            instance_id: self.instance_id,
            type_: self.type_,
            credential: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_CloudSqlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_CloudSqlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_CloudSqlRef {
        BigqueryConnection_CloudSqlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_CloudSqlRef {
    #[allow(dead_code)]
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
    pub fn credential(&self) -> terrars::ListRef<BigqueryConnection_CloudSql_CredentialRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credential", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnection_Configuration_Asset {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryConnection_Configuration_Asset {
    #[doc = "Set the field `database`.\nThe name of the database."]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `google_cloud_resource`.\nThe full resource name of the Google Cloud resource.\nFor AlloyDB, this is in the format of\n'//alloydb.googleapis.com/projects/{project}/locations/{region}/clusters/{cluster}/instances/{instance}'."]
    pub fn set_google_cloud_resource(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.google_cloud_resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryConnection_Configuration_Asset {
    type O = terrars::BlockAssignable<BigqueryConnection_Configuration_Asset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Configuration_Asset {}

impl BuildBigqueryConnection_Configuration_Asset {
    pub fn build(self) -> BigqueryConnection_Configuration_Asset {
        BigqueryConnection_Configuration_Asset {
            database: core::default::Default::default(),
            google_cloud_resource: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_Configuration_AssetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_Configuration_AssetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_Configuration_AssetRef {
        BigqueryConnection_Configuration_AssetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_Configuration_AssetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe name of the database."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `google_cloud_resource` after provisioning.\nThe full resource name of the Google Cloud resource.\nFor AlloyDB, this is in the format of\n'//alloydb.googleapis.com/projects/{project}/locations/{region}/clusters/{cluster}/instances/{instance}'."]
    pub fn google_cloud_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_cloud_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnection_Configuration_Authentication_UsernamePassword_Password {
    plaintext: terrars::PrimField<std::string::String>,
}

impl BigqueryConnection_Configuration_Authentication_UsernamePassword_Password { }

impl terrars::ToListMappable for BigqueryConnection_Configuration_Authentication_UsernamePassword_Password {
    type O = terrars::BlockAssignable<BigqueryConnection_Configuration_Authentication_UsernamePassword_Password>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Configuration_Authentication_UsernamePassword_Password {
    #[doc = "The plaintext password."]
    pub plaintext: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnection_Configuration_Authentication_UsernamePassword_Password {
    pub fn build(self) -> BigqueryConnection_Configuration_Authentication_UsernamePassword_Password {
        BigqueryConnection_Configuration_Authentication_UsernamePassword_Password { plaintext: self.plaintext }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_Configuration_Authentication_UsernamePassword_PasswordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_Configuration_Authentication_UsernamePassword_PasswordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryConnection_Configuration_Authentication_UsernamePassword_PasswordRef {
        BigqueryConnection_Configuration_Authentication_UsernamePassword_PasswordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_Configuration_Authentication_UsernamePassword_PasswordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `plaintext` after provisioning.\nThe plaintext password."]
    pub fn plaintext(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plaintext", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_type` after provisioning.\nOutput only. The type of the secret."]
    pub fn secret_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnection_Configuration_Authentication_UsernamePasswordDynamic {
    password: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryConnection_Configuration_Authentication_UsernamePassword_Password>>,
}

#[derive(Serialize)]
pub struct BigqueryConnection_Configuration_Authentication_UsernamePassword {
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std
    ::option
    ::Option<std::vec::Vec<BigqueryConnection_Configuration_Authentication_UsernamePassword_Password>>,
    dynamic: BigqueryConnection_Configuration_Authentication_UsernamePasswordDynamic,
}

impl BigqueryConnection_Configuration_Authentication_UsernamePassword {
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BigqueryConnection_Configuration_Authentication_UsernamePassword_Password>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.password = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.password = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryConnection_Configuration_Authentication_UsernamePassword {
    type O = terrars::BlockAssignable<BigqueryConnection_Configuration_Authentication_UsernamePassword>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Configuration_Authentication_UsernamePassword {
    #[doc = "Username for the connector."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnection_Configuration_Authentication_UsernamePassword {
    pub fn build(self) -> BigqueryConnection_Configuration_Authentication_UsernamePassword {
        BigqueryConnection_Configuration_Authentication_UsernamePassword {
            username: self.username,
            password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_Configuration_Authentication_UsernamePasswordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_Configuration_Authentication_UsernamePasswordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryConnection_Configuration_Authentication_UsernamePasswordRef {
        BigqueryConnection_Configuration_Authentication_UsernamePasswordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_Configuration_Authentication_UsernamePasswordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for the connector."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(
        &self,
    ) -> terrars::ListRef<BigqueryConnection_Configuration_Authentication_UsernamePassword_PasswordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnection_Configuration_AuthenticationDynamic {
    username_password: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryConnection_Configuration_Authentication_UsernamePassword>>,
}

#[derive(Serialize)]
pub struct BigqueryConnection_Configuration_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_password: std
    ::option
    ::Option<std::vec::Vec<BigqueryConnection_Configuration_Authentication_UsernamePassword>>,
    dynamic: BigqueryConnection_Configuration_AuthenticationDynamic,
}

impl BigqueryConnection_Configuration_Authentication {
    #[doc = "Set the field `username_password`.\n"]
    pub fn set_username_password(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BigqueryConnection_Configuration_Authentication_UsernamePassword>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.username_password = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.username_password = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryConnection_Configuration_Authentication {
    type O = terrars::BlockAssignable<BigqueryConnection_Configuration_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Configuration_Authentication {}

impl BuildBigqueryConnection_Configuration_Authentication {
    pub fn build(self) -> BigqueryConnection_Configuration_Authentication {
        BigqueryConnection_Configuration_Authentication {
            username_password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_Configuration_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_Configuration_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryConnection_Configuration_AuthenticationRef {
        BigqueryConnection_Configuration_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_Configuration_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nOutput only. The service account used for authenticating with the connector."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `username_password` after provisioning.\n"]
    pub fn username_password(
        &self,
    ) -> terrars::ListRef<BigqueryConnection_Configuration_Authentication_UsernamePasswordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_password", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnection_Configuration_Endpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_port: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryConnection_Configuration_Endpoint {
    #[doc = "Set the field `host_port`.\nHost and port in the format of 'host:port' for the connector endpoint."]
    pub fn set_host_port(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryConnection_Configuration_Endpoint {
    type O = terrars::BlockAssignable<BigqueryConnection_Configuration_Endpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Configuration_Endpoint {}

impl BuildBigqueryConnection_Configuration_Endpoint {
    pub fn build(self) -> BigqueryConnection_Configuration_Endpoint {
        BigqueryConnection_Configuration_Endpoint { host_port: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_Configuration_EndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_Configuration_EndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_Configuration_EndpointRef {
        BigqueryConnection_Configuration_EndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_Configuration_EndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_port` after provisioning.\nHost and port in the format of 'host:port' for the connector endpoint."]
    pub fn host_port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_port", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnection_Configuration_Network_PrivateServiceConnect {
    network_attachment: terrars::PrimField<std::string::String>,
}

impl BigqueryConnection_Configuration_Network_PrivateServiceConnect { }

impl terrars::ToListMappable for BigqueryConnection_Configuration_Network_PrivateServiceConnect {
    type O = terrars::BlockAssignable<BigqueryConnection_Configuration_Network_PrivateServiceConnect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Configuration_Network_PrivateServiceConnect {
    #[doc =
        "The resource name of a network attachment in the format of\n'projects/{project}/regions/{region}/networkAttachments/{networkAttachment}'."]
    pub network_attachment: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnection_Configuration_Network_PrivateServiceConnect {
    pub fn build(self) -> BigqueryConnection_Configuration_Network_PrivateServiceConnect {
        BigqueryConnection_Configuration_Network_PrivateServiceConnect {
            network_attachment: self.network_attachment,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_Configuration_Network_PrivateServiceConnectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_Configuration_Network_PrivateServiceConnectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryConnection_Configuration_Network_PrivateServiceConnectRef {
        BigqueryConnection_Configuration_Network_PrivateServiceConnectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_Configuration_Network_PrivateServiceConnectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_attachment` after provisioning.\nThe resource name of a network attachment in the format of\n'projects/{project}/regions/{region}/networkAttachments/{networkAttachment}'."]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnection_Configuration_NetworkDynamic {
    private_service_connect: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryConnection_Configuration_Network_PrivateServiceConnect>>,
}

#[derive(Serialize)]
pub struct BigqueryConnection_Configuration_Network {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_service_connect: std
    ::option
    ::Option<std::vec::Vec<BigqueryConnection_Configuration_Network_PrivateServiceConnect>>,
    dynamic: BigqueryConnection_Configuration_NetworkDynamic,
}

impl BigqueryConnection_Configuration_Network {
    #[doc = "Set the field `private_service_connect`.\n"]
    pub fn set_private_service_connect(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BigqueryConnection_Configuration_Network_PrivateServiceConnect>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_service_connect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_service_connect = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryConnection_Configuration_Network {
    type O = terrars::BlockAssignable<BigqueryConnection_Configuration_Network>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Configuration_Network {}

impl BuildBigqueryConnection_Configuration_Network {
    pub fn build(self) -> BigqueryConnection_Configuration_Network {
        BigqueryConnection_Configuration_Network {
            private_service_connect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_Configuration_NetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_Configuration_NetworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_Configuration_NetworkRef {
        BigqueryConnection_Configuration_NetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_Configuration_NetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `private_service_connect` after provisioning.\n"]
    pub fn private_service_connect(
        &self,
    ) -> terrars::ListRef<BigqueryConnection_Configuration_Network_PrivateServiceConnectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_service_connect", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnection_ConfigurationDynamic {
    asset: std::option::Option<terrars::DynamicBlock<BigqueryConnection_Configuration_Asset>>,
    authentication: std::option::Option<terrars::DynamicBlock<BigqueryConnection_Configuration_Authentication>>,
    endpoint: std::option::Option<terrars::DynamicBlock<BigqueryConnection_Configuration_Endpoint>>,
    network: std::option::Option<terrars::DynamicBlock<BigqueryConnection_Configuration_Network>>,
}

#[derive(Serialize)]
pub struct BigqueryConnection_Configuration {
    connector_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset: std::option::Option<std::vec::Vec<BigqueryConnection_Configuration_Asset>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<BigqueryConnection_Configuration_Authentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<std::vec::Vec<BigqueryConnection_Configuration_Endpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<std::vec::Vec<BigqueryConnection_Configuration_Network>>,
    dynamic: BigqueryConnection_ConfigurationDynamic,
}

impl BigqueryConnection_Configuration {
    #[doc = "Set the field `asset`.\n"]
    pub fn set_asset(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Configuration_Asset>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.asset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.asset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Configuration_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Configuration_Endpoint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Configuration_Network>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryConnection_Configuration {
    type O = terrars::BlockAssignable<BigqueryConnection_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Configuration {
    #[doc =
        "The ID of the connector. Possible values include 'google-alloydb', 'google-cloudsql-mysql',\n'google-cloudsql-postgres', and other connector IDs supported by the BigQuery Connector framework."]
    pub connector_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryConnection_Configuration {
    pub fn build(self) -> BigqueryConnection_Configuration {
        BigqueryConnection_Configuration {
            connector_id: self.connector_id,
            asset: core::default::Default::default(),
            authentication: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            network: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_ConfigurationRef {
        BigqueryConnection_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connector_id` after provisioning.\nThe ID of the connector. Possible values include 'google-alloydb', 'google-cloudsql-mysql',\n'google-cloudsql-postgres', and other connector IDs supported by the BigQuery Connector framework."]
    pub fn connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_id", self.base))
    }

    #[doc = "Get a reference to the value of field `asset` after provisioning.\n"]
    pub fn asset(&self) -> terrars::ListRef<BigqueryConnection_Configuration_AssetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asset", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<BigqueryConnection_Configuration_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<BigqueryConnection_Configuration_EndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::ListRef<BigqueryConnection_Configuration_NetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnection_Spark_MetastoreServiceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metastore_service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryConnection_Spark_MetastoreServiceConfig {
    #[doc =
        "Set the field `metastore_service`.\nResource name of an existing Dataproc Metastore service in the form of projects/[projectId]/locations/[region]/services/[serviceId]."]
    pub fn set_metastore_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metastore_service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryConnection_Spark_MetastoreServiceConfig {
    type O = terrars::BlockAssignable<BigqueryConnection_Spark_MetastoreServiceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Spark_MetastoreServiceConfig {}

impl BuildBigqueryConnection_Spark_MetastoreServiceConfig {
    pub fn build(self) -> BigqueryConnection_Spark_MetastoreServiceConfig {
        BigqueryConnection_Spark_MetastoreServiceConfig { metastore_service: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_Spark_MetastoreServiceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_Spark_MetastoreServiceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryConnection_Spark_MetastoreServiceConfigRef {
        BigqueryConnection_Spark_MetastoreServiceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_Spark_MetastoreServiceConfigRef {
    #[allow(dead_code)]
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
pub struct BigqueryConnection_Spark_SparkHistoryServerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataproc_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryConnection_Spark_SparkHistoryServerConfig {
    #[doc =
        "Set the field `dataproc_cluster`.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the connection if the form of projects/[projectId]/regions/[region]/clusters/[cluster_name]."]
    pub fn set_dataproc_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataproc_cluster = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryConnection_Spark_SparkHistoryServerConfig {
    type O = terrars::BlockAssignable<BigqueryConnection_Spark_SparkHistoryServerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Spark_SparkHistoryServerConfig {}

impl BuildBigqueryConnection_Spark_SparkHistoryServerConfig {
    pub fn build(self) -> BigqueryConnection_Spark_SparkHistoryServerConfig {
        BigqueryConnection_Spark_SparkHistoryServerConfig { dataproc_cluster: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_Spark_SparkHistoryServerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_Spark_SparkHistoryServerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryConnection_Spark_SparkHistoryServerConfigRef {
        BigqueryConnection_Spark_SparkHistoryServerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_Spark_SparkHistoryServerConfigRef {
    #[allow(dead_code)]
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
struct BigqueryConnection_SparkDynamic {
    metastore_service_config: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryConnection_Spark_MetastoreServiceConfig>>,
    spark_history_server_config: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryConnection_Spark_SparkHistoryServerConfig>>,
}

#[derive(Serialize)]
pub struct BigqueryConnection_Spark {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metastore_service_config: std::option::Option<std::vec::Vec<BigqueryConnection_Spark_MetastoreServiceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_history_server_config: std::option::Option<std::vec::Vec<BigqueryConnection_Spark_SparkHistoryServerConfig>>,
    dynamic: BigqueryConnection_SparkDynamic,
}

impl BigqueryConnection_Spark {
    #[doc = "Set the field `metastore_service_config`.\n"]
    pub fn set_metastore_service_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Spark_MetastoreServiceConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryConnection_Spark_SparkHistoryServerConfig>>,
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

impl terrars::ToListMappable for BigqueryConnection_Spark {
    type O = terrars::BlockAssignable<BigqueryConnection_Spark>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Spark {}

impl BuildBigqueryConnection_Spark {
    pub fn build(self) -> BigqueryConnection_Spark {
        BigqueryConnection_Spark {
            metastore_service_config: core::default::Default::default(),
            spark_history_server_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_SparkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_SparkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_SparkRef {
        BigqueryConnection_SparkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_SparkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account_id` after provisioning.\nThe account ID of the service created for the purpose of this connection."]
    pub fn service_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `metastore_service_config` after provisioning.\n"]
    pub fn metastore_service_config(&self) -> terrars::ListRef<BigqueryConnection_Spark_MetastoreServiceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metastore_service_config", self.base))
    }

    #[doc = "Get a reference to the value of field `spark_history_server_config` after provisioning.\n"]
    pub fn spark_history_server_config(&self) -> terrars::ListRef<BigqueryConnection_Spark_SparkHistoryServerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_history_server_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryConnection_Timeouts {
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

impl terrars::ToListMappable for BigqueryConnection_Timeouts {
    type O = terrars::BlockAssignable<BigqueryConnection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnection_Timeouts {}

impl BuildBigqueryConnection_Timeouts {
    pub fn build(self) -> BigqueryConnection_Timeouts {
        BigqueryConnection_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryConnection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryConnection_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryConnection_TimeoutsRef {
        BigqueryConnection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnection_TimeoutsRef {
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
