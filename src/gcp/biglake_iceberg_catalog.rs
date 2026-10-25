#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BiglakeIcebergCatalogData_ {
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
    catalog_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    federated_catalog_options: std::option::Option<std::vec::Vec<BiglakeIcebergCatalog_FederatedCatalogOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_locations_config: std::option::Option<std::vec::Vec<BiglakeIcebergCatalog_RestrictedLocationsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BiglakeIcebergCatalog_Timeouts>,
    dynamic: BiglakeIcebergCatalogDynamic,
}

struct BiglakeIcebergCatalog_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BiglakeIcebergCatalogData_>,
}

#[derive(Clone)]
pub struct BiglakeIcebergCatalog(std::rc::Rc<BiglakeIcebergCatalog_>);

impl BiglakeIcebergCatalog {
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
        "Set the field `credential_mode`.\nThe credential mode used for the catalog. CREDENTIAL_MODE_END_USER - End user credentials, default. The authenticating user must have access to the catalog resources and the corresponding Google Cloud Storage files. CREDENTIAL_MODE_VENDED_CREDENTIALS - Use credential vending. The authenticating user must have access to the catalog resources and the system will provide the caller with downscoped credentials to access the Google Cloud Storage files. All table operations in this mode would require 'X-Iceberg-Access-Delegation' header with 'vended-credentials' value included. System will generate a service account and the catalog administrator must grant the service account appropriate permissions. Possible values: [\"CREDENTIAL_MODE_END_USER\", \"CREDENTIAL_MODE_VENDED_CREDENTIALS\"]"]
    pub fn set_credential_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().credential_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_location`.\nThe default storage location for the catalog, e.g., 'gs://my-bucket'.\nOutput only when the catalog type is CATALOG_TYPE_GCS_BUCKET.\nRequired when the catalog type is CATALOG_TYPE_BIGLAKE."]
    pub fn set_default_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA user-provided description of the catalog. Maximum 1024 UTF-8 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `primary_location`.\nThe primary location for mirroring the remote catalog metadata. It must be\na BigLake-supported location, and it should be proximate to the remote\ncatalog's location."]
    pub fn set_primary_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().primary_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `federated_catalog_options`.\n"]
    pub fn set_federated_catalog_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().federated_catalog_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.federated_catalog_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restricted_locations_config`.\n"]
    pub fn set_restricted_locations_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BiglakeIcebergCatalog_RestrictedLocationsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restricted_locations_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restricted_locations_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BiglakeIcebergCatalog_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `biglake_service_account` after provisioning.\nOutput only. The service account used for credential vending. It might be empty if credential vending was never enabled for the catalog."]
    pub fn biglake_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.biglake_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `biglake_service_account_id` after provisioning.\nOutput only. The unique ID of the service account used for credential vending. Used for federation scenarios."]
    pub fn biglake_service_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.biglake_service_account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `catalog_type` after provisioning.\nThe catalog type of the IcebergCatalog.\n* 'CATALOG_TYPE_GCS_BUCKET': Google Cloud Storage bucket catalog type.\n* 'CATALOG_TYPE_BIGLAKE': BigLake catalog type.\n* 'CATALOG_TYPE_FEDERATED': Federated catalog type, for integrating with external Iceberg REST Catalogs such as Databricks Unity Catalog or AWS Glue. Possible values: [\"CATALOG_TYPE_GCS_BUCKET\", \"CATALOG_TYPE_BIGLAKE\", \"CATALOG_TYPE_FEDERATED\"]"]
    pub fn catalog_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time of the IcebergCatalog."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `credential_mode` after provisioning.\nThe credential mode used for the catalog. CREDENTIAL_MODE_END_USER - End user credentials, default. The authenticating user must have access to the catalog resources and the corresponding Google Cloud Storage files. CREDENTIAL_MODE_VENDED_CREDENTIALS - Use credential vending. The authenticating user must have access to the catalog resources and the system will provide the caller with downscoped credentials to access the Google Cloud Storage files. All table operations in this mode would require 'X-Iceberg-Access-Delegation' header with 'vended-credentials' value included. System will generate a service account and the catalog administrator must grant the service account appropriate permissions. Possible values: [\"CREDENTIAL_MODE_END_USER\", \"CREDENTIAL_MODE_VENDED_CREDENTIALS\"]"]
    pub fn credential_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_location` after provisioning.\nThe default storage location for the catalog, e.g., 'gs://my-bucket'.\nOutput only when the catalog type is CATALOG_TYPE_GCS_BUCKET.\nRequired when the catalog type is CATALOG_TYPE_BIGLAKE."]
    pub fn default_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-provided description of the catalog. Maximum 1024 UTF-8 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the IcebergCatalog.\nFor CATALOG_TYPE_GCS_BUCKET typed catalogs, the name needs to be the\nexact same value of the GCS bucket's name. For example, for a bucket:\ngs://bucket-name, the catalog name will be exactly \"bucket-name\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `primary_location` after provisioning.\nThe primary location for mirroring the remote catalog metadata. It must be\na BigLake-supported location, and it should be proximate to the remote\ncatalog's location."]
    pub fn primary_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replicas` after provisioning.\nOutput only. The replicas for the catalog metadata."]
    pub fn replicas(&self) -> terrars::ListRef<BiglakeIcebergCatalog_ReplicasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_regions` after provisioning.\nOutput only. The GCP region(s) where the physical metadata for the tables is stored, e.g. 'us-central1', 'nam4' or 'us'. This will contain one value for all locations, except for the catalogs that are configured to use custom dual region buckets."]
    pub fn storage_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_regions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last modification time of the IcebergCatalog."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `federated_catalog_options` after provisioning.\n"]
    pub fn federated_catalog_options(&self) -> terrars::ListRef<BiglakeIcebergCatalog_FederatedCatalogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.federated_catalog_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restricted_locations_config` after provisioning.\n"]
    pub fn restricted_locations_config(&self) -> terrars::ListRef<BiglakeIcebergCatalog_RestrictedLocationsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restricted_locations_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeIcebergCatalog_TimeoutsRef {
        BiglakeIcebergCatalog_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BiglakeIcebergCatalog {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BiglakeIcebergCatalog { }

impl terrars::ToListMappable for BiglakeIcebergCatalog {
    type O = terrars::ListRef<BiglakeIcebergCatalogRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BiglakeIcebergCatalog_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_biglake_iceberg_catalog".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBiglakeIcebergCatalog {
    pub tf_id: std::string::String,
    #[doc =
        "The catalog type of the IcebergCatalog.\n* 'CATALOG_TYPE_GCS_BUCKET': Google Cloud Storage bucket catalog type.\n* 'CATALOG_TYPE_BIGLAKE': BigLake catalog type.\n* 'CATALOG_TYPE_FEDERATED': Federated catalog type, for integrating with external Iceberg REST Catalogs such as Databricks Unity Catalog or AWS Glue. Possible values: [\"CATALOG_TYPE_GCS_BUCKET\", \"CATALOG_TYPE_BIGLAKE\", \"CATALOG_TYPE_FEDERATED\"]"]
    pub catalog_type: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the IcebergCatalog.\nFor CATALOG_TYPE_GCS_BUCKET typed catalogs, the name needs to be the\nexact same value of the GCS bucket's name. For example, for a bucket:\ngs://bucket-name, the catalog name will be exactly \"bucket-name\"."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeIcebergCatalog {
    pub fn build(self, stack: &mut terrars::Stack) -> BiglakeIcebergCatalog {
        let out = BiglakeIcebergCatalog(std::rc::Rc::new(BiglakeIcebergCatalog_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BiglakeIcebergCatalogData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                catalog_type: self.catalog_type,
                credential_mode: core::default::Default::default(),
                default_location: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                primary_location: core::default::Default::default(),
                project: core::default::Default::default(),
                federated_catalog_options: core::default::Default::default(),
                restricted_locations_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BiglakeIcebergCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BiglakeIcebergCatalogRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `biglake_service_account` after provisioning.\nOutput only. The service account used for credential vending. It might be empty if credential vending was never enabled for the catalog."]
    pub fn biglake_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.biglake_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `biglake_service_account_id` after provisioning.\nOutput only. The unique ID of the service account used for credential vending. Used for federation scenarios."]
    pub fn biglake_service_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.biglake_service_account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `catalog_type` after provisioning.\nThe catalog type of the IcebergCatalog.\n* 'CATALOG_TYPE_GCS_BUCKET': Google Cloud Storage bucket catalog type.\n* 'CATALOG_TYPE_BIGLAKE': BigLake catalog type.\n* 'CATALOG_TYPE_FEDERATED': Federated catalog type, for integrating with external Iceberg REST Catalogs such as Databricks Unity Catalog or AWS Glue. Possible values: [\"CATALOG_TYPE_GCS_BUCKET\", \"CATALOG_TYPE_BIGLAKE\", \"CATALOG_TYPE_FEDERATED\"]"]
    pub fn catalog_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time of the IcebergCatalog."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `credential_mode` after provisioning.\nThe credential mode used for the catalog. CREDENTIAL_MODE_END_USER - End user credentials, default. The authenticating user must have access to the catalog resources and the corresponding Google Cloud Storage files. CREDENTIAL_MODE_VENDED_CREDENTIALS - Use credential vending. The authenticating user must have access to the catalog resources and the system will provide the caller with downscoped credentials to access the Google Cloud Storage files. All table operations in this mode would require 'X-Iceberg-Access-Delegation' header with 'vended-credentials' value included. System will generate a service account and the catalog administrator must grant the service account appropriate permissions. Possible values: [\"CREDENTIAL_MODE_END_USER\", \"CREDENTIAL_MODE_VENDED_CREDENTIALS\"]"]
    pub fn credential_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_location` after provisioning.\nThe default storage location for the catalog, e.g., 'gs://my-bucket'.\nOutput only when the catalog type is CATALOG_TYPE_GCS_BUCKET.\nRequired when the catalog type is CATALOG_TYPE_BIGLAKE."]
    pub fn default_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-provided description of the catalog. Maximum 1024 UTF-8 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the IcebergCatalog.\nFor CATALOG_TYPE_GCS_BUCKET typed catalogs, the name needs to be the\nexact same value of the GCS bucket's name. For example, for a bucket:\ngs://bucket-name, the catalog name will be exactly \"bucket-name\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `primary_location` after provisioning.\nThe primary location for mirroring the remote catalog metadata. It must be\na BigLake-supported location, and it should be proximate to the remote\ncatalog's location."]
    pub fn primary_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `replicas` after provisioning.\nOutput only. The replicas for the catalog metadata."]
    pub fn replicas(&self) -> terrars::ListRef<BiglakeIcebergCatalog_ReplicasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_regions` after provisioning.\nOutput only. The GCP region(s) where the physical metadata for the tables is stored, e.g. 'us-central1', 'nam4' or 'us'. This will contain one value for all locations, except for the catalogs that are configured to use custom dual region buckets."]
    pub fn storage_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_regions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last modification time of the IcebergCatalog."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `federated_catalog_options` after provisioning.\n"]
    pub fn federated_catalog_options(&self) -> terrars::ListRef<BiglakeIcebergCatalog_FederatedCatalogOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.federated_catalog_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restricted_locations_config` after provisioning.\n"]
    pub fn restricted_locations_config(&self) -> terrars::ListRef<BiglakeIcebergCatalog_RestrictedLocationsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restricted_locations_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeIcebergCatalog_TimeoutsRef {
        BiglakeIcebergCatalog_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BiglakeIcebergCatalogDynamic {
    federated_catalog_options: std
    ::option
    ::Option<terrars::DynamicBlock<BiglakeIcebergCatalog_FederatedCatalogOptions>>,
    restricted_locations_config: std
    ::option
    ::Option<terrars::DynamicBlock<BiglakeIcebergCatalog_RestrictedLocationsConfig>>,
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_Replicas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BiglakeIcebergCatalog_Replicas {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergCatalog_Replicas {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_Replicas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_Replicas {}

impl BuildBiglakeIcebergCatalog_Replicas {
    pub fn build(self) -> BiglakeIcebergCatalog_Replicas {
        BiglakeIcebergCatalog_Replicas {
            region: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_ReplicasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_ReplicasRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeIcebergCatalog_ReplicasRef {
        BiglakeIcebergCatalog_ReplicasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_ReplicasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_Status {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_Status {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_Status {}

impl BuildBiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_Status {
    pub fn build(self) -> BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_Status {
        BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_Status {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_StatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_StatusRef {
        BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::ListField<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_Status>>,
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_Status>>,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus {}

impl BuildBiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus {
    pub fn build(self) -> BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus {
        BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatusRef {
        BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::ListRef<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatus_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo {
    aws_region: terrars::PrimField<std::string::String>,
    aws_role_arn: terrars::PrimField<std::string::String>,
    warehouse: terrars::PrimField<std::string::String>,
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo { }

impl terrars::ToListMappable for BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo {
    #[doc = "The AWS region where the Glue catalog is located."]
    pub aws_region: terrars::PrimField<std::string::String>,
    #[doc = "The ARN of the AWS IAM role to assume for accessing the Glue catalog."]
    pub aws_role_arn: terrars::PrimField<std::string::String>,
    #[doc = "The AWS Glue warehouse identifier (account ID or S3 table bucket)."]
    pub warehouse: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo {
    pub fn build(self) -> BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo {
        BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo {
            aws_region: self.aws_region,
            aws_role_arn: self.aws_role_arn,
            warehouse: self.warehouse,
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfoRef {
        BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aws_region` after provisioning.\nThe AWS region where the Glue catalog is located."]
    pub fn aws_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `aws_role_arn` after provisioning.\nThe ARN of the AWS IAM role to assume for accessing the Glue catalog."]
    pub fn aws_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_role_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `warehouse` after provisioning.\nThe AWS Glue warehouse identifier (account ID or S3 table bucket)."]
    pub fn warehouse(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warehouse", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_interval: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule {
    #[doc =
        "Set the field `refresh_interval`.\nThe interval between metadata refreshes, expressed as a duration\nstring (e.g., '300s').\nThe value must be at least 300s or 0s to disable refresh."]
    pub fn set_refresh_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule {}

impl BuildBiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule {
    pub fn build(self) -> BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule {
        BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule {
            refresh_interval: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScheduleRef {
        BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `refresh_interval` after provisioning.\nThe interval between metadata refreshes, expressed as a duration\nstring (e.g., '300s').\nThe value must be at least 300s or 0s to disable refresh."]
    pub fn refresh_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_interval", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace_filters: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope {
    #[doc =
        "Set the field `namespace_filters`.\nA list of namespace filters to limit which namespaces are\nsynchronized from the remote catalog."]
    pub fn set_namespace_filters(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.namespace_filters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope {}

impl BuildBiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope {
    pub fn build(self) -> BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope {
        BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope {
            namespace_filters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScopeRef {
        BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `namespace_filters` after provisioning.\nA list of namespace filters to limit which namespaces are\nsynchronized from the remote catalog."]
    pub fn namespace_filters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.namespace_filters", self.base))
    }
}

#[derive(Serialize, Default)]
struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptionsDynamic {
    refresh_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule>>,
    refresh_scope: std
    ::option
    ::Option<terrars::DynamicBlock<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope>>,
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_schedule: std
    ::option
    ::Option<std::vec::Vec<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_scope: std
    ::option
    ::Option<std::vec::Vec<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope>>,
    dynamic: BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptionsDynamic,
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions {
    #[doc = "Set the field `refresh_schedule`.\n"]
    pub fn set_refresh_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshSchedule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.refresh_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.refresh_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `refresh_scope`.\n"]
    pub fn set_refresh_scope(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScope>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.refresh_scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.refresh_scope = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions {}

impl BuildBiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions {
    pub fn build(self) -> BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions {
        BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions {
            refresh_schedule: core::default::Default::default(),
            refresh_scope: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptionsRef {
        BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `refresh_schedule` after provisioning.\n"]
    pub fn refresh_schedule(
        &self,
    ) -> terrars::ListRef<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_scope` after provisioning.\n"]
    pub fn refresh_scope(
        &self,
    ) -> terrars::ListRef<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions_RefreshScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_scope", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo {
    catalog_name: terrars::PrimField<std::string::String>,
    instance_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_principal_application_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo {
    #[doc =
        "Set the field `service_principal_application_id`.\nThe application ID of the Databricks service principal for OIDC authentication."]
    pub fn set_service_principal_application_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_principal_application_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo {
    #[doc = "The name of the catalog within the Unity Catalog instance."]
    pub catalog_name: terrars::PrimField<std::string::String>,
    #[doc = "The Databricks workspace instance name."]
    pub instance_name: terrars::PrimField<std::string::String>,
}

impl BuildBiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo {
    pub fn build(self) -> BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo {
        BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo {
            catalog_name: self.catalog_name,
            instance_name: self.instance_name,
            service_principal_application_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfoRef {
        BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `catalog_name` after provisioning.\nThe name of the catalog within the Unity Catalog instance."]
    pub fn catalog_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_name` after provisioning.\nThe Databricks workspace instance name."]
    pub fn instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_principal_application_id` after provisioning.\nThe application ID of the Databricks service principal for OIDC authentication."]
    pub fn service_principal_application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_principal_application_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BiglakeIcebergCatalog_FederatedCatalogOptionsDynamic {
    glue_catalog_info: std
    ::option
    ::Option<terrars::DynamicBlock<BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo>>,
    refresh_options: std
    ::option
    ::Option<terrars::DynamicBlock<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions>>,
    unity_catalog_info: std
    ::option
    ::Option<terrars::DynamicBlock<BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo>>,
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glue_catalog_info: std::option::Option<std::vec::Vec<BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_options: std::option::Option<std::vec::Vec<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unity_catalog_info: std
    ::option
    ::Option<std::vec::Vec<BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo>>,
    dynamic: BiglakeIcebergCatalog_FederatedCatalogOptionsDynamic,
}

impl BiglakeIcebergCatalog_FederatedCatalogOptions {
    #[doc =
        "Set the field `secret_name`.\nThe secret resource name in Secret Manager, in the format\n'projects/{projectId}/locations/{location}/secrets/{secret_id}'.\nUsed to store credentials for authenticating with the remote catalog."]
    pub fn set_secret_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_directory_name`.\nThe Service Directory service name for private network connectivity\nthrough Cross-Cloud Interconnect."]
    pub fn set_service_directory_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_directory_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `glue_catalog_info`.\n"]
    pub fn set_glue_catalog_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.glue_catalog_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.glue_catalog_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `refresh_options`.\n"]
    pub fn set_refresh_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.refresh_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.refresh_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unity_catalog_info`.\n"]
    pub fn set_unity_catalog_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unity_catalog_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unity_catalog_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergCatalog_FederatedCatalogOptions {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_FederatedCatalogOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_FederatedCatalogOptions {}

impl BuildBiglakeIcebergCatalog_FederatedCatalogOptions {
    pub fn build(self) -> BiglakeIcebergCatalog_FederatedCatalogOptions {
        BiglakeIcebergCatalog_FederatedCatalogOptions {
            secret_name: core::default::Default::default(),
            service_directory_name: core::default::Default::default(),
            glue_catalog_info: core::default::Default::default(),
            refresh_options: core::default::Default::default(),
            unity_catalog_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_FederatedCatalogOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_FederatedCatalogOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeIcebergCatalog_FederatedCatalogOptionsRef {
        BiglakeIcebergCatalog_FederatedCatalogOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_FederatedCatalogOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `refresh_status` after provisioning.\nOutput only. The status of the most recent metadata refresh."]
    pub fn refresh_status(&self) -> terrars::ListRef<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_name` after provisioning.\nThe secret resource name in Secret Manager, in the format\n'projects/{projectId}/locations/{location}/secrets/{secret_id}'.\nUsed to store credentials for authenticating with the remote catalog."]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_directory_name` after provisioning.\nThe Service Directory service name for private network connectivity\nthrough Cross-Cloud Interconnect."]
    pub fn service_directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_directory_name", self.base))
    }

    #[doc = "Get a reference to the value of field `glue_catalog_info` after provisioning.\n"]
    pub fn glue_catalog_info(
        &self,
    ) -> terrars::ListRef<BiglakeIcebergCatalog_FederatedCatalogOptions_GlueCatalogInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glue_catalog_info", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_options` after provisioning.\n"]
    pub fn refresh_options(&self) -> terrars::ListRef<BiglakeIcebergCatalog_FederatedCatalogOptions_RefreshOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.refresh_options", self.base))
    }

    #[doc = "Get a reference to the value of field `unity_catalog_info` after provisioning.\n"]
    pub fn unity_catalog_info(
        &self,
    ) -> terrars::ListRef<BiglakeIcebergCatalog_FederatedCatalogOptions_UnityCatalogInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unity_catalog_info", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_RestrictedLocationsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BiglakeIcebergCatalog_RestrictedLocationsConfig {
    #[doc =
        "Set the field `restricted_locations`.\nA list of GCS locations (e.g., 'gs://my-other-bucket/...') that are\npermitted for use by resources within this catalog. Each entry can be\neither a GCS bucket or a path within it."]
    pub fn set_restricted_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.restricted_locations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BiglakeIcebergCatalog_RestrictedLocationsConfig {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_RestrictedLocationsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_RestrictedLocationsConfig {}

impl BuildBiglakeIcebergCatalog_RestrictedLocationsConfig {
    pub fn build(self) -> BiglakeIcebergCatalog_RestrictedLocationsConfig {
        BiglakeIcebergCatalog_RestrictedLocationsConfig { restricted_locations: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_RestrictedLocationsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_RestrictedLocationsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BiglakeIcebergCatalog_RestrictedLocationsConfigRef {
        BiglakeIcebergCatalog_RestrictedLocationsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_RestrictedLocationsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `restricted_locations` after provisioning.\nA list of GCS locations (e.g., 'gs://my-other-bucket/...') that are\npermitted for use by resources within this catalog. Each entry can be\neither a GCS bucket or a path within it."]
    pub fn restricted_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restricted_locations", self.base))
    }
}

#[derive(Serialize)]
pub struct BiglakeIcebergCatalog_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BiglakeIcebergCatalog_Timeouts {
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

impl terrars::ToListMappable for BiglakeIcebergCatalog_Timeouts {
    type O = terrars::BlockAssignable<BiglakeIcebergCatalog_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBiglakeIcebergCatalog_Timeouts {}

impl BuildBiglakeIcebergCatalog_Timeouts {
    pub fn build(self) -> BiglakeIcebergCatalog_Timeouts {
        BiglakeIcebergCatalog_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BiglakeIcebergCatalog_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BiglakeIcebergCatalog_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BiglakeIcebergCatalog_TimeoutsRef {
        BiglakeIcebergCatalog_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BiglakeIcebergCatalog_TimeoutsRef {
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
