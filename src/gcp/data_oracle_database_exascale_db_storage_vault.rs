#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataOracleDatabaseExascaleDbStorageVaultData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    exascale_db_storage_vault_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOracleDatabaseExascaleDbStorageVault_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOracleDatabaseExascaleDbStorageVaultData_>,
}

#[derive(Clone)]
pub struct DataOracleDatabaseExascaleDbStorageVault(std::rc::Rc<DataOracleDatabaseExascaleDbStorageVault_>);

impl DataOracleDatabaseExascaleDbStorageVault {
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
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time when the ExascaleDbStorageVault was created."]
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
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the ExascaleDbStorageVault. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the\nExascaleDbStorageVault."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_infrastructure` after provisioning.\nThe Exadata Infrastructure resource on which ExascaleDbStorageVault resource is created.\nIn the format: projects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_extradata_infrastructure}"]
    pub fn exadata_infrastructure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exadata_infrastructure", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exascale_db_storage_vault_id` after provisioning.\nThe ID of the ExascaleDbStorageVault to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn exascale_db_storage_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exascale_db_storage_vault_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle ExascaleDbStorageVault is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the ExascaleDbStorageVault.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the ExascaleDbStorageVault.\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nThe properties of the ExascaleDbStorageVault.\nnext ID: 12"]
    pub fn properties(&self) -> terrars::ListRef<DataOracleDatabaseExascaleDbStorageVault_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

impl terrars::Referable for DataOracleDatabaseExascaleDbStorageVault {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOracleDatabaseExascaleDbStorageVault { }

impl terrars::ToListMappable for DataOracleDatabaseExascaleDbStorageVault {
    type O = terrars::ListRef<DataOracleDatabaseExascaleDbStorageVaultRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOracleDatabaseExascaleDbStorageVault_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_oracle_database_exascale_db_storage_vault".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOracleDatabaseExascaleDbStorageVault {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the ExascaleDbStorageVault to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub exascale_db_storage_vault_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataOracleDatabaseExascaleDbStorageVault {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOracleDatabaseExascaleDbStorageVault {
        let out =
            DataOracleDatabaseExascaleDbStorageVault(std::rc::Rc::new(DataOracleDatabaseExascaleDbStorageVault_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataOracleDatabaseExascaleDbStorageVaultData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    exascale_db_storage_vault_id: self.exascale_db_storage_vault_id,
                    id: core::default::Default::default(),
                    location: self.location,
                    project: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOracleDatabaseExascaleDbStorageVaultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseExascaleDbStorageVaultRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOracleDatabaseExascaleDbStorageVaultRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time when the ExascaleDbStorageVault was created."]
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
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the ExascaleDbStorageVault. The name does not have to\nbe unique within your project. The name must be 1-255 characters long and\ncan only contain alphanumeric characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the\nExascaleDbStorageVault."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exadata_infrastructure` after provisioning.\nThe Exadata Infrastructure resource on which ExascaleDbStorageVault resource is created.\nIn the format: projects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_extradata_infrastructure}"]
    pub fn exadata_infrastructure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exadata_infrastructure", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exascale_db_storage_vault_id` after provisioning.\nThe ID of the ExascaleDbStorageVault to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn exascale_db_storage_vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exascale_db_storage_vault_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle ExascaleDbStorageVault is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the ExascaleDbStorageVault.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the ExascaleDbStorageVault.\nFormat:\nprojects/{project}/locations/{location}/exascaleDbStorageVaults/{exascale_db_storage_vault}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nThe properties of the ExascaleDbStorageVault.\nnext ID: 12"]
    pub fn properties(&self) -> terrars::ListRef<DataOracleDatabaseExascaleDbStorageVault_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_size_gbs: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_size_gbs: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetails {
    #[doc = "Set the field `available_size_gbs`.\n"]
    pub fn set_available_size_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_size_gbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_size_gbs`.\n"]
    pub fn set_total_size_gbs(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_size_gbs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetails {
    type O = terrars::BlockAssignable<DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetails {}

impl BuildDataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetails {
    pub fn build(self) -> DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetails {
        DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetails {
            available_size_gbs: core::default::Default::default(),
            total_size_gbs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetailsRef {
        DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_size_gbs` after provisioning.\n"]
    pub fn available_size_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_size_gbs", self.base))
    }

    #[doc = "Get a reference to the value of field `total_size_gbs` after provisioning.\n"]
    pub fn total_size_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_size_gbs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZone {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZone {
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

impl terrars::ToListMappable for DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZone {
    type O = terrars::BlockAssignable<DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseExascaleDbStorageVault_Properties_TimeZone {}

impl BuildDataOracleDatabaseExascaleDbStorageVault_Properties_TimeZone {
    pub fn build(self) -> DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZone {
        DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZone {
            id: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZoneRef {
        DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZoneRef {
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
pub struct DataOracleDatabaseExascaleDbStorageVault_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_flash_cache_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_shape_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_shape_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exascale_db_storage_details: std
    ::option
    ::Option<terrars::ListField<DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::ListField<DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZone>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_cluster_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_cluster_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataOracleDatabaseExascaleDbStorageVault_Properties {
    #[doc = "Set the field `additional_flash_cache_percent`.\n"]
    pub fn set_additional_flash_cache_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.additional_flash_cache_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_shape_attributes`.\n"]
    pub fn set_attached_shape_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attached_shape_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_shape_attributes`.\n"]
    pub fn set_available_shape_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.available_shape_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exascale_db_storage_details`.\n"]
    pub fn set_exascale_db_storage_details(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetails>,
                    >,
    ) -> Self {
        self.exascale_db_storage_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oci_uri`.\n"]
    pub fn set_oci_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oci_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ocid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZone>>,
    ) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_cluster_count`.\n"]
    pub fn set_vm_cluster_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vm_cluster_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_cluster_ids`.\n"]
    pub fn set_vm_cluster_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.vm_cluster_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseExascaleDbStorageVault_Properties {
    type O = terrars::BlockAssignable<DataOracleDatabaseExascaleDbStorageVault_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseExascaleDbStorageVault_Properties {}

impl BuildDataOracleDatabaseExascaleDbStorageVault_Properties {
    pub fn build(self) -> DataOracleDatabaseExascaleDbStorageVault_Properties {
        DataOracleDatabaseExascaleDbStorageVault_Properties {
            additional_flash_cache_percent: core::default::Default::default(),
            attached_shape_attributes: core::default::Default::default(),
            available_shape_attributes: core::default::Default::default(),
            exascale_db_storage_details: core::default::Default::default(),
            oci_uri: core::default::Default::default(),
            ocid: core::default::Default::default(),
            state: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            vm_cluster_count: core::default::Default::default(),
            vm_cluster_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseExascaleDbStorageVault_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseExascaleDbStorageVault_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseExascaleDbStorageVault_PropertiesRef {
        DataOracleDatabaseExascaleDbStorageVault_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseExascaleDbStorageVault_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_flash_cache_percent` after provisioning.\n"]
    pub fn additional_flash_cache_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_flash_cache_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_shape_attributes` after provisioning.\n"]
    pub fn attached_shape_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_shape_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `available_shape_attributes` after provisioning.\n"]
    pub fn available_shape_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_shape_attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `exascale_db_storage_details` after provisioning.\n"]
    pub fn exascale_db_storage_details(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseExascaleDbStorageVault_Properties_ExascaleDbStorageDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exascale_db_storage_details", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_uri` after provisioning.\n"]
    pub fn oci_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::ListRef<DataOracleDatabaseExascaleDbStorageVault_Properties_TimeZoneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_cluster_count` after provisioning.\n"]
    pub fn vm_cluster_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_cluster_count", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_cluster_ids` after provisioning.\n"]
    pub fn vm_cluster_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_cluster_ids", self.base))
    }
}
