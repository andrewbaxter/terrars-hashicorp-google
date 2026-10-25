#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataOracleDatabaseCloudExadataInfrastructureData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cloud_exadata_infrastructure_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOracleDatabaseCloudExadataInfrastructure_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOracleDatabaseCloudExadataInfrastructureData_>,
}

#[derive(Clone)]
pub struct DataOracleDatabaseCloudExadataInfrastructure(std::rc::Rc<DataOracleDatabaseCloudExadataInfrastructure_>);

impl DataOracleDatabaseCloudExadataInfrastructure {
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
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe ID of the Exadata Infrastructure to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the Exadata Infrastructure was created."]
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
        "Get a reference to the value of field `entitlement_id` after provisioning.\nEntitlement ID of the private offer against which this infrastructure\nresource is provisioned."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nGCP location where Oracle Exadata is hosted."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels or tags associated with the resource. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbServer'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the Exadata Infrastructure resource with the following format:\nprojects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_exadata_infrastructure}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nVarious properties of Exadata Infrastructure."]
    pub fn properties(&self) -> terrars::ListRef<DataOracleDatabaseCloudExadataInfrastructure_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

impl terrars::Referable for DataOracleDatabaseCloudExadataInfrastructure {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOracleDatabaseCloudExadataInfrastructure { }

impl terrars::ToListMappable for DataOracleDatabaseCloudExadataInfrastructure {
    type O = terrars::ListRef<DataOracleDatabaseCloudExadataInfrastructureRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOracleDatabaseCloudExadataInfrastructure_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_oracle_database_cloud_exadata_infrastructure".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOracleDatabaseCloudExadataInfrastructure {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the Exadata Infrastructure to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub cloud_exadata_infrastructure_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbServer'."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataOracleDatabaseCloudExadataInfrastructure {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOracleDatabaseCloudExadataInfrastructure {
        let out =
            DataOracleDatabaseCloudExadataInfrastructure(
                std::rc::Rc::new(DataOracleDatabaseCloudExadataInfrastructure_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataOracleDatabaseCloudExadataInfrastructureData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        cloud_exadata_infrastructure_id: self.cloud_exadata_infrastructure_id,
                        id: core::default::Default::default(),
                        location: self.location,
                        project: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOracleDatabaseCloudExadataInfrastructureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudExadataInfrastructureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOracleDatabaseCloudExadataInfrastructureRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `cloud_exadata_infrastructure_id` after provisioning.\nThe ID of the Exadata Infrastructure to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub fn cloud_exadata_infrastructure_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_exadata_infrastructure_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the Exadata Infrastructure was created."]
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
        "Get a reference to the value of field `entitlement_id` after provisioning.\nEntitlement ID of the private offer against which this infrastructure\nresource is provisioned."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nGCP location where Oracle Exadata is hosted."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels or tags associated with the resource. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbServer'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the Exadata Infrastructure resource with the following format:\nprojects/{project}/locations/{region}/cloudExadataInfrastructures/{cloud_exadata_infrastructure}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nVarious properties of Exadata Infrastructure."]
    pub fn properties(&self) -> terrars::ListRef<DataOracleDatabaseCloudExadataInfrastructure_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {}

impl BuildDataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
    pub fn build(self) -> DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
        DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
            email: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef {
        DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_vm_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_vm_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
    #[doc = "Set the field `available_storage_size_gb`.\n"]
    pub fn set_available_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_vm_storage_size_gb`.\n"]
    pub fn set_available_vm_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_vm_storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_storage_size_gb`.\n"]
    pub fn set_total_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_vm_storage_size_gb`.\n"]
    pub fn set_total_vm_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_vm_storage_size_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {}

impl BuildDataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
    pub fn build(self) -> DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
        DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
            available_storage_size_gb: core::default::Default::default(),
            available_vm_storage_size_gb: core::default::Default::default(),
            total_storage_size_gb: core::default::Default::default(),
            total_vm_storage_size_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef {
        DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_storage_size_gb` after provisioning.\n"]
    pub fn available_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `available_vm_storage_size_gb` after provisioning.\n"]
    pub fn available_vm_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_vm_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `total_storage_size_gb` after provisioning.\n"]
    pub fn total_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `total_vm_storage_size_gb` after provisioning.\n"]
    pub fn total_vm_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_vm_storage_size_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_action_timeout_mins: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_of_week: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours_of_day: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_custom_action_timeout_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lead_time_week: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    months: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patching_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weeks_of_month: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
}

impl DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
    #[doc = "Set the field `custom_action_timeout_mins`.\n"]
    pub fn set_custom_action_timeout_mins(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_action_timeout_mins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `days_of_week`.\n"]
    pub fn set_days_of_week(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.days_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hours_of_day`.\n"]
    pub fn set_hours_of_day(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.hours_of_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_custom_action_timeout_enabled`.\n"]
    pub fn set_is_custom_action_timeout_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_custom_action_timeout_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lead_time_week`.\n"]
    pub fn set_lead_time_week(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lead_time_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `months`.\n"]
    pub fn set_months(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.months = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `patching_mode`.\n"]
    pub fn set_patching_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.patching_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preference`.\n"]
    pub fn set_preference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weeks_of_month`.\n"]
    pub fn set_weeks_of_month(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.weeks_of_month = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {}

impl BuildDataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
    pub fn build(self) -> DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
        DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
            custom_action_timeout_mins: core::default::Default::default(),
            days_of_week: core::default::Default::default(),
            hours_of_day: core::default::Default::default(),
            is_custom_action_timeout_enabled: core::default::Default::default(),
            lead_time_week: core::default::Default::default(),
            months: core::default::Default::default(),
            patching_mode: core::default::Default::default(),
            preference: core::default::Default::default(),
            weeks_of_month: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef {
        DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_action_timeout_mins` after provisioning.\n"]
    pub fn custom_action_timeout_mins(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_action_timeout_mins", self.base))
    }

    #[doc = "Get a reference to the value of field `days_of_week` after provisioning.\n"]
    pub fn days_of_week(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `hours_of_day` after provisioning.\n"]
    pub fn hours_of_day(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hours_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `is_custom_action_timeout_enabled` after provisioning.\n"]
    pub fn is_custom_action_timeout_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_custom_action_timeout_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `lead_time_week` after provisioning.\n"]
    pub fn lead_time_week(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lead_time_week", self.base))
    }

    #[doc = "Get a reference to the value of field `months` after provisioning.\n"]
    pub fn months(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.months", self.base))
    }

    #[doc = "Get a reference to the value of field `patching_mode` after provisioning.\n"]
    pub fn patching_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patching_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `preference` after provisioning.\n"]
    pub fn preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }

    #[doc = "Get a reference to the value of field `weeks_of_month` after provisioning.\n"]
    pub fn weeks_of_month(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weeks_of_month", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseCloudExadataInfrastructure_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activated_storage_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_storage_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_contacts: std
    ::option
    ::Option<terrars::ListField<DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_storage_size_tb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_node_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_server_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exascale_config: std
    ::option
    ::Option<terrars::ListField<DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std
    ::option
    ::Option<terrars::ListField<DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_cpu_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_data_storage_tb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_db_node_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_db_server_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly_storage_server_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_maintenance_run_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_maintenance_run_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_security_maintenance_run_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shape: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_server_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOracleDatabaseCloudExadataInfrastructure_Properties {
    #[doc = "Set the field `activated_storage_count`.\n"]
    pub fn set_activated_storage_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.activated_storage_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_storage_count`.\n"]
    pub fn set_additional_storage_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.additional_storage_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_storage_size_gb`.\n"]
    pub fn set_available_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_count`.\n"]
    pub fn set_compute_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.compute_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_count`.\n"]
    pub fn set_cpu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_contacts`.\n"]
    pub fn set_customer_contacts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts>,
                    >,
    ) -> Self {
        self.customer_contacts = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `db_server_version`.\n"]
    pub fn set_db_server_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_server_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exascale_config`.\n"]
    pub fn set_exascale_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig>,
                    >,
    ) -> Self {
        self.exascale_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow>,
                    >,
    ) -> Self {
        self.maintenance_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_cpu_count`.\n"]
    pub fn set_max_cpu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_cpu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_data_storage_tb`.\n"]
    pub fn set_max_data_storage_tb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_data_storage_tb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_db_node_storage_size_gb`.\n"]
    pub fn set_max_db_node_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_db_node_storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_memory_gb`.\n"]
    pub fn set_max_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size_gb`.\n"]
    pub fn set_memory_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_db_server_version`.\n"]
    pub fn set_monthly_db_server_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.monthly_db_server_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly_storage_server_version`.\n"]
    pub fn set_monthly_storage_server_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.monthly_storage_server_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_maintenance_run_id`.\n"]
    pub fn set_next_maintenance_run_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_maintenance_run_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_maintenance_run_time`.\n"]
    pub fn set_next_maintenance_run_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_maintenance_run_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_security_maintenance_run_time`.\n"]
    pub fn set_next_security_maintenance_run_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_security_maintenance_run_time = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `shape`.\n"]
    pub fn set_shape(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shape = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_count`.\n"]
    pub fn set_storage_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_server_version`.\n"]
    pub fn set_storage_server_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.storage_server_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_storage_size_gb`.\n"]
    pub fn set_total_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_storage_size_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseCloudExadataInfrastructure_Properties {
    type O = terrars::BlockAssignable<DataOracleDatabaseCloudExadataInfrastructure_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseCloudExadataInfrastructure_Properties {}

impl BuildDataOracleDatabaseCloudExadataInfrastructure_Properties {
    pub fn build(self) -> DataOracleDatabaseCloudExadataInfrastructure_Properties {
        DataOracleDatabaseCloudExadataInfrastructure_Properties {
            activated_storage_count: core::default::Default::default(),
            additional_storage_count: core::default::Default::default(),
            available_storage_size_gb: core::default::Default::default(),
            compute_count: core::default::Default::default(),
            cpu_count: core::default::Default::default(),
            customer_contacts: core::default::Default::default(),
            data_storage_size_tb: core::default::Default::default(),
            db_node_storage_size_gb: core::default::Default::default(),
            db_server_version: core::default::Default::default(),
            exascale_config: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            max_cpu_count: core::default::Default::default(),
            max_data_storage_tb: core::default::Default::default(),
            max_db_node_storage_size_gb: core::default::Default::default(),
            max_memory_gb: core::default::Default::default(),
            memory_size_gb: core::default::Default::default(),
            monthly_db_server_version: core::default::Default::default(),
            monthly_storage_server_version: core::default::Default::default(),
            next_maintenance_run_id: core::default::Default::default(),
            next_maintenance_run_time: core::default::Default::default(),
            next_security_maintenance_run_time: core::default::Default::default(),
            oci_url: core::default::Default::default(),
            ocid: core::default::Default::default(),
            shape: core::default::Default::default(),
            state: core::default::Default::default(),
            storage_count: core::default::Default::default(),
            storage_server_version: core::default::Default::default(),
            total_storage_size_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseCloudExadataInfrastructure_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseCloudExadataInfrastructure_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseCloudExadataInfrastructure_PropertiesRef {
        DataOracleDatabaseCloudExadataInfrastructure_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseCloudExadataInfrastructure_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activated_storage_count` after provisioning.\n"]
    pub fn activated_storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activated_storage_count", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_storage_count` after provisioning.\n"]
    pub fn additional_storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_storage_count", self.base))
    }

    #[doc = "Get a reference to the value of field `available_storage_size_gb` after provisioning.\n"]
    pub fn available_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `compute_count` after provisioning.\n"]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\n"]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_contacts` after provisioning.\n"]
    pub fn customer_contacts(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_contacts", self.base))
    }

    #[doc = "Get a reference to the value of field `data_storage_size_tb` after provisioning.\n"]
    pub fn data_storage_size_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_tb", self.base))
    }

    #[doc = "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\n"]
    pub fn db_node_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `db_server_version` after provisioning.\n"]
    pub fn db_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_server_version", self.base))
    }

    #[doc = "Get a reference to the value of field `exascale_config` after provisioning.\n"]
    pub fn exascale_config(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exascale_config", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `max_cpu_count` after provisioning.\n"]
    pub fn max_cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cpu_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_data_storage_tb` after provisioning.\n"]
    pub fn max_data_storage_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_data_storage_tb", self.base))
    }

    #[doc = "Get a reference to the value of field `max_db_node_storage_size_gb` after provisioning.\n"]
    pub fn max_db_node_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_db_node_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `max_memory_gb` after provisioning.\n"]
    pub fn max_memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_memory_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\n"]
    pub fn memory_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_db_server_version` after provisioning.\n"]
    pub fn monthly_db_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monthly_db_server_version", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly_storage_server_version` after provisioning.\n"]
    pub fn monthly_storage_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monthly_storage_server_version", self.base))
    }

    #[doc = "Get a reference to the value of field `next_maintenance_run_id` after provisioning.\n"]
    pub fn next_maintenance_run_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_maintenance_run_id", self.base))
    }

    #[doc = "Get a reference to the value of field `next_maintenance_run_time` after provisioning.\n"]
    pub fn next_maintenance_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_maintenance_run_time", self.base))
    }

    #[doc = "Get a reference to the value of field `next_security_maintenance_run_time` after provisioning.\n"]
    pub fn next_security_maintenance_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_security_maintenance_run_time", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_url` after provisioning.\n"]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `shape` after provisioning.\n"]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_count` after provisioning.\n"]
    pub fn storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_count", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_server_version` after provisioning.\n"]
    pub fn storage_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_server_version", self.base))
    }

    #[doc = "Get a reference to the value of field `total_storage_size_gb` after provisioning.\n"]
    pub fn total_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_size_gb", self.base))
    }
}
