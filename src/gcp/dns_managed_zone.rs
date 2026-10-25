#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DnsManagedZoneData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    dns_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_logging_config: std::option::Option<std::vec::Vec<DnsManagedZone_CloudLoggingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dnssec_config: std::option::Option<std::vec::Vec<DnsManagedZone_DnssecConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_config: std::option::Option<std::vec::Vec<DnsManagedZone_ForwardingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peering_config: std::option::Option<std::vec::Vec<DnsManagedZone_PeeringConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_visibility_config: std::option::Option<std::vec::Vec<DnsManagedZone_PrivateVisibilityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DnsManagedZone_Timeouts>,
    dynamic: DnsManagedZoneDynamic,
}

struct DnsManagedZone_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DnsManagedZoneData_>,
}

#[derive(Clone)]
pub struct DnsManagedZone(std::rc::Rc<DnsManagedZone_>);

impl DnsManagedZone {
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

    #[doc = "Set the field `description`.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\nSet this true to delete all records in the zone."]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA set of key/value label pairs to assign to this ManagedZone.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc =
        "Set the field `visibility`.\nThe zone's visibility: public zones are exposed to the Internet,\nwhile private zones are visible only to Virtual Private Cloud resources. Default value: \"public\" Possible values: [\"private\", \"public\"]"]
    pub fn set_visibility(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().visibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_logging_config`.\n"]
    pub fn set_cloud_logging_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_CloudLoggingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_logging_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_logging_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dnssec_config`.\n"]
    pub fn set_dnssec_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_DnssecConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dnssec_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dnssec_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forwarding_config`.\n"]
    pub fn set_forwarding_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_ForwardingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().forwarding_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.forwarding_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `peering_config`.\n"]
    pub fn set_peering_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_PeeringConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().peering_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.peering_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_visibility_config`.\n"]
    pub fn set_private_visibility_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_PrivateVisibilityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_visibility_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_visibility_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DnsManagedZone_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nThe time that this resource was created on the server.\nThis is in RFC3339 text format."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_name` after provisioning.\nThe DNS name of this managed zone, for instance \"example.com.\"."]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nSet this true to delete all records in the zone."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this ManagedZone.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_zone_id` after provisioning.\nUnique identifier for the resource; defined by the server."]
    pub fn managed_zone_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUser assigned name for this resource.\nMust be unique within the project."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name_servers` after provisioning.\nDelegate your managed_zone to these virtual name servers;\ndefined by the server"]
    pub fn name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_servers", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `visibility` after provisioning.\nThe zone's visibility: public zones are exposed to the Internet,\nwhile private zones are visible only to Virtual Private Cloud resources. Default value: \"public\" Possible values: [\"private\", \"public\"]"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_logging_config` after provisioning.\n"]
    pub fn cloud_logging_config(&self) -> terrars::ListRef<DnsManagedZone_CloudLoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dnssec_config` after provisioning.\n"]
    pub fn dnssec_config(&self) -> terrars::ListRef<DnsManagedZone_DnssecConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dnssec_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forwarding_config` after provisioning.\n"]
    pub fn forwarding_config(&self) -> terrars::ListRef<DnsManagedZone_ForwardingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarding_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peering_config` after provisioning.\n"]
    pub fn peering_config(&self) -> terrars::ListRef<DnsManagedZone_PeeringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peering_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_visibility_config` after provisioning.\n"]
    pub fn private_visibility_config(&self) -> terrars::ListRef<DnsManagedZone_PrivateVisibilityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_visibility_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsManagedZone_TimeoutsRef {
        DnsManagedZone_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DnsManagedZone {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DnsManagedZone { }

impl terrars::ToListMappable for DnsManagedZone {
    type O = terrars::ListRef<DnsManagedZoneRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DnsManagedZone_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dns_managed_zone".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDnsManagedZone {
    pub tf_id: std::string::String,
    #[doc = "The DNS name of this managed zone, for instance \"example.com.\"."]
    pub dns_name: terrars::PrimField<std::string::String>,
    #[doc = "User assigned name for this resource.\nMust be unique within the project."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDnsManagedZone {
    pub fn build(self, stack: &mut terrars::Stack) -> DnsManagedZone {
        let out = DnsManagedZone(std::rc::Rc::new(DnsManagedZone_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DnsManagedZoneData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                dns_name: self.dns_name,
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                visibility: core::default::Default::default(),
                cloud_logging_config: core::default::Default::default(),
                dnssec_config: core::default::Default::default(),
                forwarding_config: core::default::Default::default(),
                peering_config: core::default::Default::default(),
                private_visibility_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DnsManagedZoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DnsManagedZoneRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nThe time that this resource was created on the server.\nThis is in RFC3339 text format."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA textual description field. Defaults to 'Managed by Terraform'."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dns_name` after provisioning.\nThe DNS name of this managed zone, for instance \"example.com.\"."]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nSet this true to delete all records in the zone."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this ManagedZone.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `managed_zone_id` after provisioning.\nUnique identifier for the resource; defined by the server."]
    pub fn managed_zone_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_zone_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUser assigned name for this resource.\nMust be unique within the project."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name_servers` after provisioning.\nDelegate your managed_zone to these virtual name servers;\ndefined by the server"]
    pub fn name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_servers", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `visibility` after provisioning.\nThe zone's visibility: public zones are exposed to the Internet,\nwhile private zones are visible only to Virtual Private Cloud resources. Default value: \"public\" Possible values: [\"private\", \"public\"]"]
    pub fn visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_logging_config` after provisioning.\n"]
    pub fn cloud_logging_config(&self) -> terrars::ListRef<DnsManagedZone_CloudLoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_logging_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dnssec_config` after provisioning.\n"]
    pub fn dnssec_config(&self) -> terrars::ListRef<DnsManagedZone_DnssecConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dnssec_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forwarding_config` after provisioning.\n"]
    pub fn forwarding_config(&self) -> terrars::ListRef<DnsManagedZone_ForwardingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarding_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peering_config` after provisioning.\n"]
    pub fn peering_config(&self) -> terrars::ListRef<DnsManagedZone_PeeringConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.peering_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_visibility_config` after provisioning.\n"]
    pub fn private_visibility_config(&self) -> terrars::ListRef<DnsManagedZone_PrivateVisibilityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_visibility_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsManagedZone_TimeoutsRef {
        DnsManagedZone_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DnsManagedZoneDynamic {
    cloud_logging_config: std::option::Option<terrars::DynamicBlock<DnsManagedZone_CloudLoggingConfig>>,
    dnssec_config: std::option::Option<terrars::DynamicBlock<DnsManagedZone_DnssecConfig>>,
    forwarding_config: std::option::Option<terrars::DynamicBlock<DnsManagedZone_ForwardingConfig>>,
    peering_config: std::option::Option<terrars::DynamicBlock<DnsManagedZone_PeeringConfig>>,
    private_visibility_config: std::option::Option<terrars::DynamicBlock<DnsManagedZone_PrivateVisibilityConfig>>,
}

#[derive(Serialize)]
pub struct DnsManagedZone_CloudLoggingConfig {
    enable_logging: terrars::PrimField<bool>,
}

impl DnsManagedZone_CloudLoggingConfig { }

impl terrars::ToListMappable for DnsManagedZone_CloudLoggingConfig {
    type O = terrars::BlockAssignable<DnsManagedZone_CloudLoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_CloudLoggingConfig {
    #[doc = "If set, enable query logging for this ManagedZone. False by default, making logging opt-in."]
    pub enable_logging: terrars::PrimField<bool>,
}

impl BuildDnsManagedZone_CloudLoggingConfig {
    pub fn build(self) -> DnsManagedZone_CloudLoggingConfig {
        DnsManagedZone_CloudLoggingConfig { enable_logging: self.enable_logging }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_CloudLoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_CloudLoggingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsManagedZone_CloudLoggingConfigRef {
        DnsManagedZone_CloudLoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_CloudLoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_logging` after provisioning.\nIf set, enable query logging for this ManagedZone. False by default, making logging opt-in."]
    pub fn enable_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsManagedZone_DnssecConfig_DefaultKeySpecs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsManagedZone_DnssecConfig_DefaultKeySpecs {
    #[doc =
        "Set the field `algorithm`.\nString mnemonic specifying the DNSSEC algorithm of this key Possible values: [\"ecdsap256sha256\", \"ecdsap384sha384\", \"rsasha1\", \"rsasha256\", \"rsasha512\"]"]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_length`.\nLength of the keys in bits"]
    pub fn set_key_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.key_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_type`.\nSpecifies whether this is a key signing key (KSK) or a zone\nsigning key (ZSK). Key signing keys have the Secure Entry\nPoint flag set and, when active, will only be used to sign\nresource record sets of type DNSKEY. Zone signing keys do\nnot have the Secure Entry Point flag set and will be used\nto sign all other types of resource record sets. Possible values: [\"keySigning\", \"zoneSigning\"]"]
    pub fn set_key_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kind`.\nIdentifies what kind of resource this is"]
    pub fn set_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kind = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsManagedZone_DnssecConfig_DefaultKeySpecs {
    type O = terrars::BlockAssignable<DnsManagedZone_DnssecConfig_DefaultKeySpecs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_DnssecConfig_DefaultKeySpecs {}

impl BuildDnsManagedZone_DnssecConfig_DefaultKeySpecs {
    pub fn build(self) -> DnsManagedZone_DnssecConfig_DefaultKeySpecs {
        DnsManagedZone_DnssecConfig_DefaultKeySpecs {
            algorithm: core::default::Default::default(),
            key_length: core::default::Default::default(),
            key_type: core::default::Default::default(),
            kind: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_DnssecConfig_DefaultKeySpecsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_DnssecConfig_DefaultKeySpecsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsManagedZone_DnssecConfig_DefaultKeySpecsRef {
        DnsManagedZone_DnssecConfig_DefaultKeySpecsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_DnssecConfig_DefaultKeySpecsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `algorithm` after provisioning.\nString mnemonic specifying the DNSSEC algorithm of this key Possible values: [\"ecdsap256sha256\", \"ecdsap384sha384\", \"rsasha1\", \"rsasha256\", \"rsasha512\"]"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `key_length` after provisioning.\nLength of the keys in bits"]
    pub fn key_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_type` after provisioning.\nSpecifies whether this is a key signing key (KSK) or a zone\nsigning key (ZSK). Key signing keys have the Secure Entry\nPoint flag set and, when active, will only be used to sign\nresource record sets of type DNSKEY. Zone signing keys do\nnot have the Secure Entry Point flag set and will be used\nto sign all other types of resource record sets. Possible values: [\"keySigning\", \"zoneSigning\"]"]
    pub fn key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nIdentifies what kind of resource this is"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsManagedZone_DnssecConfigDynamic {
    default_key_specs: std::option::Option<terrars::DynamicBlock<DnsManagedZone_DnssecConfig_DefaultKeySpecs>>,
}

#[derive(Serialize)]
pub struct DnsManagedZone_DnssecConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    non_existence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_key_specs: std::option::Option<std::vec::Vec<DnsManagedZone_DnssecConfig_DefaultKeySpecs>>,
    dynamic: DnsManagedZone_DnssecConfigDynamic,
}

impl DnsManagedZone_DnssecConfig {
    #[doc = "Set the field `kind`.\nIdentifies what kind of resource this is"]
    pub fn set_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kind = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `non_existence`.\nSpecifies the mechanism used to provide authenticated denial-of-existence responses.\nnon_existence can only be updated when the state is 'off'. Possible values: [\"nsec\", \"nsec3\"]"]
    pub fn set_non_existence(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.non_existence = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `state`.\nSpecifies whether DNSSEC is enabled, and what mode it is in Possible values: [\"off\", \"on\", \"transfer\"]"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_key_specs`.\n"]
    pub fn set_default_key_specs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_DnssecConfig_DefaultKeySpecs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_key_specs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_key_specs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsManagedZone_DnssecConfig {
    type O = terrars::BlockAssignable<DnsManagedZone_DnssecConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_DnssecConfig {}

impl BuildDnsManagedZone_DnssecConfig {
    pub fn build(self) -> DnsManagedZone_DnssecConfig {
        DnsManagedZone_DnssecConfig {
            kind: core::default::Default::default(),
            non_existence: core::default::Default::default(),
            state: core::default::Default::default(),
            default_key_specs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_DnssecConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_DnssecConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsManagedZone_DnssecConfigRef {
        DnsManagedZone_DnssecConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_DnssecConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nIdentifies what kind of resource this is"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }

    #[doc =
        "Get a reference to the value of field `non_existence` after provisioning.\nSpecifies the mechanism used to provide authenticated denial-of-existence responses.\nnon_existence can only be updated when the state is 'off'. Possible values: [\"nsec\", \"nsec3\"]"]
    pub fn non_existence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.non_existence", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nSpecifies whether DNSSEC is enabled, and what mode it is in Possible values: [\"off\", \"on\", \"transfer\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `default_key_specs` after provisioning.\n"]
    pub fn default_key_specs(&self) -> terrars::ListRef<DnsManagedZone_DnssecConfig_DefaultKeySpecsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_key_specs", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsManagedZone_ForwardingConfig_OutboundEndpoints {
    name: terrars::PrimField<std::string::String>,
}

impl DnsManagedZone_ForwardingConfig_OutboundEndpoints { }

impl terrars::ToListMappable for DnsManagedZone_ForwardingConfig_OutboundEndpoints {
    type O = terrars::BlockAssignable<DnsManagedZone_ForwardingConfig_OutboundEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_ForwardingConfig_OutboundEndpoints {
    #[doc =
        "Specified as a resource path including project and region.\nThis should be formatted like\n'projects/{project}/locations/{location}/outboundEndpoints/{endpoint}'"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDnsManagedZone_ForwardingConfig_OutboundEndpoints {
    pub fn build(self) -> DnsManagedZone_ForwardingConfig_OutboundEndpoints {
        DnsManagedZone_ForwardingConfig_OutboundEndpoints { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_ForwardingConfig_OutboundEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_ForwardingConfig_OutboundEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsManagedZone_ForwardingConfig_OutboundEndpointsRef {
        DnsManagedZone_ForwardingConfig_OutboundEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_ForwardingConfig_OutboundEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nSpecified as a resource path including project and region.\nThis should be formatted like\n'projects/{project}/locations/{location}/outboundEndpoints/{endpoint}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsManagedZone_ForwardingConfig_TargetNameServers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsManagedZone_ForwardingConfig_TargetNameServers {
    #[doc = "Set the field `domain_name`.\nFully qualified domain name for the forwarding target."]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `forwarding_path`.\nForwarding path for this TargetNameServer. If unset or 'default'\nCloud DNS will make forwarding decision based on address ranges,\ni.e. RFC1918 addresses go to the VPC, Non-RFC1918 addresses go\nto the Internet. When set to 'private', Cloud DNS will always\nsend queries through VPC for this target. Possible values: [\"default\", \"private\"]"]
    pub fn set_forwarding_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forwarding_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv4_address`.\nIPv4 address of a target name server.\nDoes not accept both fields (ipv4 & ipv6) being populated."]
    pub fn set_ipv4_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4_address = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv6_address`.\nIPv6 address of a target name server.\nDoes not accept both fields (ipv4 & ipv6) being populated."]
    pub fn set_ipv6_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsManagedZone_ForwardingConfig_TargetNameServers {
    type O = terrars::BlockAssignable<DnsManagedZone_ForwardingConfig_TargetNameServers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_ForwardingConfig_TargetNameServers {}

impl BuildDnsManagedZone_ForwardingConfig_TargetNameServers {
    pub fn build(self) -> DnsManagedZone_ForwardingConfig_TargetNameServers {
        DnsManagedZone_ForwardingConfig_TargetNameServers {
            domain_name: core::default::Default::default(),
            forwarding_path: core::default::Default::default(),
            ipv4_address: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_ForwardingConfig_TargetNameServersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_ForwardingConfig_TargetNameServersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsManagedZone_ForwardingConfig_TargetNameServersRef {
        DnsManagedZone_ForwardingConfig_TargetNameServersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_ForwardingConfig_TargetNameServersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domain_name` after provisioning.\nFully qualified domain name for the forwarding target."]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_path` after provisioning.\nForwarding path for this TargetNameServer. If unset or 'default'\nCloud DNS will make forwarding decision based on address ranges,\ni.e. RFC1918 addresses go to the VPC, Non-RFC1918 addresses go\nto the Internet. When set to 'private', Cloud DNS will always\nsend queries through VPC for this target. Possible values: [\"default\", \"private\"]"]
    pub fn forwarding_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_address` after provisioning.\nIPv4 address of a target name server.\nDoes not accept both fields (ipv4 & ipv6) being populated."]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_address` after provisioning.\nIPv6 address of a target name server.\nDoes not accept both fields (ipv4 & ipv6) being populated."]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsManagedZone_ForwardingConfigDynamic {
    outbound_endpoints: std::option::Option<terrars::DynamicBlock<DnsManagedZone_ForwardingConfig_OutboundEndpoints>>,
    target_name_servers: std
    ::option
    ::Option<terrars::DynamicBlock<DnsManagedZone_ForwardingConfig_TargetNameServers>>,
}

#[derive(Serialize)]
pub struct DnsManagedZone_ForwardingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound_endpoints: std::option::Option<std::vec::Vec<DnsManagedZone_ForwardingConfig_OutboundEndpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_name_servers: std::option::Option<std::vec::Vec<DnsManagedZone_ForwardingConfig_TargetNameServers>>,
    dynamic: DnsManagedZone_ForwardingConfigDynamic,
}

impl DnsManagedZone_ForwardingConfig {
    #[doc = "Set the field `outbound_endpoints`.\n"]
    pub fn set_outbound_endpoints(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_ForwardingConfig_OutboundEndpoints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.outbound_endpoints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.outbound_endpoints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_name_servers`.\n"]
    pub fn set_target_name_servers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_ForwardingConfig_TargetNameServers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_name_servers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_name_servers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsManagedZone_ForwardingConfig {
    type O = terrars::BlockAssignable<DnsManagedZone_ForwardingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_ForwardingConfig {}

impl BuildDnsManagedZone_ForwardingConfig {
    pub fn build(self) -> DnsManagedZone_ForwardingConfig {
        DnsManagedZone_ForwardingConfig {
            outbound_endpoints: core::default::Default::default(),
            target_name_servers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_ForwardingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_ForwardingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsManagedZone_ForwardingConfigRef {
        DnsManagedZone_ForwardingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_ForwardingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `outbound_endpoints` after provisioning.\n"]
    pub fn outbound_endpoints(&self) -> terrars::ListRef<DnsManagedZone_ForwardingConfig_OutboundEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_endpoints", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsManagedZone_PeeringConfig_TargetNetwork {
    network_url: terrars::PrimField<std::string::String>,
}

impl DnsManagedZone_PeeringConfig_TargetNetwork { }

impl terrars::ToListMappable for DnsManagedZone_PeeringConfig_TargetNetwork {
    type O = terrars::BlockAssignable<DnsManagedZone_PeeringConfig_TargetNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_PeeringConfig_TargetNetwork {
    #[doc =
        "The id or fully qualified URL of the VPC network to forward queries to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub network_url: terrars::PrimField<std::string::String>,
}

impl BuildDnsManagedZone_PeeringConfig_TargetNetwork {
    pub fn build(self) -> DnsManagedZone_PeeringConfig_TargetNetwork {
        DnsManagedZone_PeeringConfig_TargetNetwork { network_url: self.network_url }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_PeeringConfig_TargetNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_PeeringConfig_TargetNetworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsManagedZone_PeeringConfig_TargetNetworkRef {
        DnsManagedZone_PeeringConfig_TargetNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_PeeringConfig_TargetNetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_url` after provisioning.\nThe id or fully qualified URL of the VPC network to forward queries to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub fn network_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsManagedZone_PeeringConfigDynamic {
    target_network: std::option::Option<terrars::DynamicBlock<DnsManagedZone_PeeringConfig_TargetNetwork>>,
}

#[derive(Serialize)]
pub struct DnsManagedZone_PeeringConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_network: std::option::Option<std::vec::Vec<DnsManagedZone_PeeringConfig_TargetNetwork>>,
    dynamic: DnsManagedZone_PeeringConfigDynamic,
}

impl DnsManagedZone_PeeringConfig {
    #[doc = "Set the field `target_network`.\n"]
    pub fn set_target_network(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_PeeringConfig_TargetNetwork>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_network = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsManagedZone_PeeringConfig {
    type O = terrars::BlockAssignable<DnsManagedZone_PeeringConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_PeeringConfig {}

impl BuildDnsManagedZone_PeeringConfig {
    pub fn build(self) -> DnsManagedZone_PeeringConfig {
        DnsManagedZone_PeeringConfig {
            target_network: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_PeeringConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_PeeringConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsManagedZone_PeeringConfigRef {
        DnsManagedZone_PeeringConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_PeeringConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_network` after provisioning.\n"]
    pub fn target_network(&self) -> terrars::ListRef<DnsManagedZone_PeeringConfig_TargetNetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_network", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsManagedZone_PrivateVisibilityConfig_GkeClusters {
    gke_cluster_name: terrars::PrimField<std::string::String>,
}

impl DnsManagedZone_PrivateVisibilityConfig_GkeClusters { }

impl terrars::ToListMappable for DnsManagedZone_PrivateVisibilityConfig_GkeClusters {
    type O = terrars::BlockAssignable<DnsManagedZone_PrivateVisibilityConfig_GkeClusters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_PrivateVisibilityConfig_GkeClusters {
    #[doc =
        "The resource name of the cluster to bind this ManagedZone to.\nThis should be specified in the format like\n'projects/*/locations/*/clusters/*'"]
    pub gke_cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildDnsManagedZone_PrivateVisibilityConfig_GkeClusters {
    pub fn build(self) -> DnsManagedZone_PrivateVisibilityConfig_GkeClusters {
        DnsManagedZone_PrivateVisibilityConfig_GkeClusters { gke_cluster_name: self.gke_cluster_name }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_PrivateVisibilityConfig_GkeClustersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_PrivateVisibilityConfig_GkeClustersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsManagedZone_PrivateVisibilityConfig_GkeClustersRef {
        DnsManagedZone_PrivateVisibilityConfig_GkeClustersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_PrivateVisibilityConfig_GkeClustersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gke_cluster_name` after provisioning.\nThe resource name of the cluster to bind this ManagedZone to.\nThis should be specified in the format like\n'projects/*/locations/*/clusters/*'"]
    pub fn gke_cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_cluster_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsManagedZone_PrivateVisibilityConfig_Networks {
    network_url: terrars::PrimField<std::string::String>,
}

impl DnsManagedZone_PrivateVisibilityConfig_Networks { }

impl terrars::ToListMappable for DnsManagedZone_PrivateVisibilityConfig_Networks {
    type O = terrars::BlockAssignable<DnsManagedZone_PrivateVisibilityConfig_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_PrivateVisibilityConfig_Networks {
    #[doc =
        "The id or fully qualified URL of the VPC network to bind to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub network_url: terrars::PrimField<std::string::String>,
}

impl BuildDnsManagedZone_PrivateVisibilityConfig_Networks {
    pub fn build(self) -> DnsManagedZone_PrivateVisibilityConfig_Networks {
        DnsManagedZone_PrivateVisibilityConfig_Networks { network_url: self.network_url }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_PrivateVisibilityConfig_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_PrivateVisibilityConfig_NetworksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsManagedZone_PrivateVisibilityConfig_NetworksRef {
        DnsManagedZone_PrivateVisibilityConfig_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_PrivateVisibilityConfig_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_url` after provisioning.\nThe id or fully qualified URL of the VPC network to bind to.\nThis should be formatted like 'projects/{project}/global/networks/{network}' or\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub fn network_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsManagedZone_PrivateVisibilityConfigDynamic {
    gke_clusters: std::option::Option<terrars::DynamicBlock<DnsManagedZone_PrivateVisibilityConfig_GkeClusters>>,
    networks: std::option::Option<terrars::DynamicBlock<DnsManagedZone_PrivateVisibilityConfig_Networks>>,
}

#[derive(Serialize)]
pub struct DnsManagedZone_PrivateVisibilityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_clusters: std::option::Option<std::vec::Vec<DnsManagedZone_PrivateVisibilityConfig_GkeClusters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks: std::option::Option<std::vec::Vec<DnsManagedZone_PrivateVisibilityConfig_Networks>>,
    dynamic: DnsManagedZone_PrivateVisibilityConfigDynamic,
}

impl DnsManagedZone_PrivateVisibilityConfig {
    #[doc = "Set the field `gke_clusters`.\n"]
    pub fn set_gke_clusters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_PrivateVisibilityConfig_GkeClusters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gke_clusters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gke_clusters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `networks`.\n"]
    pub fn set_networks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsManagedZone_PrivateVisibilityConfig_Networks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.networks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.networks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsManagedZone_PrivateVisibilityConfig {
    type O = terrars::BlockAssignable<DnsManagedZone_PrivateVisibilityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_PrivateVisibilityConfig {}

impl BuildDnsManagedZone_PrivateVisibilityConfig {
    pub fn build(self) -> DnsManagedZone_PrivateVisibilityConfig {
        DnsManagedZone_PrivateVisibilityConfig {
            gke_clusters: core::default::Default::default(),
            networks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_PrivateVisibilityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_PrivateVisibilityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsManagedZone_PrivateVisibilityConfigRef {
        DnsManagedZone_PrivateVisibilityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_PrivateVisibilityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gke_clusters` after provisioning.\n"]
    pub fn gke_clusters(&self) -> terrars::ListRef<DnsManagedZone_PrivateVisibilityConfig_GkeClustersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_clusters", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsManagedZone_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsManagedZone_Timeouts {
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

impl terrars::ToListMappable for DnsManagedZone_Timeouts {
    type O = terrars::BlockAssignable<DnsManagedZone_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsManagedZone_Timeouts {}

impl BuildDnsManagedZone_Timeouts {
    pub fn build(self) -> DnsManagedZone_Timeouts {
        DnsManagedZone_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsManagedZone_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsManagedZone_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsManagedZone_TimeoutsRef {
        DnsManagedZone_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsManagedZone_TimeoutsRef {
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
