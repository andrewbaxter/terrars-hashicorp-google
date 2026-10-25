#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DatabaseMigrationServicePrivateConnectionData_ {
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
    create_without_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    private_connection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_interface_config: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServicePrivateConnection_PscInterfaceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_public_ip_config: std
    ::option
    ::Option<std::vec::Vec<DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DatabaseMigrationServicePrivateConnection_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_peering_config: std::option::Option<std::vec::Vec<DatabaseMigrationServicePrivateConnection_VpcPeeringConfig>>,
    dynamic: DatabaseMigrationServicePrivateConnectionDynamic,
}

struct DatabaseMigrationServicePrivateConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabaseMigrationServicePrivateConnectionData_>,
}

#[derive(Clone)]
pub struct DatabaseMigrationServicePrivateConnection(std::rc::Rc<DatabaseMigrationServicePrivateConnection_>);

impl DatabaseMigrationServicePrivateConnection {
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

    #[doc = "Set the field `create_without_validation`.\nIf set to true, will skip validations."]
    pub fn set_create_without_validation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_without_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nDisplay name."]
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
        "Set the field `labels`.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `psc_interface_config`.\n"]
    pub fn set_psc_interface_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatabaseMigrationServicePrivateConnection_PscInterfaceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_interface_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_interface_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reserved_public_ip_config`.\n"]
    pub fn set_reserved_public_ip_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reserved_public_ip_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reserved_public_ip_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DatabaseMigrationServicePrivateConnection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_config`.\n"]
    pub fn set_vpc_peering_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatabaseMigrationServicePrivateConnection_VpcPeeringConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_peering_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_peering_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_without_validation` after provisioning.\nIf set to true, will skip validations."]
    pub fn create_without_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_without_validation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error` after provisioning.\nThe PrivateConnection error in case of failure."]
    pub fn error(&self) -> terrars::ListRef<DatabaseMigrationServicePrivateConnection_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this private connection is located in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_connection_id` after provisioning.\nThe private connectivity identifier."]
    pub fn private_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the PrivateConnection."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_interface_config` after provisioning.\n"]
    pub fn psc_interface_config(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServicePrivateConnection_PscInterfaceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_interface_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reserved_public_ip_config` after provisioning.\n"]
    pub fn reserved_public_ip_config(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reserved_public_ip_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServicePrivateConnection_TimeoutsRef {
        DatabaseMigrationServicePrivateConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_peering_config` after provisioning.\n"]
    pub fn vpc_peering_config(&self) -> terrars::ListRef<DatabaseMigrationServicePrivateConnection_VpcPeeringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_peering_config", self.extract_ref()))
    }
}

impl terrars::Referable for DatabaseMigrationServicePrivateConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabaseMigrationServicePrivateConnection { }

impl terrars::ToListMappable for DatabaseMigrationServicePrivateConnection {
    type O = terrars::ListRef<DatabaseMigrationServicePrivateConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabaseMigrationServicePrivateConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_database_migration_service_private_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseMigrationServicePrivateConnection {
    pub tf_id: std::string::String,
    #[doc = "The name of the location this private connection is located in."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The private connectivity identifier."]
    pub private_connection_id: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServicePrivateConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabaseMigrationServicePrivateConnection {
        let out =
            DatabaseMigrationServicePrivateConnection(std::rc::Rc::new(DatabaseMigrationServicePrivateConnection_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DatabaseMigrationServicePrivateConnectionData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    create_without_validation: core::default::Default::default(),
                    deletion_policy: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
                    private_connection_id: self.private_connection_id,
                    project: core::default::Default::default(),
                    psc_interface_config: core::default::Default::default(),
                    reserved_public_ip_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    vpc_peering_config: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseMigrationServicePrivateConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServicePrivateConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseMigrationServicePrivateConnectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_without_validation` after provisioning.\nIf set to true, will skip validations."]
    pub fn create_without_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_without_validation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error` after provisioning.\nThe PrivateConnection error in case of failure."]
    pub fn error(&self) -> terrars::ListRef<DatabaseMigrationServicePrivateConnection_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this private connection is located in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_connection_id` after provisioning.\nThe private connectivity identifier."]
    pub fn private_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the PrivateConnection."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_interface_config` after provisioning.\n"]
    pub fn psc_interface_config(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServicePrivateConnection_PscInterfaceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_interface_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reserved_public_ip_config` after provisioning.\n"]
    pub fn reserved_public_ip_config(
        &self,
    ) -> terrars::ListRef<DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reserved_public_ip_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatabaseMigrationServicePrivateConnection_TimeoutsRef {
        DatabaseMigrationServicePrivateConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_peering_config` after provisioning.\n"]
    pub fn vpc_peering_config(&self) -> terrars::ListRef<DatabaseMigrationServicePrivateConnection_VpcPeeringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_peering_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DatabaseMigrationServicePrivateConnectionDynamic {
    psc_interface_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServicePrivateConnection_PscInterfaceConfig>>,
    reserved_public_ip_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig>>,
    vpc_peering_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatabaseMigrationServicePrivateConnection_VpcPeeringConfig>>,
}

#[derive(Serialize)]
pub struct DatabaseMigrationServicePrivateConnection_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServicePrivateConnection_Error {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServicePrivateConnection_Error {
    type O = terrars::BlockAssignable<DatabaseMigrationServicePrivateConnection_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServicePrivateConnection_Error {}

impl BuildDatabaseMigrationServicePrivateConnection_Error {
    pub fn build(self) -> DatabaseMigrationServicePrivateConnection_Error {
        DatabaseMigrationServicePrivateConnection_Error {
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServicePrivateConnection_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServicePrivateConnection_ErrorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServicePrivateConnection_ErrorRef {
        DatabaseMigrationServicePrivateConnection_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServicePrivateConnection_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServicePrivateConnection_PscInterfaceConfig {
    network_attachment: terrars::PrimField<std::string::String>,
}

impl DatabaseMigrationServicePrivateConnection_PscInterfaceConfig { }

impl terrars::ToListMappable for DatabaseMigrationServicePrivateConnection_PscInterfaceConfig {
    type O = terrars::BlockAssignable<DatabaseMigrationServicePrivateConnection_PscInterfaceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServicePrivateConnection_PscInterfaceConfig {
    #[doc =
        "Fully qualified name of the Network Attachment that DMS will connect to.\nFormat: projects/{project}/regions/{region}/networkAttachments/{name}"]
    pub network_attachment: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServicePrivateConnection_PscInterfaceConfig {
    pub fn build(self) -> DatabaseMigrationServicePrivateConnection_PscInterfaceConfig {
        DatabaseMigrationServicePrivateConnection_PscInterfaceConfig { network_attachment: self.network_attachment }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServicePrivateConnection_PscInterfaceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServicePrivateConnection_PscInterfaceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServicePrivateConnection_PscInterfaceConfigRef {
        DatabaseMigrationServicePrivateConnection_PscInterfaceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServicePrivateConnection_PscInterfaceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_attachment` after provisioning.\nFully qualified name of the Network Attachment that DMS will connect to.\nFormat: projects/{project}/regions/{region}/networkAttachments/{name}"]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ips_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig {
    #[doc = "Set the field `nat_ips_count`.\nOptional. Number of static public IP addresses to reserve."]
    pub fn set_nat_ips_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nat_ips_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig {
    type O = terrars::BlockAssignable<DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig {}

impl BuildDatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig {
    pub fn build(self) -> DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig {
        DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfig {
            nat_ips_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfigRef {
        DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServicePrivateConnection_ReservedPublicIpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `egress_public_ips` after provisioning.\nOutput only. The reserved public IPs."]
    pub fn egress_public_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_public_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nat_ips_count` after provisioning.\nOptional. Number of static public IP addresses to reserve."]
    pub fn nat_ips_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_ips_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DatabaseMigrationServicePrivateConnection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatabaseMigrationServicePrivateConnection_Timeouts {
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

impl terrars::ToListMappable for DatabaseMigrationServicePrivateConnection_Timeouts {
    type O = terrars::BlockAssignable<DatabaseMigrationServicePrivateConnection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServicePrivateConnection_Timeouts {}

impl BuildDatabaseMigrationServicePrivateConnection_Timeouts {
    pub fn build(self) -> DatabaseMigrationServicePrivateConnection_Timeouts {
        DatabaseMigrationServicePrivateConnection_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServicePrivateConnection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServicePrivateConnection_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServicePrivateConnection_TimeoutsRef {
        DatabaseMigrationServicePrivateConnection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServicePrivateConnection_TimeoutsRef {
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

#[derive(Serialize)]
pub struct DatabaseMigrationServicePrivateConnection_VpcPeeringConfig {
    subnet: terrars::PrimField<std::string::String>,
    vpc_name: terrars::PrimField<std::string::String>,
}

impl DatabaseMigrationServicePrivateConnection_VpcPeeringConfig { }

impl terrars::ToListMappable for DatabaseMigrationServicePrivateConnection_VpcPeeringConfig {
    type O = terrars::BlockAssignable<DatabaseMigrationServicePrivateConnection_VpcPeeringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseMigrationServicePrivateConnection_VpcPeeringConfig {
    #[doc = "A free subnet for peering. (CIDR of /29)"]
    pub subnet: terrars::PrimField<std::string::String>,
    #[doc =
        "Fully qualified name of the VPC that Database Migration Service will peer to.\nFormat: projects/{project}/global/{networks}/{name}"]
    pub vpc_name: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseMigrationServicePrivateConnection_VpcPeeringConfig {
    pub fn build(self) -> DatabaseMigrationServicePrivateConnection_VpcPeeringConfig {
        DatabaseMigrationServicePrivateConnection_VpcPeeringConfig {
            subnet: self.subnet,
            vpc_name: self.vpc_name,
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseMigrationServicePrivateConnection_VpcPeeringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseMigrationServicePrivateConnection_VpcPeeringConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatabaseMigrationServicePrivateConnection_VpcPeeringConfigRef {
        DatabaseMigrationServicePrivateConnection_VpcPeeringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseMigrationServicePrivateConnection_VpcPeeringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `subnet` after provisioning.\nA free subnet for peering. (CIDR of /29)"]
    pub fn subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vpc_name` after provisioning.\nFully qualified name of the VPC that Database Migration Service will peer to.\nFormat: projects/{project}/global/{networks}/{name}"]
    pub fn vpc_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_name", self.base))
    }
}
