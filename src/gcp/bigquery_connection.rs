use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    friendly_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws: Option<Vec<BigqueryConnectionAwsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    azure: Option<Vec<BigqueryConnectionAzureEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_resource: Option<Vec<BigqueryConnectionCloudResourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_spanner: Option<Vec<BigqueryConnectionCloudSpannerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql: Option<Vec<BigqueryConnectionCloudSqlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark: Option<Vec<BigqueryConnectionSparkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryConnectionTimeoutsEl>,
    dynamic: BigqueryConnectionDynamic,
}

struct BigqueryConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryConnectionData>,
}

#[derive(Clone)]
pub struct BigqueryConnection(Rc<BigqueryConnection_>);

impl BigqueryConnection {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `connection_id`.\nOptional connection id that should be assigned to the created connection."]
    pub fn set_connection_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().connection_id = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA descriptive description for the connection"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `friendly_name`.\nA descriptive name for the connection"]
    pub fn set_friendly_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().friendly_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\nThe geographic location where the connection should reside.\nCloud SQL instance must be in the same location as the connection\nwith following exceptions: Cloud SQL us-central1 maps to BigQuery US, Cloud SQL europe-west1 maps to BigQuery EU.\nExamples: US, EU, asia-northeast1, us-central1, europe-west1.\nSpanner Connections same as spanner region\nAWS allowed regions are aws-us-east-1\nAzure allowed regions are azure-eastus2"]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `aws`.\n"]
    pub fn set_aws(self, v: impl Into<BlockAssignable<BigqueryConnectionAwsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aws = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aws = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `azure`.\n"]
    pub fn set_azure(self, v: impl Into<BlockAssignable<BigqueryConnectionAzureEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().azure = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.azure = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `cloud_resource`.\n"]
    pub fn set_cloud_resource(self, v: impl Into<BlockAssignable<BigqueryConnectionCloudResourceEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_resource = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_resource = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `cloud_spanner`.\n"]
    pub fn set_cloud_spanner(self, v: impl Into<BlockAssignable<BigqueryConnectionCloudSpannerEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_spanner = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_spanner = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `cloud_sql`.\n"]
    pub fn set_cloud_sql(self, v: impl Into<BlockAssignable<BigqueryConnectionCloudSqlEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_sql = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_sql = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `spark`.\n"]
    pub fn set_spark(self, v: impl Into<BlockAssignable<BigqueryConnectionSparkEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryConnectionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `connection_id` after provisioning.\nOptional connection id that should be assigned to the created connection."]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA descriptive description for the connection"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the connection"]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_credential` after provisioning.\nTrue if the connection has credential assigned."]
    pub fn has_credential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_credential", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the connection should reside.\nCloud SQL instance must be in the same location as the connection\nwith following exceptions: Cloud SQL us-central1 maps to BigQuery US, Cloud SQL europe-west1 maps to BigQuery EU.\nExamples: US, EU, asia-northeast1, us-central1, europe-west1.\nSpanner Connections same as spanner region\nAWS allowed regions are aws-us-east-1\nAzure allowed regions are azure-eastus2"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the connection in the form of:\n\"projects/{project_id}/locations/{location_id}/connections/{connectionId}\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `aws` after provisioning.\n"]
    pub fn aws(&self) -> ListRef<BigqueryConnectionAwsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> ListRef<BigqueryConnectionAzureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.azure", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloud_resource` after provisioning.\n"]
    pub fn cloud_resource(&self) -> ListRef<BigqueryConnectionCloudResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_resource", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloud_spanner` after provisioning.\n"]
    pub fn cloud_spanner(&self) -> ListRef<BigqueryConnectionCloudSpannerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_spanner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloud_sql` after provisioning.\n"]
    pub fn cloud_sql(&self) -> ListRef<BigqueryConnectionCloudSqlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_sql", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> ListRef<BigqueryConnectionSparkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spark", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryConnectionTimeoutsElRef {
        BigqueryConnectionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for BigqueryConnection {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BigqueryConnection { }

impl ToListMappable for BigqueryConnection {
    type O = ListRef<BigqueryConnectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BigqueryConnection_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_connection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryConnection {
    pub tf_id: String,
}

impl BuildBigqueryConnection {
    pub fn build(self, stack: &mut Stack) -> BigqueryConnection {
        let out = BigqueryConnection(Rc::new(BigqueryConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryConnectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryConnectionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `connection_id` after provisioning.\nOptional connection id that should be assigned to the created connection."]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA descriptive description for the connection"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the connection"]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `has_credential` after provisioning.\nTrue if the connection has credential assigned."]
    pub fn has_credential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.has_credential", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the connection should reside.\nCloud SQL instance must be in the same location as the connection\nwith following exceptions: Cloud SQL us-central1 maps to BigQuery US, Cloud SQL europe-west1 maps to BigQuery EU.\nExamples: US, EU, asia-northeast1, us-central1, europe-west1.\nSpanner Connections same as spanner region\nAWS allowed regions are aws-us-east-1\nAzure allowed regions are azure-eastus2"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the connection in the form of:\n\"projects/{project_id}/locations/{location_id}/connections/{connectionId}\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `aws` after provisioning.\n"]
    pub fn aws(&self) -> ListRef<BigqueryConnectionAwsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.aws", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `azure` after provisioning.\n"]
    pub fn azure(&self) -> ListRef<BigqueryConnectionAzureElRef> {
        ListRef::new(self.shared().clone(), format!("{}.azure", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloud_resource` after provisioning.\n"]
    pub fn cloud_resource(&self) -> ListRef<BigqueryConnectionCloudResourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_resource", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloud_spanner` after provisioning.\n"]
    pub fn cloud_spanner(&self) -> ListRef<BigqueryConnectionCloudSpannerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_spanner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloud_sql` after provisioning.\n"]
    pub fn cloud_sql(&self) -> ListRef<BigqueryConnectionCloudSqlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_sql", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> ListRef<BigqueryConnectionSparkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spark", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryConnectionTimeoutsElRef {
        BigqueryConnectionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionAwsElAccessRoleEl {
    iam_role_id: PrimField<String>,
}

impl BigqueryConnectionAwsElAccessRoleEl { }

impl ToListMappable for BigqueryConnectionAwsElAccessRoleEl {
    type O = BlockAssignable<BigqueryConnectionAwsElAccessRoleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionAwsElAccessRoleEl {
    #[doc= "The user’s AWS IAM Role that trusts the Google-owned AWS IAM user Connection."]
    pub iam_role_id: PrimField<String>,
}

impl BuildBigqueryConnectionAwsElAccessRoleEl {
    pub fn build(self) -> BigqueryConnectionAwsElAccessRoleEl {
        BigqueryConnectionAwsElAccessRoleEl { iam_role_id: self.iam_role_id }
    }
}

pub struct BigqueryConnectionAwsElAccessRoleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionAwsElAccessRoleElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionAwsElAccessRoleElRef {
        BigqueryConnectionAwsElAccessRoleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionAwsElAccessRoleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `iam_role_id` after provisioning.\nThe user’s AWS IAM Role that trusts the Google-owned AWS IAM user Connection."]
    pub fn iam_role_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_role_id", self.base))
    }

    #[doc= "Get a reference to the value of field `identity` after provisioning.\nA unique Google-owned and Google-generated identity for the Connection. This identity will be used to access the user's AWS IAM Role."]
    pub fn identity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnectionAwsElDynamic {
    access_role: Option<DynamicBlock<BigqueryConnectionAwsElAccessRoleEl>>,
}

#[derive(Serialize)]
pub struct BigqueryConnectionAwsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_role: Option<Vec<BigqueryConnectionAwsElAccessRoleEl>>,
    dynamic: BigqueryConnectionAwsElDynamic,
}

impl BigqueryConnectionAwsEl {
    #[doc= "Set the field `access_role`.\n"]
    pub fn set_access_role(mut self, v: impl Into<BlockAssignable<BigqueryConnectionAwsElAccessRoleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_role = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_role = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BigqueryConnectionAwsEl {
    type O = BlockAssignable<BigqueryConnectionAwsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct BigqueryConnectionAwsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionAwsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionAwsElRef {
        BigqueryConnectionAwsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionAwsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_role` after provisioning.\n"]
    pub fn access_role(&self) -> ListRef<BigqueryConnectionAwsElAccessRoleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_role", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionAzureEl {
    customer_tenant_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    federated_application_client_id: Option<PrimField<String>>,
}

impl BigqueryConnectionAzureEl {
    #[doc= "Set the field `federated_application_client_id`.\nThe Azure Application (client) ID where the federated credentials will be hosted."]
    pub fn set_federated_application_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.federated_application_client_id = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryConnectionAzureEl {
    type O = BlockAssignable<BigqueryConnectionAzureEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionAzureEl {
    #[doc= "The id of customer's directory that host the data."]
    pub customer_tenant_id: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionAzureElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionAzureElRef {
        BigqueryConnectionAzureElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionAzureElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `application` after provisioning.\nThe name of the Azure Active Directory Application."]
    pub fn application(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application", self.base))
    }

    #[doc= "Get a reference to the value of field `client_id` after provisioning.\nThe client id of the Azure Active Directory Application."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc= "Get a reference to the value of field `customer_tenant_id` after provisioning.\nThe id of customer's directory that host the data."]
    pub fn customer_tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_tenant_id", self.base))
    }

    #[doc= "Get a reference to the value of field `federated_application_client_id` after provisioning.\nThe Azure Application (client) ID where the federated credentials will be hosted."]
    pub fn federated_application_client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.federated_application_client_id", self.base))
    }

    #[doc= "Get a reference to the value of field `identity` after provisioning.\nA unique Google-owned and Google-generated identity for the Connection. This identity will be used to access the user's Azure Active Directory Application."]
    pub fn identity(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity", self.base))
    }

    #[doc= "Get a reference to the value of field `object_id` after provisioning.\nThe object id of the Azure Active Directory Application."]
    pub fn object_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_id", self.base))
    }

    #[doc= "Get a reference to the value of field `redirect_uri` after provisioning.\nThe URL user will be redirected to after granting consent during connection setup."]
    pub fn redirect_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionCloudResourceEl {}

impl BigqueryConnectionCloudResourceEl { }

impl ToListMappable for BigqueryConnectionCloudResourceEl {
    type O = BlockAssignable<BigqueryConnectionCloudResourceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionCloudResourceElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionCloudResourceElRef {
        BigqueryConnectionCloudResourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionCloudResourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `service_account_id` after provisioning.\nThe account ID of the service created for the purpose of this connection."]
    pub fn service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionCloudSpannerEl {
    database: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_parallelism: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_data_boost: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_parallelism: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_serverless_analytics: Option<PrimField<bool>>,
}

impl BigqueryConnectionCloudSpannerEl {
    #[doc= "Set the field `database_role`.\nCloud Spanner database role for fine-grained access control. The Cloud Spanner admin should have provisioned the database role with appropriate permissions, such as 'SELECT' and 'INSERT'. Other users should only use roles provided by their Cloud Spanner admins. The database role name must start with a letter, and can only contain letters, numbers, and underscores. For more details, see https://cloud.google.com/spanner/docs/fgac-about."]
    pub fn set_database_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_role = Some(v.into());
        self
    }

    #[doc= "Set the field `max_parallelism`.\nAllows setting max parallelism per query when executing on Spanner independent compute resources. If unspecified, default values of parallelism are chosen that are dependent on the Cloud Spanner instance configuration. 'useParallelism' and 'useDataBoost' must be set when setting max parallelism."]
    pub fn set_max_parallelism(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_parallelism = Some(v.into());
        self
    }

    #[doc= "Set the field `use_data_boost`.\nIf set, the request will be executed via Spanner independent compute resources. 'use_parallelism' must be set when using data boost."]
    pub fn set_use_data_boost(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_data_boost = Some(v.into());
        self
    }

    #[doc= "Set the field `use_parallelism`.\nIf parallelism should be used when reading from Cloud Spanner."]
    pub fn set_use_parallelism(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_parallelism = Some(v.into());
        self
    }

    #[doc= "Set the field `use_serverless_analytics`.\nIf the serverless analytics service should be used to read data from Cloud Spanner. 'useParallelism' must be set when using serverless analytics."]
    pub fn set_use_serverless_analytics(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_serverless_analytics = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryConnectionCloudSpannerEl {
    type O = BlockAssignable<BigqueryConnectionCloudSpannerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionCloudSpannerEl {
    #[doc= "Cloud Spanner database in the form 'project/instance/database'."]
    pub database: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionCloudSpannerElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionCloudSpannerElRef {
        BigqueryConnectionCloudSpannerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionCloudSpannerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\nCloud Spanner database in the form 'project/instance/database'."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc= "Get a reference to the value of field `database_role` after provisioning.\nCloud Spanner database role for fine-grained access control. The Cloud Spanner admin should have provisioned the database role with appropriate permissions, such as 'SELECT' and 'INSERT'. Other users should only use roles provided by their Cloud Spanner admins. The database role name must start with a letter, and can only contain letters, numbers, and underscores. For more details, see https://cloud.google.com/spanner/docs/fgac-about."]
    pub fn database_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_role", self.base))
    }

    #[doc= "Get a reference to the value of field `max_parallelism` after provisioning.\nAllows setting max parallelism per query when executing on Spanner independent compute resources. If unspecified, default values of parallelism are chosen that are dependent on the Cloud Spanner instance configuration. 'useParallelism' and 'useDataBoost' must be set when setting max parallelism."]
    pub fn max_parallelism(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_parallelism", self.base))
    }

    #[doc= "Get a reference to the value of field `use_data_boost` after provisioning.\nIf set, the request will be executed via Spanner independent compute resources. 'use_parallelism' must be set when using data boost."]
    pub fn use_data_boost(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_data_boost", self.base))
    }

    #[doc= "Get a reference to the value of field `use_parallelism` after provisioning.\nIf parallelism should be used when reading from Cloud Spanner."]
    pub fn use_parallelism(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_parallelism", self.base))
    }

    #[doc= "Get a reference to the value of field `use_serverless_analytics` after provisioning.\nIf the serverless analytics service should be used to read data from Cloud Spanner. 'useParallelism' must be set when using serverless analytics."]
    pub fn use_serverless_analytics(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_serverless_analytics", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionCloudSqlElCredentialEl {
    password: PrimField<String>,
    username: PrimField<String>,
}

impl BigqueryConnectionCloudSqlElCredentialEl { }

impl ToListMappable for BigqueryConnectionCloudSqlElCredentialEl {
    type O = BlockAssignable<BigqueryConnectionCloudSqlElCredentialEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionCloudSqlElCredentialEl {
    #[doc= "Password for database."]
    pub password: PrimField<String>,
    #[doc= "Username for database."]
    pub username: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionCloudSqlElCredentialElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionCloudSqlElCredentialElRef {
        BigqueryConnectionCloudSqlElCredentialElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionCloudSqlElCredentialElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nPassword for database."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nUsername for database."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnectionCloudSqlElDynamic {
    credential: Option<DynamicBlock<BigqueryConnectionCloudSqlElCredentialEl>>,
}

#[derive(Serialize)]
pub struct BigqueryConnectionCloudSqlEl {
    database: PrimField<String>,
    instance_id: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credential: Option<Vec<BigqueryConnectionCloudSqlElCredentialEl>>,
    dynamic: BigqueryConnectionCloudSqlElDynamic,
}

impl BigqueryConnectionCloudSqlEl {
    #[doc= "Set the field `credential`.\n"]
    pub fn set_credential(mut self, v: impl Into<BlockAssignable<BigqueryConnectionCloudSqlElCredentialEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.credential = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.credential = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BigqueryConnectionCloudSqlEl {
    type O = BlockAssignable<BigqueryConnectionCloudSqlEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryConnectionCloudSqlEl {
    #[doc= "Database name."]
    pub database: PrimField<String>,
    #[doc= "Cloud SQL instance ID in the form project:location:instance."]
    pub instance_id: PrimField<String>,
    #[doc= "Type of the Cloud SQL database. Possible values: [\"DATABASE_TYPE_UNSPECIFIED\", \"POSTGRES\", \"MYSQL\"]"]
    pub type_: PrimField<String>,
}

impl BuildBigqueryConnectionCloudSqlEl {
    pub fn build(self) -> BigqueryConnectionCloudSqlEl {
        BigqueryConnectionCloudSqlEl {
            database: self.database,
            instance_id: self.instance_id,
            type_: self.type_,
            credential: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BigqueryConnectionCloudSqlElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionCloudSqlElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionCloudSqlElRef {
        BigqueryConnectionCloudSqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionCloudSqlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\nCloud SQL instance ID in the form project:location:instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc= "Get a reference to the value of field `service_account_id` after provisioning.\nWhen the connection is used in the context of an operation in BigQuery, this service account will serve as the identity being used for connecting to the CloudSQL instance specified in this connection."]
    pub fn service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_id", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nType of the Cloud SQL database. Possible values: [\"DATABASE_TYPE_UNSPECIFIED\", \"POSTGRES\", \"MYSQL\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `credential` after provisioning.\n"]
    pub fn credential(&self) -> ListRef<BigqueryConnectionCloudSqlElCredentialElRef> {
        ListRef::new(self.shared().clone(), format!("{}.credential", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionSparkElMetastoreServiceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metastore_service: Option<PrimField<String>>,
}

impl BigqueryConnectionSparkElMetastoreServiceConfigEl {
    #[doc= "Set the field `metastore_service`.\nResource name of an existing Dataproc Metastore service in the form of projects/[projectId]/locations/[region]/services/[serviceId]."]
    pub fn set_metastore_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metastore_service = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryConnectionSparkElMetastoreServiceConfigEl {
    type O = BlockAssignable<BigqueryConnectionSparkElMetastoreServiceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionSparkElMetastoreServiceConfigElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionSparkElMetastoreServiceConfigElRef {
        BigqueryConnectionSparkElMetastoreServiceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionSparkElMetastoreServiceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `metastore_service` after provisioning.\nResource name of an existing Dataproc Metastore service in the form of projects/[projectId]/locations/[region]/services/[serviceId]."]
    pub fn metastore_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metastore_service", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionSparkElSparkHistoryServerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataproc_cluster: Option<PrimField<String>>,
}

impl BigqueryConnectionSparkElSparkHistoryServerConfigEl {
    #[doc= "Set the field `dataproc_cluster`.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the connection if the form of projects/[projectId]/regions/[region]/clusters/[cluster_name]."]
    pub fn set_dataproc_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataproc_cluster = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryConnectionSparkElSparkHistoryServerConfigEl {
    type O = BlockAssignable<BigqueryConnectionSparkElSparkHistoryServerConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionSparkElSparkHistoryServerConfigElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionSparkElSparkHistoryServerConfigElRef {
        BigqueryConnectionSparkElSparkHistoryServerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionSparkElSparkHistoryServerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dataproc_cluster` after provisioning.\nResource name of an existing Dataproc Cluster to act as a Spark History Server for the connection if the form of projects/[projectId]/regions/[region]/clusters/[cluster_name]."]
    pub fn dataproc_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataproc_cluster", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnectionSparkElDynamic {
    metastore_service_config: Option<DynamicBlock<BigqueryConnectionSparkElMetastoreServiceConfigEl>>,
    spark_history_server_config: Option<DynamicBlock<BigqueryConnectionSparkElSparkHistoryServerConfigEl>>,
}

#[derive(Serialize)]
pub struct BigqueryConnectionSparkEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metastore_service_config: Option<Vec<BigqueryConnectionSparkElMetastoreServiceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_history_server_config: Option<Vec<BigqueryConnectionSparkElSparkHistoryServerConfigEl>>,
    dynamic: BigqueryConnectionSparkElDynamic,
}

impl BigqueryConnectionSparkEl {
    #[doc= "Set the field `metastore_service_config`.\n"]
    pub fn set_metastore_service_config(
        mut self,
        v: impl Into<BlockAssignable<BigqueryConnectionSparkElMetastoreServiceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metastore_service_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metastore_service_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `spark_history_server_config`.\n"]
    pub fn set_spark_history_server_config(
        mut self,
        v: impl Into<BlockAssignable<BigqueryConnectionSparkElSparkHistoryServerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spark_history_server_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spark_history_server_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BigqueryConnectionSparkEl {
    type O = BlockAssignable<BigqueryConnectionSparkEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct BigqueryConnectionSparkElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionSparkElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionSparkElRef {
        BigqueryConnectionSparkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionSparkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `service_account_id` after provisioning.\nThe account ID of the service created for the purpose of this connection."]
    pub fn service_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_id", self.base))
    }

    #[doc= "Get a reference to the value of field `metastore_service_config` after provisioning.\n"]
    pub fn metastore_service_config(&self) -> ListRef<BigqueryConnectionSparkElMetastoreServiceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.metastore_service_config", self.base))
    }

    #[doc= "Get a reference to the value of field `spark_history_server_config` after provisioning.\n"]
    pub fn spark_history_server_config(&self) -> ListRef<BigqueryConnectionSparkElSparkHistoryServerConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spark_history_server_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BigqueryConnectionTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryConnectionTimeoutsEl {
    type O = BlockAssignable<BigqueryConnectionTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryConnectionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryConnectionTimeoutsElRef {
        BigqueryConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryConnectionTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryConnectionDynamic {
    aws: Option<DynamicBlock<BigqueryConnectionAwsEl>>,
    azure: Option<DynamicBlock<BigqueryConnectionAzureEl>>,
    cloud_resource: Option<DynamicBlock<BigqueryConnectionCloudResourceEl>>,
    cloud_spanner: Option<DynamicBlock<BigqueryConnectionCloudSpannerEl>>,
    cloud_sql: Option<DynamicBlock<BigqueryConnectionCloudSqlEl>>,
    spark: Option<DynamicBlock<BigqueryConnectionSparkEl>>,
}
