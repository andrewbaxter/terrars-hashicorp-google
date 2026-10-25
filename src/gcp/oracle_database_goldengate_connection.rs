#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OracleDatabaseGoldengateConnectionData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_oracle_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    goldengate_connection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_subnet: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OracleDatabaseGoldengateConnection_Timeouts>,
    dynamic: OracleDatabaseGoldengateConnectionDynamic,
}

struct OracleDatabaseGoldengateConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OracleDatabaseGoldengateConnectionData_>,
}

#[derive(Clone)]
pub struct OracleDatabaseGoldengateConnection(std::rc::Rc<OracleDatabaseGoldengateConnection_>);

impl OracleDatabaseGoldengateConnection {
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

    #[doc =
        "Set the field `gcp_oracle_zone`.\nThe GCP Oracle zone where Oracle GoldengateConnection is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
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
        "Set the field `labels`.\nThe labels or tags associated with the GoldengateConnection.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_network`.\nThe name of the OdbNetwork associated with the GoldengateConnection.\nThe format is\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}.\nIt is optional but if specified, this should match the parent ODBNetwork\nof the OdbSubnet."]
    pub fn set_odb_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().odb_network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_subnet`.\nThe name of the OdbSubnet associated with the GoldengateConnection for IP\nallocation.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
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
        v: impl std::convert::Into<terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<OracleDatabaseGoldengateConnection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the GoldengateConnection was created."]
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
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the\nGoldengateConnection."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle GoldengateConnection is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `goldengate_connection_id` after provisioning.\nThe ID of the GoldengateConnection to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn goldengate_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.goldengate_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the GoldengateConnection.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the GoldengateConnection resource in the following format:\nprojects/{project}/locations/{region}/goldengateConnections/{goldengate_connection}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nHTTPS link to OCI resources exposed to Customer via UI Interface."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the GoldengateConnection.\nThe format is\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}.\nIt is optional but if specified, this should match the parent ODBNetwork\nof the OdbSubnet."]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the GoldengateConnection for IP\nallocation.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
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
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseGoldengateConnection_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseGoldengateConnection_TimeoutsRef {
        OracleDatabaseGoldengateConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OracleDatabaseGoldengateConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OracleDatabaseGoldengateConnection { }

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection {
    type O = terrars::ListRef<OracleDatabaseGoldengateConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OracleDatabaseGoldengateConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_oracle_database_goldengate_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOracleDatabaseGoldengateConnection {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the GoldengateConnection to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub goldengate_connection_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> OracleDatabaseGoldengateConnection {
        let out = OracleDatabaseGoldengateConnection(std::rc::Rc::new(OracleDatabaseGoldengateConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OracleDatabaseGoldengateConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                gcp_oracle_zone: core::default::Default::default(),
                goldengate_connection_id: self.goldengate_connection_id,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
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

pub struct OracleDatabaseGoldengateConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OracleDatabaseGoldengateConnectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the GoldengateConnection was created."]
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
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the\nGoldengateConnection."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_zone` after provisioning.\nThe GCP Oracle zone where Oracle GoldengateConnection is hosted.\nExample: us-east4-b-r2.\nIf not specified, the system will pick a zone based on availability."]
    pub fn gcp_oracle_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `goldengate_connection_id` after provisioning.\nThe ID of the GoldengateConnection to create. This value is\nrestricted to (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of\n63 characters in length. The value must start with a letter and end with a\nletter or a number."]
    pub fn goldengate_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.goldengate_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the GoldengateConnection.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the GoldengateConnection resource in the following format:\nprojects/{project}/locations/{region}/goldengateConnections/{goldengate_connection}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nHTTPS link to OCI resources exposed to Customer via UI Interface."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the GoldengateConnection.\nThe format is\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}.\nIt is optional but if specified, this should match the parent ODBNetwork\nof the OdbSubnet."]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the GoldengateConnection for IP\nallocation.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
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
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseGoldengateConnection_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseGoldengateConnection_TimeoutsRef {
        OracleDatabaseGoldengateConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateConnectionDynamic {
    properties: std::option::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties {
    #[doc = "Set the field `access_key_id`.\nAccess key ID to access the Amazon Kinesis."]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `aws_region`.\nThe name of the AWS region.\nIf not provided, Goldengate will default to 'us-west-1'."]
    pub fn set_aws_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `endpoint`.\nThe endpoint URL of the Amazon Kinesis service.\ne.g.: 'https://kinesis.us-east-1.amazonaws.com'\nIf not provided, Goldengate will default to\n'https://kinesis..amazonaws.com'."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_access_key_secret`.\nSecret access key to access the Amazon Kinesis."]
    pub fn set_secret_access_key_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_access_key_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of AmazonKinesisConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties {
    type O =
        terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties {
            access_key_id: core::default::Default::default(),
            aws_region: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            secret_access_key_secret: core::default::Default::default(),
            technology_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nAccess key ID to access the Amazon Kinesis."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `aws_region` after provisioning.\nThe name of the AWS region.\nIf not provided, Goldengate will default to 'us-west-1'."]
    pub fn aws_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint URL of the Amazon Kinesis service.\ne.g.: 'https://kinesis.us-east-1.amazonaws.com'\nIf not provided, Goldengate will default to\n'https://kinesis..amazonaws.com'."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key_secret` after provisioning.\nSecret access key to access the Amazon Kinesis."]
    pub fn secret_access_key_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of AmazonKinesisConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties {
    #[doc =
        "Set the field `connection_url`.\nConnection URL.\ne.g.:\n'jdbc:redshift://aws-redshift-instance.aaaaaaaaaaaa.us-east-2.redshift.amazonaws.com:5439/mydb'"]
    pub fn set_connection_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses for Amazon Redshift connection\nin plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Amazon Redshift connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of AmazonRedshiftConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties {
    type O =
        terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties {
            connection_url: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_url` after provisioning.\nConnection URL.\ne.g.:\n'jdbc:redshift://aws-redshift-instance.aaaaaaaaaaaa.us-east-2.redshift.amazonaws.com:5439/mydb'"]
    pub fn connection_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses for Amazon Redshift connection\nin plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Amazon Redshift connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of AmazonRedshiftConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties {
    #[doc = "Set the field `access_key_id`.\nAccess key ID to access the Amazon S3 bucket."]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\nThe Amazon Endpoint for S3."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe name of the AWS region where the bucket is created."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_access_key_secret`.\nSecret access key to access the Amazon S3 bucket."]
    pub fn set_secret_access_key_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_access_key_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of AmazonS3Connection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties {
            access_key_id: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            region: core::default::Default::default(),
            secret_access_key_secret: core::default::Default::default(),
            technology_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nAccess key ID to access the Amazon S3 bucket."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe Amazon Endpoint for S3."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the AWS region where the bucket is created."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key_secret` after provisioning.\nSecret access key to access the Amazon S3 bucket."]
    pub fn secret_access_key_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of AmazonS3Connection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_key_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_authority_host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sas_token_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties {
    #[doc = "Set the field `account`.\nSets the Azure storage account name."]
    pub fn set_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `account_key_secret`.\nAzure storage account key. This property is required when\n'authentication_type' is set to 'SHARED_KEY'."]
    pub fn set_account_key_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.account_key_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `authentication_type`.\nAuthentication mechanism to access Azure Data Lake Storage.\nPossible values:\nSHARED_KEY\nSHARED_ACCESS_SIGNATURE\nAZURE_ACTIVE_DIRECTORY"]
    pub fn set_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `azure_authority_host`.\nThe endpoint used for authentication with Microsoft Entra ID (formerly\nAzure Active Directory). Default value:\nhttps://login.microsoftonline.com"]
    pub fn set_azure_authority_host(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.azure_authority_host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `azure_tenant_id`.\nAzure tenant ID of the application. This property is required when\n'authentication_type' is set to 'AZURE_ACTIVE_DIRECTORY'."]
    pub fn set_azure_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.azure_tenant_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_id`.\nAzure client ID of the application. This property is required when\n'authentication_type' is set to 'AZURE_ACTIVE_DIRECTORY'."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nAzure client secret (aka application password) for authentication."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\nAzure Storage service endpoint.\ne.g: https://test.blob.core.windows.net"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sas_token_secret`.\nCredential that uses a shared access signature (SAS) to authenticate to\nan Azure Service."]
    pub fn set_sas_token_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sas_token_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of AzureDataLakeStorageConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties {
    type O =
        terrars
        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties {
            account: core::default::Default::default(),
            account_key_secret: core::default::Default::default(),
            authentication_type: core::default::Default::default(),
            azure_authority_host: core::default::Default::default(),
            azure_tenant_id: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            sas_token_secret: core::default::Default::default(),
            technology_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\nSets the Azure storage account name."]
    pub fn account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `account_key_secret` after provisioning.\nAzure storage account key. This property is required when\n'authentication_type' is set to 'SHARED_KEY'."]
    pub fn account_key_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_key_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `authentication_type` after provisioning.\nAuthentication mechanism to access Azure Data Lake Storage.\nPossible values:\nSHARED_KEY\nSHARED_ACCESS_SIGNATURE\nAZURE_ACTIVE_DIRECTORY"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `azure_authority_host` after provisioning.\nThe endpoint used for authentication with Microsoft Entra ID (formerly\nAzure Active Directory). Default value:\nhttps://login.microsoftonline.com"]
    pub fn azure_authority_host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_authority_host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `azure_tenant_id` after provisioning.\nAzure tenant ID of the application. This property is required when\n'authentication_type' is set to 'AZURE_ACTIVE_DIRECTORY'."]
    pub fn azure_tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_tenant_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nAzure client ID of the application. This property is required when\n'authentication_type' is set to 'AZURE_ACTIVE_DIRECTORY'."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nAzure client secret (aka application password) for authentication."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nAzure Storage service endpoint.\ne.g: https://test.blob.core.windows.net"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sas_token_secret` after provisioning.\nCredential that uses a shared access signature (SAS) to authenticate to\nan Azure Service."]
    pub fn sas_token_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sas_token_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of AzureDataLakeStorageConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties {
    #[doc =
        "Set the field `connection_string`.\nJDBC connection string.\ne.g.:\n'jdbc:sqlserver://.sql.azuresynapse.net:1433;database=;encrypt=true;trustServerCertificate=false;hostNameInCertificate=*.sql.azuresynapse.net;loginTimeout=300;'"]
    pub fn set_connection_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_string = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses for Azure Synapse Analytics\nconnection in plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Azure Synapse Analytics\nconnection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of AzureSynapseAnalyticsConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties {
    type O =
        terrars
        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties {
            connection_string: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_string` after provisioning.\nJDBC connection string.\ne.g.:\n'jdbc:sqlserver://.sql.azuresynapse.net:1433;database=;encrypt=true;trustServerCertificate=false;hostNameInCertificate=*.sql.azuresynapse.net;loginTimeout=300;'"]
    pub fn connection_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_string", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses for Azure Synapse Analytics\nconnection in plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Azure Synapse Analytics\nconnection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of AzureSynapseAnalyticsConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_credential: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties {
    #[doc =
        "Set the field `authentication_type`.\nAuthentication type for Databricks.\nPossible values:\nPERSONAL_ACCESS_TOKEN\nOAUTH_M2M"]
    pub fn set_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\nOAuth client id, only applicable for authentication_type == OAUTH_M2M"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nOAuth client secret, only applicable for authentication_type == OAUTH_M2M"]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection_url`.\nConnection URL.\ne.g.:\n'jdbc:databricks://adb-33934.4.azuredatabricks.net:443/default;transportMode=http;ssl=1;httpPath=sql/protocolv1/o/3393########44/0##3-7-hlrb'"]
    pub fn set_connection_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\nInput only. The password used to connect to Databricks in plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password used to connect to Databricks.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_credential`.\nExternal storage credential name to access files on object\nstorage such as ADLS Gen2, S3 or Cloud Storage."]
    pub fn set_storage_credential(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.storage_credential = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of DatabricksConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties {
            authentication_type: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            connection_url: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            storage_credential: core::default::Default::default(),
            technology_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication_type` after provisioning.\nAuthentication type for Databricks.\nPossible values:\nPERSONAL_ACCESS_TOKEN\nOAUTH_M2M"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nOAuth client id, only applicable for authentication_type == OAUTH_M2M"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nOAuth client secret, only applicable for authentication_type == OAUTH_M2M"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_url` after provisioning.\nConnection URL.\ne.g.:\n'jdbc:databricks://adb-33934.4.azuredatabricks.net:443/default;transportMode=http;ssl=1;httpPath=sql/protocolv1/o/3393########44/0##3-7-hlrb'"]
    pub fn connection_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password used to connect to Databricks in plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password used to connect to Databricks.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_credential` after provisioning.\nExternal storage credential name to access files on object\nstorage such as ADLS Gen2, S3 or Cloud Storage."]
    pub fn storage_credential(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_credential", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of DatabricksConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes { }

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes {
    type O =
        terrars
        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes {
    #[doc = "The name of the property entry."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "The value of the property entry."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes {
        OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributesRef {
        OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nThe name of the property entry."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the property entry."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateConnection_Properties_Db2ConnectionPropertiesDynamic {
    additional_attributes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes>,
    >,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_client_keystash_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_client_keystoredb_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_server_certificate_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_attributes: std
    ::option
    ::Option<
        std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes>,
    >,
    dynamic: OracleDatabaseGoldengateConnection_Properties_Db2ConnectionPropertiesDynamic,
}

impl OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties {
    #[doc = "Set the field `database`.\nThe name of the database."]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nThe name or address of a host."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses for Db2 connection in plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Db2 connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe port of an endpoint usually specified for a connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_protocol`.\nSecurity protocol for the DB2 database.\nPossible values:\nPLAIN\nTLS"]
    pub fn set_security_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_client_keystash_file`.\nThe keystash file which contains the encrypted password to\nthe key database file. Not supported for IBM Db2 for i."]
    pub fn set_ssl_client_keystash_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_client_keystash_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_client_keystoredb_file`.\nThe keystore file created at the client containing the\nserver certificate / CA root certificate. Not supported for IBM Db2 for i."]
    pub fn set_ssl_client_keystoredb_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_client_keystoredb_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_server_certificate_file`.\nThe file which contains the self-signed server certificate\n/ Certificate Authority (CA) certificate."]
    pub fn set_ssl_server_certificate_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_server_certificate_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of Db2Connection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe username Oracle Goldengate uses to connect to the DB2 database."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_attributes`.\n"]
    pub fn set_additional_attributes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_attributes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties {
            database: core::default::Default::default(),
            host: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            port: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            ssl_client_keystash_file: core::default::Default::default(),
            ssl_client_keystoredb_file: core::default::Default::default(),
            ssl_server_certificate_file: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            username: core::default::Default::default(),
            additional_attributes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_Db2ConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_Db2ConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_Db2ConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_Db2ConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_Db2ConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe name of the database."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe name or address of a host."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses for Db2 connection in plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Db2 connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port of an endpoint usually specified for a connection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_protocol` after provisioning.\nSecurity protocol for the DB2 database.\nPossible values:\nPLAIN\nTLS"]
    pub fn security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_client_keystash_file` after provisioning.\nThe keystash file which contains the encrypted password to\nthe key database file. Not supported for IBM Db2 for i."]
    pub fn ssl_client_keystash_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_client_keystash_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_client_keystoredb_file` after provisioning.\nThe keystore file created at the client containing the\nserver certificate / CA root certificate. Not supported for IBM Db2 for i."]
    pub fn ssl_client_keystoredb_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_client_keystoredb_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_server_certificate_file` after provisioning.\nThe file which contains the self-signed server certificate\n/ Certificate Authority (CA) certificate."]
    pub fn ssl_server_certificate_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_server_certificate_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of Db2Connection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect to the DB2 database."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_attributes` after provisioning.\n"]
    pub fn additional_attributes(
        &self,
    ) -> terrars
    ::ListRef<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_attributes", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    servers: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties {
    #[doc =
        "Set the field `authentication_type`.\nAuthentication type for Elasticsearch.\nPossible values:\nNONE\nBASIC"]
    pub fn set_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fingerprint`.\nFingerprint required by TLS security protocol.\nEg.: '6152b2dfbff200f973c5074a5b91d06ab3b472c07c09a1ea57bb7fd406cdce9c'"]
    pub fn set_fingerprint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fingerprint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses for Elastic Search connection\nin plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Elastic Search connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_protocol`.\nSecurity protocol for Elasticsearch.\nPossible values:\nPLAIN\nTLS"]
    pub fn set_security_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `servers`.\nComma separated list of Elasticsearch server addresses, specified as\nhost:port entries, where :port is optional. If port is not specified, it\ndefaults to 9200. Example:\n\"server1.example.com:4000,server2.example.com:4000\""]
    pub fn set_servers(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of ElasticsearchConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties {
    type O =
        terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties {
            authentication_type: core::default::Default::default(),
            fingerprint: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            servers: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication_type` after provisioning.\nAuthentication type for Elasticsearch.\nPossible values:\nNONE\nBASIC"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint required by TLS security protocol.\nEg.: '6152b2dfbff200f973c5074a5b91d06ab3b472c07c09a1ea57bb7fd406cdce9c'"]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses for Elastic Search connection\nin plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Elastic Search connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_protocol` after provisioning.\nSecurity protocol for Elasticsearch.\nPossible values:\nPLAIN\nTLS"]
    pub fn security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `servers` after provisioning.\nComma separated list of Elasticsearch server addresses, specified as\nhost:port entries, where :port is optional. If port is not specified, it\ndefaults to 9200. Example:\n\"server1.example.com:4000,server2.example.com:4000\""]
    pub fn servers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.servers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of ElasticsearchConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties {
    #[doc = "Set the field `host`.\nThe host of the GenericConnection."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties {
            host: core::default::Default::default(),
            technology_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_GenericConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_GenericConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_GenericConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_GenericConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_GenericConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe host of the GenericConnection."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    goldengate_deployment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties {
    #[doc =
        "Set the field `goldengate_deployment_id`.\nThe name of the GoldengateDeployment associated with the\nGoldengateConnection.\nFormat:\nprojects/{project}/locations/{location}/goldengateDeployments/{goldengate_deployment}"]
    pub fn set_goldengate_deployment_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.goldengate_deployment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nThe host of the GoldengateConnection."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password used to connect to the Oracle Goldengate in plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password used to connect to the Oracle Goldengate.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe port of the GoldengateConnection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe username credential."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties {
            goldengate_deployment_id: core::default::Default::default(),
            host: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            port: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `goldengate_deployment_id` after provisioning.\nThe name of the GoldengateDeployment associated with the\nGoldengateConnection.\nFormat:\nprojects/{project}/locations/{location}/goldengateDeployments/{goldengate_deployment}"]
    pub fn goldengate_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.goldengate_deployment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe host of the GoldengateConnection."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password used to connect to the Oracle Goldengate in plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password used to connect to the Oracle Goldengate.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port of the GoldengateConnection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe username credential."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties {
    #[doc =
        "Set the field `service_account_key_file`.\nThe service account key file Cloud Storage containing the\ncredentials required to use Google BigQuery."]
    pub fn set_service_account_key_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_key_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties {
    type O =
        terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties {
            service_account_key_file: core::default::Default::default(),
            technology_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account_key_file` after provisioning.\nThe service account key file Cloud Storage containing the\ncredentials required to use Google BigQuery."]
    pub fn service_account_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_key_file", self.base))
    }

    #[doc = "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties {
    #[doc =
        "Set the field `service_account_key_file`.\nThe service account key Cloud Storage file containing the\ncredentials required to use Google Cloud Storage."]
    pub fn set_service_account_key_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_key_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties {
    type O =
        terrars
        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties {
            service_account_key_file: core::default::Default::default(),
            technology_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account_key_file` after provisioning.\nThe service account key Cloud Storage file containing the\ncredentials required to use Google Cloud Storage."]
    pub fn service_account_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_key_file", self.base))
    }

    #[doc = "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties {
    #[doc =
        "Set the field `service_account_key_file`.\nThe content of the service account key file containing the\ncredentials required to use Google Pub/Sub."]
    pub fn set_service_account_key_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_key_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of GooglePubsubConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties {
    type O =
        terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties {
            service_account_key_file: core::default::Default::default(),
            technology_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account_key_file` after provisioning.\nThe content of the service account key file containing the\ncredentials required to use Google Pub/Sub."]
    pub fn service_account_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_key_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of GooglePubsubConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_site_xml: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties {
    #[doc =
        "Set the field `core_site_xml`.\nThe content of the Hadoop Distributed File System\nconfiguration file (core-site.xml)."]
    pub fn set_core_site_xml(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.core_site_xml = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of HdfsConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties {
            core_site_xml: core::default::Default::default(),
            technology_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_HdfsConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_HdfsConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_HdfsConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_HdfsConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_HdfsConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `core_site_xml` after provisioning.\nThe content of the Hadoop Distributed File System\nconfiguration file (core-site.xml)."]
    pub fn core_site_xml(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_site_xml", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of HdfsConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog {
    glue_id: terrars::PrimField<std::string::String>,
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog { }

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog {
    type O =
        terrars
        ::BlockAssignable<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog {
    #[doc = "The catalog ID of Glue."]
    pub glue_id: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog {
    pub fn build(
        self,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog {
            glue_id: self.glue_id,
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalogRef {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `glue_id` after provisioning.\nThe catalog ID of Glue."]
    pub fn glue_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.glue_id", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog {
    branch: terrars::PrimField<std::string::String>,
    uri: terrars::PrimField<std::string::String>,
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog { }

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog {
    type O =
        terrars
        ::BlockAssignable<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog {
    #[doc = "The Nessie branch."]
    pub branch: terrars::PrimField<std::string::String>,
    #[doc = "The Nessie uri."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog {
    pub fn build(
        self,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog {
            branch: self.branch,
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalogRef {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch` after provisioning.\nThe Nessie branch."]
    pub fn branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe Nessie uri."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog {
    client_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    polaris_catalog: terrars::PrimField<std::string::String>,
    principal_role: terrars::PrimField<std::string::String>,
    uri: terrars::PrimField<std::string::String>,
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog {
    #[doc = "Set the field `client_secret`.\nThe Polaris client secret."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog {
    type O =
        terrars
        ::BlockAssignable<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog {
    #[doc = "The Polaris client ID."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = "The catalog name within Polaris."]
    pub polaris_catalog: terrars::PrimField<std::string::String>,
    #[doc = "The Polaris principal role."]
    pub principal_role: terrars::PrimField<std::string::String>,
    #[doc = "The Polaris uri."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog {
    pub fn build(
        self,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            polaris_catalog: self.polaris_catalog,
            principal_role: self.principal_role,
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalogRef {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe Polaris client ID."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nThe Polaris client secret."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `polaris_catalog` after provisioning.\nThe catalog name within Polaris."]
    pub fn polaris_catalog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.polaris_catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_role` after provisioning.\nThe Polaris principal role."]
    pub fn principal_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_role", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe Polaris uri."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog {
    #[doc =
        "Set the field `properties`.\nThe content of the configuration file containing additional properties for\nthe REST catalog."]
    pub fn set_properties(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog {
    type O =
        terrars
        ::BlockAssignable<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog {
    #[doc = "The REST uri."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog {
    pub fn build(
        self,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog {
            properties: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalogRef {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nThe content of the configuration file containing additional properties for\nthe REST catalog."]
    pub fn properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe REST uri."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_CatalogDynamic {
    glue_iceberg_catalog: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog,
        >,
    >,
    nessie_iceberg_catalog: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog,
        >,
    >,
    polaris_iceberg_catalog: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog,
        >,
    >,
    rest_iceberg_catalog: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog,
        >,
    >,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog {
    catalog_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glue_iceberg_catalog: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nessie_iceberg_catalog: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    polaris_iceberg_catalog: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rest_iceberg_catalog: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog>,
    >,
    dynamic: OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_CatalogDynamic,
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog {
    #[doc = "Set the field `glue_iceberg_catalog`.\n"]
    pub fn set_glue_iceberg_catalog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalog,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.glue_iceberg_catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.glue_iceberg_catalog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `nessie_iceberg_catalog`.\n"]
    pub fn set_nessie_iceberg_catalog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalog,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nessie_iceberg_catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nessie_iceberg_catalog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `polaris_iceberg_catalog`.\n"]
    pub fn set_polaris_iceberg_catalog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalog,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.polaris_iceberg_catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.polaris_iceberg_catalog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rest_iceberg_catalog`.\n"]
    pub fn set_rest_iceberg_catalog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalog,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rest_iceberg_catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rest_iceberg_catalog = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog {
    type O =
        terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog {
    #[doc = "The type of Iceberg catalog.\nPossible values:\nGLUE\nHADOOP\nNESSIE\nPOLARIS\nREST"]
    pub catalog_type: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog {
            catalog_type: self.catalog_type,
            glue_iceberg_catalog: core::default::Default::default(),
            nessie_iceberg_catalog: core::default::Default::default(),
            polaris_iceberg_catalog: core::default::Default::default(),
            rest_iceberg_catalog: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_CatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_CatalogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_CatalogRef {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_CatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_CatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `catalog_type` after provisioning.\nThe type of Iceberg catalog.\nPossible values:\nGLUE\nHADOOP\nNESSIE\nPOLARIS\nREST"]
    pub fn catalog_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_type", self.base))
    }

    #[doc = "Get a reference to the value of field `glue_iceberg_catalog` after provisioning.\n"]
    pub fn glue_iceberg_catalog(
        &self,
    ) -> terrars
    ::ListRef<
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_GlueIcebergCatalogRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glue_iceberg_catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `nessie_iceberg_catalog` after provisioning.\n"]
    pub fn nessie_iceberg_catalog(
        &self,
    ) -> terrars
    ::ListRef<
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_NessieIcebergCatalogRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nessie_iceberg_catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `polaris_iceberg_catalog` after provisioning.\n"]
    pub fn polaris_iceberg_catalog(
        &self,
    ) -> terrars
    ::ListRef<
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_PolarisIcebergCatalogRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.polaris_iceberg_catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `rest_iceberg_catalog` after provisioning.\n"]
    pub fn rest_iceberg_catalog(
        &self,
    ) -> terrars
    ::ListRef<
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog_RestIcebergCatalogRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rest_iceberg_catalog", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage {
    access_key_id: terrars::PrimField<std::string::String>,
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
    scheme_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key_secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage {
    #[doc = "Set the field `endpoint`.\nThe endpoint of Amazon S3."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_access_key_secret`.\nThe secret access key of Amazon S3."]
    pub fn set_secret_access_key_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_access_key_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage {
    type O =
        terrars
        ::BlockAssignable<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage {
    #[doc = "The access key ID of Amazon S3."]
    pub access_key_id: terrars::PrimField<std::string::String>,
    #[doc = "The bucket of Amazon S3."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "The region of Amazon S3."]
    pub region: terrars::PrimField<std::string::String>,
    #[doc = "The scheme type of Amazon S3.\nPossible values:\nS3\nS3A"]
    pub scheme_type: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage {
    pub fn build(
        self,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage {
            access_key_id: self.access_key_id,
            bucket: self.bucket,
            endpoint: core::default::Default::default(),
            region: self.region,
            scheme_type: self.scheme_type,
            secret_access_key_secret: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorageRef {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nThe access key ID of Amazon S3."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe bucket of Amazon S3."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint of Amazon S3."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of Amazon S3."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheme_type` after provisioning.\nThe scheme type of Amazon S3.\nPossible values:\nS3\nS3A"]
    pub fn scheme_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key_secret` after provisioning.\nThe secret access key of Amazon S3."]
    pub fn secret_access_key_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_key_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    azure_account: terrars::PrimField<std::string::String>,
    container: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage {
    #[doc = "Set the field `account_key_secret`.\nThe account key of Azure Data Lake Storage."]
    pub fn set_account_key_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.account_key_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\nThe endpoint of Azure Data Lake Storage."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage {
    type O =
        terrars
        ::BlockAssignable<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage {
    #[doc = "The account of Azure Data Lake Storage."]
    pub azure_account: terrars::PrimField<std::string::String>,
    #[doc = "The container of Azure Data Lake Storage."]
    pub container: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage {
    pub fn build(
        self,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage {
            account_key_secret: core::default::Default::default(),
            azure_account: self.azure_account,
            container: self.container,
            endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorageRef {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_key_secret` after provisioning.\nThe account key of Azure Data Lake Storage."]
    pub fn account_key_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_key_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `azure_account` after provisioning.\nThe account of Azure Data Lake Storage."]
    pub fn azure_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container` after provisioning.\nThe container of Azure Data Lake Storage."]
    pub fn container(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint of Azure Data Lake Storage."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage {
    bucket: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage {
    #[doc = "Set the field `service_account_key_file`.\nThe service account key file of Google Cloud Storage."]
    pub fn set_service_account_key_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_key_file = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage {
    type O =
        terrars
        ::BlockAssignable<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage {
    #[doc = "The bucket of Google Cloud Storage."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "The project ID of Google Cloud Storage."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage {
    pub fn build(
        self,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage {
            bucket: self.bucket,
            project_id: self.project_id,
            service_account_key_file: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorageRef {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe bucket of Google Cloud Storage."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe project ID of Google Cloud Storage."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_key_file` after provisioning.\nThe service account key file of Google Cloud Storage."]
    pub fn service_account_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_key_file", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_StorageDynamic {
    amazon_s3_iceberg_storage: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage,
        >,
    >,
    azure_data_lake_storage_iceberg_storage: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage,
        >,
    >,
    google_cloud_storage_iceberg_storage: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage,
        >,
    >,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage {
    storage_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_s3_iceberg_storage: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_data_lake_storage_iceberg_storage: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_storage_iceberg_storage: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage,
        >,
    >,
    dynamic: OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_StorageDynamic,
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage {
    #[doc = "Set the field `amazon_s3_iceberg_storage`.\n"]
    pub fn set_amazon_s3_iceberg_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_s3_iceberg_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_s3_iceberg_storage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_data_lake_storage_iceberg_storage`.\n"]
    pub fn set_azure_data_lake_storage_iceberg_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_data_lake_storage_iceberg_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_data_lake_storage_iceberg_storage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_cloud_storage_iceberg_storage`.\n"]
    pub fn set_google_cloud_storage_iceberg_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_cloud_storage_iceberg_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_storage_iceberg_storage = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage {
    type O =
        terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage {
    #[doc = "The type of Iceberg storage.\nPossible values:\nAMAZON_S3\nGOOGLE_CLOUD_STORAGE\nAZURE_DATA_LAKE_STORAGE"]
    pub storage_type: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage {
            storage_type: self.storage_type,
            amazon_s3_iceberg_storage: core::default::Default::default(),
            azure_data_lake_storage_iceberg_storage: core::default::Default::default(),
            google_cloud_storage_iceberg_storage: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_StorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_StorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_StorageRef {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_StorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_StorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `storage_type` after provisioning.\nThe type of Iceberg storage.\nPossible values:\nAMAZON_S3\nGOOGLE_CLOUD_STORAGE\nAZURE_DATA_LAKE_STORAGE"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_s3_iceberg_storage` after provisioning.\n"]
    pub fn amazon_s3_iceberg_storage(
        &self,
    ) -> terrars
    ::ListRef<
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AmazonS3IcebergStorageRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_s3_iceberg_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_data_lake_storage_iceberg_storage` after provisioning.\n"]
    pub fn azure_data_lake_storage_iceberg_storage(
        &self,
    ) -> terrars
    ::ListRef<
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_AzureDataLakeStorageIcebergStorageRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.azure_data_lake_storage_iceberg_storage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `google_cloud_storage_iceberg_storage` after provisioning.\n"]
    pub fn google_cloud_storage_iceberg_storage(
        &self,
    ) -> terrars
    ::ListRef<
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage_GoogleCloudStorageIcebergStorageRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_cloud_storage_iceberg_storage", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionPropertiesDynamic {
    catalog: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog>,
    >,
    storage: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage>,
    >,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties {
    technology_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage>>,
    dynamic: OracleDatabaseGoldengateConnection_Properties_IcebergConnectionPropertiesDynamic,
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties {
    #[doc = "Set the field `catalog`.\n"]
    pub fn set_catalog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Catalog,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.catalog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage`.\n"]
    pub fn set_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_Storage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties {
    #[doc = "The technology type of Iceberg connection."]
    pub technology_type: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties {
            technology_type: self.technology_type,
            catalog: core::default::Default::default(),
            storage: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_IcebergConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_IcebergConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_IcebergConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_IcebergConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_IcebergConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of Iceberg connection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc = "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_CatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `storage` after provisioning.\n"]
    pub fn storage(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties_StorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_factory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jndi_connection_factory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jndi_initial_context_factory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jndi_provider_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jndi_security_credentials_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jndi_security_principal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_key_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_key_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_jndi: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties {
    #[doc =
        "Set the field `authentication_type`.\nAuthentication type for Java Message Service.\nPossible values:\nNONE\nBASIC"]
    pub fn set_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection_factory`.\nThe Java class implementing javax.jms.ConnectionFactory interface supplied\nby the JMS provider."]
    pub fn set_connection_factory(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.connection_factory = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection_url`.\nConnection URL of the Java Message Service, specifying the protocol, host,\nand port. e.g.: 'mq://myjms.host.domain:7676'"]
    pub fn set_connection_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jndi_connection_factory`.\nThe Connection Factory can be looked up using this name.\ne.g.: 'ConnectionFactory'"]
    pub fn set_jndi_connection_factory(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.jndi_connection_factory = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jndi_initial_context_factory`.\nThe implementation of javax.naming.spi.InitialContextFactory interface\nused to obtain initial naming context."]
    pub fn set_jndi_initial_context_factory(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.jndi_initial_context_factory = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jndi_provider_url`.\nThe URL that Java Message Service will use to contact the JNDI provider.\ne.g.: 'tcp://myjms.host.domain:61616?jms.prefetchPolicy.all=1000'"]
    pub fn set_jndi_provider_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jndi_provider_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jndi_security_credentials_secret`.\nThe password associated to the principal."]
    pub fn set_jndi_security_credentials_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.jndi_security_credentials_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jndi_security_principal`.\nSpecifies the identity of the principal (user) to be authenticated."]
    pub fn set_jndi_security_principal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.jndi_security_principal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_store_file`.\nThe content of the KeyStore file."]
    pub fn set_key_store_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_store_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_store_password`.\nInput only. The KeyStore password in plain text."]
    pub fn set_key_store_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_store_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_store_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe KeyStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_key_store_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_store_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses to connect the Java Message Service\nin plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses to connect the associated Java\nMessage Service.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_protocol`.\nSecurity protocol for Java Message Service.\nPossible values:\nPLAIN\nTLS\nMTLS"]
    pub fn set_security_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_key_password`.\nInput only. The password for the cert inside of the KeyStore in plain text."]
    pub fn set_ssl_key_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_key_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_key_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password for the cert inside of the KeyStore.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_ssl_key_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_key_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of JavaMessageServiceConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_file`.\nThe content of the TrustStore file."]
    pub fn set_trust_store_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trust_store_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_password`.\nInput only. The TrustStore password in plain text."]
    pub fn set_trust_store_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.trust_store_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trust_store_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe TrustStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_trust_store_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.trust_store_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_jndi`.\nIf set to true, Java Naming and Directory Interface (JNDI) properties\nshould be provided."]
    pub fn set_use_jndi(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_jndi = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe username Oracle Goldengate uses to connect to the Java Message\nService."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties {
    type O =
        terrars
        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties {
            authentication_type: core::default::Default::default(),
            connection_factory: core::default::Default::default(),
            connection_url: core::default::Default::default(),
            jndi_connection_factory: core::default::Default::default(),
            jndi_initial_context_factory: core::default::Default::default(),
            jndi_provider_url: core::default::Default::default(),
            jndi_security_credentials_secret: core::default::Default::default(),
            jndi_security_principal: core::default::Default::default(),
            key_store_file: core::default::Default::default(),
            key_store_password: core::default::Default::default(),
            key_store_password_secret_version: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            ssl_key_password: core::default::Default::default(),
            ssl_key_password_secret_version: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            trust_store_file: core::default::Default::default(),
            trust_store_password: core::default::Default::default(),
            trust_store_password_secret_version: core::default::Default::default(),
            use_jndi: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication_type` after provisioning.\nAuthentication type for Java Message Service.\nPossible values:\nNONE\nBASIC"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_factory` after provisioning.\nThe Java class implementing javax.jms.ConnectionFactory interface supplied\nby the JMS provider."]
    pub fn connection_factory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_factory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_url` after provisioning.\nConnection URL of the Java Message Service, specifying the protocol, host,\nand port. e.g.: 'mq://myjms.host.domain:7676'"]
    pub fn connection_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jndi_connection_factory` after provisioning.\nThe Connection Factory can be looked up using this name.\ne.g.: 'ConnectionFactory'"]
    pub fn jndi_connection_factory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jndi_connection_factory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jndi_initial_context_factory` after provisioning.\nThe implementation of javax.naming.spi.InitialContextFactory interface\nused to obtain initial naming context."]
    pub fn jndi_initial_context_factory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jndi_initial_context_factory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jndi_provider_url` after provisioning.\nThe URL that Java Message Service will use to contact the JNDI provider.\ne.g.: 'tcp://myjms.host.domain:61616?jms.prefetchPolicy.all=1000'"]
    pub fn jndi_provider_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jndi_provider_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jndi_security_credentials_secret` after provisioning.\nThe password associated to the principal."]
    pub fn jndi_security_credentials_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jndi_security_credentials_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jndi_security_principal` after provisioning.\nSpecifies the identity of the principal (user) to be authenticated."]
    pub fn jndi_security_principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jndi_security_principal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_file` after provisioning.\nThe content of the KeyStore file."]
    pub fn key_store_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_password` after provisioning.\nInput only. The KeyStore password in plain text."]
    pub fn key_store_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe KeyStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn key_store_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses to connect the Java Message Service\nin plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses to connect the associated Java\nMessage Service.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_protocol` after provisioning.\nSecurity protocol for Java Message Service.\nPossible values:\nPLAIN\nTLS\nMTLS"]
    pub fn security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_key_password` after provisioning.\nInput only. The password for the cert inside of the KeyStore in plain text."]
    pub fn ssl_key_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_key_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_key_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password for the cert inside of the KeyStore.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn ssl_key_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_key_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of JavaMessageServiceConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_file` after provisioning.\nThe content of the TrustStore file."]
    pub fn trust_store_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_password` after provisioning.\nInput only. The TrustStore password in plain text."]
    pub fn trust_store_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe TrustStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn trust_store_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_jndi` after provisioning.\nIf set to true, Java Naming and Directory Interface (JNDI) properties\nshould be provided."]
    pub fn use_jndi(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_jndi", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect to the Java Message\nService."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers {
    host: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers {
    #[doc = "Set the field `port`.\nThe port of an endpoint usually specified for a connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_ip_address`.\nThe private IP address of the connection's endpoint in the customer's VCN,\ntypically a database endpoint or a big data endpoint (e.g. Kafka bootstrap\nserver). In case the privateIp is provided, the subnetId must also be\nprovided. In case the privateIp (and the subnetId) is not provided it is\nassumed the datasource is publicly accessible. In case the connection is\naccessible only privately, the lack of privateIp will result in not being\nable to access the connection."]
    pub fn set_private_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_ip_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers {
    type O =
        terrars
        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers {
    #[doc = "The name or address of a host."]
    pub host: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers {
        OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers {
            host: self.host,
            port: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServersRef {
        OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe name or address of a host."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port of an endpoint usually specified for a connection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_ip_address` after provisioning.\nThe private IP address of the connection's endpoint in the customer's VCN,\ntypically a database endpoint or a big data endpoint (e.g. Kafka bootstrap\nserver). In case the privateIp is provided, the subnetId must also be\nprovided. In case the privateIp (and the subnetId) is not provided it is\nassumed the datasource is publicly accessible. In case the connection is\naccessible only privately, the lack of privateIp will result in not being\nable to access the connection."]
    pub fn private_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateConnection_Properties_KafkaConnectionPropertiesDynamic {
    bootstrap_servers: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers>,
    >,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_properties_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    producer_properties_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_key_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_key_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_resource_principal: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bootstrap_servers: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers>>,
    dynamic: OracleDatabaseGoldengateConnection_Properties_KafkaConnectionPropertiesDynamic,
}

impl OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties {
    #[doc =
        "Set the field `cluster_id`.\nThe OCID of the Kafka cluster being referenced from OCI Streaming with\nApache Kafka."]
    pub fn set_cluster_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_properties_file`.\nThe content of the consumer.properties file."]
    pub fn set_consumer_properties_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consumer_properties_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_store_file`.\nThe content of the KeyStore file."]
    pub fn set_key_store_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_store_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_store_password`.\nInput only. The KeyStore password in plain text."]
    pub fn set_key_store_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_store_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_store_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe KeyStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_key_store_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_store_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\nInput only. The password for Kafka basic/SASL auth in plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password for Kafka basic/SASL auth.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `producer_properties_file`.\nThe content of the producer.properties file."]
    pub fn set_producer_properties_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.producer_properties_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_protocol`.\nSecurity Type for Kafka.\nPossible values:\nSSL\nSASL_SSL\nPLAINTEXT\nSASL_PLAINTEXT"]
    pub fn set_security_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_key_password`.\nInput only. The password for the cert inside of the KeyStore in plain text."]
    pub fn set_ssl_key_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_key_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_key_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password for the cert inside of the KeyStore.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_ssl_key_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_key_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_pool_id`.\nThe OCID of the stream pool being referenced."]
    pub fn set_stream_pool_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream_pool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of KafkaConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_file`.\nThe content of the TrustStore file."]
    pub fn set_trust_store_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trust_store_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_password`.\nInput only. The TrustStore password in plain text."]
    pub fn set_trust_store_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.trust_store_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trust_store_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe TrustStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_trust_store_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.trust_store_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_resource_principal`.\nSpecifies that the user intends to authenticate to the instance using a\nresource principal. Applicable only for OCI Streaming connections."]
    pub fn set_use_resource_principal(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_resource_principal = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bootstrap_servers`.\n"]
    pub fn set_bootstrap_servers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServers,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bootstrap_servers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bootstrap_servers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties {
            cluster_id: core::default::Default::default(),
            consumer_properties_file: core::default::Default::default(),
            key_store_file: core::default::Default::default(),
            key_store_password: core::default::Default::default(),
            key_store_password_secret_version: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            producer_properties_file: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            ssl_key_password: core::default::Default::default(),
            ssl_key_password_secret_version: core::default::Default::default(),
            stream_pool_id: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            trust_store_file: core::default::Default::default(),
            trust_store_password: core::default::Default::default(),
            trust_store_password_secret_version: core::default::Default::default(),
            use_resource_principal: core::default::Default::default(),
            username: core::default::Default::default(),
            bootstrap_servers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_KafkaConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_KafkaConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_KafkaConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_KafkaConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_KafkaConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nThe OCID of the Kafka cluster being referenced from OCI Streaming with\nApache Kafka."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consumer_properties_file` after provisioning.\nThe content of the consumer.properties file."]
    pub fn consumer_properties_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_properties_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_file` after provisioning.\nThe content of the KeyStore file."]
    pub fn key_store_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_password` after provisioning.\nInput only. The KeyStore password in plain text."]
    pub fn key_store_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe KeyStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn key_store_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password for Kafka basic/SASL auth in plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password for Kafka basic/SASL auth.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `producer_properties_file` after provisioning.\nThe content of the producer.properties file."]
    pub fn producer_properties_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.producer_properties_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_protocol` after provisioning.\nSecurity Type for Kafka.\nPossible values:\nSSL\nSASL_SSL\nPLAINTEXT\nSASL_PLAINTEXT"]
    pub fn security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_key_password` after provisioning.\nInput only. The password for the cert inside of the KeyStore in plain text."]
    pub fn ssl_key_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_key_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_key_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password for the cert inside of the KeyStore.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn ssl_key_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_key_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stream_pool_id` after provisioning.\nThe OCID of the stream pool being referenced."]
    pub fn stream_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_pool_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of KafkaConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_file` after provisioning.\nThe content of the TrustStore file."]
    pub fn trust_store_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_password` after provisioning.\nInput only. The TrustStore password in plain text."]
    pub fn trust_store_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe TrustStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn trust_store_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_resource_principal` after provisioning.\nSpecifies that the user intends to authenticate to the instance using a\nresource principal. Applicable only for OCI Streaming connections."]
    pub fn use_resource_principal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_resource_principal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `bootstrap_servers` after provisioning.\n"]
    pub fn bootstrap_servers(
        &self,
    ) -> terrars
    ::ListRef<OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties_BootstrapServersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bootstrap_servers", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_key_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_key_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties {
    #[doc =
        "Set the field `authentication_type`.\nUsed authentication mechanism to access Schema Registry.\nPossible values:\nNONE\nBASIC\nMUTUAL"]
    pub fn set_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_store_file`.\nThe content of the KeyStore file."]
    pub fn set_key_store_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_store_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_store_password`.\nInput only. The KeyStore password in plain text."]
    pub fn set_key_store_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_store_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_store_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe KeyStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_key_store_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_store_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\nInput only. The password to access Schema Registry in plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password to access Schema Registry using basic authentication.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_key_password`.\nInput only. The password for the cert inside the KeyStore in plain text."]
    pub fn set_ssl_key_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_key_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_key_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password for the cert inside the KeyStore.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_ssl_key_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_key_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of KafkaSchemaRegistryConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_file`.\nThe content of the TrustStore file."]
    pub fn set_trust_store_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trust_store_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_password`.\nInput only. The TrustStore password in plain text."]
    pub fn set_trust_store_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.trust_store_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trust_store_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe TrustStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_trust_store_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.trust_store_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\nKafka Schema Registry URL.\ne.g.: 'https://server1.us.oracle.com:8081'"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nThe username to access Schema Registry using basic authentication.\nThis value is injected into\n'schema.registry.basic.auth.user.info=user:password' configuration\nproperty."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties {
    type O =
        terrars
        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties {
            authentication_type: core::default::Default::default(),
            key_store_file: core::default::Default::default(),
            key_store_password: core::default::Default::default(),
            key_store_password_secret_version: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            ssl_key_password: core::default::Default::default(),
            ssl_key_password_secret_version: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            trust_store_file: core::default::Default::default(),
            trust_store_password: core::default::Default::default(),
            trust_store_password_secret_version: core::default::Default::default(),
            url: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication_type` after provisioning.\nUsed authentication mechanism to access Schema Registry.\nPossible values:\nNONE\nBASIC\nMUTUAL"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_file` after provisioning.\nThe content of the KeyStore file."]
    pub fn key_store_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_password` after provisioning.\nInput only. The KeyStore password in plain text."]
    pub fn key_store_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe KeyStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn key_store_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password to access Schema Registry in plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password to access Schema Registry using basic authentication.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_key_password` after provisioning.\nInput only. The password for the cert inside the KeyStore in plain text."]
    pub fn ssl_key_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_key_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_key_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password for the cert inside the KeyStore.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn ssl_key_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_key_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of KafkaSchemaRegistryConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_file` after provisioning.\nThe content of the TrustStore file."]
    pub fn trust_store_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_password` after provisioning.\nInput only. The TrustStore password in plain text."]
    pub fn trust_store_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe TrustStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn trust_store_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nKafka Schema Registry URL.\ne.g.: 'https://server1.us.oracle.com:8081'"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username to access Schema Registry using basic authentication.\nThis value is injected into\n'schema.registry.basic.auth.user.info=user:password' configuration\nproperty."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties {
    #[doc = "Set the field `client_id`.\nAzure client ID of the application."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nClient secret associated with the client id."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `endpoint`.\nOptional Microsoft Fabric service endpoint.\nDefault value: https://onelake.dfs.fabric.microsoft.com"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of MicrosoftFabricConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nAzure tenant ID of the application."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties {
    type O =
        terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nAzure client ID of the application."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nClient secret associated with the client id."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nOptional Microsoft Fabric service endpoint.\nDefault value: https://onelake.dfs.fabric.microsoft.com"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of MicrosoftFabricConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_id` after provisioning.\nAzure tenant ID of the application."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionPropertiesDynamic {
    additional_attributes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes>,
    >,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_certificate_validation_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_attributes: std
    ::option
    ::Option<
        std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes>,
    >,
    dynamic: OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionPropertiesDynamic,
}

impl OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties {
    #[doc = "Set the field `database`.\nThe name of the database."]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nThe name or address of a host."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses for Microsoft SQL Server connection\nin plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Microsoft SQL Server\nconnection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe port of an endpoint usually specified for a connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_protocol`.\nSecurity Type for Microsoft SQL Server.\nPossible values:\nPLAIN\nTLS"]
    pub fn set_security_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `server_certificate_validation_required`.\nIf set to true, the driver validates the certificate that is sent by the\ndatabase server."]
    pub fn set_server_certificate_validation_required(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.server_certificate_validation_required = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_ca_file`.\nDatabase Certificate - The content of a .pem or .crt file\ncontaining the server public key (for 1-way SSL)."]
    pub fn set_ssl_ca_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_ca_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of MicrosoftSqlserverConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe username Oracle Goldengate uses to connect to the Microsoft SQL\nServer."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_attributes`.\n"]
    pub fn set_additional_attributes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_attributes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties {
    type O =
        terrars
        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties {
            database: core::default::Default::default(),
            host: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            port: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            server_certificate_validation_required: core::default::Default::default(),
            ssl_ca_file: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            username: core::default::Default::default(),
            additional_attributes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe name of the database."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe name or address of a host."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses for Microsoft SQL Server connection\nin plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Microsoft SQL Server\nconnection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port of an endpoint usually specified for a connection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_protocol` after provisioning.\nSecurity Type for Microsoft SQL Server.\nPossible values:\nPLAIN\nTLS"]
    pub fn security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_certificate_validation_required` after provisioning.\nIf set to true, the driver validates the certificate that is sent by the\ndatabase server."]
    pub fn server_certificate_validation_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_certificate_validation_required", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca_file` after provisioning.\nDatabase Certificate - The content of a .pem or .crt file\ncontaining the server public key (for 1-way SSL)."]
    pub fn ssl_ca_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of MicrosoftSqlserverConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect to the Microsoft SQL\nServer."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_attributes` after provisioning.\n"]
    pub fn additional_attributes(
        &self,
    ) -> terrars
    ::ListRef<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_attributes", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_ca_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_certificate_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_certificate_key_file_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_certificate_key_file_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties {
    #[doc =
        "Set the field `connection_string`.\nMongoDB connection string.\ne.g.: 'mongodb://mongodb0.example.com:27017/recordsrecords'"]
    pub fn set_connection_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_id`.\nThe OCID of the Oracle Autonomous Json Database."]
    pub fn set_database_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses to connect the Mongodb connection in\nplain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses to connect the Mongodb connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_protocol`.\nSecurity Type for MongoDB.\nPossible values:\nPLAIN\nTLS\nMTLS"]
    pub fn set_security_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of MongodbConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls_ca_file`.\nDatabase Certificate - The content of a .pem file,\ncontaining the server public key (for 1 and 2-way SSL)."]
    pub fn set_tls_ca_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_ca_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls_certificate_key_file`.\nClient Certificate - The content of a .pem file, containing\nthe client public key (for 2-way SSL)."]
    pub fn set_tls_certificate_key_file(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tls_certificate_key_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls_certificate_key_file_password`.\nInput only. The Client Certificate key file password in plain text."]
    pub fn set_tls_certificate_key_file_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tls_certificate_key_file_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls_certificate_key_file_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe Client Certificate key file password in Secret Manager.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_tls_certificate_key_file_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tls_certificate_key_file_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe username Oracle Goldengate uses to connect to the database."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties {
            connection_string: core::default::Default::default(),
            database_id: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            tls_ca_file: core::default::Default::default(),
            tls_certificate_key_file: core::default::Default::default(),
            tls_certificate_key_file_password: core::default::Default::default(),
            tls_certificate_key_file_password_secret_version: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_MongodbConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_MongodbConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_MongodbConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_MongodbConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_MongodbConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_string` after provisioning.\nMongoDB connection string.\ne.g.: 'mongodb://mongodb0.example.com:27017/recordsrecords'"]
    pub fn connection_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_string", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_id` after provisioning.\nThe OCID of the Oracle Autonomous Json Database."]
    pub fn database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses to connect the Mongodb connection in\nplain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses to connect the Mongodb connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_protocol` after provisioning.\nSecurity Type for MongoDB.\nPossible values:\nPLAIN\nTLS\nMTLS"]
    pub fn security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of MongodbConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_ca_file` after provisioning.\nDatabase Certificate - The content of a .pem file,\ncontaining the server public key (for 1 and 2-way SSL)."]
    pub fn tls_ca_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_ca_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_certificate_key_file` after provisioning.\nClient Certificate - The content of a .pem file, containing\nthe client public key (for 2-way SSL)."]
    pub fn tls_certificate_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_certificate_key_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_certificate_key_file_password` after provisioning.\nInput only. The Client Certificate key file password in plain text."]
    pub fn tls_certificate_key_file_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_certificate_key_file_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_certificate_key_file_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe Client Certificate key file password in Secret Manager.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn tls_certificate_key_file_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_certificate_key_file_password_secret_version", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect to the database."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateConnection_Properties_MysqlConnectionPropertiesDynamic {
    additional_attributes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes>,
    >,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_system_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_cert_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_crl_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_attributes: std
    ::option
    ::Option<
        std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes>,
    >,
    dynamic: OracleDatabaseGoldengateConnection_Properties_MysqlConnectionPropertiesDynamic,
}

impl OracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties {
    #[doc = "Set the field `database`.\nThe name of the database."]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_system_id`.\nThe OCID of the database system being referenced."]
    pub fn set_db_system_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_system_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nThe name or address of a host."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses to connect to MySQL in plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses to connect to MySQL.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe port of an endpoint usually specified for a connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_protocol`.\nSecurity Type for MySQL.\nPossible values:\nPLAIN\nTLS\nMTLS"]
    pub fn set_security_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_ca_file`.\nDatabase Certificate - The content of a .pem or .crt file\ncontaining the server public key (for 1 and 2-way SSL)."]
    pub fn set_ssl_ca_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_ca_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_cert_file`.\nClient Certificate - The content of a .pem or .crt file\ncontaining the client public key (for 2-way SSL)."]
    pub fn set_ssl_cert_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_cert_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_crl_file`.\nThe list of certificates revoked by the trusted\ncertificate authorities (Trusted CA)."]
    pub fn set_ssl_crl_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_crl_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_key_file`.\nClient Key - The content of a .pem or .crt file containing\nthe client private key (for 2-way SSL)."]
    pub fn set_ssl_key_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_key_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_mode`.\nSSL modes for MySQL.\nPossible values:\nDISABLED\nPREFERRED\nREQUIRED\nVERIFY_CA\nVERIFY_IDENTITY"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of MysqlConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_attributes`.\n"]
    pub fn set_additional_attributes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_attributes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties {
            database: core::default::Default::default(),
            db_system_id: core::default::Default::default(),
            host: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            port: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            ssl_ca_file: core::default::Default::default(),
            ssl_cert_file: core::default::Default::default(),
            ssl_crl_file: core::default::Default::default(),
            ssl_key_file: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            username: core::default::Default::default(),
            additional_attributes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_MysqlConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_MysqlConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_MysqlConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_MysqlConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_MysqlConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe name of the database."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `db_system_id` after provisioning.\nThe OCID of the database system being referenced."]
    pub fn db_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe name or address of a host."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses to connect to MySQL in plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses to connect to MySQL.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port of an endpoint usually specified for a connection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_protocol` after provisioning.\nSecurity Type for MySQL.\nPossible values:\nPLAIN\nTLS\nMTLS"]
    pub fn security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca_file` after provisioning.\nDatabase Certificate - The content of a .pem or .crt file\ncontaining the server public key (for 1 and 2-way SSL)."]
    pub fn ssl_ca_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_cert_file` after provisioning.\nClient Certificate - The content of a .pem or .crt file\ncontaining the client public key (for 2-way SSL)."]
    pub fn ssl_cert_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_cert_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_crl_file` after provisioning.\nThe list of certificates revoked by the trusted\ncertificate authorities (Trusted CA)."]
    pub fn ssl_crl_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_crl_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_key_file` after provisioning.\nClient Key - The content of a .pem or .crt file containing\nthe client private key (for 2-way SSL)."]
    pub fn ssl_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_key_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_mode` after provisioning.\nSSL modes for MySQL.\nPossible values:\nDISABLED\nPREFERRED\nREQUIRED\nVERIFY_CA\nVERIFY_IDENTITY"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of MysqlConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_attributes` after provisioning.\n"]
    pub fn additional_attributes(
        &self,
    ) -> terrars
    ::ListRef<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_attributes", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_passphrase_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key_fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenancy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_resource_principal: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties {
    #[doc =
        "Set the field `private_key_file`.\nThe content of the private key file (PEM file)\ncorresponding to the API key of the fingerprint."]
    pub fn set_private_key_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key_passphrase_secret`.\nThe passphrase of the private key."]
    pub fn set_private_key_passphrase_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_key_passphrase_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `public_key_fingerprint`.\nThe fingerprint of the API Key of the user specified by the userId."]
    pub fn set_public_key_fingerprint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_key_fingerprint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe name of the region of OCI Object Storage. e.g.: us-ashburn-1\nIf the region is not provided, backend will default to the default region."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of OciObjectStorageConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenancy_id`.\nThe OCID of the related OCI tenancy."]
    pub fn set_tenancy_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenancy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_resource_principal`.\nSpecifies that the user intends to authenticate to the instance using a\nresource principal."]
    pub fn set_use_resource_principal(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_resource_principal = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_id`.\nThe OCID of the OCI user who will access the Object Storage.\nThe user must have write access to the bucket they want to connect to."]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties {
    type O =
        terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties {
            private_key_file: core::default::Default::default(),
            private_key_passphrase_secret: core::default::Default::default(),
            public_key_fingerprint: core::default::Default::default(),
            region: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            tenancy_id: core::default::Default::default(),
            use_resource_principal: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `private_key_file` after provisioning.\nThe content of the private key file (PEM file)\ncorresponding to the API key of the fingerprint."]
    pub fn private_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key_passphrase_secret` after provisioning.\nThe passphrase of the private key."]
    pub fn private_key_passphrase_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_passphrase_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_key_fingerprint` after provisioning.\nThe fingerprint of the API Key of the user specified by the userId."]
    pub fn public_key_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key_fingerprint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the region of OCI Object Storage. e.g.: us-ashburn-1\nIf the region is not provided, backend will default to the default region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of OciObjectStorageConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenancy_id` after provisioning.\nThe OCID of the related OCI tenancy."]
    pub fn tenancy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_resource_principal` after provisioning.\nSpecifies that the user intends to authenticate to the instance using a\nresource principal."]
    pub fn use_resource_principal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_resource_principal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_id` after provisioning.\nThe OCID of the OCI user who will access the Object Storage.\nThe user must have write access to the bucket they want to connect to."]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_passphrase_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key_fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenancy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_resource_principal: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties {
    #[doc = "Set the field `connection_url`.\nConnection URL. It must start with 'jdbc:spark://'"]
    pub fn set_connection_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_key_file`.\nThe content of the private key file (PEM file)\ncorresponding to the API key of the fingerprint."]
    pub fn set_private_key_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key_passphrase_secret`.\nThe passphrase of the private key."]
    pub fn set_private_key_passphrase_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_key_passphrase_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `public_key_fingerprint`.\nThe fingerprint of the API Key of the user specified by the user_id."]
    pub fn set_public_key_fingerprint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_key_fingerprint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe name of the region. e.g.: us-ashburn-1"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of OracleAiDataPlatformConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenancy_id`.\nThe OCID of the related OCI tenancy."]
    pub fn set_tenancy_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenancy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_resource_principal`.\nSpecifies that the user intends to authenticate to the instance using a\nresource principal."]
    pub fn set_use_resource_principal(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_resource_principal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\nThe OCID of the OCI user who will access."]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties {
    type O =
        terrars
        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties {
            connection_url: core::default::Default::default(),
            private_key_file: core::default::Default::default(),
            private_key_passphrase_secret: core::default::Default::default(),
            public_key_fingerprint: core::default::Default::default(),
            region: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            tenancy_id: core::default::Default::default(),
            use_resource_principal: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_url` after provisioning.\nConnection URL. It must start with 'jdbc:spark://'"]
    pub fn connection_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key_file` after provisioning.\nThe content of the private key file (PEM file)\ncorresponding to the API key of the fingerprint."]
    pub fn private_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key_passphrase_secret` after provisioning.\nThe passphrase of the private key."]
    pub fn private_key_passphrase_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_passphrase_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_key_fingerprint` after provisioning.\nThe fingerprint of the API Key of the user specified by the user_id."]
    pub fn public_key_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key_fingerprint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the region. e.g.: us-ashburn-1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of OracleAiDataPlatformConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenancy_id` after provisioning.\nThe OCID of the related OCI tenancy."]
    pub fn tenancy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_resource_principal` after provisioning.\nSpecifies that the user intends to authenticate to the instance using a\nresource principal."]
    pub fn use_resource_principal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_resource_principal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_id` after provisioning.\nThe OCID of the OCI user who will access."]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_oracle_database_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wallet_file: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties {
    #[doc = "Set the field `authentication_mode`.\nAuthentication mode.\nPossible values:\nTLS\nMTLS"]
    pub fn set_authentication_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection_string`.\nConnect descriptor or Easy Connect Naming method used to connect to a\ndatabase."]
    pub fn set_connection_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_string = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gcp_oracle_database_id`.\nDatabase instance id of database in Oracle Database @ Google Cloud.\nIf gcp_oracle_database_id is provided, connection_string must be empty."]
    pub fn set_gcp_oracle_database_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_oracle_database_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\nInput only. The password Oracle Goldengate uses in plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `session_mode`.\nThe mode of the database connection session to be established by the data\nclient.\nPossible values:\nDIRECT\nREDIRECT"]
    pub fn set_session_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.session_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe username Oracle Goldengate uses to connect."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wallet_file`.\nThe wallet contents Oracle Goldengate uses to make connections to a\ndatabase."]
    pub fn set_wallet_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.wallet_file = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties {
            authentication_mode: core::default::Default::default(),
            connection_string: core::default::Default::default(),
            gcp_oracle_database_id: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            session_mode: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            username: core::default::Default::default(),
            wallet_file: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_OracleConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_OracleConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_OracleConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_OracleConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_OracleConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication_mode` after provisioning.\nAuthentication mode.\nPossible values:\nTLS\nMTLS"]
    pub fn authentication_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_string` after provisioning.\nConnect descriptor or Easy Connect Naming method used to connect to a\ndatabase."]
    pub fn connection_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_string", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gcp_oracle_database_id` after provisioning.\nDatabase instance id of database in Oracle Database @ Google Cloud.\nIf gcp_oracle_database_id is provided, connection_string must be empty."]
    pub fn gcp_oracle_database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_oracle_database_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses in plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `session_mode` after provisioning.\nThe mode of the database connection session to be established by the data\nclient.\nPossible values:\nDIRECT\nREDIRECT"]
    pub fn session_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc =
        "Get a reference to the value of field `wallet_file` after provisioning.\nThe wallet contents Oracle Goldengate uses to make connections to a\ndatabase."]
    pub fn wallet_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wallet_file", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_passphrase_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_key_fingerprint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenancy_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_resource_principal: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties {
    #[doc =
        "Set the field `private_key_file`.\nThe content of the private key file (PEM file)\ncorresponding to the API key of the fingerprint."]
    pub fn set_private_key_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key_passphrase_secret`.\nThe passphrase of the private key."]
    pub fn set_private_key_passphrase_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_key_passphrase_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `public_key_fingerprint`.\nThe fingerprint of the API Key of the user specified by the userId."]
    pub fn set_public_key_fingerprint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_key_fingerprint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe name of the region. e.g.: us-ashburn-1"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of OracleNosqlConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenancy_id`.\nThe OCID of the OCI tenancy."]
    pub fn set_tenancy_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenancy_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_resource_principal`.\nSpecifies that the user intends to authenticate to the instance using a\nresource principal."]
    pub fn set_use_resource_principal(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_resource_principal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\nThe OCID of the OCI user who will access the Oracle NoSQL database."]
    pub fn set_user_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties {
            private_key_file: core::default::Default::default(),
            private_key_passphrase_secret: core::default::Default::default(),
            public_key_fingerprint: core::default::Default::default(),
            region: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            tenancy_id: core::default::Default::default(),
            use_resource_principal: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `private_key_file` after provisioning.\nThe content of the private key file (PEM file)\ncorresponding to the API key of the fingerprint."]
    pub fn private_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key_passphrase_secret` after provisioning.\nThe passphrase of the private key."]
    pub fn private_key_passphrase_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_passphrase_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_key_fingerprint` after provisioning.\nThe fingerprint of the API Key of the user specified by the userId."]
    pub fn public_key_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_key_fingerprint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the region. e.g.: us-ashburn-1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of OracleNosqlConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc = "Get a reference to the value of field `tenancy_id` after provisioning.\nThe OCID of the OCI tenancy."]
    pub fn tenancy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_resource_principal` after provisioning.\nSpecifies that the user intends to authenticate to the instance using a\nresource principal."]
    pub fn use_resource_principal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_resource_principal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_id` after provisioning.\nThe OCID of the OCI user who will access the Oracle NoSQL database."]
    pub fn user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionPropertiesDynamic {
    additional_attributes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes>,
    >,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_system_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_ca_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_cert_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_crl_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_attributes: std
    ::option
    ::Option<
        std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes>,
    >,
    dynamic: OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionPropertiesDynamic,
}

impl OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties {
    #[doc = "Set the field `database`.\nThe name of the database."]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_system_id`.\nThe OCID of the database system being referenced."]
    pub fn set_db_system_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_system_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\nThe name or address of a host."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses for PostgreSQL connection in plain\ntext."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for PostgreSQL connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe port of an endpoint usually specified for a connection."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_protocol`.\nSecurity protocol for PostgreSQL.\nPossible values:\nPLAIN\nTLS\nMTLS"]
    pub fn set_security_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_ca_file`.\nThe certificate of the trusted certificate authorities\n(Trusted CA) for PostgreSQL."]
    pub fn set_ssl_ca_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_ca_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_cert_file`.\nThe certificate of the PostgreSQL server."]
    pub fn set_ssl_cert_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_cert_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_crl_file`.\nThe list of certificates revoked by the trusted\ncertificate authorities (Trusted CA)."]
    pub fn set_ssl_crl_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_crl_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_key_file`.\nThe private key of the PostgreSQL server."]
    pub fn set_ssl_key_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_key_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_mode`.\nSSL modes for PostgreSQL.\nPossible values:\nPREFER\nREQUIRE\nVERIFY_CA\nVERIFY_FULL"]
    pub fn set_ssl_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of PostgresqlConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_attributes`.\n"]
    pub fn set_additional_attributes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_attributes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties {
            database: core::default::Default::default(),
            db_system_id: core::default::Default::default(),
            host: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            port: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            ssl_ca_file: core::default::Default::default(),
            ssl_cert_file: core::default::Default::default(),
            ssl_crl_file: core::default::Default::default(),
            ssl_key_file: core::default::Default::default(),
            ssl_mode: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            username: core::default::Default::default(),
            additional_attributes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe name of the database."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `db_system_id` after provisioning.\nThe OCID of the database system being referenced."]
    pub fn db_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nThe name or address of a host."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses for PostgreSQL connection in plain\ntext."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for PostgreSQL connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port of an endpoint usually specified for a connection."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_protocol` after provisioning.\nSecurity protocol for PostgreSQL.\nPossible values:\nPLAIN\nTLS\nMTLS"]
    pub fn security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_ca_file` after provisioning.\nThe certificate of the trusted certificate authorities\n(Trusted CA) for PostgreSQL."]
    pub fn ssl_ca_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_ca_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_cert_file` after provisioning.\nThe certificate of the PostgreSQL server."]
    pub fn ssl_cert_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_cert_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_crl_file` after provisioning.\nThe list of certificates revoked by the trusted\ncertificate authorities (Trusted CA)."]
    pub fn ssl_crl_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_crl_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_key_file` after provisioning.\nThe private key of the PostgreSQL server."]
    pub fn ssl_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_key_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_mode` after provisioning.\nSSL modes for PostgreSQL.\nPossible values:\nPREFER\nREQUIRE\nVERIFY_CA\nVERIFY_FULL"]
    pub fn ssl_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of PostgresqlConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_attributes` after provisioning.\n"]
    pub fn additional_attributes(
        &self,
    ) -> terrars
    ::ListRef<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties_AdditionalAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_attributes", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redis_cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    servers: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store_password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties {
    #[doc = "Set the field `authentication_type`.\nAuthentication type for Redis.\nPossible values:\nNONE\nBASIC"]
    pub fn set_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_store_file`.\nThe content of the KeyStore file."]
    pub fn set_key_store_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_store_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_store_password`.\nInput only. The KeyStore password in plain text."]
    pub fn set_key_store_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_store_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_store_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe KeyStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_key_store_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.key_store_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses for Redis connection in plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Redis connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redis_cluster_id`.\nThe OCID of the Redis cluster."]
    pub fn set_redis_cluster_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.redis_cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_protocol`.\nSecurity protocol for Redis.\nPossible values:\nPLAIN\nTLS\nMTLS"]
    pub fn set_security_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `servers`.\nComma separated list of Redis server addresses, specified as host:port\nentries, where :port is optional. If port is not specified, it defaults\nto 6379. Example: \"server1.example.com:6379,server2.example.com:6379\""]
    pub fn set_servers(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.servers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of RedisConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_file`.\nThe content of the TrustStore file."]
    pub fn set_trust_store_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trust_store_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_store_password`.\nInput only. The TrustStore password in plain text."]
    pub fn set_trust_store_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.trust_store_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trust_store_password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe TrustStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_trust_store_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.trust_store_password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties {
            authentication_type: core::default::Default::default(),
            key_store_file: core::default::Default::default(),
            key_store_password: core::default::Default::default(),
            key_store_password_secret_version: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            redis_cluster_id: core::default::Default::default(),
            security_protocol: core::default::Default::default(),
            servers: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            trust_store_file: core::default::Default::default(),
            trust_store_password: core::default::Default::default(),
            trust_store_password_secret_version: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_RedisConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_RedisConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_RedisConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_RedisConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_RedisConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication_type` after provisioning.\nAuthentication type for Redis.\nPossible values:\nNONE\nBASIC"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_file` after provisioning.\nThe content of the KeyStore file."]
    pub fn key_store_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_password` after provisioning.\nInput only. The KeyStore password in plain text."]
    pub fn key_store_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_store_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe KeyStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn key_store_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses for Redis connection in plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses for Redis connection.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redis_cluster_id` after provisioning.\nThe OCID of the Redis cluster."]
    pub fn redis_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redis_cluster_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_protocol` after provisioning.\nSecurity protocol for Redis.\nPossible values:\nPLAIN\nTLS\nMTLS"]
    pub fn security_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `servers` after provisioning.\nComma separated list of Redis server addresses, specified as host:port\nentries, where :port is optional. If port is not specified, it defaults\nto 6379. Example: \"server1.example.com:6379,server2.example.com:6379\""]
    pub fn servers(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.servers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of RedisConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_file` after provisioning.\nThe content of the TrustStore file."]
    pub fn trust_store_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_password` after provisioning.\nInput only. The TrustStore password in plain text."]
    pub fn trust_store_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_store_password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe TrustStore password.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn trust_store_password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_store_password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect the associated system of\nthe given technology."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_file: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_passphrase_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technology_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties {
    #[doc =
        "Set the field `authentication_type`.\nUsed authentication mechanism to access Snowflake.\nPossible values:\nBASIC\nKEY_PAIR"]
    pub fn set_authentication_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authentication_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection_url`.\nJDBC connection URL.\ne.g.:\n'jdbc:snowflake://.snowflakecomputing.com/?warehouse=&db='"]
    pub fn set_connection_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nInput only. The password Oracle Goldengate uses to connect to Snowflake platform\nin plain text."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_secret_version`.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses to connect to Snowflake platform.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn set_password_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key_file`.\nThe content of private key file in PEM format."]
    pub fn set_private_key_file(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key_file = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key_passphrase_secret`.\nPassword if the private key file is encrypted."]
    pub fn set_private_key_passphrase_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_key_passphrase_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `technology_type`.\nThe technology type of SnowflakeConnection."]
    pub fn set_technology_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.technology_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nThe username Oracle Goldengate uses to connect to Snowflake."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties {}

impl BuildOracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties {
        OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties {
            authentication_type: core::default::Default::default(),
            connection_url: core::default::Default::default(),
            password: core::default::Default::default(),
            password_secret_version: core::default::Default::default(),
            private_key_file: core::default::Default::default(),
            private_key_passphrase_secret: core::default::Default::default(),
            technology_type: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionPropertiesRef {
        OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authentication_type` after provisioning.\nUsed authentication mechanism to access Snowflake.\nPossible values:\nBASIC\nKEY_PAIR"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_url` after provisioning.\nJDBC connection URL.\ne.g.:\n'jdbc:snowflake://.snowflakecomputing.com/?warehouse=&db='"]
    pub fn connection_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nInput only. The password Oracle Goldengate uses to connect to Snowflake platform\nin plain text."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nInput only. The resource name of a secret version in Secret Manager which contains\nthe password Oracle Goldengate uses to connect to Snowflake platform.\nFormat: projects/{project}/secrets/{secret}/versions/{version}."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key_file` after provisioning.\nThe content of private key file in PEM format."]
    pub fn private_key_file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key_passphrase_secret` after provisioning.\nPassword if the private key file is encrypted."]
    pub fn private_key_passphrase_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_passphrase_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `technology_type` after provisioning.\nThe technology type of SnowflakeConnection."]
    pub fn technology_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.technology_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe username Oracle Goldengate uses to connect to Snowflake."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseGoldengateConnection_PropertiesDynamic {
    amazon_kinesis_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties>>,
    amazon_redshift_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties>>,
    amazon_s3_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties>>,
    azure_data_lake_storage_connection_properties: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties>,
    >,
    azure_synapse_analytics_connection_properties: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties>,
    >,
    databricks_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties>>,
    db2_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties>>,
    elasticsearch_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties>>,
    generic_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties>>,
    goldengate_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties>>,
    google_big_query_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties>>,
    google_cloud_storage_connection_properties: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties>,
    >,
    google_pubsub_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties>>,
    hdfs_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties>>,
    iceberg_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties>>,
    java_message_service_connection_properties: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties>,
    >,
    kafka_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties>>,
    kafka_schema_registry_connection_properties: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties>,
    >,
    microsoft_fabric_connection_properties: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties>,
    >,
    microsoft_sqlserver_connection_properties: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties>,
    >,
    mongodb_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties>>,
    mysql_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties>>,
    oci_object_storage_connection_properties: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties>,
    >,
    oracle_ai_data_platform_connection_properties: std
    ::option
    ::Option<
        terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties>,
    >,
    oracle_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties>>,
    oracle_nosql_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties>>,
    postgresql_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties>>,
    redis_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties>>,
    snowflake_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Properties {
    connection_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_kinesis_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_redshift_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_s3_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_data_lake_storage_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_synapse_analytics_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    databricks_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db2_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elasticsearch_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generic_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    goldengate_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_big_query_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_storage_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_pubsub_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hdfs_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iceberg_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    java_message_service_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kafka_schema_registry_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    microsoft_fabric_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    microsoft_sqlserver_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mongodb_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci_object_storage_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_ai_data_platform_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_nosql_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redis_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake_connection_properties: std
    ::option
    ::Option<std::vec::Vec<OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties>>,
    dynamic: OracleDatabaseGoldengateConnection_PropertiesDynamic,
}

impl OracleDatabaseGoldengateConnection_Properties {
    #[doc = "Set the field `description`.\nMetadata about this specific object."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `routing_method`.\nThe routing method for the GoldengateConnection.\nPossible values:\nSHARED_DEPLOYMENT_ENDPOINT\nDEDICATED_ENDPOINT"]
    pub fn set_routing_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routing_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `amazon_kinesis_connection_properties`.\n"]
    pub fn set_amazon_kinesis_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_kinesis_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_kinesis_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `amazon_redshift_connection_properties`.\n"]
    pub fn set_amazon_redshift_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_redshift_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_redshift_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `amazon_s3_connection_properties`.\n"]
    pub fn set_amazon_s3_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_s3_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_s3_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_data_lake_storage_connection_properties`.\n"]
    pub fn set_azure_data_lake_storage_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_data_lake_storage_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_data_lake_storage_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_synapse_analytics_connection_properties`.\n"]
    pub fn set_azure_synapse_analytics_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_synapse_analytics_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_synapse_analytics_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `databricks_connection_properties`.\n"]
    pub fn set_databricks_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.databricks_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.databricks_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `db2_connection_properties`.\n"]
    pub fn set_db2_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.db2_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.db2_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `elasticsearch_connection_properties`.\n"]
    pub fn set_elasticsearch_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.elasticsearch_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.elasticsearch_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generic_connection_properties`.\n"]
    pub fn set_generic_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_GenericConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generic_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generic_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `goldengate_connection_properties`.\n"]
    pub fn set_goldengate_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.goldengate_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.goldengate_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_big_query_connection_properties`.\n"]
    pub fn set_google_big_query_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_big_query_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_big_query_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_cloud_storage_connection_properties`.\n"]
    pub fn set_google_cloud_storage_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_cloud_storage_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_storage_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_pubsub_connection_properties`.\n"]
    pub fn set_google_pubsub_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_pubsub_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_pubsub_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hdfs_connection_properties`.\n"]
    pub fn set_hdfs_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_HdfsConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hdfs_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hdfs_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iceberg_connection_properties`.\n"]
    pub fn set_iceberg_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iceberg_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `java_message_service_connection_properties`.\n"]
    pub fn set_java_message_service_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.java_message_service_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.java_message_service_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kafka_connection_properties`.\n"]
    pub fn set_kafka_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_KafkaConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kafka_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kafka_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kafka_schema_registry_connection_properties`.\n"]
    pub fn set_kafka_schema_registry_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kafka_schema_registry_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kafka_schema_registry_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `microsoft_fabric_connection_properties`.\n"]
    pub fn set_microsoft_fabric_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.microsoft_fabric_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.microsoft_fabric_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `microsoft_sqlserver_connection_properties`.\n"]
    pub fn set_microsoft_sqlserver_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.microsoft_sqlserver_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.microsoft_sqlserver_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mongodb_connection_properties`.\n"]
    pub fn set_mongodb_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_MongodbConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mongodb_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mongodb_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mysql_connection_properties`.\n"]
    pub fn set_mysql_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_MysqlConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oci_object_storage_connection_properties`.\n"]
    pub fn set_oci_object_storage_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oci_object_storage_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oci_object_storage_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle_ai_data_platform_connection_properties`.\n"]
    pub fn set_oracle_ai_data_platform_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_ai_data_platform_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_ai_data_platform_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle_connection_properties`.\n"]
    pub fn set_oracle_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_OracleConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle_nosql_connection_properties`.\n"]
    pub fn set_oracle_nosql_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_nosql_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_nosql_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postgresql_connection_properties`.\n"]
    pub fn set_postgresql_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redis_connection_properties`.\n"]
    pub fn set_redis_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_RedisConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redis_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redis_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snowflake_connection_properties`.\n"]
    pub fn set_snowflake_connection_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionProperties>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snowflake_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Properties {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Properties {
    #[doc =
        "The connection type.\nPossible values:\nGOLDENGATE\nKAFKA\nKAFKA_SCHEMA_REGISTRY\nMYSQL\nJAVA_MESSAGE_SERVICE\nMICROSOFT_SQLSERVER\nOCI_OBJECT_STORAGE\nORACLE\nAZURE_DATA_LAKE_STORAGE\nPOSTGRESQL\nAZURE_SYNAPSE_ANALYTICS\nSNOWFLAKE\nAMAZON_S3\nHDFS\nORACLE_AI_DATA_PLATFORM\nORACLE_NOSQL\nMONGODB\nAMAZON_KINESIS\nAMAZON_REDSHIFT\nDB2\nREDIS\nELASTICSEARCH\nGENERIC\nGOOGLE_CLOUD_STORAGE\nGOOGLE_BIGQUERY\nDATABRICKS\nGOOGLE_PUBSUB\nMICROSOFT_FABRIC\nICEBERG"]
    pub connection_type: terrars::PrimField<std::string::String>,
    #[doc = "An object's Display Name."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseGoldengateConnection_Properties {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Properties {
        OracleDatabaseGoldengateConnection_Properties {
            connection_type: self.connection_type,
            description: core::default::Default::default(),
            display_name: self.display_name,
            routing_method: core::default::Default::default(),
            amazon_kinesis_connection_properties: core::default::Default::default(),
            amazon_redshift_connection_properties: core::default::Default::default(),
            amazon_s3_connection_properties: core::default::Default::default(),
            azure_data_lake_storage_connection_properties: core::default::Default::default(),
            azure_synapse_analytics_connection_properties: core::default::Default::default(),
            databricks_connection_properties: core::default::Default::default(),
            db2_connection_properties: core::default::Default::default(),
            elasticsearch_connection_properties: core::default::Default::default(),
            generic_connection_properties: core::default::Default::default(),
            goldengate_connection_properties: core::default::Default::default(),
            google_big_query_connection_properties: core::default::Default::default(),
            google_cloud_storage_connection_properties: core::default::Default::default(),
            google_pubsub_connection_properties: core::default::Default::default(),
            hdfs_connection_properties: core::default::Default::default(),
            iceberg_connection_properties: core::default::Default::default(),
            java_message_service_connection_properties: core::default::Default::default(),
            kafka_connection_properties: core::default::Default::default(),
            kafka_schema_registry_connection_properties: core::default::Default::default(),
            microsoft_fabric_connection_properties: core::default::Default::default(),
            microsoft_sqlserver_connection_properties: core::default::Default::default(),
            mongodb_connection_properties: core::default::Default::default(),
            mysql_connection_properties: core::default::Default::default(),
            oci_object_storage_connection_properties: core::default::Default::default(),
            oracle_ai_data_platform_connection_properties: core::default::Default::default(),
            oracle_connection_properties: core::default::Default::default(),
            oracle_nosql_connection_properties: core::default::Default::default(),
            postgresql_connection_properties: core::default::Default::default(),
            redis_connection_properties: core::default::Default::default(),
            snowflake_connection_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_PropertiesRef {
        OracleDatabaseGoldengateConnection_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_type` after provisioning.\nThe connection type.\nPossible values:\nGOLDENGATE\nKAFKA\nKAFKA_SCHEMA_REGISTRY\nMYSQL\nJAVA_MESSAGE_SERVICE\nMICROSOFT_SQLSERVER\nOCI_OBJECT_STORAGE\nORACLE\nAZURE_DATA_LAKE_STORAGE\nPOSTGRESQL\nAZURE_SYNAPSE_ANALYTICS\nSNOWFLAKE\nAMAZON_S3\nHDFS\nORACLE_AI_DATA_PLATFORM\nORACLE_NOSQL\nMONGODB\nAMAZON_KINESIS\nAMAZON_REDSHIFT\nDB2\nREDIS\nELASTICSEARCH\nGENERIC\nGOOGLE_CLOUD_STORAGE\nGOOGLE_BIGQUERY\nDATABRICKS\nGOOGLE_PUBSUB\nMICROSOFT_FABRIC\nICEBERG"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nMetadata about this specific object."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nAn object's Display Name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ingress_ip_addresses` after provisioning.\nThe Ingress IPs of the GoldengateConnection."]
    pub fn ingress_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_ip_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_details` after provisioning.\nDescribes the object's current state in detail. For example, it can be\nused to provide actionable information for a resource in a Failed state."]
    pub fn lifecycle_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_details", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_state` after provisioning.\nThe lifecycle state of the connection.\nPossible values:\nCREATING\nACTIVE\nUPDATING\nDELETING\nDELETED\nFAILED"]
    pub fn lifecycle_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nThe [OCID] of the connection being\nreferenced."]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `routing_method` after provisioning.\nThe routing method for the GoldengateConnection.\nPossible values:\nSHARED_DEPLOYMENT_ENDPOINT\nDEDICATED_ENDPOINT"]
    pub fn routing_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the resource was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_kinesis_connection_properties` after provisioning.\n"]
    pub fn amazon_kinesis_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_AmazonKinesisConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_kinesis_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_redshift_connection_properties` after provisioning.\n"]
    pub fn amazon_redshift_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_AmazonRedshiftConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_redshift_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_s3_connection_properties` after provisioning.\n"]
    pub fn amazon_s3_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_AmazonS3ConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_s3_connection_properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `azure_data_lake_storage_connection_properties` after provisioning.\n"]
    pub fn azure_data_lake_storage_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_AzureDataLakeStorageConnectionPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.azure_data_lake_storage_connection_properties", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `azure_synapse_analytics_connection_properties` after provisioning.\n"]
    pub fn azure_synapse_analytics_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_AzureSynapseAnalyticsConnectionPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.azure_synapse_analytics_connection_properties", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `databricks_connection_properties` after provisioning.\n"]
    pub fn databricks_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_DatabricksConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.databricks_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `db2_connection_properties` after provisioning.\n"]
    pub fn db2_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_Db2ConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.db2_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `elasticsearch_connection_properties` after provisioning.\n"]
    pub fn elasticsearch_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_ElasticsearchConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elasticsearch_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `generic_connection_properties` after provisioning.\n"]
    pub fn generic_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_GenericConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generic_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `goldengate_connection_properties` after provisioning.\n"]
    pub fn goldengate_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_GoldengateConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.goldengate_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `google_big_query_connection_properties` after provisioning.\n"]
    pub fn google_big_query_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_GoogleBigQueryConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_big_query_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `google_cloud_storage_connection_properties` after provisioning.\n"]
    pub fn google_cloud_storage_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_GoogleCloudStorageConnectionPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.google_cloud_storage_connection_properties", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `google_pubsub_connection_properties` after provisioning.\n"]
    pub fn google_pubsub_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_GooglePubsubConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_pubsub_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `hdfs_connection_properties` after provisioning.\n"]
    pub fn hdfs_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_HdfsConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hdfs_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `iceberg_connection_properties` after provisioning.\n"]
    pub fn iceberg_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_IcebergConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iceberg_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `java_message_service_connection_properties` after provisioning.\n"]
    pub fn java_message_service_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_JavaMessageServiceConnectionPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.java_message_service_connection_properties", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `kafka_connection_properties` after provisioning.\n"]
    pub fn kafka_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_KafkaConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kafka_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `kafka_schema_registry_connection_properties` after provisioning.\n"]
    pub fn kafka_schema_registry_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_KafkaSchemaRegistryConnectionPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_schema_registry_connection_properties", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `microsoft_fabric_connection_properties` after provisioning.\n"]
    pub fn microsoft_fabric_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_MicrosoftFabricConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.microsoft_fabric_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `microsoft_sqlserver_connection_properties` after provisioning.\n"]
    pub fn microsoft_sqlserver_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_MicrosoftSqlserverConnectionPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.microsoft_sqlserver_connection_properties", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `mongodb_connection_properties` after provisioning.\n"]
    pub fn mongodb_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_MongodbConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_connection_properties` after provisioning.\n"]
    pub fn mysql_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_MysqlConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `oci_object_storage_connection_properties` after provisioning.\n"]
    pub fn oci_object_storage_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_OciObjectStorageConnectionPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.oci_object_storage_connection_properties", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `oracle_ai_data_platform_connection_properties` after provisioning.\n"]
    pub fn oracle_ai_data_platform_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_OracleAiDataPlatformConnectionPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.oracle_ai_data_platform_connection_properties", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `oracle_connection_properties` after provisioning.\n"]
    pub fn oracle_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_OracleConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_nosql_connection_properties` after provisioning.\n"]
    pub fn oracle_nosql_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_OracleNosqlConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_nosql_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_connection_properties` after provisioning.\n"]
    pub fn postgresql_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_PostgresqlConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `redis_connection_properties` after provisioning.\n"]
    pub fn redis_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_RedisConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redis_connection_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake_connection_properties` after provisioning.\n"]
    pub fn snowflake_connection_properties(
        &self,
    ) -> terrars::ListRef<OracleDatabaseGoldengateConnection_Properties_SnowflakeConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake_connection_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseGoldengateConnection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseGoldengateConnection_Timeouts {
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

impl terrars::ToListMappable for OracleDatabaseGoldengateConnection_Timeouts {
    type O = terrars::BlockAssignable<OracleDatabaseGoldengateConnection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseGoldengateConnection_Timeouts {}

impl BuildOracleDatabaseGoldengateConnection_Timeouts {
    pub fn build(self) -> OracleDatabaseGoldengateConnection_Timeouts {
        OracleDatabaseGoldengateConnection_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseGoldengateConnection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseGoldengateConnection_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseGoldengateConnection_TimeoutsRef {
        OracleDatabaseGoldengateConnection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseGoldengateConnection_TimeoutsRef {
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
