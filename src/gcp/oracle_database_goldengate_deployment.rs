#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OracleDatabaseGoldengateDeploymentData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_oracle_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    goldengate_deployment_id: terrars::PrimField<std::string::String>,
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
    properties: std::option::Option<std::vec::Vec<OracleDatabaseGoldengateDeployment_Properties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OracleDatabaseGoldengateDeployment_Timeouts>,
    dynamic: OracleDatabaseGoldengateDeploymentDynamic,
}

struct OracleDatabaseGoldengateDeployment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OracleDatabaseGoldengateDeploymentData_>,
}

#[derive(Clone)]
pub struct OracleDatabaseGoldengateDeployment(std::rc::Rc<OracleDatabaseGoldengateDeployment_>);

impl OracleDatabaseGoldengateDeployment {
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gcp_oracle_zone`.\nThe GCP Oracle zone where Oracle GoldengateDeployment is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
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
        "Set the field `labels`.\nThe labels or tags associated with the GoldengateDeployment.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `odb_network`.\nThe name of the OdbNetwork associated with the GoldengateDeployment."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<OracleDatabaseGoldengateDeployment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the GoldengateDeployment was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the GoldengateDeployment."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the\nGoldengateDeployment"]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle GoldengateDeployment is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `goldengate_deployment_id` after provisioning.\nThe ID of the GoldengateDeployment to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn goldengate_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.goldengate_deployment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the GoldengateDeployment.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the GoldengateDeployment resource in the following format:\nprojects/{project}/locations/{region}/goldengateDeployments/{goldengate_deployment}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nHTTPS link to OCI resources exposed to Customer via UI Interface."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the GoldengateDeployment."]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the GoldengateDeployment for IP\nallocation."]
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
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseGoldengateDeployment_TimeoutsRef {
        OracleDatabaseGoldengateDeployment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OracleDatabaseGoldengateDeployment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OracleDatabaseGoldengateDeployment { }

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment {
    type O = terrars::ListRef<OracleDatabaseGoldengateDeploymentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OracleDatabaseGoldengateDeployment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_oracle_database_goldengate_deployment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment {
    pub tf_id: std::string::String,
    #[doc = "The display name for the GoldengateDeployment."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the GoldengateDeployment to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub goldengate_deployment_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of the OdbSubnet associated with the GoldengateDeployment for IP\nallocation."]
    pub odb_subnet: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateDeployment {
    pub fn build(self, stack: &mut terrars::Stack) -> OracleDatabaseGoldengateDeployment {
        let out = OracleDatabaseGoldengateDeployment(std::rc::Rc::new(OracleDatabaseGoldengateDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OracleDatabaseGoldengateDeploymentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                gcp_oracle_zone: core::default::Default::default(),
                goldengate_deployment_id: self.goldengate_deployment_id,
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

pub struct OracleDatabaseGoldengateDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OracleDatabaseGoldengateDeploymentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the GoldengateDeployment was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the GoldengateDeployment."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the\nGoldengateDeployment"]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle GoldengateDeployment is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `goldengate_deployment_id` after provisioning.\nThe ID of the GoldengateDeployment to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn goldengate_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.goldengate_deployment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the GoldengateDeployment.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the GoldengateDeployment resource in the following format:\nprojects/{project}/locations/{region}/goldengateDeployments/{goldengate_deployment}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nHTTPS link to OCI resources exposed to Customer via UI Interface."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the GoldengateDeployment."]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the GoldengateDeployment for IP\nallocation."]
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
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseGoldengateDeployment_TimeoutsRef {
        OracleDatabaseGoldengateDeployment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateDeploymentDynamic {
    properties: std::option::Option<terrars::DynamicBlock<OracleDatabaseGoldengateDeployment_Properties>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Properties_IngressIps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateDeployment_Properties_IngressIps {
    #[doc = "Set the field `ingress_ip_address`.\n"]
    pub fn set_ingress_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ingress_ip_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Properties_IngressIps {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_IngressIps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Properties_IngressIps {}

impl BuildOracleDatabaseGoldengateDeployment_Properties_IngressIps {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Properties_IngressIps {
        OracleDatabaseGoldengateDeployment_Properties_IngressIps {
            ingress_ip_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_Properties_IngressIpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_Properties_IngressIpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_Properties_IngressIpsRef {
        OracleDatabaseGoldengateDeployment_Properties_IngressIpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_Properties_IngressIpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ingress_ip_address` after provisioning.\n"]
    pub fn ingress_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_ip_address", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Properties_Locks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compartment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_resource_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateDeployment_Properties_Locks {
    #[doc = "Set the field `compartment_id`.\n"]
    pub fn set_compartment_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compartment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `related_resource_id`.\n"]
    pub fn set_related_resource_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.related_resource_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Properties_Locks {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_Locks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Properties_Locks {}

impl BuildOracleDatabaseGoldengateDeployment_Properties_Locks {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Properties_Locks {
        OracleDatabaseGoldengateDeployment_Properties_Locks {
            compartment_id: core::default::Default::default(),
            create_time: core::default::Default::default(),
            message: core::default::Default::default(),
            related_resource_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_Properties_LocksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_Properties_LocksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_Properties_LocksRef {
        OracleDatabaseGoldengateDeployment_Properties_LocksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_Properties_LocksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compartment_id` after provisioning.\n"]
    pub fn compartment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compartment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `related_resource_id` after provisioning.\n"]
    pub fn related_resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.related_resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Properties_Placements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fault_domain: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateDeployment_Properties_Placements {
    #[doc = "Set the field `availability_domain`.\n"]
    pub fn set_availability_domain(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.availability_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fault_domain`.\n"]
    pub fn set_fault_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fault_domain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Properties_Placements {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_Placements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Properties_Placements {}

impl BuildOracleDatabaseGoldengateDeployment_Properties_Placements {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Properties_Placements {
        OracleDatabaseGoldengateDeployment_Properties_Placements {
            availability_domain: core::default::Default::default(),
            fault_domain: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_Properties_PlacementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_Properties_PlacementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_Properties_PlacementsRef {
        OracleDatabaseGoldengateDeployment_Properties_PlacementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_Properties_PlacementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_domain` after provisioning.\n"]
    pub fn availability_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `fault_domain` after provisioning.\n"]
    pub fn fault_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fault_domain", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Properties_BackupSchedule {}

impl OracleDatabaseGoldengateDeployment_Properties_BackupSchedule { }

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Properties_BackupSchedule {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_BackupSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Properties_BackupSchedule {}

impl BuildOracleDatabaseGoldengateDeployment_Properties_BackupSchedule {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Properties_BackupSchedule {
        OracleDatabaseGoldengateDeployment_Properties_BackupSchedule {}
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_Properties_BackupScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_Properties_BackupScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_Properties_BackupScheduleRef {
        OracleDatabaseGoldengateDeployment_Properties_BackupScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_Properties_BackupScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_scheduled_time` after provisioning.\nThe timestamp of when the backup was scheduled."]
    pub fn backup_scheduled_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_scheduled_time", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe bucket name."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `compartment_id` after provisioning.\nThe compartment id."]
    pub fn compartment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compartment_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `frequency_backup_scheduled` after provisioning.\nThe frequency backup scheduled.\nPossible values:\nDAILY\nWEEKLY\nMONTHLY"]
    pub fn frequency_backup_scheduled(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency_backup_scheduled", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_only` after provisioning.\nIf metadata only."]
    pub fn metadata_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_only", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe namespace name."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData {}

impl OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData { }

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData {}

impl BuildOracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData {
        OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData {}
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticDataRef {
        OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe bucket name."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `diagnostic_end_time` after provisioning.\nThe time diagnostic end."]
    pub fn diagnostic_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.diagnostic_end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `diagnostic_start_time` after provisioning.\nThe time diagnostic start."]
    pub fn diagnostic_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.diagnostic_start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `diagnostic_state` after provisioning.\nThe diagnostic state.\nPossible values:\nIN_PROGRESS\nSUCCEEDED\nFAILED"]
    pub fn diagnostic_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.diagnostic_state", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe namespace name."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nThe object name."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bundle_release_upgrade_period_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interim_release_upgrade_period_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_interim_release_auto_upgrade_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    major_release_upgrade_period_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_patch_upgrade_period_days: std::option::Option<terrars::PrimField<f64>>,
}

impl OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig {
    #[doc =
        "Set the field `bundle_release_upgrade_period_days`.\nDefines auto upgrade period for bundle releases. Manually configured period\ncannot be longer than service defined period for bundle releases. This\nperiod must be shorter or equal to major release upgrade period. Not\npassing this field during create will equate to using the service default."]
    pub fn set_bundle_release_upgrade_period_days(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.bundle_release_upgrade_period_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interim_release_upgrade_period_days`.\nDefines auto upgrade period for interim releases. This period must be\nshorter or equal to bundle release upgrade period."]
    pub fn set_interim_release_upgrade_period_days(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.interim_release_upgrade_period_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_interim_release_auto_upgrade_enabled`.\nBy default auto upgrade for interim releases are not enabled. If\nauto-upgrade is enabled for interim release,  you have to specify\ninterim_release_upgrade_period_days too."]
    pub fn set_is_interim_release_auto_upgrade_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.is_interim_release_auto_upgrade_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `major_release_upgrade_period_days`.\nDefines auto upgrade period for major releases. Manually configured period\ncannot be longer than service defined period for major releases. Not\npassing this field during create will equate to using the service default."]
    pub fn set_major_release_upgrade_period_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.major_release_upgrade_period_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_patch_upgrade_period_days`.\nDefines auto upgrade period for releases with security fix. Manually\nconfigured period cannot be longer than service defined period for security\nreleases. Not passing this field during create will equate to using the\nservice default."]
    pub fn set_security_patch_upgrade_period_days(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.security_patch_upgrade_period_days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig {}

impl BuildOracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig {
        OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig {
            bundle_release_upgrade_period_days: core::default::Default::default(),
            interim_release_upgrade_period_days: core::default::Default::default(),
            is_interim_release_auto_upgrade_enabled: core::default::Default::default(),
            major_release_upgrade_period_days: core::default::Default::default(),
            security_patch_upgrade_period_days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfigRef {
        OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bundle_release_upgrade_period_days` after provisioning.\nDefines auto upgrade period for bundle releases. Manually configured period\ncannot be longer than service defined period for bundle releases. This\nperiod must be shorter or equal to major release upgrade period. Not\npassing this field during create will equate to using the service default."]
    pub fn bundle_release_upgrade_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_release_upgrade_period_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interim_release_upgrade_period_days` after provisioning.\nDefines auto upgrade period for interim releases. This period must be\nshorter or equal to bundle release upgrade period."]
    pub fn interim_release_upgrade_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interim_release_upgrade_period_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_interim_release_auto_upgrade_enabled` after provisioning.\nBy default auto upgrade for interim releases are not enabled. If\nauto-upgrade is enabled for interim release,  you have to specify\ninterim_release_upgrade_period_days too."]
    pub fn is_interim_release_auto_upgrade_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_interim_release_auto_upgrade_enabled", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `major_release_upgrade_period_days` after provisioning.\nDefines auto upgrade period for major releases. Manually configured period\ncannot be longer than service defined period for major releases. Not\npassing this field during create will equate to using the service default."]
    pub fn major_release_upgrade_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.major_release_upgrade_period_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_patch_upgrade_period_days` after provisioning.\nDefines auto upgrade period for releases with security fix. Manually\nconfigured period cannot be longer than service defined period for security\nreleases. Not passing this field during create will equate to using the\nservice default."]
    pub fn security_patch_upgrade_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_patch_upgrade_period_days", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow {
    day: terrars::PrimField<std::string::String>,
    start_hour: terrars::PrimField<f64>,
}

impl OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow { }

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow {
    #[doc = "Possible values:\nMONDAY\nTUESDAY\nWEDNESDAY\nTHURSDAY\nFRIDAY\nSATURDAY\nSUNDAY"]
    pub day: terrars::PrimField<std::string::String>,
    #[doc = "Start hour for maintenance period. Hour is in UTC."]
    pub start_hour: terrars::PrimField<f64>,
}

impl BuildOracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow {
        OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow {
            day: self.day,
            start_hour: self.start_hour,
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindowRef {
        OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nPossible values:\nMONDAY\nTUESDAY\nWEDNESDAY\nTHURSDAY\nFRIDAY\nSATURDAY\nSUNDAY"]
    pub fn day(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_hour` after provisioning.\nStart hour for maintenance period. Hour is in UTC."]
    pub fn start_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_hour", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping {}

impl OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping { }

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping {}

impl BuildOracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping {
        OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping {}
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMappingRef {
        OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `administrator_group_id` after provisioning.\nThe administrator group id."]
    pub fn administrator_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.administrator_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `operator_group_id` after provisioning.\nThe operator group id."]
    pub fn operator_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\nThe security group id."]
    pub fn security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_group_id` after provisioning.\nThe user group id."]
    pub fn user_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_group_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateDeployment_Properties_OggDataDynamic {
    group_roles_mapping: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Properties_OggData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    admin_username: terrars::PrimField<std::string::String>,
    deployment: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ogg_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_roles_mapping: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping>>,
    dynamic: OracleDatabaseGoldengateDeployment_Properties_OggDataDynamic,
}

impl OracleDatabaseGoldengateDeployment_Properties_OggData {
    #[doc = "Set the field `admin_password`.\nThe Goldengate deployment console password in plain text."]
    pub fn set_admin_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.admin_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `admin_password_secret_version`.\nInput only. The Goldengate deployment console password secret version."]
    pub fn set_admin_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.admin_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ogg_version`.\nVersion of OGG"]
    pub fn set_ogg_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ogg_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_roles_mapping`.\n"]
    pub fn set_group_roles_mapping(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMapping>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.group_roles_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.group_roles_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Properties_OggData {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_OggData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Properties_OggData {
    #[doc = "The Goldengate deployment console username."]
    pub admin_username: terrars::PrimField<std::string::String>,
    #[doc =
        "The name given to the Goldengate service deployment. The name must be 1 to\n32 characters long, must contain only alphanumeric characters and must\nstart with a letter."]
    pub deployment: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateDeployment_Properties_OggData {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Properties_OggData {
        OracleDatabaseGoldengateDeployment_Properties_OggData {
            admin_password: core::default::Default::default(),
            admin_password_secret_version: core::default::Default::default(),
            admin_username: self.admin_username,
            deployment: self.deployment,
            ogg_version: core::default::Default::default(),
            group_roles_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_Properties_OggDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_Properties_OggDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_Properties_OggDataRef {
        OracleDatabaseGoldengateDeployment_Properties_OggDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_Properties_OggDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_password` after provisioning.\nThe Goldengate deployment console password in plain text."]
    pub fn admin_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `admin_password_secret_version` after provisioning.\nInput only. The Goldengate deployment console password secret version."]
    pub fn admin_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `admin_username` after provisioning.\nThe Goldengate deployment console username."]
    pub fn admin_username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_username", self.base))
    }

    #[doc =
        "Get a reference to the value of field `certificate` after provisioning.\nThe certificate of the GoldengateDeployment."]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `credential_store` after provisioning.\nThe credential store of the GoldengateDeployment.\nPossible values:\nGOLDENGATE\nIAM"]
    pub fn credential_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credential_store", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment` after provisioning.\nThe name given to the Goldengate service deployment. The name must be 1 to\n32 characters long, must contain only alphanumeric characters and must\nstart with a letter."]
    pub fn deployment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_domain_id` after provisioning.\nThe identity domain id of the GoldengateDeployment."]
    pub fn identity_domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_domain_id", self.base))
    }

    #[doc = "Get a reference to the value of field `ogg_version` after provisioning.\nVersion of OGG"]
    pub fn ogg_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ogg_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_id` after provisioning.\nThe password secret id of the GoldengateDeployment."]
    pub fn password_secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_id", self.base))
    }

    #[doc = "Get a reference to the value of field `group_roles_mapping` after provisioning.\n"]
    pub fn group_roles_mapping(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_Properties_OggData_GroupRolesMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_roles_mapping", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateDeployment_PropertiesDynamic {
    backup_schedule: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateDeployment_Properties_BackupSchedule>>,
    deployment_diagnostic_data: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData>>,
    maintenance_config: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig>>,
    maintenance_window: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow>>,
    ogg_data: std::option::Option<terrars::DynamicBlock<OracleDatabaseGoldengateDeployment_Properties_OggData>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    deployment_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_auto_scaling_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_schedule: std::option::Option<std::vec::Vec<OracleDatabaseGoldengateDeployment_Properties_BackupSchedule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_diagnostic_data: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_config: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ogg_data: std::option::Option<std::vec::Vec<OracleDatabaseGoldengateDeployment_Properties_OggData>>,
    dynamic: OracleDatabaseGoldengateDeployment_PropertiesDynamic,
}

impl OracleDatabaseGoldengateDeployment_Properties {
    #[doc = "Set the field `cpu_core_count`.\nThe Minimum number of OCPUs to be made available for this Deployment."]
    pub fn set_cpu_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the GoldengateDeployment."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_type`.\nThe environment type of the GoldengateDeployment."]
    pub fn set_environment_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_auto_scaling_enabled`.\nIndicates if auto scaling is enabled for the Deployment's CPU core count."]
    pub fn set_is_auto_scaling_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_auto_scaling_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `license_model`.\nThe Oracle license model that applies to a Deployment.\nPossible values:\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub fn set_license_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.license_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_schedule`.\n"]
    pub fn set_backup_schedule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_BackupSchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.backup_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment_diagnostic_data`.\n"]
    pub fn set_deployment_diagnostic_data(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticData>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deployment_diagnostic_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deployment_diagnostic_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_config`.\n"]
    pub fn set_maintenance_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_config = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindow>>,
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

    #[doc = "Set the field `ogg_data`.\n"]
    pub fn set_ogg_data(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties_OggData>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ogg_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ogg_data = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Properties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Properties {
    #[doc =
        "A valid Goldengate Deployment type. For a list of supported types, use\nthe 'ListGoldengateDeploymentTypes' operation."]
    pub deployment_type: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateDeployment_Properties {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Properties {
        OracleDatabaseGoldengateDeployment_Properties {
            cpu_core_count: core::default::Default::default(),
            deployment_type: self.deployment_type,
            description: core::default::Default::default(),
            environment_type: core::default::Default::default(),
            is_auto_scaling_enabled: core::default::Default::default(),
            license_model: core::default::Default::default(),
            backup_schedule: core::default::Default::default(),
            deployment_diagnostic_data: core::default::Default::default(),
            maintenance_config: core::default::Default::default(),
            maintenance_window: core::default::Default::default(),
            ogg_data: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_PropertiesRef {
        OracleDatabaseGoldengateDeployment_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `category` after provisioning.\nThe category of the GoldengateDeployment.\nPossible values:\nDATA_REPLICATION\nDATA_TRANSFORMS"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe Minimum number of OCPUs to be made available for this Deployment."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_backup_id` after provisioning.\nThe deployment backup id of the GoldengateDeployment."]
    pub fn deployment_backup_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_backup_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_role` after provisioning.\nThe deployment role of the GoldengateDeployment.\nPossible values:\nPRIMARY\nSTANDBY"]
    pub fn deployment_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_role", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_type` after provisioning.\nA valid Goldengate Deployment type. For a list of supported types, use\nthe 'ListGoldengateDeploymentTypes' operation."]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_url` after provisioning.\nThe deployment url of the GoldengateDeployment."]
    pub fn deployment_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the GoldengateDeployment."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_type` after provisioning.\nThe environment type of the GoldengateDeployment."]
    pub fn environment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fqdn` after provisioning.\nThe Fully Qualified Domain Name of the GoldengateDeployment."]
    pub fn fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `healthy` after provisioning.\nWhether the GoldengateDeployment is healthy."]
    pub fn healthy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ingress_ips` after provisioning.\nThe ingress ips of the GoldengateDeployment."]
    pub fn ingress_ips(&self) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_Properties_IngressIpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_auto_scaling_enabled` after provisioning.\nIndicates if auto scaling is enabled for the Deployment's CPU core count."]
    pub fn is_auto_scaling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_auto_scaling_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_latest_version` after provisioning.\nWhether the GoldengateDeployment is of the latest version."]
    pub fn is_latest_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_latest_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_public` after provisioning.\nWhether the GoldengateDeployment is public."]
    pub fn is_public(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_public", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_storage_utilization_limit_exceeded` after provisioning.\nWhether storage utilization limit is exceeded of the GoldengateDeployment."]
    pub fn is_storage_utilization_limit_exceeded(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_storage_utilization_limit_exceeded", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_backup_schedule_time` after provisioning.\nThe time last backup scheduled of the GoldengateDeployment."]
    pub fn last_backup_schedule_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_backup_schedule_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `license_model` after provisioning.\nThe Oracle license model that applies to a Deployment.\nPossible values:\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub fn license_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_details` after provisioning.\nThe lifecycle details of the GoldengateDeployment."]
    pub fn lifecycle_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_details", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_state` after provisioning.\nState of the GoldengateDeployment.\nPossible values:\nCREATING\nUPDATING\nACTIVE\nINACTIVE\nDELETING\nDELETED\nFAILED\nNEEDS_ATTENTION\nIN_PROGRESS\nCANCELLING\nCANCELLED\nSUCCEEDED\nWAITING"]
    pub fn lifecycle_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_sub_state` after provisioning.\nThe lifecycle sub-state of the GoldengateDeployment.\nPossible values:\nRECOVERING\nSTARTING\nSTOPPING\nMOVING\nUPGRADING\nRESTORING\nBACKING_UP\nROLLING_BACK"]
    pub fn lifecycle_sub_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_sub_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `load_balancer_id` after provisioning.\nThe load balancer id of the GoldengateDeployment."]
    pub fn load_balancer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `load_balancer_subnet_id` after provisioning.\nThe load balancer subnet id of the GoldengateDeployment."]
    pub fn load_balancer_subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_subnet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `locks` after provisioning.\nThe locks of the GoldengateDeployment."]
    pub fn locks(&self) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_Properties_LocksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.locks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_backup_schedule_time` after provisioning.\nThe time next backup scheduled of the GoldengateDeployment."]
    pub fn next_backup_schedule_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_backup_schedule_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_maintenance_action_type` after provisioning.\nThe next maintenance action type of the GoldengateDeployment.\nPossible values:\nUPGRADE"]
    pub fn next_maintenance_action_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_maintenance_action_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_maintenance_description` after provisioning.\nThe next maintenance description of the GoldengateDeployment."]
    pub fn next_maintenance_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_maintenance_description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_maintenance_time` after provisioning.\nThe time of next maintenance of the GoldengateDeployment."]
    pub fn next_maintenance_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_maintenance_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nsg_ids` after provisioning.\nThe nsg ids of the GoldengateDeployment."]
    pub fn nsg_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nsg_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\nOCID of the GoldengateDeployment."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ogg_version_support_end_time` after provisioning.\nThe time ogg version supported until of the GoldengateDeployment."]
    pub fn ogg_version_support_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ogg_version_support_end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `placements` after provisioning.\nThe placements of the GoldengateDeployment."]
    pub fn placements(&self) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_Properties_PlacementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placements", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_ip_address` after provisioning.\nThe private ip address of the GoldengateDeployment."]
    pub fn private_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_address` after provisioning.\nThe public ip address of the GoldengateDeployment."]
    pub fn public_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `role_change_time` after provisioning.\nThe time when the role of the GoldengateDeployment was changed."]
    pub fn role_change_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_change_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_utilization_bytes` after provisioning.\nThe storage utilization in bytes of the GoldengateDeployment."]
    pub fn storage_utilization_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_utilization_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the GoldengateDeployment was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `upgrade_required_time` after provisioning.\nThe time upgrade required of the GoldengateDeployment."]
    pub fn upgrade_required_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upgrade_required_time", self.base))
    }

    #[doc = "Get a reference to the value of field `backup_schedule` after provisioning.\n"]
    pub fn backup_schedule(&self) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_Properties_BackupScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment_diagnostic_data` after provisioning.\n"]
    pub fn deployment_diagnostic_data(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_Properties_DeploymentDiagnosticDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_diagnostic_data", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_config` after provisioning.\n"]
    pub fn maintenance_config(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_Properties_MaintenanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_Properties_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }

    #[doc = "Get a reference to the value of field `ogg_data` after provisioning.\n"]
    pub fn ogg_data(&self) -> terrars::ListRef<OracleDatabaseGoldengateDeployment_Properties_OggDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ogg_data", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateDeployment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateDeployment_Timeouts {
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

impl terrars::ToListMappable for OracleDatabaseGoldengateDeployment_Timeouts {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateDeployment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateDeployment_Timeouts {}

impl BuildOracleDatabaseGoldengateDeployment_Timeouts {
    pub fn build(self) -> OracleDatabaseGoldengateDeployment_Timeouts {
        OracleDatabaseGoldengateDeployment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateDeployment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateDeployment_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateDeployment_TimeoutsRef {
        OracleDatabaseGoldengateDeployment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateDeployment_TimeoutsRef {
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
