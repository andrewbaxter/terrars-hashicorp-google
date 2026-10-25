#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OracleDatabaseExadbVmClusterData_ {
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
    backup_odb_subnet: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    display_name: terrars::PrimField<std::string::String>,
    exadb_vm_cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_network: std::option::Option<terrars::PrimField<std::string::String>>,
    odb_subnet: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<std::vec::Vec<OracleDatabaseExadbVmCluster_Properties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OracleDatabaseExadbVmCluster_Timeouts>,
    dynamic: OracleDatabaseExadbVmClusterDynamic,
}

struct OracleDatabaseExadbVmCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OracleDatabaseExadbVmClusterData_>,
}

#[derive(Clone)]
pub struct OracleDatabaseExadbVmCluster(std::rc::Rc<OracleDatabaseExadbVmCluster_>);

impl OracleDatabaseExadbVmCluster {
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
        "Set the field `deletion_protection`.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels or tags associated with the ExadbVmCluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_network`.\nThe name of the OdbNetwork associated with the ExadbVmCluster.\nFormat: projects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe OdbSubnet."]
    pub fn set_odb_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().odb_network = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<OracleDatabaseExadbVmCluster_Properties>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<OracleDatabaseExadbVmCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `backup_odb_subnet` after provisioning.\nThe name of the backup OdbSubnet associated with the ExadbVmCluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn backup_odb_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_odb_subnet", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the ExadbVmCluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the ExadbVmCluster. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the ExadbVmCluster."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadb_vm_cluster_id` after provisioning.\nThe ID of the ExadbVmCluster to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn exadb_vm_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exadb_vm_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle ExadbVmCluster is hosted.\nExample: us-east4-b-r2.\nDuring creation, the system will pick the zone assigned to the\nExascaleDbStorageVault."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity_connector` after provisioning.\nThe identity connector details which will allow OCI to securely access\nthe resources in the customer project."]
    pub fn identity_connector(&self) -> terrars::ListRef<OracleDatabaseExadbVmCluster_IdentityConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_connector", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the ExadbVmCluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the ExadbVmCluster resource in the following format:\nprojects/{project}/locations/{region}/exadbVmClusters/{exadb_vm_cluster}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the ExadbVmCluster.\nFormat: projects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe OdbSubnet."]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the ExadbVmCluster for IP\nallocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
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
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseExadbVmCluster_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseExadbVmCluster_TimeoutsRef {
        OracleDatabaseExadbVmCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OracleDatabaseExadbVmCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OracleDatabaseExadbVmCluster { }

impl terrars::ToListMappable for OracleDatabaseExadbVmCluster {
    type O = terrars::ListRef<OracleDatabaseExadbVmClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OracleDatabaseExadbVmCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_oracle_database_exadb_vm_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOracleDatabaseExadbVmCluster {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the backup OdbSubnet associated with the ExadbVmCluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub backup_odb_subnet: terrars::PrimField<std::string::String>,
    #[doc =
        "The display name for the ExadbVmCluster. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the ExadbVmCluster to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub exadb_vm_cluster_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the OdbSubnet associated with the ExadbVmCluster for IP\nallocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub odb_subnet: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseExadbVmCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> OracleDatabaseExadbVmCluster {
        let out = OracleDatabaseExadbVmCluster(std::rc::Rc::new(OracleDatabaseExadbVmCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OracleDatabaseExadbVmClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                backup_odb_subnet: self.backup_odb_subnet,
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                display_name: self.display_name,
                exadb_vm_cluster_id: self.exadb_vm_cluster_id,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                odb_network: core::default::Default::default(),
                odb_subnet: self.odb_subnet,
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

pub struct OracleDatabaseExadbVmClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseExadbVmClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OracleDatabaseExadbVmClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_odb_subnet` after provisioning.\nThe name of the backup OdbSubnet associated with the ExadbVmCluster.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn backup_odb_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_odb_subnet", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the ExadbVmCluster was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether or not to allow Terraform to destroy the instance. Unless this field is set to false in Terraform state, a terraform destroy or terraform apply that would delete the instance will fail."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the ExadbVmCluster. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the ExadbVmCluster."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadb_vm_cluster_id` after provisioning.\nThe ID of the ExadbVmCluster to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn exadb_vm_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exadb_vm_cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle ExadbVmCluster is hosted.\nExample: us-east4-b-r2.\nDuring creation, the system will pick the zone assigned to the\nExascaleDbStorageVault."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `identity_connector` after provisioning.\nThe identity connector details which will allow OCI to securely access\nthe resources in the customer project."]
    pub fn identity_connector(&self) -> terrars::ListRef<OracleDatabaseExadbVmCluster_IdentityConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_connector", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the ExadbVmCluster.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the ExadbVmCluster resource in the following format:\nprojects/{project}/locations/{region}/exadbVmClusters/{exadb_vm_cluster}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the ExadbVmCluster.\nFormat: projects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe OdbSubnet."]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the ExadbVmCluster for IP\nallocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
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
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseExadbVmCluster_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseExadbVmCluster_TimeoutsRef {
        OracleDatabaseExadbVmCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseExadbVmClusterDynamic {
    properties: std::option::Option<terrars::DynamicBlock<OracleDatabaseExadbVmCluster_Properties>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseExadbVmCluster_IdentityConnector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseExadbVmCluster_IdentityConnector {
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

impl terrars::ToListMappable for OracleDatabaseExadbVmCluster_IdentityConnector {
    type O = terrars::BlockAssignable<OracleDatabaseExadbVmCluster_IdentityConnector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseExadbVmCluster_IdentityConnector {}

impl BuildOracleDatabaseExadbVmCluster_IdentityConnector {
    pub fn build(self) -> OracleDatabaseExadbVmCluster_IdentityConnector {
        OracleDatabaseExadbVmCluster_IdentityConnector {
            connection_state: core::default::Default::default(),
            service_agent_email: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseExadbVmCluster_IdentityConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseExadbVmCluster_IdentityConnectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseExadbVmCluster_IdentityConnectorRef {
        OracleDatabaseExadbVmCluster_IdentityConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseExadbVmCluster_IdentityConnectorRef {
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
pub struct OracleDatabaseExadbVmCluster_Properties_DataCollectionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_diagnostics_events_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_health_monitoring_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_incident_logs_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl OracleDatabaseExadbVmCluster_Properties_DataCollectionOptions {
    #[doc =
        "Set the field `is_diagnostics_events_enabled`.\nIndicates whether to enable data collection for diagnostics."]
    pub fn set_is_diagnostics_events_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_diagnostics_events_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_health_monitoring_enabled`.\nIndicates whether to enable health monitoring."]
    pub fn set_is_health_monitoring_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_health_monitoring_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_incident_logs_enabled`.\nIndicates whether to enable incident logs and trace collection."]
    pub fn set_is_incident_logs_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_incident_logs_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseExadbVmCluster_Properties_DataCollectionOptions {
    type O = terrars::BlockAssignable<OracleDatabaseExadbVmCluster_Properties_DataCollectionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseExadbVmCluster_Properties_DataCollectionOptions {}

impl BuildOracleDatabaseExadbVmCluster_Properties_DataCollectionOptions {
    pub fn build(self) -> OracleDatabaseExadbVmCluster_Properties_DataCollectionOptions {
        OracleDatabaseExadbVmCluster_Properties_DataCollectionOptions {
            is_diagnostics_events_enabled: core::default::Default::default(),
            is_health_monitoring_enabled: core::default::Default::default(),
            is_incident_logs_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseExadbVmCluster_Properties_DataCollectionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseExadbVmCluster_Properties_DataCollectionOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseExadbVmCluster_Properties_DataCollectionOptionsRef {
        OracleDatabaseExadbVmCluster_Properties_DataCollectionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseExadbVmCluster_Properties_DataCollectionOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_diagnostics_events_enabled` after provisioning.\nIndicates whether to enable data collection for diagnostics."]
    pub fn is_diagnostics_events_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_diagnostics_events_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_health_monitoring_enabled` after provisioning.\nIndicates whether to enable health monitoring."]
    pub fn is_health_monitoring_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_health_monitoring_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_incident_logs_enabled` after provisioning.\nIndicates whether to enable incident logs and trace collection."]
    pub fn is_incident_logs_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_incident_logs_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseExadbVmCluster_Properties_TimeZone {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseExadbVmCluster_Properties_TimeZone {
    #[doc = "Set the field `id`.\nIANA Time Zone Database time zone. For example \"America/New_York\"."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nIANA Time Zone Database version number. For example \"2019a\"."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseExadbVmCluster_Properties_TimeZone {
    type O = terrars::BlockAssignable<OracleDatabaseExadbVmCluster_Properties_TimeZone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseExadbVmCluster_Properties_TimeZone {}

impl BuildOracleDatabaseExadbVmCluster_Properties_TimeZone {
    pub fn build(self) -> OracleDatabaseExadbVmCluster_Properties_TimeZone {
        OracleDatabaseExadbVmCluster_Properties_TimeZone {
            id: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseExadbVmCluster_Properties_TimeZoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseExadbVmCluster_Properties_TimeZoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseExadbVmCluster_Properties_TimeZoneRef {
        OracleDatabaseExadbVmCluster_Properties_TimeZoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseExadbVmCluster_Properties_TimeZoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIANA Time Zone Database time zone. For example \"America/New_York\"."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIANA Time Zone Database version number. For example \"2019a\"."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage {
    size_in_gbs_per_node: terrars::PrimField<f64>,
}

impl OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage { }

impl terrars::ToListMappable for OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage {
    type O = terrars::BlockAssignable<OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage {
    #[doc =
        "The storage allocation for the exadbvmcluster per node, in gigabytes (GB).\nThis field is used to calculate the total storage allocation for the\nexadbvmcluster."]
    pub size_in_gbs_per_node: terrars::PrimField<f64>,
}

impl BuildOracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage {
    pub fn build(self) -> OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage {
        OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage {
            size_in_gbs_per_node: self.size_in_gbs_per_node,
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorageRef {
        OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `size_in_gbs_per_node` after provisioning.\nThe storage allocation for the exadbvmcluster per node, in gigabytes (GB).\nThis field is used to calculate the total storage allocation for the\nexadbvmcluster."]
    pub fn size_in_gbs_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_gbs_per_node", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseExadbVmCluster_PropertiesDynamic {
    data_collection_options: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseExadbVmCluster_Properties_DataCollectionOptions>>,
    time_zone: std::option::Option<terrars::DynamicBlock<OracleDatabaseExadbVmCluster_Properties_TimeZone>>,
    vm_file_system_storage: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseExadbVmCluster_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_ecpu_count_per_node: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_name: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled_ecpu_count_per_node: terrars::PrimField<f64>,
    exascale_db_storage_vault: terrars::PrimField<std::string::String>,
    grid_image_id: terrars::PrimField<std::string::String>,
    hostname_prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_model: std::option::Option<terrars::PrimField<std::string::String>>,
    node_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_listener_port_tcp: std::option::Option<terrars::PrimField<f64>>,
    shape_attribute: terrars::PrimField<std::string::String>,
    ssh_public_keys: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_collection_options: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseExadbVmCluster_Properties_DataCollectionOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<std::vec::Vec<OracleDatabaseExadbVmCluster_Properties_TimeZone>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_file_system_storage: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage>>,
    dynamic: OracleDatabaseExadbVmCluster_PropertiesDynamic,
}

impl OracleDatabaseExadbVmCluster_Properties {
    #[doc =
        "Set the field `additional_ecpu_count_per_node`.\nThe number of additional ECPUs per node for an Exadata VM cluster on\nexascale infrastructure."]
    pub fn set_additional_ecpu_count_per_node(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.additional_ecpu_count_per_node = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cluster_name`.\nThe cluster name for Exascale vm cluster. The cluster name must begin with\nan alphabetic character and may contain hyphens(-) but can not contain\nunderscores(_). It should be not more than 11 characters and is not case\nsensitive.\nOCI Cluster name."]
    pub fn set_cluster_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `license_model`.\nThe license type of the ExadbVmCluster.\nPossible values:\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub fn set_license_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.license_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_listener_port_tcp`.\nSCAN listener port - TCP"]
    pub fn set_scan_listener_port_tcp(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scan_listener_port_tcp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_collection_options`.\n"]
    pub fn set_data_collection_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OracleDatabaseExadbVmCluster_Properties_DataCollectionOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_collection_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_collection_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OracleDatabaseExadbVmCluster_Properties_TimeZone>>,
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

    #[doc = "Set the field `vm_file_system_storage`.\n"]
    pub fn set_vm_file_system_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vm_file_system_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vm_file_system_storage = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseExadbVmCluster_Properties {
    type O = terrars::BlockAssignable<OracleDatabaseExadbVmCluster_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseExadbVmCluster_Properties {
    #[doc = "The number of ECPUs enabled per node for an exadata vm cluster on\nexascale infrastructure."]
    pub enabled_ecpu_count_per_node: terrars::PrimField<f64>,
    #[doc =
        "The name of ExascaleDbStorageVault associated with the ExadbVmCluster.\nIt can refer to an existing ExascaleDbStorageVault. Or a new one can be\ncreated during the ExadbVmCluster creation (requires\nstorage_vault_properties to be set).\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub exascale_db_storage_vault: terrars::PrimField<std::string::String>,
    #[doc = "Grid Infrastructure Version."]
    pub grid_image_id: terrars::PrimField<std::string::String>,
    #[doc = "Prefix for VM cluster host names."]
    pub hostname_prefix: terrars::PrimField<std::string::String>,
    #[doc = "The number of nodes/VMs in the ExadbVmCluster."]
    pub node_count: terrars::PrimField<f64>,
    #[doc =
        "The shape attribute of the VM cluster. The type of Exascale storage used\nfor Exadata VM cluster. The default is SMART_STORAGE which supports Oracle\nDatabase 23ai and later\nPossible values:\nSMART_STORAGE\nBLOCK_STORAGE"]
    pub shape_attribute: terrars::PrimField<std::string::String>,
    #[doc = "The SSH public keys for the ExadbVmCluster."]
    pub ssh_public_keys: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildOracleDatabaseExadbVmCluster_Properties {
    pub fn build(self) -> OracleDatabaseExadbVmCluster_Properties {
        OracleDatabaseExadbVmCluster_Properties {
            additional_ecpu_count_per_node: core::default::Default::default(),
            cluster_name: core::default::Default::default(),
            enabled_ecpu_count_per_node: self.enabled_ecpu_count_per_node,
            exascale_db_storage_vault: self.exascale_db_storage_vault,
            grid_image_id: self.grid_image_id,
            hostname_prefix: self.hostname_prefix,
            license_model: core::default::Default::default(),
            node_count: self.node_count,
            scan_listener_port_tcp: core::default::Default::default(),
            shape_attribute: self.shape_attribute,
            ssh_public_keys: self.ssh_public_keys,
            data_collection_options: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            vm_file_system_storage: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseExadbVmCluster_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseExadbVmCluster_PropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OracleDatabaseExadbVmCluster_PropertiesRef {
        OracleDatabaseExadbVmCluster_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseExadbVmCluster_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_ecpu_count_per_node` after provisioning.\nThe number of additional ECPUs per node for an Exadata VM cluster on\nexascale infrastructure."]
    pub fn additional_ecpu_count_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_ecpu_count_per_node", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_name` after provisioning.\nThe cluster name for Exascale vm cluster. The cluster name must begin with\nan alphabetic character and may contain hyphens(-) but can not contain\nunderscores(_). It should be not more than 11 characters and is not case\nsensitive.\nOCI Cluster name."]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled_ecpu_count_per_node` after provisioning.\nThe number of ECPUs enabled per node for an exadata vm cluster on\nexascale infrastructure."]
    pub fn enabled_ecpu_count_per_node(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled_ecpu_count_per_node", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exascale_db_storage_vault` after provisioning.\nThe name of ExascaleDbStorageVault associated with the ExadbVmCluster.\nIt can refer to an existing ExascaleDbStorageVault. Or a new one can be\ncreated during the ExadbVmCluster creation (requires\nstorage_vault_properties to be set).\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub fn exascale_db_storage_vault(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exascale_db_storage_vault", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gi_version` after provisioning.\nThe Oracle Grid Infrastructure (GI) software version."]
    pub fn gi_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gi_version", self.base))
    }

    #[doc = "Get a reference to the value of field `grid_image_id` after provisioning.\nGrid Infrastructure Version."]
    pub fn grid_image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grid_image_id", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nThe hostname of the ExadbVmCluster."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hostname_prefix` after provisioning.\nPrefix for VM cluster host names."]
    pub fn hostname_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe license type of the ExadbVmCluster.\nPossible values:\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_state` after provisioning.\nState of the cluster.\nPossible values:\nPROVISIONING\nAVAILABLE\nUPDATING\nTERMINATING\nTERMINATED\nFAILED\nMAINTENANCE_IN_PROGRESS"]
    pub fn lifecycle_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_size_gb` after provisioning.\nMemory per VM (GB) (Read-only): Shows the amount of memory allocated to\neach VM. Memory is calculated based on 2.75 GB per Total ECPUs."]
    pub fn memory_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_count` after provisioning.\nThe number of nodes/VMs in the ExadbVmCluster."]
    pub fn node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oci_uri` after provisioning.\nDeep link to the OCI console to view this resource."]
    pub fn oci_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scan_listener_port_tcp` after provisioning.\nSCAN listener port - TCP"]
    pub fn scan_listener_port_tcp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_listener_port_tcp", self.base))
    }

    #[doc =
        "Get a reference to the value of field `shape_attribute` after provisioning.\nThe shape attribute of the VM cluster. The type of Exascale storage used\nfor Exadata VM cluster. The default is SMART_STORAGE which supports Oracle\nDatabase 23ai and later\nPossible values:\nSMART_STORAGE\nBLOCK_STORAGE"]
    pub fn shape_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape_attribute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssh_public_keys` after provisioning.\nThe SSH public keys for the ExadbVmCluster."]
    pub fn ssh_public_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_public_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `data_collection_options` after provisioning.\n"]
    pub fn data_collection_options(
        &self,
    ) -> terrars::ListRef<OracleDatabaseExadbVmCluster_Properties_DataCollectionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_collection_options", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::ListRef<OracleDatabaseExadbVmCluster_Properties_TimeZoneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_file_system_storage` after provisioning.\n"]
    pub fn vm_file_system_storage(
        &self,
    ) -> terrars::ListRef<OracleDatabaseExadbVmCluster_Properties_VmFileSystemStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_file_system_storage", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseExadbVmCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseExadbVmCluster_Timeouts {
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

impl terrars::ToListMappable for OracleDatabaseExadbVmCluster_Timeouts {
    type O = terrars::BlockAssignable<OracleDatabaseExadbVmCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseExadbVmCluster_Timeouts {}

impl BuildOracleDatabaseExadbVmCluster_Timeouts {
    pub fn build(self) -> OracleDatabaseExadbVmCluster_Timeouts {
        OracleDatabaseExadbVmCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseExadbVmCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseExadbVmCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OracleDatabaseExadbVmCluster_TimeoutsRef {
        OracleDatabaseExadbVmCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseExadbVmCluster_TimeoutsRef {
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
