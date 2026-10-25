#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OracleDatabaseCloudExadataInfrastructureData_ {
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
    cloud_exadata_infrastructure_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_oracle_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<std::vec::Vec<OracleDatabaseCloudExadataInfrastructure_Properties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OracleDatabaseCloudExadataInfrastructure_Timeouts>,
    dynamic: OracleDatabaseCloudExadataInfrastructureDynamic,
}

struct OracleDatabaseCloudExadataInfrastructure_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OracleDatabaseCloudExadataInfrastructureData_>,
}

#[derive(Clone)]
pub struct OracleDatabaseCloudExadataInfrastructure(std::rc::Rc<OracleDatabaseCloudExadataInfrastructure_>);

impl OracleDatabaseCloudExadataInfrastructure {
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

    #[doc = "Set the field `display_name`.\nUser friendly name for this resource."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_oracle_zone`.\nGCP location where Oracle Exadata is hosted."]
    pub fn set_gcp_oracle_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gcp_oracle_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels or tags associated with the resource. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<OracleDatabaseCloudExadataInfrastructure_Properties>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<OracleDatabaseCloudExadataInfrastructure_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseCloudExadataInfrastructure_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseCloudExadataInfrastructure_TimeoutsRef {
        OracleDatabaseCloudExadataInfrastructure_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OracleDatabaseCloudExadataInfrastructure {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OracleDatabaseCloudExadataInfrastructure { }

impl terrars::ToListMappable for OracleDatabaseCloudExadataInfrastructure {
    type O = terrars::ListRef<OracleDatabaseCloudExadataInfrastructureRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OracleDatabaseCloudExadataInfrastructure_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_oracle_database_cloud_exadata_infrastructure".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOracleDatabaseCloudExadataInfrastructure {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the Exadata Infrastructure to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub cloud_exadata_infrastructure_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/DbServer'."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseCloudExadataInfrastructure {
    pub fn build(self, stack: &mut terrars::Stack) -> OracleDatabaseCloudExadataInfrastructure {
        let out =
            OracleDatabaseCloudExadataInfrastructure(std::rc::Rc::new(OracleDatabaseCloudExadataInfrastructure_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(OracleDatabaseCloudExadataInfrastructureData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    cloud_exadata_infrastructure_id: self.cloud_exadata_infrastructure_id,
                    deletion_policy: core::default::Default::default(),
                    deletion_protection: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    gcp_oracle_zone: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
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

pub struct OracleDatabaseCloudExadataInfrastructureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudExadataInfrastructureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OracleDatabaseCloudExadataInfrastructureRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseCloudExadataInfrastructure_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseCloudExadataInfrastructure_TimeoutsRef {
        OracleDatabaseCloudExadataInfrastructure_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseCloudExadataInfrastructureDynamic {
    properties: std::option::Option<terrars::DynamicBlock<OracleDatabaseCloudExadataInfrastructure_Properties>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_vm_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_vm_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
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

impl terrars::ToListMappable for OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
    type O = terrars::BlockAssignable<OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {}

impl BuildOracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
    pub fn build(self) -> OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
        OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfig {
            available_storage_size_gb: core::default::Default::default(),
            available_vm_storage_size_gb: core::default::Default::default(),
            total_storage_size_gb: core::default::Default::default(),
            total_vm_storage_size_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef {
        OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef {
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
pub struct OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
    email: terrars::PrimField<std::string::String>,
}

impl OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts { }

impl terrars::ToListMappable for OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
    type O = terrars::BlockAssignable<OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
    #[doc = "The email address used by Oracle to send notifications regarding databases\nand infrastructure."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
    pub fn build(self) -> OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts {
        OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts { email: self.email }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef {
        OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nThe email address used by Oracle to send notifications regarding databases\nand infrastructure."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
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

impl OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
    #[doc =
        "Set the field `custom_action_timeout_mins`.\nDetermines the amount of time the system will wait before the start of each\ndatabase server patching operation. Custom action timeout is in minutes and\nvalid value is between 15 to 120 (inclusive)."]
    pub fn set_custom_action_timeout_mins(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_action_timeout_mins = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `days_of_week`.\nDays during the week when maintenance should be performed."]
    pub fn set_days_of_week(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.days_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hours_of_day`.\nThe window of hours during the day when maintenance should be performed.\nThe window is a 4 hour slot. Valid values are:\n  0 - represents time slot 0:00 - 3:59 UTC\n  4 - represents time slot 4:00 - 7:59 UTC\n  8 - represents time slot 8:00 - 11:59 UTC\n  12 - represents time slot 12:00 - 15:59 UTC\n  16 - represents time slot 16:00 - 19:59 UTC\n  20 - represents time slot 20:00 - 23:59 UTC"]
    pub fn set_hours_of_day(mut self, v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>) -> Self {
        self.hours_of_day = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_custom_action_timeout_enabled`.\nIf true, enables the configuration of a custom action timeout (waiting\nperiod) between database server patching operations."]
    pub fn set_is_custom_action_timeout_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_custom_action_timeout_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lead_time_week`.\nLead time window allows user to set a lead time to prepare for a down time.\nThe lead time is in weeks and valid value is between 1 to 4."]
    pub fn set_lead_time_week(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lead_time_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `months`.\nMonths during the year when maintenance should be performed."]
    pub fn set_months(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.months = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `patching_mode`.\nCloud CloudExadataInfrastructure node patching method, either \"ROLLING\"\n or \"NONROLLING\". Default value is ROLLING. \n Possible values:\n PATCHING_MODE_UNSPECIFIED\nROLLING\nNON_ROLLING"]
    pub fn set_patching_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.patching_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preference`.\nThe maintenance window scheduling preference. \n Possible values:\n MAINTENANCE_WINDOW_PREFERENCE_UNSPECIFIED\nCUSTOM_PREFERENCE\nNO_PREFERENCE"]
    pub fn set_preference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preference = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `weeks_of_month`.\nWeeks during the month when maintenance should be performed. Weeks start on\nthe 1st, 8th, 15th, and 22nd days of the month, and have a duration of 7\ndays. Weeks start and end based on calendar dates, not days of the week."]
    pub fn set_weeks_of_month(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.weeks_of_month = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
    type O = terrars::BlockAssignable<OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {}

impl BuildOracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
    pub fn build(self) -> OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
        OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow {
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
pub struct OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef {
        OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `custom_action_timeout_mins` after provisioning.\nDetermines the amount of time the system will wait before the start of each\ndatabase server patching operation. Custom action timeout is in minutes and\nvalid value is between 15 to 120 (inclusive)."]
    pub fn custom_action_timeout_mins(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_action_timeout_mins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `days_of_week` after provisioning.\nDays during the week when maintenance should be performed."]
    pub fn days_of_week(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.days_of_week", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hours_of_day` after provisioning.\nThe window of hours during the day when maintenance should be performed.\nThe window is a 4 hour slot. Valid values are:\n  0 - represents time slot 0:00 - 3:59 UTC\n  4 - represents time slot 4:00 - 7:59 UTC\n  8 - represents time slot 8:00 - 11:59 UTC\n  12 - represents time slot 12:00 - 15:59 UTC\n  16 - represents time slot 16:00 - 19:59 UTC\n  20 - represents time slot 20:00 - 23:59 UTC"]
    pub fn hours_of_day(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hours_of_day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_custom_action_timeout_enabled` after provisioning.\nIf true, enables the configuration of a custom action timeout (waiting\nperiod) between database server patching operations."]
    pub fn is_custom_action_timeout_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_custom_action_timeout_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lead_time_week` after provisioning.\nLead time window allows user to set a lead time to prepare for a down time.\nThe lead time is in weeks and valid value is between 1 to 4."]
    pub fn lead_time_week(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lead_time_week", self.base))
    }

    #[doc =
        "Get a reference to the value of field `months` after provisioning.\nMonths during the year when maintenance should be performed."]
    pub fn months(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.months", self.base))
    }

    #[doc =
        "Get a reference to the value of field `patching_mode` after provisioning.\nCloud CloudExadataInfrastructure node patching method, either \"ROLLING\"\n or \"NONROLLING\". Default value is ROLLING. \n Possible values:\n PATCHING_MODE_UNSPECIFIED\nROLLING\nNON_ROLLING"]
    pub fn patching_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patching_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preference` after provisioning.\nThe maintenance window scheduling preference. \n Possible values:\n MAINTENANCE_WINDOW_PREFERENCE_UNSPECIFIED\nCUSTOM_PREFERENCE\nNO_PREFERENCE"]
    pub fn preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preference", self.base))
    }

    #[doc =
        "Get a reference to the value of field `weeks_of_month` after provisioning.\nWeeks during the month when maintenance should be performed. Weeks start on\nthe 1st, 8th, 15th, and 22nd days of the month, and have a duration of 7\ndays. Weeks start and end based on calendar dates, not days of the week."]
    pub fn weeks_of_month(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weeks_of_month", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseCloudExadataInfrastructure_PropertiesDynamic {
    customer_contacts: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts>>,
    maintenance_window: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseCloudExadataInfrastructure_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_count: std::option::Option<terrars::PrimField<f64>>,
    shape: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_contacts: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow>>,
    dynamic: OracleDatabaseCloudExadataInfrastructure_PropertiesDynamic,
}

impl OracleDatabaseCloudExadataInfrastructure_Properties {
    #[doc = "Set the field `compute_count`.\nThe number of compute servers for the Exadata Infrastructure."]
    pub fn set_compute_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.compute_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_count`.\nThe number of Cloud Exadata storage servers for the Exadata Infrastructure."]
    pub fn set_storage_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.storage_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `total_storage_size_gb`.\nThe total storage allocated to the Exadata Infrastructure\nresource, in gigabytes (GB)."]
    pub fn set_total_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_storage_size_gb = std::option::Option::Some(v.into());
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
                        terrars::BlockAssignable<OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContacts>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customer_contacts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_contacts = std::option::Option::Some(d);
            },
        }
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
                        terrars
                        ::BlockAssignable<OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindow>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseCloudExadataInfrastructure_Properties {
    type O = terrars::BlockAssignable<OracleDatabaseCloudExadataInfrastructure_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseCloudExadataInfrastructure_Properties {
    #[doc =
        "The shape of the Exadata Infrastructure. The shape determines the\namount of CPU, storage, and memory resources allocated to the instance."]
    pub shape: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseCloudExadataInfrastructure_Properties {
    pub fn build(self) -> OracleDatabaseCloudExadataInfrastructure_Properties {
        OracleDatabaseCloudExadataInfrastructure_Properties {
            compute_count: core::default::Default::default(),
            shape: self.shape,
            storage_count: core::default::Default::default(),
            total_storage_size_gb: core::default::Default::default(),
            customer_contacts: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseCloudExadataInfrastructure_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudExadataInfrastructure_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseCloudExadataInfrastructure_PropertiesRef {
        OracleDatabaseCloudExadataInfrastructure_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseCloudExadataInfrastructure_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `activated_storage_count` after provisioning.\nThe requested number of additional storage servers activated for the\nExadata Infrastructure."]
    pub fn activated_storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activated_storage_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `additional_storage_count` after provisioning.\nThe requested number of additional storage servers for the Exadata\nInfrastructure."]
    pub fn additional_storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_storage_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `available_storage_size_gb` after provisioning.\nThe available storage can be allocated to the Exadata Infrastructure\nresource, in gigabytes (GB)."]
    pub fn available_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_storage_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compute_count` after provisioning.\nThe number of compute servers for the Exadata Infrastructure."]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\nThe number of enabled CPU cores."]
    pub fn cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_tb` after provisioning.\nSize, in terabytes, of the DATA disk group."]
    pub fn data_storage_size_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_tb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `db_node_storage_size_gb` after provisioning.\nThe local node storage allocated in GBs."]
    pub fn db_node_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_node_storage_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `db_server_version` after provisioning.\nThe software version of the database servers (dom0) in the Exadata\nInfrastructure."]
    pub fn db_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exascale_config` after provisioning.\nThe Exascale configuration for the Exadata Infrastructure."]
    pub fn exascale_config(
        &self,
    ) -> terrars::ListRef<OracleDatabaseCloudExadataInfrastructure_Properties_ExascaleConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exascale_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_cpu_count` after provisioning.\nThe total number of CPU cores available."]
    pub fn max_cpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cpu_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_data_storage_tb` after provisioning.\nThe total available DATA disk group size."]
    pub fn max_data_storage_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_data_storage_tb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_db_node_storage_size_gb` after provisioning.\nThe total local node storage available in GBs."]
    pub fn max_db_node_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_db_node_storage_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_memory_gb` after provisioning.\nThe total memory available in GBs."]
    pub fn max_memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_memory_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size_gb` after provisioning.\nThe memory allocated in GBs."]
    pub fn memory_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monthly_db_server_version` after provisioning.\nThe monthly software version of the database servers (dom0)\nin the Exadata Infrastructure. Example: 20.1.15"]
    pub fn monthly_db_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monthly_db_server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `monthly_storage_server_version` after provisioning.\nThe monthly software version of the storage servers (cells)\nin the Exadata Infrastructure. Example: 20.1.15"]
    pub fn monthly_storage_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monthly_storage_server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_maintenance_run_id` after provisioning.\nThe OCID of the next maintenance run."]
    pub fn next_maintenance_run_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_maintenance_run_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_maintenance_run_time` after provisioning.\nThe time when the next maintenance run will occur."]
    pub fn next_maintenance_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_maintenance_run_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_security_maintenance_run_time` after provisioning.\nThe time when the next security maintenance run will occur."]
    pub fn next_security_maintenance_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_security_maintenance_run_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nDeep link to the OCI console to view this resource."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nOCID of created infra.\nhttps://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm#Oracle"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `shape` after provisioning.\nThe shape of the Exadata Infrastructure. The shape determines the\namount of CPU, storage, and memory resources allocated to the instance."]
    pub fn shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shape", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current lifecycle state of the Exadata Infrastructure. \n Possible values:\n STATE_UNSPECIFIED\nPROVISIONING\nAVAILABLE\nUPDATING\nTERMINATING\nTERMINATED\nFAILED\nMAINTENANCE_IN_PROGRESS"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_count` after provisioning.\nThe number of Cloud Exadata storage servers for the Exadata Infrastructure."]
    pub fn storage_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_server_version` after provisioning.\nThe software version of the storage servers (cells) in the Exadata\nInfrastructure."]
    pub fn storage_server_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_server_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_storage_size_gb` after provisioning.\nThe total storage allocated to the Exadata Infrastructure\nresource, in gigabytes (GB)."]
    pub fn total_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_storage_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_contacts` after provisioning.\n"]
    pub fn customer_contacts(
        &self,
    ) -> terrars::ListRef<OracleDatabaseCloudExadataInfrastructure_Properties_CustomerContactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_contacts", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(
        &self,
    ) -> terrars::ListRef<OracleDatabaseCloudExadataInfrastructure_Properties_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseCloudExadataInfrastructure_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseCloudExadataInfrastructure_Timeouts {
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

impl terrars::ToListMappable for OracleDatabaseCloudExadataInfrastructure_Timeouts {
    type O = terrars::BlockAssignable<OracleDatabaseCloudExadataInfrastructure_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseCloudExadataInfrastructure_Timeouts {}

impl BuildOracleDatabaseCloudExadataInfrastructure_Timeouts {
    pub fn build(self) -> OracleDatabaseCloudExadataInfrastructure_Timeouts {
        OracleDatabaseCloudExadataInfrastructure_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseCloudExadataInfrastructure_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseCloudExadataInfrastructure_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseCloudExadataInfrastructure_TimeoutsRef {
        OracleDatabaseCloudExadataInfrastructure_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseCloudExadataInfrastructure_TimeoutsRef {
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
