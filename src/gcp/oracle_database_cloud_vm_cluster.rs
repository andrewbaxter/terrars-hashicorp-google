#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OracleDatabaseCloudVmClusterData_ {
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
    backup_odb_subnet: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_subnet_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    cloud_vm_cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    exadata_infrastructure: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exascale_db_storage_vault: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_subnet: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<std::vec::Vec<OracleDatabaseCloudVmCluster_Properties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OracleDatabaseCloudVmCluster_Timeouts>,
    dynamic: OracleDatabaseCloudVmClusterDynamic,
}

struct OracleDatabaseCloudVmCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OracleDatabaseCloudVmClusterData_>,
}

#[derive(Clone)]
pub struct OracleDatabaseCloudVmCluster(std::rc::Rc<OracleDatabaseCloudVmCluster_>);

impl OracleDatabaseCloudVmCluster {
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
        "Set the field `backup_odb_subnet`.\nThe name of the backup OdbSubnet associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn set_backup_odb_subnet(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().backup_odb_subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_subnet_cidr`.\nCIDR range of the backup subnet."]
    pub fn set_backup_subnet_cidr(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().backup_subnet_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cidr`.\nNetwork settings. CIDR to use for cluster IP allocation."]
    pub fn set_cidr(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the cluster. Deleting this cluster via terraform destroy or terraform apply will only succeed if this field is false in the Terraform state."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser friendly name for this resource."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exascale_db_storage_vault`.\nThe name of ExascaleDbStorageVault associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub fn set_exascale_db_storage_vault(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().exascale_db_storage_vault = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels or tags associated with the VM Cluster. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe name of the VPC network.\nFormat: projects/{project}/global/networks/{network}"]
    pub fn set_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_network`.\nThe name of the OdbNetwork associated with the VM Cluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe odb_subnet and backup_odb_subnet."]
    pub fn set_odb_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().odb_network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_subnet`.\nThe name of the OdbSubnet associated with the VM Cluster for\nIP allocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn set_odb_subnet(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().odb_subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OracleDatabaseCloudVmCluster_Properties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OracleDatabaseCloudVmCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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
    pub fn identity_connector(&self) -> terrars::ListRef<OracleDatabaseCloudVmCluster_IdentityConnectorRef> {
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
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseCloudVmCluster_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseCloudVmCluster_TimeoutsRef {
        OracleDatabaseCloudVmCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OracleDatabaseCloudVmCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OracleDatabaseCloudVmCluster { }

impl terrars::ToListMappable for OracleDatabaseCloudVmCluster {
    type O = terrars::ListRef<OracleDatabaseCloudVmClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OracleDatabaseCloudVmCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_oracle_database_cloud_vm_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOracleDatabaseCloudVmCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the VM Cluster to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub cloud_vm_cluster_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the Exadata Infrastructure resource on which VM cluster\nresource is created, in the following format:\nprojects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure}"]
    pub exadata_infrastructure: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbNode'."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseCloudVmCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> OracleDatabaseCloudVmCluster {
        let out = OracleDatabaseCloudVmCluster(std::rc::Rc::new(OracleDatabaseCloudVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OracleDatabaseCloudVmClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                backup_odb_subnet: core::default::Default::default(),
                backup_subnet_cidr: core::default::Default::default(),
                cidr: core::default::Default::default(),
                cloud_vm_cluster_id: self.cloud_vm_cluster_id,
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                display_name: core::default::Default::default(),
                exadata_infrastructure: self.exadata_infrastructure,
                exascale_db_storage_vault: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                network: core::default::Default::default(),
                odb_network: core::default::Default::default(),
                odb_subnet: core::default::Default::default(),
                project: core::default::Default::default(),
                properties: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OracleDatabaseCloudVmClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudVmClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OracleDatabaseCloudVmClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
    pub fn identity_connector(&self) -> terrars::ListRef<OracleDatabaseCloudVmCluster_IdentityConnectorRef> {
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
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseCloudVmCluster_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseCloudVmCluster_TimeoutsRef {
        OracleDatabaseCloudVmCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseCloudVmClusterDynamic {
    properties: std::option::Option<terrars::DynamicBlock<OracleDatabaseCloudVmCluster_Properties>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseCloudVmCluster_IdentityConnector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseCloudVmCluster_IdentityConnector {
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

impl terrars::ToListMappable for OracleDatabaseCloudVmCluster_IdentityConnector {
    type O = terrars::BlockAssignable<OracleDatabaseCloudVmCluster_IdentityConnector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseCloudVmCluster_IdentityConnector {}

impl BuildOracleDatabaseCloudVmCluster_IdentityConnector {
    pub fn build(self) -> OracleDatabaseCloudVmCluster_IdentityConnector {
        OracleDatabaseCloudVmCluster_IdentityConnector {
            connection_state: core::default::Default::default(),
            service_agent_email: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseCloudVmCluster_IdentityConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudVmCluster_IdentityConnectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseCloudVmCluster_IdentityConnectorRef {
        OracleDatabaseCloudVmCluster_IdentityConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseCloudVmCluster_IdentityConnectorRef {
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
pub struct OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    diagnostics_events_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_monitoring_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incident_logs_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
    #[doc =
        "Set the field `diagnostics_events_enabled`.\nIndicates whether diagnostic collection is enabled for the VM cluster"]
    pub fn set_diagnostics_events_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.diagnostics_events_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `health_monitoring_enabled`.\nIndicates whether health monitoring is enabled for the VM cluster"]
    pub fn set_health_monitoring_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.health_monitoring_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `incident_logs_enabled`.\nIndicates whether incident logs and trace collection are enabled for the VM\ncluster"]
    pub fn set_incident_logs_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.incident_logs_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
    type O = terrars::BlockAssignable<OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {}

impl BuildOracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
    pub fn build(self) -> OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
        OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions {
            diagnostics_events_enabled: core::default::Default::default(),
            health_monitoring_enabled: core::default::Default::default(),
            incident_logs_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef {
        OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `diagnostics_events_enabled` after provisioning.\nIndicates whether diagnostic collection is enabled for the VM cluster"]
    pub fn diagnostics_events_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.diagnostics_events_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `health_monitoring_enabled` after provisioning.\nIndicates whether health monitoring is enabled for the VM cluster"]
    pub fn health_monitoring_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_monitoring_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `incident_logs_enabled` after provisioning.\nIndicates whether incident logs and trace collection are enabled for the VM\ncluster"]
    pub fn incident_logs_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.incident_logs_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseCloudVmCluster_Properties_TimeZone {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseCloudVmCluster_Properties_TimeZone {
    #[doc = "Set the field `id`.\nIANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nIANA Time Zone Database version number, e.g. \"2019a\"."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseCloudVmCluster_Properties_TimeZone {
    type O = terrars::BlockAssignable<OracleDatabaseCloudVmCluster_Properties_TimeZone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseCloudVmCluster_Properties_TimeZone {}

impl BuildOracleDatabaseCloudVmCluster_Properties_TimeZone {
    pub fn build(self) -> OracleDatabaseCloudVmCluster_Properties_TimeZone {
        OracleDatabaseCloudVmCluster_Properties_TimeZone {
            id: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseCloudVmCluster_Properties_TimeZoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudVmCluster_Properties_TimeZoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseCloudVmCluster_Properties_TimeZoneRef {
        OracleDatabaseCloudVmCluster_Properties_TimeZoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseCloudVmCluster_Properties_TimeZoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIANA Time Zone Database version number, e.g. \"2019a\"."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseCloudVmCluster_PropertiesDynamic {
    diagnostics_data_collection_options: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions>>,
    time_zone: std::option::Option<terrars::DynamicBlock<OracleDatabaseCloudVmCluster_Properties_TimeZone>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseCloudVmCluster_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    cpu_core_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_storage_size_tb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_node_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_server_ocids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_redundancy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gi_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    license_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_backup_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocpu_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sparse_diskgroup_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_public_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_backup_storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_file_system_storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    diagnostics_data_collection_options: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<std::vec::Vec<OracleDatabaseCloudVmCluster_Properties_TimeZone>>,
    dynamic: OracleDatabaseCloudVmCluster_PropertiesDynamic,
}

impl OracleDatabaseCloudVmCluster_Properties {
    #[doc = "Set the field `cluster_name`.\nOCI Cluster name."]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_storage_size_tb`.\nThe data disk group size to be allocated in TBs."]
    pub fn set_data_storage_size_tb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_storage_size_tb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_node_storage_size_gb`.\nLocal storage per VM"]
    pub fn set_db_node_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.db_node_storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_server_ocids`.\nOCID of database servers."]
    pub fn set_db_server_ocids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.db_server_ocids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_redundancy`.\nThe type of redundancy. \n Possible values:\n DISK_REDUNDANCY_UNSPECIFIED\nHIGH\nNORMAL"]
    pub fn set_disk_redundancy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_redundancy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gi_version`.\nGrid Infrastructure Version."]
    pub fn set_gi_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gi_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname_prefix`.\nPrefix for VM cluster host names."]
    pub fn set_hostname_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_backup_enabled`.\nUse local backup."]
    pub fn set_local_backup_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.local_backup_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size_gb`.\nMemory allocated in GBs."]
    pub fn set_memory_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_count`.\nNumber of database servers."]
    pub fn set_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocpu_count`.\nOCPU count per VM. Minimum is 0.1."]
    pub fn set_ocpu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ocpu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sparse_diskgroup_enabled`.\nUse exadata sparse snapshots."]
    pub fn set_sparse_diskgroup_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sparse_diskgroup_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssh_public_keys`.\nSSH public keys to be stored with cluster."]
    pub fn set_ssh_public_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ssh_public_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vm_backup_storage_type`.\nSpecifies whether VM backups are stored on local DB server storage or Exascale storage.\nPossible values:\nVM_BACKUP_STORAGE_TYPE_UNSPECIFIED\nVM_BACKUP_STORAGE_TYPE_LOCAL\nVM_BACKUP_STORAGE_TYPE_EXASCALE"]
    pub fn set_vm_backup_storage_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vm_backup_storage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vm_file_system_storage_type`.\nSpecifies whether VM file system storage / VM images are stored on local DB server storage or Exascale storage.\nPossible values:\nVM_FILE_SYSTEM_STORAGE_TYPE_UNSPECIFIED\nVM_FILE_SYSTEM_STORAGE_TYPE_LOCAL\nVM_FILE_SYSTEM_STORAGE_TYPE_EXASCALE"]
    pub fn set_vm_file_system_storage_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vm_file_system_storage_type = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.diagnostics_data_collection_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.diagnostics_data_collection_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OracleDatabaseCloudVmCluster_Properties_TimeZone>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_zone = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_zone = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseCloudVmCluster_Properties {
    type O = terrars::BlockAssignable<OracleDatabaseCloudVmCluster_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseCloudVmCluster_Properties {
    #[doc = "Number of enabled CPU cores."]
    pub cpu_core_count: terrars::PrimField<f64>,
    #[doc =
        "License type of VM Cluster. \n Possible values:\n LICENSE_TYPE_UNSPECIFIED\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub license_type: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseCloudVmCluster_Properties {
    pub fn build(self) -> OracleDatabaseCloudVmCluster_Properties {
        OracleDatabaseCloudVmCluster_Properties {
            cluster_name: core::default::Default::default(),
            cpu_core_count: self.cpu_core_count,
            data_storage_size_tb: core::default::Default::default(),
            db_node_storage_size_gb: core::default::Default::default(),
            db_server_ocids: core::default::Default::default(),
            disk_redundancy: core::default::Default::default(),
            gi_version: core::default::Default::default(),
            hostname_prefix: core::default::Default::default(),
            license_type: self.license_type,
            local_backup_enabled: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            node_count: core::default::Default::default(),
            ocpu_count: core::default::Default::default(),
            sparse_diskgroup_enabled: core::default::Default::default(),
            ssh_public_keys: core::default::Default::default(),
            vm_backup_storage_type: core::default::Default::default(),
            vm_file_system_storage_type: core::default::Default::default(),
            diagnostics_data_collection_options: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseCloudVmCluster_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudVmCluster_PropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OracleDatabaseCloudVmCluster_PropertiesRef {
        OracleDatabaseCloudVmCluster_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseCloudVmCluster_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\nOCI Cluster name."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `compartment_id` after provisioning.\nCompartment ID of cluster."]
    pub fn compartment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compartment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\nNumber of enabled CPU cores."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_tb` after provisioning.\nThe data disk group size to be allocated in TBs."]
    pub fn data_storage_size_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_tb", self.base))
    }

    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\nLocal storage per VM"]
    pub fn db_node_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `db_server_ocids` after provisioning.\nOCID of database servers."]
    pub fn db_server_ocids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db_server_ocids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_redundancy` after provisioning.\nThe type of redundancy. \n Possible values:\n DISK_REDUNDANCY_UNSPECIFIED\nHIGH\nNORMAL"]
    pub fn disk_redundancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_redundancy", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_listener_ip` after provisioning.\nDNS listener IP."]
    pub fn dns_listener_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_listener_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nParent DNS domain where SCAN DNS and hosts names are qualified.\nex: ocispdelegated.ocisp10jvnet.oraclevcn.com"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `gi_version` after provisioning.\nGrid Infrastructure Version."]
    pub fn gi_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nhost name without domain.\nformat: \"-\" with some suffix.\nex: sp2-yi0xq where \"sp2\" is the hostname_prefix."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname_prefix` after provisioning.\nPrefix for VM cluster host names."]
    pub fn hostname_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `license_type` after provisioning.\nLicense type of VM Cluster. \n Possible values:\n LICENSE_TYPE_UNSPECIFIED\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub fn license_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_type", self.base))
    }

    #[doc = "Get a reference to the value of field `local_backup_enabled` after provisioning.\nUse local backup."]
    pub fn local_backup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_backup_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\nMemory allocated in GBs."]
    pub fn memory_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nNumber of database servers."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nDeep link to the OCI console to view this resource."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nOracle Cloud Infrastructure ID of VM Cluster."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ocpu_count` after provisioning.\nOCPU count per VM. Minimum is 0.1."]
    pub fn ocpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocpu_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scan_dns` after provisioning.\nSCAN DNS name.\nex: sp2-yi0xq-scan.ocispdelegated.ocisp10jvnet.oraclevcn.com"]
    pub fn scan_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_dns_record_id` after provisioning.\nOCID of scan DNS record."]
    pub fn scan_dns_record_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_dns_record_id", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_ip_ids` after provisioning.\nOCIDs of scan IPs."]
    pub fn scan_ip_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_ip_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\nSCAN listener port - TCP"]
    pub fn scan_listener_port_tcp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tcp", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_tcp_ssl` after provisioning.\nSCAN listener port - TLS"]
    pub fn scan_listener_port_tcp_ssl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tcp_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\nShape of VM Cluster."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sparse_diskgroup_enabled` after provisioning.\nUse exadata sparse snapshots."]
    pub fn sparse_diskgroup_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sparse_diskgroup_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssh_public_keys` after provisioning.\nSSH public keys to be stored with cluster."]
    pub fn ssh_public_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_public_keys", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nState of the cluster. \n Possible values:\n STATE_UNSPECIFIED\nPROVISIONING\nAVAILABLE\nUPDATING\nTERMINATING\nTERMINATED\nFAILED\nMAINTENANCE_IN_PROGRESS"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_management_type` after provisioning.\nThe storage management type of the VM Cluster.\nPossible values:\nSTORAGE_MANAGEMENT_TYPE_UNSPECIFIED\nASM\nEXASCALE"]
    pub fn storage_management_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_management_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_size_gb` after provisioning.\nThe storage allocation for the disk group, in gigabytes (GB)."]
    pub fn storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `system_version` after provisioning.\nOperating system version of the image."]
    pub fn system_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vm_backup_storage_type` after provisioning.\nSpecifies whether VM backups are stored on local DB server storage or Exascale storage.\nPossible values:\nVM_BACKUP_STORAGE_TYPE_UNSPECIFIED\nVM_BACKUP_STORAGE_TYPE_LOCAL\nVM_BACKUP_STORAGE_TYPE_EXASCALE"]
    pub fn vm_backup_storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_backup_storage_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vm_file_system_storage_type` after provisioning.\nSpecifies whether VM file system storage / VM images are stored on local DB server storage or Exascale storage.\nPossible values:\nVM_FILE_SYSTEM_STORAGE_TYPE_UNSPECIFIED\nVM_FILE_SYSTEM_STORAGE_TYPE_LOCAL\nVM_FILE_SYSTEM_STORAGE_TYPE_EXASCALE"]
    pub fn vm_file_system_storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_file_system_storage_type", self.base))
    }

    #[doc = "Get a reference to the value of field `diagnostics_data_collection_options` after provisioning.\n"]
    pub fn diagnostics_data_collection_options(
        &self,
    ) -> terrars::ListRef<OracleDatabaseCloudVmCluster_Properties_DiagnosticsDataCollectionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.diagnostics_data_collection_options", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::ListRef<OracleDatabaseCloudVmCluster_Properties_TimeZoneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseCloudVmCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseCloudVmCluster_Timeouts {
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

impl terrars::ToListMappable for OracleDatabaseCloudVmCluster_Timeouts {
    type O = terrars::BlockAssignable<OracleDatabaseCloudVmCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseCloudVmCluster_Timeouts {}

impl BuildOracleDatabaseCloudVmCluster_Timeouts {
    pub fn build(self) -> OracleDatabaseCloudVmCluster_Timeouts {
        OracleDatabaseCloudVmCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseCloudVmCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudVmCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OracleDatabaseCloudVmCluster_TimeoutsRef {
        OracleDatabaseCloudVmCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseCloudVmCluster_TimeoutsRef {
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
