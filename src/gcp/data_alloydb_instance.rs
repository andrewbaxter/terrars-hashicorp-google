#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataAlloydbInstanceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAlloydbInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAlloydbInstanceData_>,
}

#[derive(Clone)]
pub struct DataAlloydbInstance(std::rc::Rc<DataAlloydbInstance_>);

impl DataAlloydbInstance {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `location`.\nThe canonical ID for the location. For example: \"us-east1\"."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nProject ID of the project."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `activation_policy` after provisioning.\n'Specifies whether an instance needs to spin up. Once the instance is\nactive, the activation policy can be updated to the 'NEVER' to stop the\ninstance. Likewise, the activation policy can be updated to 'ALWAYS' to\nstart the instance.\nThere are restrictions around when an instance can/cannot be activated (for\nexample, a read pool instance should be stopped before stopping primary\netc.). Please refer to the API documentation for more details.\nPossible values are: 'ACTIVATION_POLICY_UNSPECIFIED', 'ALWAYS', 'NEVER'.' Possible values: [\"ACTIVATION_POLICY_UNSPECIFIED\", \"ALWAYS\", \"NEVER\"]"]
    pub fn activation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_type` after provisioning.\n'Availability type of a primary Instance. Defaults to REGIONAL.\nPrimary instances can be either ZONAL or REGIONAL. Read Pool instances\nderive their availability from 'read_pool_config.node_count': a pool\nwith one node is ZONAL, while a pool with two or more nodes is REGIONAL.\nRead Pool instances do not support setting this field directly.\nPossible values are: 'AVAILABILITY_TYPE_UNSPECIFIED', 'ZONAL', 'REGIONAL'.' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_connection_config` after provisioning.\nClient connection specific configurations."]
    pub fn client_connection_config(&self) -> terrars::ListRef<DataAlloydbInstance_ClientConnectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connection_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nIdentifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID of the alloydb cluster that the instance belongs to.'alloydb_cluster_id'"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_pool_config` after provisioning.\nConfiguration for Managed Connection Pool."]
    pub fn connection_pool_config(&self) -> terrars::ListRef<DataAlloydbInstance_ConnectionPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_pool_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn database_flags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.database_flags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Instance."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gce_zone` after provisioning.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn gce_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gce_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the alloydb instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe canonical ID for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `machine_config` after provisioning.\nConfigurations for the machines that host the underlying database engine."]
    pub fn machine_config(&self) -> terrars::ListRef<DataAlloydbInstance_MachineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_config` after provisioning.\nInstance level network configuration."]
    pub fn network_config(&self) -> terrars::ListRef<DataAlloydbInstance_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `outbound_public_ip_addresses` after provisioning.\nThe outbound public IP addresses for the instance. This is available ONLY when\nnetworkConfig.enableOutboundPublicIp is set to true. These IP addresses are used\nfor outbound connections."]
    pub fn outbound_public_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_public_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_instance_config` after provisioning.\nConfiguration for Private Service Connect (PSC) for the instance."]
    pub fn psc_instance_config(&self) -> terrars::ListRef<DataAlloydbInstance_PscInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_instance_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_address` after provisioning.\nThe public IP addresses for the Instance. This is available ONLY when\nnetworkConfig.enablePublicIp is set to true. This is the connection\nendpoint for an end-user application."]
    pub fn public_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `query_insights_config` after provisioning.\nConfiguration for query insights."]
    pub fn query_insights_config(&self) -> terrars::ListRef<DataAlloydbInstance_QueryInsightsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_insights_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_pool_config` after provisioning.\nRead pool specific config. If the instance type is READ_POOL, this configuration must be provided."]
    pub fn read_pool_config(&self) -> terrars::ListRef<DataAlloydbInstance_ReadPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_pool_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nSet to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the alloydb instance."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataAlloydbInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAlloydbInstance { }

impl terrars::ToListMappable for DataAlloydbInstance {
    type O = terrars::ListRef<DataAlloydbInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAlloydbInstance_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_alloydb_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAlloydbInstance {
    pub tf_id: std::string::String,
    #[doc = "The ID of the alloydb cluster that the instance belongs to.'alloydb_cluster_id'"]
    pub cluster_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the alloydb instance."]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildDataAlloydbInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAlloydbInstance {
        let out = DataAlloydbInstance(std::rc::Rc::new(DataAlloydbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAlloydbInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAlloydbInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAlloydbInstanceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `activation_policy` after provisioning.\n'Specifies whether an instance needs to spin up. Once the instance is\nactive, the activation policy can be updated to the 'NEVER' to stop the\ninstance. Likewise, the activation policy can be updated to 'ALWAYS' to\nstart the instance.\nThere are restrictions around when an instance can/cannot be activated (for\nexample, a read pool instance should be stopped before stopping primary\netc.). Please refer to the API documentation for more details.\nPossible values are: 'ACTIVATION_POLICY_UNSPECIFIED', 'ALWAYS', 'NEVER'.' Possible values: [\"ACTIVATION_POLICY_UNSPECIFIED\", \"ALWAYS\", \"NEVER\"]"]
    pub fn activation_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `availability_type` after provisioning.\n'Availability type of a primary Instance. Defaults to REGIONAL.\nPrimary instances can be either ZONAL or REGIONAL. Read Pool instances\nderive their availability from 'read_pool_config.node_count': a pool\nwith one node is ZONAL, while a pool with two or more nodes is REGIONAL.\nRead Pool instances do not support setting this field directly.\nPossible values are: 'AVAILABILITY_TYPE_UNSPECIFIED', 'ZONAL', 'REGIONAL'.' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn availability_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_connection_config` after provisioning.\nClient connection specific configurations."]
    pub fn client_connection_config(&self) -> terrars::ListRef<DataAlloydbInstance_ClientConnectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_connection_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nIdentifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nThe ID of the alloydb cluster that the instance belongs to.'alloydb_cluster_id'"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_pool_config` after provisioning.\nConfiguration for Managed Connection Pool."]
    pub fn connection_pool_config(&self) -> terrars::ListRef<DataAlloydbInstance_ConnectionPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_pool_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn database_flags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.database_flags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Instance."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gce_zone` after provisioning.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn gce_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gce_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the alloydb instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe canonical ID for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `machine_config` after provisioning.\nConfigurations for the machines that host the underlying database engine."]
    pub fn machine_config(&self) -> terrars::ListRef<DataAlloydbInstance_MachineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_config` after provisioning.\nInstance level network configuration."]
    pub fn network_config(&self) -> terrars::ListRef<DataAlloydbInstance_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `outbound_public_ip_addresses` after provisioning.\nThe outbound public IP addresses for the instance. This is available ONLY when\nnetworkConfig.enableOutboundPublicIp is set to true. These IP addresses are used\nfor outbound connections."]
    pub fn outbound_public_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_public_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_instance_config` after provisioning.\nConfiguration for Private Service Connect (PSC) for the instance."]
    pub fn psc_instance_config(&self) -> terrars::ListRef<DataAlloydbInstance_PscInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_instance_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_address` after provisioning.\nThe public IP addresses for the Instance. This is available ONLY when\nnetworkConfig.enablePublicIp is set to true. This is the connection\nendpoint for an end-user application."]
    pub fn public_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `query_insights_config` after provisioning.\nConfiguration for query insights."]
    pub fn query_insights_config(&self) -> terrars::ListRef<DataAlloydbInstance_QueryInsightsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_insights_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_pool_config` after provisioning.\nRead pool specific config. If the instance type is READ_POOL, this configuration must be provided."]
    pub fn read_pool_config(&self) -> terrars::ListRef<DataAlloydbInstance_ReadPoolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.read_pool_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nSet to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the alloydb instance."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_ClientConnectionConfig_SslConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbInstance_ClientConnectionConfig_SslConfig {
    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_ClientConnectionConfig_SslConfig {
    type O = terrars::BlockAssignable<DataAlloydbInstance_ClientConnectionConfig_SslConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_ClientConnectionConfig_SslConfig {}

impl BuildDataAlloydbInstance_ClientConnectionConfig_SslConfig {
    pub fn build(self) -> DataAlloydbInstance_ClientConnectionConfig_SslConfig {
        DataAlloydbInstance_ClientConnectionConfig_SslConfig { ssl_mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_ClientConnectionConfig_SslConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_ClientConnectionConfig_SslConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbInstance_ClientConnectionConfig_SslConfigRef {
        DataAlloydbInstance_ClientConnectionConfig_SslConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_ClientConnectionConfig_SslConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_ClientConnectionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_connectors: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_config: std::option::Option<terrars::ListField<DataAlloydbInstance_ClientConnectionConfig_SslConfig>>,
}

impl DataAlloydbInstance_ClientConnectionConfig {
    #[doc = "Set the field `require_connectors`.\n"]
    pub fn set_require_connectors(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_connectors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlloydbInstance_ClientConnectionConfig_SslConfig>>,
    ) -> Self {
        self.ssl_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_ClientConnectionConfig {
    type O = terrars::BlockAssignable<DataAlloydbInstance_ClientConnectionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_ClientConnectionConfig {}

impl BuildDataAlloydbInstance_ClientConnectionConfig {
    pub fn build(self) -> DataAlloydbInstance_ClientConnectionConfig {
        DataAlloydbInstance_ClientConnectionConfig {
            require_connectors: core::default::Default::default(),
            ssl_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_ClientConnectionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_ClientConnectionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbInstance_ClientConnectionConfigRef {
        DataAlloydbInstance_ClientConnectionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_ClientConnectionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `require_connectors` after provisioning.\n"]
    pub fn require_connectors(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_connectors", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<DataAlloydbInstance_ClientConnectionConfig_SslConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_ConnectionPoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pooler_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAlloydbInstance_ConnectionPoolConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flags`.\n"]
    pub fn set_flags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.flags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pooler_count`.\n"]
    pub fn set_pooler_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pooler_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_ConnectionPoolConfig {
    type O = terrars::BlockAssignable<DataAlloydbInstance_ConnectionPoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_ConnectionPoolConfig {}

impl BuildDataAlloydbInstance_ConnectionPoolConfig {
    pub fn build(self) -> DataAlloydbInstance_ConnectionPoolConfig {
        DataAlloydbInstance_ConnectionPoolConfig {
            enabled: core::default::Default::default(),
            flags: core::default::Default::default(),
            pooler_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_ConnectionPoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_ConnectionPoolConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbInstance_ConnectionPoolConfigRef {
        DataAlloydbInstance_ConnectionPoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_ConnectionPoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `flags` after provisioning.\n"]
    pub fn flags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.flags", self.base))
    }

    #[doc = "Get a reference to the value of field `pooler_count` after provisioning.\n"]
    pub fn pooler_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pooler_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_MachineConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbInstance_MachineConfig {
    #[doc = "Set the field `cpu_count`.\n"]
    pub fn set_cpu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_MachineConfig {
    type O = terrars::BlockAssignable<DataAlloydbInstance_MachineConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_MachineConfig {}

impl BuildDataAlloydbInstance_MachineConfig {
    pub fn build(self) -> DataAlloydbInstance_MachineConfig {
        DataAlloydbInstance_MachineConfig {
            cpu_count: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_MachineConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_MachineConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbInstance_MachineConfigRef {
        DataAlloydbInstance_MachineConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_MachineConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\n"]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_range: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
    #[doc = "Set the field `cidr_range`.\n"]
    pub fn set_cidr_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
    type O = terrars::BlockAssignable<DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {}

impl BuildDataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
    pub fn build(self) -> DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
        DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks {
            cidr_range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef {
        DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_range` after provisioning.\n"]
    pub fn cidr_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocated_ip_range_override: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_external_networks: std
    ::option
    ::Option<terrars::ListField<DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_outbound_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_public_ip: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAlloydbInstance_NetworkConfig {
    #[doc = "Set the field `allocated_ip_range_override`.\n"]
    pub fn set_allocated_ip_range_override(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allocated_ip_range_override = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorized_external_networks`.\n"]
    pub fn set_authorized_external_networks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworks>>,
    ) -> Self {
        self.authorized_external_networks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_outbound_public_ip`.\n"]
    pub fn set_enable_outbound_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_outbound_public_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_public_ip`.\n"]
    pub fn set_enable_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_public_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_NetworkConfig {
    type O = terrars::BlockAssignable<DataAlloydbInstance_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_NetworkConfig {}

impl BuildDataAlloydbInstance_NetworkConfig {
    pub fn build(self) -> DataAlloydbInstance_NetworkConfig {
        DataAlloydbInstance_NetworkConfig {
            allocated_ip_range_override: core::default::Default::default(),
            authorized_external_networks: core::default::Default::default(),
            enable_outbound_public_ip: core::default::Default::default(),
            enable_public_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbInstance_NetworkConfigRef {
        DataAlloydbInstance_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocated_ip_range_override` after provisioning.\n"]
    pub fn allocated_ip_range_override(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_ip_range_override", self.base))
    }

    #[doc = "Get a reference to the value of field `authorized_external_networks` after provisioning.\n"]
    pub fn authorized_external_networks(
        &self,
    ) -> terrars::ListRef<DataAlloydbInstance_NetworkConfig_AuthorizedExternalNetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_external_networks", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_outbound_public_ip` after provisioning.\n"]
    pub fn enable_outbound_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_outbound_public_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_public_ip` after provisioning.\n"]
    pub fn enable_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_public_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_PscInstanceConfig_PscAutoConnections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_network_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbInstance_PscInstanceConfig_PscAutoConnections {
    #[doc = "Set the field `consumer_network`.\n"]
    pub fn set_consumer_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_network_status`.\n"]
    pub fn set_consumer_network_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consumer_network_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_project`.\n"]
    pub fn set_consumer_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_PscInstanceConfig_PscAutoConnections {
    type O = terrars::BlockAssignable<DataAlloydbInstance_PscInstanceConfig_PscAutoConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_PscInstanceConfig_PscAutoConnections {}

impl BuildDataAlloydbInstance_PscInstanceConfig_PscAutoConnections {
    pub fn build(self) -> DataAlloydbInstance_PscInstanceConfig_PscAutoConnections {
        DataAlloydbInstance_PscInstanceConfig_PscAutoConnections {
            consumer_network: core::default::Default::default(),
            consumer_network_status: core::default::Default::default(),
            consumer_project: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef {
        DataAlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_network` after provisioning.\n"]
    pub fn consumer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_network_status` after provisioning.\n"]
    pub fn consumer_network_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network_status", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_project` after provisioning.\n"]
    pub fn consumer_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_project", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment_resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
    #[doc = "Set the field `network_attachment_resource`.\n"]
    pub fn set_network_attachment_resource(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment_resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
    type O = terrars::BlockAssignable<DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {}

impl BuildDataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
    pub fn build(self) -> DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
        DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs {
            network_attachment_resource: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef {
        DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_attachment_resource` after provisioning.\n"]
    pub fn network_attachment_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_PscInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_consumer_projects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_auto_connections: std
    ::option
    ::Option<terrars::ListField<DataAlloydbInstance_PscInstanceConfig_PscAutoConnections>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_interface_configs: std
    ::option
    ::Option<terrars::ListField<DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment_link: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAlloydbInstance_PscInstanceConfig {
    #[doc = "Set the field `allowed_consumer_projects`.\n"]
    pub fn set_allowed_consumer_projects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_consumer_projects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_auto_connections`.\n"]
    pub fn set_psc_auto_connections(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlloydbInstance_PscInstanceConfig_PscAutoConnections>>,
    ) -> Self {
        self.psc_auto_connections = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_dns_name`.\n"]
    pub fn set_psc_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.psc_dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_interface_configs`.\n"]
    pub fn set_psc_interface_configs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigs>>,
    ) -> Self {
        self.psc_interface_configs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_attachment_link`.\n"]
    pub fn set_service_attachment_link(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_attachment_link = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_PscInstanceConfig {
    type O = terrars::BlockAssignable<DataAlloydbInstance_PscInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_PscInstanceConfig {}

impl BuildDataAlloydbInstance_PscInstanceConfig {
    pub fn build(self) -> DataAlloydbInstance_PscInstanceConfig {
        DataAlloydbInstance_PscInstanceConfig {
            allowed_consumer_projects: core::default::Default::default(),
            psc_auto_connections: core::default::Default::default(),
            psc_dns_name: core::default::Default::default(),
            psc_interface_configs: core::default::Default::default(),
            service_attachment_link: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_PscInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_PscInstanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbInstance_PscInstanceConfigRef {
        DataAlloydbInstance_PscInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_PscInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_consumer_projects` after provisioning.\n"]
    pub fn allowed_consumer_projects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_consumer_projects", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\n"]
    pub fn psc_auto_connections(&self) -> terrars::ListRef<DataAlloydbInstance_PscInstanceConfig_PscAutoConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_auto_connections", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_dns_name` after provisioning.\n"]
    pub fn psc_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_interface_configs` after provisioning.\n"]
    pub fn psc_interface_configs(
        &self,
    ) -> terrars::ListRef<DataAlloydbInstance_PscInstanceConfig_PscInterfaceConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_interface_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `service_attachment_link` after provisioning.\n"]
    pub fn service_attachment_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment_link", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_QueryInsightsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_plans_per_minute: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_application_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_client_address: std::option::Option<terrars::PrimField<bool>>,
}

impl DataAlloydbInstance_QueryInsightsConfig {
    #[doc = "Set the field `query_plans_per_minute`.\n"]
    pub fn set_query_plans_per_minute(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_plans_per_minute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string_length`.\n"]
    pub fn set_query_string_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.query_string_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_application_tags`.\n"]
    pub fn set_record_application_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.record_application_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `record_client_address`.\n"]
    pub fn set_record_client_address(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.record_client_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_QueryInsightsConfig {
    type O = terrars::BlockAssignable<DataAlloydbInstance_QueryInsightsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_QueryInsightsConfig {}

impl BuildDataAlloydbInstance_QueryInsightsConfig {
    pub fn build(self) -> DataAlloydbInstance_QueryInsightsConfig {
        DataAlloydbInstance_QueryInsightsConfig {
            query_plans_per_minute: core::default::Default::default(),
            query_string_length: core::default::Default::default(),
            record_application_tags: core::default::Default::default(),
            record_client_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_QueryInsightsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_QueryInsightsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbInstance_QueryInsightsConfigRef {
        DataAlloydbInstance_QueryInsightsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_QueryInsightsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_plans_per_minute` after provisioning.\n"]
    pub fn query_plans_per_minute(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_plans_per_minute", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string_length` after provisioning.\n"]
    pub fn query_string_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_string_length", self.base))
    }

    #[doc = "Get a reference to the value of field `record_application_tags` after provisioning.\n"]
    pub fn record_application_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_application_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `record_client_address` after provisioning.\n"]
    pub fn record_client_address(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.record_client_address", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAlloydbInstance_ReadPoolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAlloydbInstance_ReadPoolConfig {
    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAlloydbInstance_ReadPoolConfig {
    type O = terrars::BlockAssignable<DataAlloydbInstance_ReadPoolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAlloydbInstance_ReadPoolConfig {}

impl BuildDataAlloydbInstance_ReadPoolConfig {
    pub fn build(self) -> DataAlloydbInstance_ReadPoolConfig {
        DataAlloydbInstance_ReadPoolConfig { node_count: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAlloydbInstance_ReadPoolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAlloydbInstance_ReadPoolConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAlloydbInstance_ReadPoolConfigRef {
        DataAlloydbInstance_ReadPoolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAlloydbInstance_ReadPoolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
}
