#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataOracleDatabaseCloudVmClusterData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cloud_vm_cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOracleDatabaseCloudVmCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOracleDatabaseCloudVmClusterData_>,
}

#[derive(Clone)]
pub struct DataOracleDatabaseCloudVmCluster(std::rc::Rc<DataOracleDatabaseCloudVmCluster_>);

impl DataOracleDatabaseCloudVmCluster {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `backup_odb_subnet` after provisioning.\nThe name of the backup OdbSubnet associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn backup_odb_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_odb_subnet", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\nCIDR range of the backup subnet."]
    pub fn backup_subnet_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_subnet_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cidr` after provisioning.\nNetwork settings. CIDR to use for cluster IP allocation."]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_vm_cluster_id` after provisioning.\nThe ID of the VM Cluster to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub fn cloud_vm_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_vm_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the VM cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the cluster. Deleting this cluster via terraform destroy or terraform apply will only succeed if this field is false in the Terraform state."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser friendly name for this resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_infrastructure` after provisioning.\nThe name of the Exadata Infrastructure resource on which VM cluster\nresource is created, in the following format:\nprojects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure}"]
    pub fn exadata_infrastructure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exadata_infrastructure", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exascale_db_storage_vault` after provisioning.\nThe name of ExascaleDbStorageVault associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub fn exascale_db_storage_vault(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exascale_db_storage_vault", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nGCP location where Oracle Exadata is hosted. It is same as GCP Oracle zone\nof Exadata infrastructure."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity_connector` after provisioning.\nThe identity connector details which will allow OCI to securely access\nthe resources in the customer project."]
    pub fn identity_connector(&self) -> terrars::ListRef<DataOracleDatabaseCloudVmCluster_IdentityConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_connector", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels or tags associated with the VM Cluster. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbNode'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the VM Cluster resource with the format:\nprojects/{project}/locations/{region}/cloudVmClusters/{cloud_vm_cluster}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC network.\nFormat: projects/{project}/global/networks/{network}"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe odb_subnet and backup_odb_subnet."]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the VM Cluster for\nIP allocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn odb_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_subnet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nVarious properties and settings associated with Exadata VM cluster."]
    pub fn properties(&self) -> terrars::ListRef<DataOracleDatabaseCloudVmCluster_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

impl terrars::Referable for DataOracleDatabaseCloudVmCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOracleDatabaseCloudVmCluster { }

impl terrars::ToListMappable for DataOracleDatabaseCloudVmCluster {
    type O = terrars::ListRef<DataOracleDatabaseCloudVmClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOracleDatabaseCloudVmCluster_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_oracle_database_cloud_vm_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOracleDatabaseCloudVmCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the VM Cluster to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub cloud_vm_cluster_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbNode'."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataOracleDatabaseCloudVmCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOracleDatabaseCloudVmCluster {
        let out = DataOracleDatabaseCloudVmCluster(std::rc::Rc::new(DataOracleDatabaseCloudVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOracleDatabaseCloudVmClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cloud_vm_cluster_id: self.cloud_vm_cluster_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOracleDatabaseCloudVmClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudVmClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOracleDatabaseCloudVmClusterRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `backup_odb_subnet` after provisioning.\nThe name of the backup OdbSubnet associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn backup_odb_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_odb_subnet", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backup_subnet_cidr` after provisioning.\nCIDR range of the backup subnet."]
    pub fn backup_subnet_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_subnet_cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cidr` after provisioning.\nNetwork settings. CIDR to use for cluster IP allocation."]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_vm_cluster_id` after provisioning.\nThe ID of the VM Cluster to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub fn cloud_vm_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_vm_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the VM cluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the cluster. Deleting this cluster via terraform destroy or terraform apply will only succeed if this field is false in the Terraform state."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser friendly name for this resource."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_infrastructure` after provisioning.\nThe name of the Exadata Infrastructure resource on which VM cluster\nresource is created, in the following format:\nprojects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure}"]
    pub fn exadata_infrastructure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exadata_infrastructure", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exascale_db_storage_vault` after provisioning.\nThe name of ExascaleDbStorageVault associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub fn exascale_db_storage_vault(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exascale_db_storage_vault", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nGCP location where Oracle Exadata is hosted. It is same as GCP Oracle zone\nof Exadata infrastructure."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity_connector` after provisioning.\nThe identity connector details which will allow OCI to securely access\nthe resources in the customer project."]
    pub fn identity_connector(&self) -> terrars::ListRef<DataOracleDatabaseCloudVmCluster_IdentityConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_connector", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels or tags associated with the VM Cluster. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbNode'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the VM Cluster resource with the format:\nprojects/{project}/locations/{region}/cloudVmClusters/{cloud_vm_cluster}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC network.\nFormat: projects/{project}/global/networks/{network}"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe odb_subnet and backup_odb_subnet."]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the VM Cluster for\nIP allocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn odb_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_subnet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nVarious properties and settings associated with Exadata VM cluster."]
    pub fn properties(&self) -> terrars::ListRef<DataOracleDatabaseCloudVmCluster_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmCluster_IdentityConnector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOracleDatabaseCloudVmCluster_IdentityConnector {
    #[doc = "Set the field `connection_state`.\n"]
    pub fn set_connection_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_agent_email`.\n"]
    pub fn set_service_agent_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_agent_email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudVmCluster_IdentityConnector {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudVmCluster_IdentityConnector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudVmCluster_IdentityConnector {}

impl BuildDataOracleDatabaseCloudVmCluster_IdentityConnector {
    pub fn build(self) -> DataOracleDatabaseCloudVmCluster_IdentityConnector {
        DataOracleDatabaseCloudVmCluster_IdentityConnector {
            connection_state: core::default::Default::default(),
            service_agent_email: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudVmCluster_IdentityConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudVmCluster_IdentityConnectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudVmCluster_IdentityConnectorRef {
        DataOracleDatabaseCloudVmCluster_IdentityConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudVmCluster_IdentityConnectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_state` after provisioning.\n"]
    pub fn connection_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_state", self.base))
    }

    #[doc = "Get a reference to the value of field `service_agent_email` after provisioning.\n"]
    pub fn service_agent_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_agent_email", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    diagnostics_events_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_monitoring_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incident_logs_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
    #[doc = "Set the field `diagnostics_events_enabled`.\n"]
    pub fn set_diagnostics_events_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.diagnostics_events_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_monitoring_enabled`.\n"]
    pub fn set_health_monitoring_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.health_monitoring_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `incident_logs_enabled`.\n"]
    pub fn set_incident_logs_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.incident_logs_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {}

impl BuildDataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
    pub fn build(self) -> DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
        DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
            diagnostics_events_enabled: core::default::Default::default(),
            health_monitoring_enabled: core::default::Default::default(),
            incident_logs_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef {
        DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `diagnostics_events_enabled` after provisioning.\n"]
    pub fn diagnostics_events_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.diagnostics_events_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `health_monitoring_enabled` after provisioning.\n"]
    pub fn health_monitoring_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_monitoring_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `incident_logs_enabled` after provisioning.\n"]
    pub fn incident_logs_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.incident_logs_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmCluster_Properties_TimeZone {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOracleDatabaseCloudVmCluster_Properties_TimeZone {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudVmCluster_Properties_TimeZone {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudVmCluster_Properties_TimeZone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudVmCluster_Properties_TimeZone {}

impl BuildDataOracleDatabaseCloudVmCluster_Properties_TimeZone {
    pub fn build(self) -> DataOracleDatabaseCloudVmCluster_Properties_TimeZone {
        DataOracleDatabaseCloudVmCluster_Properties_TimeZone {
            id: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudVmCluster_Properties_TimeZoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudVmCluster_Properties_TimeZoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudVmCluster_Properties_TimeZoneRef {
        DataOracleDatabaseCloudVmCluster_Properties_TimeZoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudVmCluster_Properties_TimeZoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudVmCluster_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compartment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_storage_size_tb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_node_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_server_ocids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    diagnostics_data_collection_options: std
    ::option
    ::Option<terrars::ListField<DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_redundancy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_listener_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gi_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_backup_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocpu_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_dns: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_dns_record_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_ip_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_listener_port_tcp: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_listener_port_tcp_ssl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shape: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sparse_diskgroup_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_public_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_management_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::ListField<DataOracleDatabaseCloudVmCluster_Properties_TimeZone>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_backup_storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_file_system_storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOracleDatabaseCloudVmCluster_Properties {
    #[doc = "Set the field `cluster_name`.\n"]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compartment_id`.\n"]
    pub fn set_compartment_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compartment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_core_count`.\n"]
    pub fn set_cpu_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_storage_size_tb`.\n"]
    pub fn set_data_storage_size_tb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_storage_size_tb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_node_storage_size_gb`.\n"]
    pub fn set_db_node_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.db_node_storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_server_ocids`.\n"]
    pub fn set_db_server_ocids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.db_server_ocids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `diagnostics_data_collection_options`.\n"]
    pub fn set_diagnostics_data_collection_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions>,
                    >,
    ) -> Self {
        self.diagnostics_data_collection_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_redundancy`.\n"]
    pub fn set_disk_redundancy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_redundancy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_listener_ip`.\n"]
    pub fn set_dns_listener_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_listener_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gi_version`.\n"]
    pub fn set_gi_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gi_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname_prefix`.\n"]
    pub fn set_hostname_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `license_type`.\n"]
    pub fn set_license_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.license_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_backup_enabled`.\n"]
    pub fn set_local_backup_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.local_backup_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size_gb`.\n"]
    pub fn set_memory_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\n"]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oci_url`.\n"]
    pub fn set_oci_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oci_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ocid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocpu_count`.\n"]
    pub fn set_ocpu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ocpu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_dns`.\n"]
    pub fn set_scan_dns(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scan_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_dns_record_id`.\n"]
    pub fn set_scan_dns_record_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scan_dns_record_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_ip_ids`.\n"]
    pub fn set_scan_ip_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scan_ip_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_listener_port_tcp`.\n"]
    pub fn set_scan_listener_port_tcp(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scan_listener_port_tcp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_listener_port_tcp_ssl`.\n"]
    pub fn set_scan_listener_port_tcp_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scan_listener_port_tcp_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shape`.\n"]
    pub fn set_shape(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shape = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sparse_diskgroup_enabled`.\n"]
    pub fn set_sparse_diskgroup_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sparse_diskgroup_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssh_public_keys`.\n"]
    pub fn set_ssh_public_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ssh_public_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_management_type`.\n"]
    pub fn set_storage_management_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.storage_management_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_size_gb`.\n"]
    pub fn set_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_version`.\n"]
    pub fn set_system_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.system_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOracleDatabaseCloudVmCluster_Properties_TimeZone>>,
    ) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_backup_storage_type`.\n"]
    pub fn set_vm_backup_storage_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vm_backup_storage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_file_system_storage_type`.\n"]
    pub fn set_vm_file_system_storage_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vm_file_system_storage_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudVmCluster_Properties {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudVmCluster_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudVmCluster_Properties {}

impl BuildDataOracleDatabaseCloudVmCluster_Properties {
    pub fn build(self) -> DataOracleDatabaseCloudVmCluster_Properties {
        DataOracleDatabaseCloudVmCluster_Properties {
            cluster_name: core::default::Default::default(),
            compartment_id: core::default::Default::default(),
            cpu_core_count: core::default::Default::default(),
            data_storage_size_tb: core::default::Default::default(),
            db_node_storage_size_gb: core::default::Default::default(),
            db_server_ocids: core::default::Default::default(),
            diagnostics_data_collection_options: core::default::Default::default(),
            disk_redundancy: core::default::Default::default(),
            dns_listener_ip: core::default::Default::default(),
            domain: core::default::Default::default(),
            gi_version: core::default::Default::default(),
            hostname: core::default::Default::default(),
            hostname_prefix: core::default::Default::default(),
            license_type: core::default::Default::default(),
            local_backup_enabled: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            node_count: core::default::Default::default(),
            oci_url: core::default::Default::default(),
            ocid: core::default::Default::default(),
            ocpu_count: core::default::Default::default(),
            scan_dns: core::default::Default::default(),
            scan_dns_record_id: core::default::Default::default(),
            scan_ip_ids: core::default::Default::default(),
            scan_listener_port_tcp: core::default::Default::default(),
            scan_listener_port_tcp_ssl: core::default::Default::default(),
            shape: core::default::Default::default(),
            sparse_diskgroup_enabled: core::default::Default::default(),
            ssh_public_keys: core::default::Default::default(),
            state: core::default::Default::default(),
            storage_management_type: core::default::Default::default(),
            storage_size_gb: core::default::Default::default(),
            system_version: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            vm_backup_storage_type: core::default::Default::default(),
            vm_file_system_storage_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudVmCluster_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudVmCluster_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudVmCluster_PropertiesRef {
        DataOracleDatabaseCloudVmCluster_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudVmCluster_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `compartment_id` after provisioning.\n"]
    pub fn compartment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compartment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\n"]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `data_storage_size_tb` after provisioning.\n"]
    pub fn data_storage_size_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_tb", self.base))
    }

    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\n"]
    pub fn db_node_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `db_server_ocids` after provisioning.\n"]
    pub fn db_server_ocids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_server_ocids", self.base))
    }

    #[doc = "Get a reference to the value of field `diagnostics_data_collection_options` after provisioning.\n"]
    pub fn diagnostics_data_collection_options(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.diagnostics_data_collection_options", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_redundancy` after provisioning.\n"]
    pub fn disk_redundancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_redundancy", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_listener_ip` after provisioning.\n"]
    pub fn dns_listener_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_listener_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `gi_version` after provisioning.\n"]
    pub fn gi_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname_prefix` after provisioning.\n"]
    pub fn hostname_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `license_type` after provisioning.\n"]
    pub fn license_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_type", self.base))
    }

    #[doc = "Get a reference to the value of field `local_backup_enabled` after provisioning.\n"]
    pub fn local_backup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_backup_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\n"]
    pub fn memory_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\n"]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\n"]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `ocpu_count` after provisioning.\n"]
    pub fn ocpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocpu_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_dns` after provisioning.\n"]
    pub fn scan_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_dns_record_id` after provisioning.\n"]
    pub fn scan_dns_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_record_id", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_ip_ids` after provisioning.\n"]
    pub fn scan_ip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\n"]
    pub fn scan_listener_port_tcp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tcp", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_listener_port_tcp_ssl` after provisioning.\n"]
    pub fn scan_listener_port_tcp_ssl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tcp_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\n"]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }

    #[doc = "Get a reference to the value of field `sparse_diskgroup_enabled` after provisioning.\n"]
    pub fn sparse_diskgroup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sparse_diskgroup_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_public_keys` after provisioning.\n"]
    pub fn ssh_public_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_public_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_management_type` after provisioning.\n"]
    pub fn storage_management_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_management_type", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_size_gb` after provisioning.\n"]
    pub fn storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `system_version` after provisioning.\n"]
    pub fn system_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_version", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::ListRef<DataOracleDatabaseCloudVmCluster_Properties_TimeZoneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_backup_storage_type` after provisioning.\n"]
    pub fn vm_backup_storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_backup_storage_type", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_file_system_storage_type` after provisioning.\n"]
    pub fn vm_file_system_storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_file_system_storage_type", self.base))
    }
}
