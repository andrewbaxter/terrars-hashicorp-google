#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OracleDatabaseAutonomousDatabaseData_ {
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
    admin_password: std::option::Option<terrars::PrimField<std::string::String>>,
    autonomous_database_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
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
    properties: std::option::Option<std::vec::Vec<OracleDatabaseAutonomousDatabase_Properties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_config: std::option::Option<std::vec::Vec<OracleDatabaseAutonomousDatabase_SourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OracleDatabaseAutonomousDatabase_Timeouts>,
    dynamic: OracleDatabaseAutonomousDatabaseDynamic,
}

struct OracleDatabaseAutonomousDatabase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OracleDatabaseAutonomousDatabaseData_>,
}

#[derive(Clone)]
pub struct OracleDatabaseAutonomousDatabase(std::rc::Rc<OracleDatabaseAutonomousDatabase_>);

impl OracleDatabaseAutonomousDatabase {
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

    #[doc = "Set the field `admin_password`.\nThe password for the default ADMIN user."]
    pub fn set_admin_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().admin_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cidr`.\nThe subnet CIDR range for the Autonmous Database."]
    pub fn set_cidr(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `database`.\nThe name of the Autonomous Database. The database name must be unique in\nthe project. The name must begin with a letter and can\ncontain a maximum of 30 alphanumeric characters."]
    pub fn set_database(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database = std::option::Option::Some(v.into());
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
        "Set the field `display_name`.\nThe display name for the Autonomous Database. The name does not have to\nbe unique within your project."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels or tags associated with the Autonomous Database. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe name of the VPC network used by the Autonomous Database.\nFormat: projects/{project}/global/networks/{network}"]
    pub fn set_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_network`.\nThe name of the OdbNetwork associated with the Autonomous Database.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe odb_subnet and backup_odb_subnet."]
    pub fn set_odb_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().odb_network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `odb_subnet`.\nThe name of the OdbSubnet associated with the Autonomous Database for\nIP allocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
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
        v: impl std::convert::Into<terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties>>,
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

    #[doc = "Set the field `source_config`.\n"]
    pub fn set_source_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_SourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OracleDatabaseAutonomousDatabase_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `admin_password` after provisioning.\nThe password for the default ADMIN user."]
    pub fn admin_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autonomous_database_id` after provisioning.\nThe ID of the Autonomous Database to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub fn autonomous_database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autonomous_database_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cidr` after provisioning.\nThe subnet CIDR range for the Autonmous Database."]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the Autonomous Database was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe name of the Autonomous Database. The database name must be unique in\nthe project. The name must begin with a letter and can\ncontain a maximum of 30 alphanumeric characters."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
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
        "Get a reference to the value of field `disaster_recovery_supported_locations` after provisioning.\nList of supported GCP region to clone the Autonomous Database for disaster recovery."]
    pub fn disaster_recovery_supported_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.disaster_recovery_supported_locations", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the Autonomous Database. The name does not have to\nbe unique within your project."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the Autonomous\nDatabase."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the Autonomous Database. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/AutonomousDatabaseBackup'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the Autonomous Database resource in the following format:\nprojects/{project}/locations/{region}/autonomousDatabases/{autonomous_database}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC network used by the Autonomous Database.\nFormat: projects/{project}/global/networks/{network}"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the Autonomous Database.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe odb_subnet and backup_odb_subnet."]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the Autonomous Database for\nIP allocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn odb_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_subnet", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_autonomous_databases` after provisioning.\nThe peer Autonomous Database names of the given Autonomous Database."]
    pub fn peer_autonomous_databases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_autonomous_databases", self.extract_ref()))
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
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_config` after provisioning.\n"]
    pub fn source_config(&self) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_SourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseAutonomousDatabase_TimeoutsRef {
        OracleDatabaseAutonomousDatabase_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OracleDatabaseAutonomousDatabase {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OracleDatabaseAutonomousDatabase { }

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase {
    type O = terrars::ListRef<OracleDatabaseAutonomousDatabaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OracleDatabaseAutonomousDatabase_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_oracle_database_autonomous_database".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the Autonomous Database to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub autonomous_database_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/AutonomousDatabaseBackup'."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseAutonomousDatabase {
    pub fn build(self, stack: &mut terrars::Stack) -> OracleDatabaseAutonomousDatabase {
        let out = OracleDatabaseAutonomousDatabase(std::rc::Rc::new(OracleDatabaseAutonomousDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OracleDatabaseAutonomousDatabaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                admin_password: core::default::Default::default(),
                autonomous_database_id: self.autonomous_database_id,
                cidr: core::default::Default::default(),
                database: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                network: core::default::Default::default(),
                odb_network: core::default::Default::default(),
                odb_subnet: core::default::Default::default(),
                project: core::default::Default::default(),
                properties: core::default::Default::default(),
                source_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OracleDatabaseAutonomousDatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OracleDatabaseAutonomousDatabaseRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `admin_password` after provisioning.\nThe password for the default ADMIN user."]
    pub fn admin_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.admin_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autonomous_database_id` after provisioning.\nThe ID of the Autonomous Database to create. This value is restricted\nto (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$) and must be a maximum of 63\ncharacters in length. The value must start with a letter and end with\na letter or a number."]
    pub fn autonomous_database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autonomous_database_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cidr` after provisioning.\nThe subnet CIDR range for the Autonmous Database."]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe date and time that the Autonomous Database was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe name of the Autonomous Database. The database name must be unique in\nthe project. The name must begin with a letter and can\ncontain a maximum of 30 alphanumeric characters."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
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
        "Get a reference to the value of field `disaster_recovery_supported_locations` after provisioning.\nList of supported GCP region to clone the Autonomous Database for disaster recovery."]
    pub fn disaster_recovery_supported_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.disaster_recovery_supported_locations", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for the Autonomous Database. The name does not have to\nbe unique within your project."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entitlement_id` after provisioning.\nThe ID of the subscription entitlement associated with the Autonomous\nDatabase."]
    pub fn entitlement_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entitlement_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels or tags associated with the Autonomous Database. \n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. See documentation for resource type 'oracledatabase.googleapis.com/AutonomousDatabaseBackup'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the Autonomous Database resource in the following format:\nprojects/{project}/locations/{region}/autonomousDatabases/{autonomous_database}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC network used by the Autonomous Database.\nFormat: projects/{project}/global/networks/{network}"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_network` after provisioning.\nThe name of the OdbNetwork associated with the Autonomous Database.\nFormat:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}\nIt is optional but if specified, this should match the parent ODBNetwork of\nthe odb_subnet and backup_odb_subnet."]
    pub fn odb_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `odb_subnet` after provisioning.\nThe name of the OdbSubnet associated with the Autonomous Database for\nIP allocation. Format:\nprojects/{project}/locations/{location}/odbNetworks/{odb_network}/odbSubnets/{odb_subnet}"]
    pub fn odb_subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_subnet", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `peer_autonomous_databases` after provisioning.\nThe peer Autonomous Database names of the given Autonomous Database."]
    pub fn peer_autonomous_databases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_autonomous_databases", self.extract_ref()))
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
    pub fn properties(&self) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_config` after provisioning.\n"]
    pub fn source_config(&self) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_SourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OracleDatabaseAutonomousDatabase_TimeoutsRef {
        OracleDatabaseAutonomousDatabase_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct OracleDatabaseAutonomousDatabaseDynamic {
    properties: std::option::Option<terrars::DynamicBlock<OracleDatabaseAutonomousDatabase_Properties>>,
    source_config: std::option::Option<terrars::DynamicBlock<OracleDatabaseAutonomousDatabase_SourceConfig>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ApexDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apex_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ords_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseAutonomousDatabase_Properties_ApexDetails {
    #[doc = "Set the field `apex_version`.\n"]
    pub fn set_apex_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.apex_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ords_version`.\n"]
    pub fn set_ords_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ords_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties_ApexDetails {
    type O = terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_ApexDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties_ApexDetails {}

impl BuildOracleDatabaseAutonomousDatabase_Properties_ApexDetails {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties_ApexDetails {
        OracleDatabaseAutonomousDatabase_Properties_ApexDetails {
            apex_version: core::default::Default::default(),
            ords_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ApexDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_Properties_ApexDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_Properties_ApexDetailsRef {
        OracleDatabaseAutonomousDatabase_Properties_ApexDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_Properties_ApexDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apex_version` after provisioning.\n"]
    pub fn apex_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apex_version", self.base))
    }

    #[doc = "Get a reference to the value of field `ords_version` after provisioning.\n"]
    pub fn ords_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ords_version", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStrings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    low: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    medium: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStrings {
    #[doc = "Set the field `high`.\n"]
    pub fn set_high(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.high = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `low`.\n"]
    pub fn set_low(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.low = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `medium`.\n"]
    pub fn set_medium(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.medium = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStrings {
    type O =
        terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStrings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStrings {}

impl BuildOracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStrings {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStrings {
        OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStrings {
            high: core::default::Default::default(),
            low: core::default::Default::default(),
            medium: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStringsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStringsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStringsRef {
        OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStringsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStringsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `high` after provisioning.\n"]
    pub fn high(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.high", self.base))
    }

    #[doc = "Get a reference to the value of field `low` after provisioning.\n"]
    pub fn low(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.low", self.base))
    }

    #[doc = "Get a reference to the value of field `medium` after provisioning.\n"]
    pub fn medium(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_Profiles {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_regional: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    syntax_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_authentication: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_Profiles {
    #[doc = "Set the field `consumer_group`.\n"]
    pub fn set_consumer_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_format`.\n"]
    pub fn set_host_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_regional`.\n"]
    pub fn set_is_regional(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_regional = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_mode`.\n"]
    pub fn set_session_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.session_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `syntax_format`.\n"]
    pub fn set_syntax_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.syntax_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_authentication`.\n"]
    pub fn set_tls_authentication(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tls_authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_Profiles {
    type O = terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_Profiles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_Profiles {}

impl BuildOracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_Profiles {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_Profiles {
        OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_Profiles {
            consumer_group: core::default::Default::default(),
            display_name: core::default::Default::default(),
            host_format: core::default::Default::default(),
            is_regional: core::default::Default::default(),
            protocol: core::default::Default::default(),
            session_mode: core::default::Default::default(),
            syntax_format: core::default::Default::default(),
            tls_authentication: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_ProfilesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_ProfilesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_ProfilesRef {
        OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_ProfilesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_ProfilesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_group` after provisioning.\n"]
    pub fn consumer_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_group", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `host_format` after provisioning.\n"]
    pub fn host_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_format", self.base))
    }

    #[doc = "Get a reference to the value of field `is_regional` after provisioning.\n"]
    pub fn is_regional(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_regional", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `session_mode` after provisioning.\n"]
    pub fn session_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `syntax_format` after provisioning.\n"]
    pub fn syntax_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.syntax_format", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_authentication` after provisioning.\n"]
    pub fn tls_authentication(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_connection_strings: std
    ::option
    ::Option<terrars::ListField<OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStrings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    high: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    low: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    medium: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profiles: std
    ::option
    ::Option<terrars::ListField<OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_Profiles>>,
}

impl OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings {
    #[doc = "Set the field `all_connection_strings`.\n"]
    pub fn set_all_connection_strings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStrings,
                        >,
                    >,
    ) -> Self {
        self.all_connection_strings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated`.\n"]
    pub fn set_dedicated(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dedicated = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `high`.\n"]
    pub fn set_high(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.high = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `low`.\n"]
    pub fn set_low(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.low = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `medium`.\n"]
    pub fn set_medium(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.medium = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profiles`.\n"]
    pub fn set_profiles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_Profiles>,
                    >,
    ) -> Self {
        self.profiles = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings {
    type O = terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties_ConnectionStrings {}

impl BuildOracleDatabaseAutonomousDatabase_Properties_ConnectionStrings {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings {
        OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings {
            all_connection_strings: core::default::Default::default(),
            dedicated: core::default::Default::default(),
            high: core::default::Default::default(),
            low: core::default::Default::default(),
            medium: core::default::Default::default(),
            profiles: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ConnectionStringsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_Properties_ConnectionStringsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_Properties_ConnectionStringsRef {
        OracleDatabaseAutonomousDatabase_Properties_ConnectionStringsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_Properties_ConnectionStringsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_connection_strings` after provisioning.\n"]
    pub fn all_connection_strings(
        &self,
    ) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_AllConnectionStringsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_connection_strings", self.base))
    }

    #[doc = "Get a reference to the value of field `dedicated` after provisioning.\n"]
    pub fn dedicated(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated", self.base))
    }

    #[doc = "Get a reference to the value of field `high` after provisioning.\n"]
    pub fn high(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.high", self.base))
    }

    #[doc = "Get a reference to the value of field `low` after provisioning.\n"]
    pub fn low(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.low", self.base))
    }

    #[doc = "Get a reference to the value of field `medium` after provisioning.\n"]
    pub fn medium(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }

    #[doc = "Get a reference to the value of field `profiles` after provisioning.\n"]
    pub fn profiles(
        &self,
    ) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_Properties_ConnectionStrings_ProfilesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profiles", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ConnectionUrls {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apex_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_transforms_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    graph_studio_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_learning_notebook_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_learning_user_management_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mongo_db_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ords_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_dev_web_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseAutonomousDatabase_Properties_ConnectionUrls {
    #[doc = "Set the field `apex_uri`.\n"]
    pub fn set_apex_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.apex_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_transforms_uri`.\n"]
    pub fn set_database_transforms_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.database_transforms_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `graph_studio_uri`.\n"]
    pub fn set_graph_studio_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.graph_studio_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_learning_notebook_uri`.\n"]
    pub fn set_machine_learning_notebook_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.machine_learning_notebook_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_learning_user_management_uri`.\n"]
    pub fn set_machine_learning_user_management_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.machine_learning_user_management_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mongo_db_uri`.\n"]
    pub fn set_mongo_db_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mongo_db_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ords_uri`.\n"]
    pub fn set_ords_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ords_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sql_dev_web_uri`.\n"]
    pub fn set_sql_dev_web_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql_dev_web_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties_ConnectionUrls {
    type O = terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_ConnectionUrls>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties_ConnectionUrls {}

impl BuildOracleDatabaseAutonomousDatabase_Properties_ConnectionUrls {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties_ConnectionUrls {
        OracleDatabaseAutonomousDatabase_Properties_ConnectionUrls {
            apex_uri: core::default::Default::default(),
            database_transforms_uri: core::default::Default::default(),
            graph_studio_uri: core::default::Default::default(),
            machine_learning_notebook_uri: core::default::Default::default(),
            machine_learning_user_management_uri: core::default::Default::default(),
            mongo_db_uri: core::default::Default::default(),
            ords_uri: core::default::Default::default(),
            sql_dev_web_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ConnectionUrlsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_Properties_ConnectionUrlsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_Properties_ConnectionUrlsRef {
        OracleDatabaseAutonomousDatabase_Properties_ConnectionUrlsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_Properties_ConnectionUrlsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apex_uri` after provisioning.\n"]
    pub fn apex_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apex_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `database_transforms_uri` after provisioning.\n"]
    pub fn database_transforms_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_transforms_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `graph_studio_uri` after provisioning.\n"]
    pub fn graph_studio_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.graph_studio_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_learning_notebook_uri` after provisioning.\n"]
    pub fn machine_learning_notebook_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_learning_notebook_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_learning_user_management_uri` after provisioning.\n"]
    pub fn machine_learning_user_management_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_learning_user_management_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `mongo_db_uri` after provisioning.\n"]
    pub fn mongo_db_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mongo_db_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `ords_uri` after provisioning.\n"]
    pub fn ords_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ords_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_dev_web_uri` after provisioning.\n"]
    pub fn sql_dev_web_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_dev_web_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_guard_role_changed_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disaster_recovery_role_changed_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lag_time_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDb {
    #[doc = "Set the field `data_guard_role_changed_time`.\n"]
    pub fn set_data_guard_role_changed_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_guard_role_changed_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disaster_recovery_role_changed_time`.\n"]
    pub fn set_disaster_recovery_role_changed_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disaster_recovery_role_changed_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lag_time_duration`.\n"]
    pub fn set_lag_time_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lag_time_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_details`.\n"]
    pub fn set_lifecycle_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lifecycle_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDb {
    type O = terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties_LocalStandbyDb {}

impl BuildOracleDatabaseAutonomousDatabase_Properties_LocalStandbyDb {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDb {
        OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDb {
            data_guard_role_changed_time: core::default::Default::default(),
            disaster_recovery_role_changed_time: core::default::Default::default(),
            lag_time_duration: core::default::Default::default(),
            lifecycle_details: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDbRef {
        OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_guard_role_changed_time` after provisioning.\n"]
    pub fn data_guard_role_changed_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_guard_role_changed_time", self.base))
    }

    #[doc = "Get a reference to the value of field `disaster_recovery_role_changed_time` after provisioning.\n"]
    pub fn disaster_recovery_role_changed_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disaster_recovery_role_changed_time", self.base))
    }

    #[doc = "Get a reference to the value of field `lag_time_duration` after provisioning.\n"]
    pub fn lag_time_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lag_time_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_details` after provisioning.\n"]
    pub fn lifecycle_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_details", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTime {
    type O =
        terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTime {}

impl BuildOracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTime {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTime {
        OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTimeRef {
        OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTime {
    type O =
        terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTime {}

impl BuildOracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTime {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTime {
        OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTimeRef {
        OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<terrars::ListField<OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_time: std
    ::option
    ::Option<terrars::ListField<OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTime>>,
}

impl OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTime>,
                    >,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stop_time`.\n"]
    pub fn set_stop_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTime>,
                    >,
    ) -> Self {
        self.stop_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails {
    type O = terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails {}

impl BuildOracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails {
        OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails {
            day_of_week: core::default::Default::default(),
            start_time: core::default::Default::default(),
            stop_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetailsRef {
        OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `stop_time` after provisioning.\n"]
    pub fn stop_time(
        &self,
    ) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetails_StopTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stop_time", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties_CustomerContacts {
    email: terrars::PrimField<std::string::String>,
}

impl OracleDatabaseAutonomousDatabase_Properties_CustomerContacts { }

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties_CustomerContacts {
    type O = terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_CustomerContacts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties_CustomerContacts {
    #[doc = "The email address used by Oracle to send notifications regarding databases\nand infrastructure."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseAutonomousDatabase_Properties_CustomerContacts {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties_CustomerContacts {
        OracleDatabaseAutonomousDatabase_Properties_CustomerContacts { email: self.email }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_Properties_CustomerContactsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_Properties_CustomerContactsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_Properties_CustomerContactsRef {
        OracleDatabaseAutonomousDatabase_Properties_CustomerContactsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_Properties_CustomerContactsRef {
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

#[derive(Serialize, Default)]
struct OracleDatabaseAutonomousDatabase_PropertiesDynamic {
    customer_contacts: std
    ::option
    ::Option<terrars::DynamicBlock<OracleDatabaseAutonomousDatabase_Properties_CustomerContacts>>,
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_retention_period_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    character_set: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_storage_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_storage_size_tb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_edition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_version: std::option::Option<terrars::PrimField<std::string::String>>,
    db_workload: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_auto_scaling_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_storage_auto_scaling_enabled: std::option::Option<terrars::PrimField<bool>>,
    license_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_schedule_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtls_connection_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    n_character_set: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations_insights_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vault_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_contacts: std::option::Option<std::vec::Vec<OracleDatabaseAutonomousDatabase_Properties_CustomerContacts>>,
    dynamic: OracleDatabaseAutonomousDatabase_PropertiesDynamic,
}

impl OracleDatabaseAutonomousDatabase_Properties {
    #[doc =
        "Set the field `backup_retention_period_days`.\nThe retention period for the Autonomous Database. This field is specified\nin days, can range from 1 day to 60 days, and has a default value of\n60 days."]
    pub fn set_backup_retention_period_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.backup_retention_period_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `character_set`.\nThe character set for the Autonomous Database. The default is AL32UTF8."]
    pub fn set_character_set(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.character_set = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_count`.\nThe number of compute servers for the Autonomous Database."]
    pub fn set_compute_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.compute_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_core_count`.\nThe number of CPU cores to be made available to the database."]
    pub fn set_cpu_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_storage_size_gb`.\nThe size of the data stored in the database, in gigabytes."]
    pub fn set_data_storage_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_storage_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_storage_size_tb`.\nThe size of the data stored in the database, in terabytes."]
    pub fn set_data_storage_size_tb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.data_storage_size_tb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `db_edition`.\nThe edition of the Autonomous Databases. \n Possible values:\n DATABASE_EDITION_UNSPECIFIED\nSTANDARD_EDITION\nENTERPRISE_EDITION"]
    pub fn set_db_edition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_edition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_version`.\nThe Oracle Database version for the Autonomous Database."]
    pub fn set_db_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.db_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_auto_scaling_enabled`.\nThis field indicates if auto scaling is enabled for the Autonomous Database\nCPU core count."]
    pub fn set_is_auto_scaling_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_auto_scaling_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_storage_auto_scaling_enabled`.\nThis field indicates if auto scaling is enabled for the Autonomous Database\nstorage."]
    pub fn set_is_storage_auto_scaling_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_storage_auto_scaling_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `maintenance_schedule_type`.\nThe maintenance schedule of the Autonomous Database. \n Possible values:\n MAINTENANCE_SCHEDULE_TYPE_UNSPECIFIED\nEARLY\nREGULAR"]
    pub fn set_maintenance_schedule_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.maintenance_schedule_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mtls_connection_required`.\nThis field specifies if the Autonomous Database requires mTLS connections."]
    pub fn set_mtls_connection_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mtls_connection_required = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `n_character_set`.\nThe national character set for the Autonomous Database. The default is\nAL16UTF16."]
    pub fn set_n_character_set(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.n_character_set = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operations_insights_state`.\nPossible values:\n OPERATIONS_INSIGHTS_STATE_UNSPECIFIED\nENABLING\nENABLED\nDISABLING\nNOT_ENABLED\nFAILED_ENABLING\nFAILED_DISABLING"]
    pub fn set_operations_insights_state(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.operations_insights_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_endpoint_ip`.\nThe private endpoint IP address for the Autonomous Database."]
    pub fn set_private_endpoint_ip(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_endpoint_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_endpoint_label`.\nThe private endpoint label for the Autonomous Database."]
    pub fn set_private_endpoint_label(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_endpoint_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_id`.\nThe ID of the Oracle Cloud Infrastructure vault secret."]
    pub fn set_secret_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vault_id`.\nThe ID of the Oracle Cloud Infrastructure vault."]
    pub fn set_vault_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vault_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_contacts`.\n"]
    pub fn set_customer_contacts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties_CustomerContacts>>,
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
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Properties {
    type O = terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Properties {
    #[doc = "Possible values:\n DB_WORKLOAD_UNSPECIFIED\nOLTP\nDW\nAJD\nAPEX"]
    pub db_workload: terrars::PrimField<std::string::String>,
    #[doc =
        "The license type used for the Autonomous Database. \n Possible values:\n LICENSE_TYPE_UNSPECIFIED\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub license_type: terrars::PrimField<std::string::String>,
}

impl BuildOracleDatabaseAutonomousDatabase_Properties {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Properties {
        OracleDatabaseAutonomousDatabase_Properties {
            backup_retention_period_days: core::default::Default::default(),
            character_set: core::default::Default::default(),
            compute_count: core::default::Default::default(),
            cpu_core_count: core::default::Default::default(),
            data_storage_size_gb: core::default::Default::default(),
            data_storage_size_tb: core::default::Default::default(),
            db_edition: core::default::Default::default(),
            db_version: core::default::Default::default(),
            db_workload: self.db_workload,
            is_auto_scaling_enabled: core::default::Default::default(),
            is_storage_auto_scaling_enabled: core::default::Default::default(),
            license_type: self.license_type,
            maintenance_schedule_type: core::default::Default::default(),
            mtls_connection_required: core::default::Default::default(),
            n_character_set: core::default::Default::default(),
            operations_insights_state: core::default::Default::default(),
            private_endpoint_ip: core::default::Default::default(),
            private_endpoint_label: core::default::Default::default(),
            secret_id: core::default::Default::default(),
            vault_id: core::default::Default::default(),
            customer_contacts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_PropertiesRef {
        OracleDatabaseAutonomousDatabase_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `actual_used_data_storage_size_tb` after provisioning.\nThe amount of storage currently being used for user and system data, in\nterabytes."]
    pub fn actual_used_data_storage_size_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.actual_used_data_storage_size_tb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allocated_storage_size_tb` after provisioning.\nThe amount of storage currently allocated for the database tables and\nbilled for, rounded up in terabytes."]
    pub fn allocated_storage_size_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocated_storage_size_tb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `apex_details` after provisioning.\nOracle APEX Application Development.\nhttps://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseApex"]
    pub fn apex_details(&self) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_Properties_ApexDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apex_details", self.base))
    }

    #[doc =
        "Get a reference to the value of field `are_primary_allowlisted_ips_used` after provisioning.\nThis field indicates the status of Data Guard and Access control for the\nAutonomous Database. The field's value is null if Data Guard is disabled\nor Access Control is disabled. The field's value is TRUE if both Data Guard\nand Access Control are enabled, and the Autonomous Database is using\nprimary IP access control list (ACL) for standby. The field's value is\nFALSE if both Data Guard and Access Control are enabled, and the Autonomous\nDatabase is using a different IP access control list (ACL) for standby\ncompared to primary."]
    pub fn are_primary_allowlisted_ips_used(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.are_primary_allowlisted_ips_used", self.base))
    }

    #[doc =
        "Get a reference to the value of field `autonomous_container_database_id` after provisioning.\nThe Autonomous Container Database OCID."]
    pub fn autonomous_container_database_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autonomous_container_database_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `available_upgrade_versions` after provisioning.\nThe list of available Oracle Database upgrade versions for an Autonomous\nDatabase."]
    pub fn available_upgrade_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_upgrade_versions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `backup_retention_period_days` after provisioning.\nThe retention period for the Autonomous Database. This field is specified\nin days, can range from 1 day to 60 days, and has a default value of\n60 days."]
    pub fn backup_retention_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_period_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `character_set` after provisioning.\nThe character set for the Autonomous Database. The default is AL32UTF8."]
    pub fn character_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.character_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `compute_count` after provisioning.\nThe number of compute servers for the Autonomous Database."]
    pub fn compute_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_strings` after provisioning.\nThe connection string used to connect to the Autonomous Database.\nhttps://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseConnectionStrings"]
    pub fn connection_strings(
        &self,
    ) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_Properties_ConnectionStringsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_strings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_urls` after provisioning.\nThe URLs for accessing Oracle Application Express (APEX) and SQL Developer\nWeb with a browser from a Compute instance.\nhttps://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseConnectionUrls"]
    pub fn connection_urls(&self) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_Properties_ConnectionUrlsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_urls", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_core_count` after provisioning.\nThe number of CPU cores to be made available to the database."]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_safe_state` after provisioning.\nThe current state of the Data Safe registration for the\nAutonomous Database. \n Possible values:\n DATA_SAFE_STATE_UNSPECIFIED\nREGISTERING\nREGISTERED\nDEREGISTERING\nNOT_REGISTERED\nFAILED"]
    pub fn data_safe_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_safe_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_gb` after provisioning.\nThe size of the data stored in the database, in gigabytes."]
    pub fn data_storage_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_storage_size_tb` after provisioning.\nThe size of the data stored in the database, in terabytes."]
    pub fn data_storage_size_tb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_storage_size_tb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_management_state` after provisioning.\nThe current state of database management for the Autonomous Database. \n Possible values:\n DATABASE_MANAGEMENT_STATE_UNSPECIFIED\nENABLING\nENABLED\nDISABLING\nNOT_ENABLED\nFAILED_ENABLING\nFAILED_DISABLING"]
    pub fn database_management_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_management_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `db_edition` after provisioning.\nThe edition of the Autonomous Databases. \n Possible values:\n DATABASE_EDITION_UNSPECIFIED\nSTANDARD_EDITION\nENTERPRISE_EDITION"]
    pub fn db_edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_edition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `db_version` after provisioning.\nThe Oracle Database version for the Autonomous Database."]
    pub fn db_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `db_workload` after provisioning.\nPossible values:\n DB_WORKLOAD_UNSPECIFIED\nOLTP\nDW\nAJD\nAPEX"]
    pub fn db_workload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_workload", self.base))
    }

    #[doc =
        "Get a reference to the value of field `failed_data_recovery_duration` after provisioning.\nThis field indicates the number of seconds of data loss during a Data\nGuard failover."]
    pub fn failed_data_recovery_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failed_data_recovery_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_auto_scaling_enabled` after provisioning.\nThis field indicates if auto scaling is enabled for the Autonomous Database\nCPU core count."]
    pub fn is_auto_scaling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_auto_scaling_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_local_data_guard_enabled` after provisioning.\nThis field indicates whether the Autonomous Database has local (in-region)\nData Guard enabled."]
    pub fn is_local_data_guard_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_local_data_guard_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_storage_auto_scaling_enabled` after provisioning.\nThis field indicates if auto scaling is enabled for the Autonomous Database\nstorage."]
    pub fn is_storage_auto_scaling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_storage_auto_scaling_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `license_type` after provisioning.\nThe license type used for the Autonomous Database. \n Possible values:\n LICENSE_TYPE_UNSPECIFIED\nLICENSE_INCLUDED\nBRING_YOUR_OWN_LICENSE"]
    pub fn license_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lifecycle_details` after provisioning.\nThe details of the current lifestyle state of the Autonomous Database."]
    pub fn lifecycle_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_details", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_adg_auto_failover_max_data_loss_limit` after provisioning.\nThis field indicates the maximum data loss limit for an Autonomous\nDatabase, in seconds."]
    pub fn local_adg_auto_failover_max_data_loss_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.local_adg_auto_failover_max_data_loss_limit", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `local_disaster_recovery_type` after provisioning.\nThis field indicates the local disaster recovery (DR) type of an\nAutonomous Database. \n Possible values:\n LOCAL_DISASTER_RECOVERY_TYPE_UNSPECIFIED\nADG\nBACKUP_BASED"]
    pub fn local_disaster_recovery_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_disaster_recovery_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_standby_db` after provisioning.\nAutonomous Data Guard standby database details.\nhttps://docs.oracle.com/en-us/iaas/api/#/en/database/20160918/datatypes/AutonomousDatabaseStandbySummary"]
    pub fn local_standby_db(&self) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_Properties_LocalStandbyDbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_standby_db", self.base))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_begin_time` after provisioning.\nThe date and time when maintenance will begin."]
    pub fn maintenance_begin_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_begin_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_end_time` after provisioning.\nThe date and time when maintenance will end."]
    pub fn maintenance_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_schedule_type` after provisioning.\nThe maintenance schedule of the Autonomous Database. \n Possible values:\n MAINTENANCE_SCHEDULE_TYPE_UNSPECIFIED\nEARLY\nREGULAR"]
    pub fn maintenance_schedule_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maintenance_schedule_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_per_oracle_compute_unit_gbs` after provisioning.\nThe amount of memory enabled per ECPU, in gigabytes."]
    pub fn memory_per_oracle_compute_unit_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_per_oracle_compute_unit_gbs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_table_gbs` after provisioning.\nThe memory assigned to in-memory tables in an Autonomous Database."]
    pub fn memory_table_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_table_gbs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mtls_connection_required` after provisioning.\nThis field specifies if the Autonomous Database requires mTLS connections."]
    pub fn mtls_connection_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtls_connection_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `n_character_set` after provisioning.\nThe national character set for the Autonomous Database. The default is\nAL16UTF16."]
    pub fn n_character_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.n_character_set", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_long_term_backup_time` after provisioning.\nThe long term backup schedule of the Autonomous Database."]
    pub fn next_long_term_backup_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_long_term_backup_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oci_url` after provisioning.\nThe Oracle Cloud Infrastructure link for the Autonomous Database."]
    pub fn oci_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oci_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ocid` after provisioning.\nOCID of the Autonomous Database.\nhttps://docs.oracle.com/en-us/iaas/Content/General/Concepts/identifiers.htm#Oracle"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `open_mode` after provisioning.\nThis field indicates the current mode of the Autonomous Database. \n Possible values:\n OPEN_MODE_UNSPECIFIED\nREAD_ONLY\nREAD_WRITE"]
    pub fn open_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operations_insights_state` after provisioning.\nPossible values:\n OPERATIONS_INSIGHTS_STATE_UNSPECIFIED\nENABLING\nENABLED\nDISABLING\nNOT_ENABLED\nFAILED_ENABLING\nFAILED_DISABLING"]
    pub fn operations_insights_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operations_insights_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `peer_db_ids` after provisioning.\nThe list of OCIDs of standby databases located in Autonomous Data Guard\nremote regions that are associated with the source database."]
    pub fn peer_db_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peer_db_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permission_level` after provisioning.\nThe permission level of the Autonomous Database. \n Possible values:\n PERMISSION_LEVEL_UNSPECIFIED\nRESTRICTED\nUNRESTRICTED"]
    pub fn permission_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint` after provisioning.\nThe private endpoint for the Autonomous Database."]
    pub fn private_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint_ip` after provisioning.\nThe private endpoint IP address for the Autonomous Database."]
    pub fn private_endpoint_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint_label` after provisioning.\nThe private endpoint label for the Autonomous Database."]
    pub fn private_endpoint_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `refreshable_mode` after provisioning.\nThe refresh mode of the cloned Autonomous Database. \n Possible values:\n REFRESHABLE_MODE_UNSPECIFIED\nAUTOMATIC\nMANUAL"]
    pub fn refreshable_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refreshable_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `refreshable_state` after provisioning.\nThe refresh State of the clone. \n Possible values:\n REFRESHABLE_STATE_UNSPECIFIED\nREFRESHING\nNOT_REFRESHING"]
    pub fn refreshable_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refreshable_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nThe Data Guard role of the Autonomous Database. \n Possible values:\n ROLE_UNSPECIFIED\nPRIMARY\nSTANDBY\nDISABLED_STANDBY\nBACKUP_COPY\nSNAPSHOT_STANDBY"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scheduled_operation_details` after provisioning.\nThe list and details of the scheduled operations of the Autonomous\nDatabase."]
    pub fn scheduled_operation_details(
        &self,
    ) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_Properties_ScheduledOperationDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_operation_details", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_id` after provisioning.\nThe ID of the Oracle Cloud Infrastructure vault secret."]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sql_web_developer_url` after provisioning.\nThe SQL Web Developer URL for the Autonomous Database."]
    pub fn sql_web_developer_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_web_developer_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nPossible values:\n STATE_UNSPECIFIED\nPROVISIONING\nAVAILABLE\nSTOPPING\nSTOPPED\nSTARTING\nTERMINATING\nTERMINATED\nUNAVAILABLE\nRESTORE_IN_PROGRESS\nRESTORE_FAILED\nBACKUP_IN_PROGRESS\nSCALE_IN_PROGRESS\nAVAILABLE_NEEDS_ATTENTION\nUPDATING\nMAINTENANCE_IN_PROGRESS\nRESTARTING\nRECREATING\nROLE_CHANGE_IN_PROGRESS\nUPGRADING\nINACCESSIBLE\nSTANDBY"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `supported_clone_regions` after provisioning.\nThe list of available regions that can be used to create a clone for the\nAutonomous Database."]
    pub fn supported_clone_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_clone_regions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `total_auto_backup_storage_size_gbs` after provisioning.\nThe storage space used by automatic backups of Autonomous Database, in\ngigabytes."]
    pub fn total_auto_backup_storage_size_gbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_auto_backup_storage_size_gbs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `used_data_storage_size_tbs` after provisioning.\nThe storage space used by Autonomous Database, in gigabytes."]
    pub fn used_data_storage_size_tbs(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.used_data_storage_size_tbs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vault_id` after provisioning.\nThe ID of the Oracle Cloud Infrastructure vault."]
    pub fn vault_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vault_id", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_contacts` after provisioning.\n"]
    pub fn customer_contacts(&self) -> terrars::ListRef<OracleDatabaseAutonomousDatabase_Properties_CustomerContactsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_contacts", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_SourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_backups_replication_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autonomous_database: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseAutonomousDatabase_SourceConfig {
    #[doc =
        "Set the field `automatic_backups_replication_enabled`.\nThis field specifies if the replication of automatic backups is enabled when creating a Data Guard."]
    pub fn set_automatic_backups_replication_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.automatic_backups_replication_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `autonomous_database`.\nThe name of the primary Autonomous Database that is used to create a Peer Autonomous Database from a source."]
    pub fn set_autonomous_database(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.autonomous_database = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_SourceConfig {
    type O = terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_SourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_SourceConfig {}

impl BuildOracleDatabaseAutonomousDatabase_SourceConfig {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_SourceConfig {
        OracleDatabaseAutonomousDatabase_SourceConfig {
            automatic_backups_replication_enabled: core::default::Default::default(),
            autonomous_database: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_SourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_SourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OracleDatabaseAutonomousDatabase_SourceConfigRef {
        OracleDatabaseAutonomousDatabase_SourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_SourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `automatic_backups_replication_enabled` after provisioning.\nThis field specifies if the replication of automatic backups is enabled when creating a Data Guard."]
    pub fn automatic_backups_replication_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_backups_replication_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `autonomous_database` after provisioning.\nThe name of the primary Autonomous Database that is used to create a Peer Autonomous Database from a source."]
    pub fn autonomous_database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autonomous_database", self.base))
    }
}

#[derive(Serialize)]
pub struct OracleDatabaseAutonomousDatabase_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OracleDatabaseAutonomousDatabase_Timeouts {
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

impl terrars::ToListMappable for OracleDatabaseAutonomousDatabase_Timeouts {
    type O = terrars::BlockAssignable<OracleDatabaseAutonomousDatabase_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOracleDatabaseAutonomousDatabase_Timeouts {}

impl BuildOracleDatabaseAutonomousDatabase_Timeouts {
    pub fn build(self) -> OracleDatabaseAutonomousDatabase_Timeouts {
        OracleDatabaseAutonomousDatabase_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OracleDatabaseAutonomousDatabase_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OracleDatabaseAutonomousDatabase_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OracleDatabaseAutonomousDatabase_TimeoutsRef {
        OracleDatabaseAutonomousDatabase_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OracleDatabaseAutonomousDatabase_TimeoutsRef {
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
