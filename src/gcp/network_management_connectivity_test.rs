#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkManagementConnectivityTestData_ {
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
    bypass_firewall_checks: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_projects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    round_trip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<NetworkManagementConnectivityTest_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<NetworkManagementConnectivityTest_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkManagementConnectivityTest_Timeouts>,
    dynamic: NetworkManagementConnectivityTestDynamic,
}

struct NetworkManagementConnectivityTest_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkManagementConnectivityTestData_>,
}

#[derive(Clone)]
pub struct NetworkManagementConnectivityTest(std::rc::Rc<NetworkManagementConnectivityTest_>);

impl NetworkManagementConnectivityTest {
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
        "Set the field `bypass_firewall_checks`.\nWhether the analysis should skip firewall checking. Default value is false."]
    pub fn set_bypass_firewall_checks(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bypass_firewall_checks = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nThe user-supplied description of the Connectivity Test.\nMaximum of 512 characters."]
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
        "Set the field `labels`.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `protocol`.\nIP Protocol of the test. When not provided, \"TCP\" is assumed."]
    pub fn set_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `related_projects`.\nOther projects that may be relevant for reachability analysis.\nThis is applicable to scenarios where a test can cross project\nboundaries."]
    pub fn set_related_projects(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().related_projects = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `round_trip`.\nWhether run analysis for the return path from destination to source.\nDefault value is false."]
    pub fn set_round_trip(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().round_trip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkManagementConnectivityTest_Destination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkManagementConnectivityTest_Source>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkManagementConnectivityTest_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `bypass_firewall_checks` after provisioning.\nWhether the analysis should skip firewall checking. Default value is false."]
    pub fn bypass_firewall_checks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bypass_firewall_checks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe user-supplied description of the Connectivity Test.\nMaximum of 512 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the connectivity test."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nIP Protocol of the test. When not provided, \"TCP\" is assumed."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `related_projects` after provisioning.\nOther projects that may be relevant for reachability analysis.\nThis is applicable to scenarios where a test can cross project\nboundaries."]
    pub fn related_projects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.related_projects", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `round_trip` after provisioning.\nWhether run analysis for the return path from destination to source.\nDefault value is false."]
    pub fn round_trip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.round_trip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<NetworkManagementConnectivityTest_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<NetworkManagementConnectivityTest_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkManagementConnectivityTest_TimeoutsRef {
        NetworkManagementConnectivityTest_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkManagementConnectivityTest {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkManagementConnectivityTest { }

impl terrars::ToListMappable for NetworkManagementConnectivityTest {
    type O = terrars::ListRef<NetworkManagementConnectivityTestRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkManagementConnectivityTest_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_management_connectivity_test".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkManagementConnectivityTest {
    pub tf_id: std::string::String,
    #[doc = "Unique name for the connectivity test."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildNetworkManagementConnectivityTest {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkManagementConnectivityTest {
        let out = NetworkManagementConnectivityTest(std::rc::Rc::new(NetworkManagementConnectivityTest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkManagementConnectivityTestData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bypass_firewall_checks: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                protocol: core::default::Default::default(),
                related_projects: core::default::Default::default(),
                round_trip: core::default::Default::default(),
                destination: core::default::Default::default(),
                source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkManagementConnectivityTestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkManagementConnectivityTestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkManagementConnectivityTestRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bypass_firewall_checks` after provisioning.\nWhether the analysis should skip firewall checking. Default value is false."]
    pub fn bypass_firewall_checks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bypass_firewall_checks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe user-supplied description of the Connectivity Test.\nMaximum of 512 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the connectivity test."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nIP Protocol of the test. When not provided, \"TCP\" is assumed."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `related_projects` after provisioning.\nOther projects that may be relevant for reachability analysis.\nThis is applicable to scenarios where a test can cross project\nboundaries."]
    pub fn related_projects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.related_projects", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `round_trip` after provisioning.\nWhether run analysis for the return path from destination to source.\nDefault value is false."]
    pub fn round_trip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.round_trip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<NetworkManagementConnectivityTest_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<NetworkManagementConnectivityTest_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkManagementConnectivityTest_TimeoutsRef {
        NetworkManagementConnectivityTest_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct NetworkManagementConnectivityTestDynamic {
    destination: std::option::Option<terrars::DynamicBlock<NetworkManagementConnectivityTest_Destination>>,
    source: std::option::Option<terrars::DynamicBlock<NetworkManagementConnectivityTest_Source>>,
}

#[derive(Serialize)]
pub struct NetworkManagementConnectivityTest_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_master_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_pod: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redis_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redis_instance: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkManagementConnectivityTest_Destination {
    #[doc = "Set the field `cloud_sql_instance`.\nA Cloud SQL instance URI."]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_sql_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `forwarding_rule`.\nForwarding rule URI. Forwarding rules are frontends for load balancers,\nPSC endpoints, and Protocol Forwarding."]
    pub fn set_forwarding_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.forwarding_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fqdn`.\nA DNS endpoint of Google Kubernetes Engine cluster control plane.\nRequires gke_master_cluster to be set, can't be used simultaneoulsly with\nip_address or network. Applicable only to destination endpoint."]
    pub fn set_fqdn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fqdn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_master_cluster`.\nA cluster URI for Google Kubernetes Engine cluster control plane."]
    pub fn set_gke_master_cluster(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gke_master_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_pod`.\nA [GKE Pod](https://cloud.google.com/kubernetes-engine/docs/concepts/pod) URI."]
    pub fn set_gke_pod(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gke_pod = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance`.\nA Compute Engine instance URI."]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\nThe IP address of the endpoint, which can be an external or internal IP."]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\nA VPC network URI."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_type`.\nFor source endpoints, type of the network where the endpoint is located. Not relevant for destination endpoints. Possible values: [\"GCP_NETWORK\", \"NON_GCP_NETWORK\", \"INTERNET\"]"]
    pub fn set_network_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nThe IP protocol port of the endpoint. Only applicable when protocol is\nTCP or UDP."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_id`.\nProject ID where the endpoint is located.\nThe project ID can be derived from the URI if you provide a endpoint or\nnetwork URI.\nThe following are two cases where you may need to provide the project ID:\n1. Only the IP address is specified, and the IP address is within a Google\nCloud project.\n2. When you are using Shared VPC and the IP address that you provide is\nfrom the service project. In this case, the network that the IP address\nresides in is defined in the host project."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redis_cluster`.\nA Redis Cluster URI."]
    pub fn set_redis_cluster(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.redis_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redis_instance`.\nA Redis Instance URI."]
    pub fn set_redis_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.redis_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkManagementConnectivityTest_Destination {
    type O = terrars::BlockAssignable<NetworkManagementConnectivityTest_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkManagementConnectivityTest_Destination {}

impl BuildNetworkManagementConnectivityTest_Destination {
    pub fn build(self) -> NetworkManagementConnectivityTest_Destination {
        NetworkManagementConnectivityTest_Destination {
            cloud_sql_instance: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            fqdn: core::default::Default::default(),
            gke_master_cluster: core::default::Default::default(),
            gke_pod: core::default::Default::default(),
            instance: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_type: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            redis_cluster: core::default::Default::default(),
            redis_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkManagementConnectivityTest_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkManagementConnectivityTest_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkManagementConnectivityTest_DestinationRef {
        NetworkManagementConnectivityTest_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkManagementConnectivityTest_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\nA Cloud SQL instance URI."]
    pub fn cloud_sql_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_instance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rule` after provisioning.\nForwarding rule URI. Forwarding rules are frontends for load balancers,\nPSC endpoints, and Protocol Forwarding."]
    pub fn forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.forwarding_rule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fqdn` after provisioning.\nA DNS endpoint of Google Kubernetes Engine cluster control plane.\nRequires gke_master_cluster to be set, can't be used simultaneoulsly with\nip_address or network. Applicable only to destination endpoint."]
    pub fn fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gke_master_cluster` after provisioning.\nA cluster URI for Google Kubernetes Engine cluster control plane."]
    pub fn gke_master_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_master_cluster", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gke_pod` after provisioning.\nA [GKE Pod](https://cloud.google.com/kubernetes-engine/docs/concepts/pod) URI."]
    pub fn gke_pod(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_pod", self.base))
    }

    #[doc = "Get a reference to the value of field `instance` after provisioning.\nA Compute Engine instance URI."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the endpoint, which can be an external or internal IP."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\nA VPC network URI."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_type` after provisioning.\nFor source endpoints, type of the network where the endpoint is located. Not relevant for destination endpoints. Possible values: [\"GCP_NETWORK\", \"NON_GCP_NETWORK\", \"INTERNET\"]"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe IP protocol port of the endpoint. Only applicable when protocol is\nTCP or UDP."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nProject ID where the endpoint is located.\nThe project ID can be derived from the URI if you provide a endpoint or\nnetwork URI.\nThe following are two cases where you may need to provide the project ID:\n1. Only the IP address is specified, and the IP address is within a Google\nCloud project.\n2. When you are using Shared VPC and the IP address that you provide is\nfrom the service project. In this case, the network that the IP address\nresides in is defined in the host project."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `redis_cluster` after provisioning.\nA Redis Cluster URI."]
    pub fn redis_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redis_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `redis_instance` after provisioning.\nA Redis Instance URI."]
    pub fn redis_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redis_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkManagementConnectivityTest_Source_AppEngineVersion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkManagementConnectivityTest_Source_AppEngineVersion {
    #[doc = "Set the field `uri`.\nAn App Engine service version name."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkManagementConnectivityTest_Source_AppEngineVersion {
    type O = terrars::BlockAssignable<NetworkManagementConnectivityTest_Source_AppEngineVersion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkManagementConnectivityTest_Source_AppEngineVersion {}

impl BuildNetworkManagementConnectivityTest_Source_AppEngineVersion {
    pub fn build(self) -> NetworkManagementConnectivityTest_Source_AppEngineVersion {
        NetworkManagementConnectivityTest_Source_AppEngineVersion { uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NetworkManagementConnectivityTest_Source_AppEngineVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkManagementConnectivityTest_Source_AppEngineVersionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkManagementConnectivityTest_Source_AppEngineVersionRef {
        NetworkManagementConnectivityTest_Source_AppEngineVersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkManagementConnectivityTest_Source_AppEngineVersionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nAn App Engine service version name."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkManagementConnectivityTest_Source_CloudFunction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkManagementConnectivityTest_Source_CloudFunction {
    #[doc = "Set the field `uri`.\nA Cloud Function name."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkManagementConnectivityTest_Source_CloudFunction {
    type O = terrars::BlockAssignable<NetworkManagementConnectivityTest_Source_CloudFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkManagementConnectivityTest_Source_CloudFunction {}

impl BuildNetworkManagementConnectivityTest_Source_CloudFunction {
    pub fn build(self) -> NetworkManagementConnectivityTest_Source_CloudFunction {
        NetworkManagementConnectivityTest_Source_CloudFunction { uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NetworkManagementConnectivityTest_Source_CloudFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkManagementConnectivityTest_Source_CloudFunctionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkManagementConnectivityTest_Source_CloudFunctionRef {
        NetworkManagementConnectivityTest_Source_CloudFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkManagementConnectivityTest_Source_CloudFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nA Cloud Function name."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkManagementConnectivityTest_Source_CloudRunRevision {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkManagementConnectivityTest_Source_CloudRunRevision {
    #[doc = "Set the field `uri`.\nA Cloud Run revision URI."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkManagementConnectivityTest_Source_CloudRunRevision {
    type O = terrars::BlockAssignable<NetworkManagementConnectivityTest_Source_CloudRunRevision>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkManagementConnectivityTest_Source_CloudRunRevision {}

impl BuildNetworkManagementConnectivityTest_Source_CloudRunRevision {
    pub fn build(self) -> NetworkManagementConnectivityTest_Source_CloudRunRevision {
        NetworkManagementConnectivityTest_Source_CloudRunRevision { uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NetworkManagementConnectivityTest_Source_CloudRunRevisionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkManagementConnectivityTest_Source_CloudRunRevisionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkManagementConnectivityTest_Source_CloudRunRevisionRef {
        NetworkManagementConnectivityTest_Source_CloudRunRevisionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkManagementConnectivityTest_Source_CloudRunRevisionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nA Cloud Run revision URI."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkManagementConnectivityTest_SourceDynamic {
    app_engine_version: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkManagementConnectivityTest_Source_AppEngineVersion>>,
    cloud_function: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkManagementConnectivityTest_Source_CloudFunction>>,
    cloud_run_revision: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkManagementConnectivityTest_Source_CloudRunRevision>>,
}

#[derive(Serialize)]
pub struct NetworkManagementConnectivityTest_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_master_cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_engine_version: std::option::Option<std::vec::Vec<NetworkManagementConnectivityTest_Source_AppEngineVersion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_function: std::option::Option<std::vec::Vec<NetworkManagementConnectivityTest_Source_CloudFunction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_run_revision: std::option::Option<std::vec::Vec<NetworkManagementConnectivityTest_Source_CloudRunRevision>>,
    dynamic: NetworkManagementConnectivityTest_SourceDynamic,
}

impl NetworkManagementConnectivityTest_Source {
    #[doc = "Set the field `cloud_sql_instance`.\nA Cloud SQL instance URI."]
    pub fn set_cloud_sql_instance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_sql_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_master_cluster`.\nA cluster URI for Google Kubernetes Engine cluster control plane."]
    pub fn set_gke_master_cluster(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gke_master_cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance`.\nA Compute Engine instance URI."]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_address`.\nThe IP address of the endpoint, which can be an external or internal IP."]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\nA VPC network URI."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_type`.\nType of the network where the endpoint is located. Possible values: [\"GCP_NETWORK\", \"NON_GCP_NETWORK\"]"]
    pub fn set_network_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nThe IP protocol port of the endpoint. Only applicable when protocol is\nTCP or UDP."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_id`.\nProject ID where the endpoint is located.\nThe project ID can be derived from the URI if you provide a endpoint or\nnetwork URI.\nThe following are two cases where you may need to provide the project ID:\n1. Only the IP address is specified, and the IP address is within a Google\nCloud project.\n2. When you are using Shared VPC and the IP address that you provide is\nfrom the service project. In this case, the network that the IP address\nresides in is defined in the host project."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_engine_version`.\n"]
    pub fn set_app_engine_version(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkManagementConnectivityTest_Source_AppEngineVersion>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.app_engine_version = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.app_engine_version = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_function`.\n"]
    pub fn set_cloud_function(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkManagementConnectivityTest_Source_CloudFunction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_function = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_function = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_run_revision`.\n"]
    pub fn set_cloud_run_revision(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkManagementConnectivityTest_Source_CloudRunRevision>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_run_revision = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_run_revision = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkManagementConnectivityTest_Source {
    type O = terrars::BlockAssignable<NetworkManagementConnectivityTest_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkManagementConnectivityTest_Source {}

impl BuildNetworkManagementConnectivityTest_Source {
    pub fn build(self) -> NetworkManagementConnectivityTest_Source {
        NetworkManagementConnectivityTest_Source {
            cloud_sql_instance: core::default::Default::default(),
            gke_master_cluster: core::default::Default::default(),
            instance: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_type: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            app_engine_version: core::default::Default::default(),
            cloud_function: core::default::Default::default(),
            cloud_run_revision: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkManagementConnectivityTest_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkManagementConnectivityTest_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkManagementConnectivityTest_SourceRef {
        NetworkManagementConnectivityTest_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkManagementConnectivityTest_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\nA Cloud SQL instance URI."]
    pub fn cloud_sql_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_sql_instance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gke_master_cluster` after provisioning.\nA cluster URI for Google Kubernetes Engine cluster control plane."]
    pub fn gke_master_cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_master_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `instance` after provisioning.\nA Compute Engine instance URI."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the endpoint, which can be an external or internal IP."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\nA VPC network URI."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_type` after provisioning.\nType of the network where the endpoint is located. Possible values: [\"GCP_NETWORK\", \"NON_GCP_NETWORK\"]"]
    pub fn network_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe IP protocol port of the endpoint. Only applicable when protocol is\nTCP or UDP."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nProject ID where the endpoint is located.\nThe project ID can be derived from the URI if you provide a endpoint or\nnetwork URI.\nThe following are two cases where you may need to provide the project ID:\n1. Only the IP address is specified, and the IP address is within a Google\nCloud project.\n2. When you are using Shared VPC and the IP address that you provide is\nfrom the service project. In this case, the network that the IP address\nresides in is defined in the host project."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `app_engine_version` after provisioning.\n"]
    pub fn app_engine_version(&self) -> terrars::ListRef<NetworkManagementConnectivityTest_Source_AppEngineVersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine_version", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\n"]
    pub fn cloud_function(&self) -> terrars::ListRef<NetworkManagementConnectivityTest_Source_CloudFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_function", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_run_revision` after provisioning.\n"]
    pub fn cloud_run_revision(&self) -> terrars::ListRef<NetworkManagementConnectivityTest_Source_CloudRunRevisionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_run_revision", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkManagementConnectivityTest_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkManagementConnectivityTest_Timeouts {
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

impl terrars::ToListMappable for NetworkManagementConnectivityTest_Timeouts {
    type O = terrars::BlockAssignable<NetworkManagementConnectivityTest_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkManagementConnectivityTest_Timeouts {}

impl BuildNetworkManagementConnectivityTest_Timeouts {
    pub fn build(self) -> NetworkManagementConnectivityTest_Timeouts {
        NetworkManagementConnectivityTest_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkManagementConnectivityTest_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkManagementConnectivityTest_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkManagementConnectivityTest_TimeoutsRef {
        NetworkManagementConnectivityTest_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkManagementConnectivityTest_TimeoutsRef {
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
