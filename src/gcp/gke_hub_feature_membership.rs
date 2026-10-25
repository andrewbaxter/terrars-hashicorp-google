#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeHubFeatureMembershipData_ {
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
    feature: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    membership: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    membership_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configmanagement: std::option::Option<std::vec::Vec<GkeHubFeatureMembership_Configmanagement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh: std::option::Option<std::vec::Vec<GkeHubFeatureMembership_Mesh>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policycontroller: std::option::Option<std::vec::Vec<GkeHubFeatureMembership_Policycontroller>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeHubFeatureMembership_Timeouts>,
    dynamic: GkeHubFeatureMembershipDynamic,
}

struct GkeHubFeatureMembership_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeHubFeatureMembershipData_>,
}

#[derive(Clone)]
pub struct GkeHubFeatureMembership(std::rc::Rc<GkeHubFeatureMembership_>);

impl GkeHubFeatureMembership {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `membership_location`.\nThe location of the membership"]
    pub fn set_membership_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().membership_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project of the feature"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configmanagement`.\n"]
    pub fn set_configmanagement(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configmanagement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configmanagement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mesh`.\n"]
    pub fn set_mesh(self, v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureMembership_Mesh>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mesh = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mesh = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policycontroller`.\n"]
    pub fn set_policycontroller(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureMembership_Policycontroller>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policycontroller = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policycontroller = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeHubFeatureMembership_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature` after provisioning.\nThe name of the feature"]
    pub fn feature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the feature"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `membership` after provisioning.\nThe name of the membership"]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `membership_location` after provisioning.\nThe location of the membership"]
    pub fn membership_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project of the feature"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configmanagement` after provisioning.\n"]
    pub fn configmanagement(&self) -> terrars::ListRef<GkeHubFeatureMembership_ConfigmanagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configmanagement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh` after provisioning.\n"]
    pub fn mesh(&self) -> terrars::ListRef<GkeHubFeatureMembership_MeshRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policycontroller` after provisioning.\n"]
    pub fn policycontroller(&self) -> terrars::ListRef<GkeHubFeatureMembership_PolicycontrollerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policycontroller", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFeatureMembership_TimeoutsRef {
        GkeHubFeatureMembership_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GkeHubFeatureMembership {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeHubFeatureMembership { }

impl terrars::ToListMappable for GkeHubFeatureMembership {
    type O = terrars::ListRef<GkeHubFeatureMembershipRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeHubFeatureMembership_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gke_hub_feature_membership".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeHubFeatureMembership {
    pub tf_id: std::string::String,
    #[doc = "The name of the feature"]
    pub feature: terrars::PrimField<std::string::String>,
    #[doc = "The location of the feature"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The name of the membership"]
    pub membership: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeatureMembership {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeHubFeatureMembership {
        let out = GkeHubFeatureMembership(std::rc::Rc::new(GkeHubFeatureMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeHubFeatureMembershipData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                feature: self.feature,
                id: core::default::Default::default(),
                location: self.location,
                membership: self.membership,
                membership_location: core::default::Default::default(),
                project: core::default::Default::default(),
                configmanagement: core::default::Default::default(),
                mesh: core::default::Default::default(),
                policycontroller: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeHubFeatureMembershipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeHubFeatureMembershipRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature` after provisioning.\nThe name of the feature"]
    pub fn feature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the feature"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `membership` after provisioning.\nThe name of the membership"]
    pub fn membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `membership_location` after provisioning.\nThe location of the membership"]
    pub fn membership_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project of the feature"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configmanagement` after provisioning.\n"]
    pub fn configmanagement(&self) -> terrars::ListRef<GkeHubFeatureMembership_ConfigmanagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configmanagement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh` after provisioning.\n"]
    pub fn mesh(&self) -> terrars::ListRef<GkeHubFeatureMembership_MeshRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policycontroller` after provisioning.\n"]
    pub fn policycontroller(&self) -> terrars::ListRef<GkeHubFeatureMembership_PolicycontrollerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policycontroller", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFeatureMembership_TimeoutsRef {
        GkeHubFeatureMembership_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembershipDynamic {
    configmanagement: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Configmanagement>>,
    mesh: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Mesh>>,
    policycontroller: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Policycontroller>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_limit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_request: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_limit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_request: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers {
    #[doc = "Set the field `container_name`.\nThe name of the container."]
    pub fn set_container_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_limit`.\nThe CPU limit of the container."]
    pub fn set_cpu_limit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_request`.\nThe CPU request of the container."]
    pub fn set_cpu_request(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_request = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_limit`.\nThe memory limit of the container."]
    pub fn set_memory_limit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_request`.\nThe memory request of the container."]
    pub fn set_memory_request(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory_request = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers {
    type O =
        terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers {}

impl BuildGkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers {
    pub fn build(self) -> GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers {
        GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers {
            container_name: core::default::Default::default(),
            cpu_limit: core::default::Default::default(),
            cpu_request: core::default::Default::default(),
            memory_limit: core::default::Default::default(),
            memory_request: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_ContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_ContainersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_ContainersRef {
        GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_ContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_ContainersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\nThe name of the container."]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_limit` after provisioning.\nThe CPU limit of the container."]
    pub fn cpu_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_request` after provisioning.\nThe CPU request of the container."]
    pub fn cpu_request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_request", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_limit` after provisioning.\nThe memory limit of the container."]
    pub fn memory_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_request` after provisioning.\nThe memory request of the container."]
    pub fn memory_request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_request", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverridesDynamic {
    containers: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers>,
    >,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers>>,
    dynamic: GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverridesDynamic,
}

impl GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides {
    #[doc = "Set the field `deployment_name`.\nThe name of the Deployment."]
    pub fn set_deployment_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployment_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_namespace`.\nThe namespace of the Deployment."]
    pub fn set_deployment_namespace(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deployment_namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_Containers,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.containers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.containers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides {}

impl BuildGkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides {
    pub fn build(self) -> GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides {
        GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides {
            deployment_name: core::default::Default::default(),
            deployment_namespace: core::default::Default::default(),
            containers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverridesRef {
        GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_name` after provisioning.\nThe name of the Deployment."]
    pub fn deployment_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deployment_namespace` after provisioning.\nThe namespace of the Deployment."]
    pub fn deployment_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Configmanagement_ConfigSync_Git {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_proxy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_repo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_rev: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_wait_secs: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembership_Configmanagement_ConfigSync_Git {
    #[doc =
        "Set the field `gcp_service_account_email`.\nThe GCP Service Account Email used for auth when secretType is gcpServiceAccount."]
    pub fn set_gcp_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `https_proxy`.\nURL for the HTTPS proxy to be used when communicating with the Git repo."]
    pub fn set_https_proxy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.https_proxy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `policy_dir`.\nThe path within the Git repository that represents the top level of the repo to sync. Default: the root directory of the repository."]
    pub fn set_policy_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_type`.\nType of secret configured for access to the Git repo. Must be one of ssh, cookiefile, gcenode, token, gcpserviceaccount or none. The validation of this is case-sensitive."]
    pub fn set_secret_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_branch`.\nThe branch of the repository to sync from. Default: master."]
    pub fn set_sync_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_repo`.\nThe URL of the Git repository to use as the source of truth."]
    pub fn set_sync_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_repo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_rev`.\nGit revision (tag or hash) to check out. Default HEAD."]
    pub fn set_sync_rev(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_rev = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_wait_secs`.\nPeriod in seconds between consecutive syncs. Default: 15."]
    pub fn set_sync_wait_secs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_wait_secs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Configmanagement_ConfigSync_Git {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_ConfigSync_Git>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Configmanagement_ConfigSync_Git {}

impl BuildGkeHubFeatureMembership_Configmanagement_ConfigSync_Git {
    pub fn build(self) -> GkeHubFeatureMembership_Configmanagement_ConfigSync_Git {
        GkeHubFeatureMembership_Configmanagement_ConfigSync_Git {
            gcp_service_account_email: core::default::Default::default(),
            https_proxy: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: core::default::Default::default(),
            sync_branch: core::default::Default::default(),
            sync_repo: core::default::Default::default(),
            sync_rev: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Configmanagement_ConfigSync_GitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Configmanagement_ConfigSync_GitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Configmanagement_ConfigSync_GitRef {
        GkeHubFeatureMembership_Configmanagement_ConfigSync_GitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Configmanagement_ConfigSync_GitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_service_account_email` after provisioning.\nThe GCP Service Account Email used for auth when secretType is gcpServiceAccount."]
    pub fn gcp_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `https_proxy` after provisioning.\nURL for the HTTPS proxy to be used when communicating with the Git repo."]
    pub fn https_proxy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_proxy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `policy_dir` after provisioning.\nThe path within the Git repository that represents the top level of the repo to sync. Default: the root directory of the repository."]
    pub fn policy_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_type` after provisioning.\nType of secret configured for access to the Git repo. Must be one of ssh, cookiefile, gcenode, token, gcpserviceaccount or none. The validation of this is case-sensitive."]
    pub fn secret_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_branch` after provisioning.\nThe branch of the repository to sync from. Default: master."]
    pub fn sync_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_repo` after provisioning.\nThe URL of the Git repository to use as the source of truth."]
    pub fn sync_repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_rev` after provisioning.\nGit revision (tag or hash) to check out. Default HEAD."]
    pub fn sync_rev(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_rev", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_wait_secs` after provisioning.\nPeriod in seconds between consecutive syncs. Default: 15."]
    pub fn sync_wait_secs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_wait_secs", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Configmanagement_ConfigSync_Oci {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_repo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_wait_secs: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembership_Configmanagement_ConfigSync_Oci {
    #[doc =
        "Set the field `gcp_service_account_email`.\nThe GCP Service Account Email used for auth when secret_type is gcpserviceaccount. "]
    pub fn set_gcp_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `policy_dir`.\nThe absolute path of the directory that contains the local resources. Default: the root directory of the image."]
    pub fn set_policy_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_type`.\nType of secret configured for access to the OCI Image. Must be one of gcenode, gcpserviceaccount or none. The validation of this is case-sensitive."]
    pub fn set_secret_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sync_repo`.\nThe OCI image repository URL for the package to sync from. e.g. LOCATION-docker.pkg.dev/PROJECT_ID/REPOSITORY_NAME/PACKAGE_NAME."]
    pub fn set_sync_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_repo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_wait_secs`.\nPeriod in seconds(int64 format) between consecutive syncs. Default: 15."]
    pub fn set_sync_wait_secs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_wait_secs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Configmanagement_ConfigSync_Oci {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_ConfigSync_Oci>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Configmanagement_ConfigSync_Oci {}

impl BuildGkeHubFeatureMembership_Configmanagement_ConfigSync_Oci {
    pub fn build(self) -> GkeHubFeatureMembership_Configmanagement_ConfigSync_Oci {
        GkeHubFeatureMembership_Configmanagement_ConfigSync_Oci {
            gcp_service_account_email: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: core::default::Default::default(),
            sync_repo: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Configmanagement_ConfigSync_OciRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Configmanagement_ConfigSync_OciRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Configmanagement_ConfigSync_OciRef {
        GkeHubFeatureMembership_Configmanagement_ConfigSync_OciRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Configmanagement_ConfigSync_OciRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcp_service_account_email` after provisioning.\nThe GCP Service Account Email used for auth when secret_type is gcpserviceaccount. "]
    pub fn gcp_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `policy_dir` after provisioning.\nThe absolute path of the directory that contains the local resources. Default: the root directory of the image."]
    pub fn policy_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_type` after provisioning.\nType of secret configured for access to the OCI Image. Must be one of gcenode, gcpserviceaccount or none. The validation of this is case-sensitive."]
    pub fn secret_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_repo` after provisioning.\nThe OCI image repository URL for the package to sync from. e.g. LOCATION-docker.pkg.dev/PROJECT_ID/REPOSITORY_NAME/PACKAGE_NAME."]
    pub fn sync_repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sync_wait_secs` after provisioning.\nPeriod in seconds(int64 format) between consecutive syncs. Default: 15."]
    pub fn sync_wait_secs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_wait_secs", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembership_Configmanagement_ConfigSyncDynamic {
    deployment_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides>>,
    git: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Configmanagement_ConfigSync_Git>>,
    oci: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Configmanagement_ConfigSync_Oci>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Configmanagement_ConfigSync {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics_gcp_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prevent_drift: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_syncing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_overrides: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<GkeHubFeatureMembership_Configmanagement_ConfigSync_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci: std::option::Option<std::vec::Vec<GkeHubFeatureMembership_Configmanagement_ConfigSync_Oci>>,
    dynamic: GkeHubFeatureMembership_Configmanagement_ConfigSyncDynamic,
}

impl GkeHubFeatureMembership_Configmanagement_ConfigSync {
    #[doc =
        "Set the field `enabled`.\nEnables the installation of ConfigSync. If set to true, ConfigSync resources will be created and the other ConfigSync fields will be applied if exist. If set to false, all other ConfigSync fields will be ignored, ConfigSync resources will be deleted. If omitted, ConfigSync resources will be managed depends on the presence of the git or oci field."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metrics_gcp_service_account_email`.\nDeprecated: If Workload Identity Federation for GKE is enabled, Google Cloud Service Account is no longer needed for exporting Config Sync metrics: https://cloud.google.com/kubernetes-engine/enterprise/config-sync/docs/how-to/monitor-config-sync-cloud-monitoring#custom-monitoring."]
    pub fn set_metrics_gcp_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metrics_gcp_service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prevent_drift`.\nSet to true to enable the Config Sync admission webhook to prevent drifts. If set to `false`, disables the Config Sync admission webhook and does not prevent drifts."]
    pub fn set_prevent_drift(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.prevent_drift = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_format`.\nSpecifies whether the Config Sync Repo is in \"hierarchical\" or \"unstructured\" mode."]
    pub fn set_source_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stop_syncing`.\nSet to true to stop syncing configs for a single cluster. Default: false."]
    pub fn set_stop_syncing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.stop_syncing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_overrides`.\n"]
    pub fn set_deployment_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverrides>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deployment_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deployment_overrides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_ConfigSync_Git>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oci`.\n"]
    pub fn set_oci(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_ConfigSync_Oci>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oci = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oci = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Configmanagement_ConfigSync {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_ConfigSync>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Configmanagement_ConfigSync {}

impl BuildGkeHubFeatureMembership_Configmanagement_ConfigSync {
    pub fn build(self) -> GkeHubFeatureMembership_Configmanagement_ConfigSync {
        GkeHubFeatureMembership_Configmanagement_ConfigSync {
            enabled: core::default::Default::default(),
            metrics_gcp_service_account_email: core::default::Default::default(),
            prevent_drift: core::default::Default::default(),
            source_format: core::default::Default::default(),
            stop_syncing: core::default::Default::default(),
            deployment_overrides: core::default::Default::default(),
            git: core::default::Default::default(),
            oci: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Configmanagement_ConfigSyncRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Configmanagement_ConfigSyncRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Configmanagement_ConfigSyncRef {
        GkeHubFeatureMembership_Configmanagement_ConfigSyncRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Configmanagement_ConfigSyncRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnables the installation of ConfigSync. If set to true, ConfigSync resources will be created and the other ConfigSync fields will be applied if exist. If set to false, all other ConfigSync fields will be ignored, ConfigSync resources will be deleted. If omitted, ConfigSync resources will be managed depends on the presence of the git or oci field."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metrics_gcp_service_account_email` after provisioning.\nDeprecated: If Workload Identity Federation for GKE is enabled, Google Cloud Service Account is no longer needed for exporting Config Sync metrics: https://cloud.google.com/kubernetes-engine/enterprise/config-sync/docs/how-to/monitor-config-sync-cloud-monitoring#custom-monitoring."]
    pub fn metrics_gcp_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metrics_gcp_service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prevent_drift` after provisioning.\nSet to true to enable the Config Sync admission webhook to prevent drifts. If set to `false`, disables the Config Sync admission webhook and does not prevent drifts."]
    pub fn prevent_drift(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prevent_drift", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_format` after provisioning.\nSpecifies whether the Config Sync Repo is in \"hierarchical\" or \"unstructured\" mode."]
    pub fn source_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stop_syncing` after provisioning.\nSet to true to stop syncing configs for a single cluster. Default: false."]
    pub fn stop_syncing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stop_syncing", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment_overrides` after provisioning.\n"]
    pub fn deployment_overrides(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureMembership_Configmanagement_ConfigSync_DeploymentOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<GkeHubFeatureMembership_Configmanagement_ConfigSync_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `oci` after provisioning.\n"]
    pub fn oci(&self) -> terrars::ListRef<GkeHubFeatureMembership_Configmanagement_ConfigSync_OciRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oci", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Configmanagement_HierarchyController {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_hierarchical_resource_quota: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_pod_tree_labels: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeHubFeatureMembership_Configmanagement_HierarchyController {
    #[doc =
        "Set the field `enable_hierarchical_resource_quota`.\nWhether hierarchical resource quota is enabled in this cluster."]
    pub fn set_enable_hierarchical_resource_quota(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_hierarchical_resource_quota = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_pod_tree_labels`.\nWhether pod tree labels are enabled in this cluster."]
    pub fn set_enable_pod_tree_labels(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_pod_tree_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enabled`.\n**DEPRECATED** Configuring Hierarchy Controller through the configmanagement feature is no longer recommended. Use https://github.com/kubernetes-sigs/hierarchical-namespaces instead."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Configmanagement_HierarchyController {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_HierarchyController>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Configmanagement_HierarchyController {}

impl BuildGkeHubFeatureMembership_Configmanagement_HierarchyController {
    pub fn build(self) -> GkeHubFeatureMembership_Configmanagement_HierarchyController {
        GkeHubFeatureMembership_Configmanagement_HierarchyController {
            enable_hierarchical_resource_quota: core::default::Default::default(),
            enable_pod_tree_labels: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Configmanagement_HierarchyControllerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Configmanagement_HierarchyControllerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Configmanagement_HierarchyControllerRef {
        GkeHubFeatureMembership_Configmanagement_HierarchyControllerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Configmanagement_HierarchyControllerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_hierarchical_resource_quota` after provisioning.\nWhether hierarchical resource quota is enabled in this cluster."]
    pub fn enable_hierarchical_resource_quota(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_hierarchical_resource_quota", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_pod_tree_labels` after provisioning.\nWhether pod tree labels are enabled in this cluster."]
    pub fn enable_pod_tree_labels(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_pod_tree_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\n**DEPRECATED** Configuring Hierarchy Controller through the configmanagement feature is no longer recommended. Use https://github.com/kubernetes-sigs/hierarchical-namespaces instead."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backends: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring {
    #[doc =
        "Set the field `backends`.\n Specifies the list of backends Policy Controller will export to. Specifying an empty value `[]` disables metrics export."]
    pub fn set_backends(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.backends = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring {}

impl BuildGkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring {
    pub fn build(self) -> GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring {
        GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring {
            backends: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Configmanagement_PolicyController_MonitoringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Configmanagement_PolicyController_MonitoringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Configmanagement_PolicyController_MonitoringRef {
        GkeHubFeatureMembership_Configmanagement_PolicyController_MonitoringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Configmanagement_PolicyController_MonitoringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backends` after provisioning.\n Specifies the list of backends Policy Controller will export to. Specifying an empty value `[]` disables metrics export."]
    pub fn backends(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backends", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembership_Configmanagement_PolicyControllerDynamic {
    monitoring: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Configmanagement_PolicyController {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_interval_seconds: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exemptable_namespaces: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_denies_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mutation_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referential_rules_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_library_installed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std::option::Option<std::vec::Vec<GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring>>,
    dynamic: GkeHubFeatureMembership_Configmanagement_PolicyControllerDynamic,
}

impl GkeHubFeatureMembership_Configmanagement_PolicyController {
    #[doc =
        "Set the field `audit_interval_seconds`.\nSets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether."]
    pub fn set_audit_interval_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audit_interval_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enabled`.\nEnables the installation of Policy Controller. If false, the rest of PolicyController fields take no effect."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exemptable_namespaces`.\nThe set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
    pub fn set_exemptable_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exemptable_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_denies_enabled`.\nLogs all denies and dry run failures."]
    pub fn set_log_denies_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_denies_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mutation_enabled`.\nEnable or disable mutation in policy controller. If true, mutation CRDs, webhook and controller deployment will be deployed to the cluster."]
    pub fn set_mutation_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mutation_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `referential_rules_enabled`.\nEnables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
    pub fn set_referential_rules_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.referential_rules_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `template_library_installed`.\nInstalls the default template library along with Policy Controller."]
    pub fn set_template_library_installed(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.template_library_installed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Configmanagement_PolicyController {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_PolicyController>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Configmanagement_PolicyController {}

impl BuildGkeHubFeatureMembership_Configmanagement_PolicyController {
    pub fn build(self) -> GkeHubFeatureMembership_Configmanagement_PolicyController {
        GkeHubFeatureMembership_Configmanagement_PolicyController {
            audit_interval_seconds: core::default::Default::default(),
            enabled: core::default::Default::default(),
            exemptable_namespaces: core::default::Default::default(),
            log_denies_enabled: core::default::Default::default(),
            mutation_enabled: core::default::Default::default(),
            referential_rules_enabled: core::default::Default::default(),
            template_library_installed: core::default::Default::default(),
            monitoring: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Configmanagement_PolicyControllerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Configmanagement_PolicyControllerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Configmanagement_PolicyControllerRef {
        GkeHubFeatureMembership_Configmanagement_PolicyControllerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Configmanagement_PolicyControllerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audit_interval_seconds` after provisioning.\nSets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether."]
    pub fn audit_interval_seconds(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_interval_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnables the installation of Policy Controller. If false, the rest of PolicyController fields take no effect."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exemptable_namespaces` after provisioning.\nThe set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
    pub fn exemptable_namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exemptable_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_denies_enabled` after provisioning.\nLogs all denies and dry run failures."]
    pub fn log_denies_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_denies_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mutation_enabled` after provisioning.\nEnable or disable mutation in policy controller. If true, mutation CRDs, webhook and controller deployment will be deployed to the cluster."]
    pub fn mutation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mutation_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `referential_rules_enabled` after provisioning.\nEnables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
    pub fn referential_rules_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.referential_rules_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `template_library_installed` after provisioning.\nInstalls the default template library along with Policy Controller."]
    pub fn template_library_installed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_library_installed", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureMembership_Configmanagement_PolicyController_MonitoringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembership_ConfigmanagementDynamic {
    config_sync: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Configmanagement_ConfigSync>>,
    hierarchy_controller: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Configmanagement_HierarchyController>>,
    policy_controller: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Configmanagement_PolicyController>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Configmanagement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_sync: std::option::Option<std::vec::Vec<GkeHubFeatureMembership_Configmanagement_ConfigSync>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hierarchy_controller: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureMembership_Configmanagement_HierarchyController>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_controller: std::option::Option<std::vec::Vec<GkeHubFeatureMembership_Configmanagement_PolicyController>>,
    dynamic: GkeHubFeatureMembership_ConfigmanagementDynamic,
}

impl GkeHubFeatureMembership_Configmanagement {
    #[doc =
        "Set the field `management`.\nSet this field to MANAGEMENT_AUTOMATIC to enable Config Sync auto-upgrades, and set this field to MANAGEMENT_MANUAL or MANAGEMENT_UNSPECIFIED to disable Config Sync auto-upgrades."]
    pub fn set_management(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nOptional. Version of ACM to install. Defaults to the latest version."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config_sync`.\n"]
    pub fn set_config_sync(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_ConfigSync>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config_sync = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config_sync = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hierarchy_controller`.\n"]
    pub fn set_hierarchy_controller(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_HierarchyController>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hierarchy_controller = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hierarchy_controller = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policy_controller`.\n"]
    pub fn set_policy_controller(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_PolicyController>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_controller = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_controller = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Configmanagement {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Configmanagement {}

impl BuildGkeHubFeatureMembership_Configmanagement {
    pub fn build(self) -> GkeHubFeatureMembership_Configmanagement {
        GkeHubFeatureMembership_Configmanagement {
            management: core::default::Default::default(),
            version: core::default::Default::default(),
            config_sync: core::default::Default::default(),
            hierarchy_controller: core::default::Default::default(),
            policy_controller: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_ConfigmanagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_ConfigmanagementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureMembership_ConfigmanagementRef {
        GkeHubFeatureMembership_ConfigmanagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_ConfigmanagementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `management` after provisioning.\nSet this field to MANAGEMENT_AUTOMATIC to enable Config Sync auto-upgrades, and set this field to MANAGEMENT_MANUAL or MANAGEMENT_UNSPECIFIED to disable Config Sync auto-upgrades."]
    pub fn management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nOptional. Version of ACM to install. Defaults to the latest version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `config_sync` after provisioning.\n"]
    pub fn config_sync(&self) -> terrars::ListRef<GkeHubFeatureMembership_Configmanagement_ConfigSyncRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_sync", self.base))
    }

    #[doc = "Get a reference to the value of field `hierarchy_controller` after provisioning.\n"]
    pub fn hierarchy_controller(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureMembership_Configmanagement_HierarchyControllerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_controller", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_controller` after provisioning.\n"]
    pub fn policy_controller(&self) -> terrars::ListRef<GkeHubFeatureMembership_Configmanagement_PolicyControllerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_controller", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Mesh {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembership_Mesh {
    #[doc =
        "Set the field `control_plane`.\n**DEPRECATED** Whether to automatically manage Service Mesh control planes. Possible values: CONTROL_PLANE_MANAGEMENT_UNSPECIFIED, AUTOMATIC, MANUAL"]
    pub fn set_control_plane(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.control_plane = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `management`.\nWhether to automatically manage Service Mesh. Possible values: MANAGEMENT_UNSPECIFIED, MANAGEMENT_AUTOMATIC, MANAGEMENT_MANUAL"]
    pub fn set_management(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.management = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Mesh {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Mesh>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Mesh {}

impl BuildGkeHubFeatureMembership_Mesh {
    pub fn build(self) -> GkeHubFeatureMembership_Mesh {
        GkeHubFeatureMembership_Mesh {
            control_plane: core::default::Default::default(),
            management: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_MeshRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_MeshRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureMembership_MeshRef {
        GkeHubFeatureMembership_MeshRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_MeshRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `control_plane` after provisioning.\n**DEPRECATED** Whether to automatically manage Service Mesh control planes. Possible values: CONTROL_PLANE_MANAGEMENT_UNSPECIFIED, AUTOMATIC, MANUAL"]
    pub fn control_plane(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_plane", self.base))
    }

    #[doc =
        "Get a reference to the value of field `management` after provisioning.\nWhether to automatically manage Service Mesh. Possible values: MANAGEMENT_UNSPECIFIED, MANAGEMENT_AUTOMATIC, MANAGEMENT_MANUAL"]
    pub fn management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
    #[doc = "Set the field `cpu`.\nCPU requirement expressed in Kubernetes resource units."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nMemory requirement expressed in Kubernetes resource units."]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {}

impl BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
    pub fn build(
        self,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU requirement expressed in Kubernetes resource units."]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nMemory requirement expressed in Kubernetes resource units."]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
    #[doc = "Set the field `cpu`.\nCPU requirement expressed in Kubernetes resource units."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\nMemory requirement expressed in Kubernetes resource units."]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {}

impl BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
    pub fn build(
        self,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu` after provisioning.\nCPU requirement expressed in Kubernetes resource units."]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory` after provisioning.\nMemory requirement expressed in Kubernetes resource units."]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesDynamic {
    limits: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits,
        >,
    >,
    requests: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests,
        >,
    >,
    dynamic: GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesDynamic,
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.requests = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.requests = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {}

impl BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
    pub fn build(
        self,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
            limits: core::default::Default::default(),
            requests: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(
        &self,
    ) -> terrars
    ::ListRef<
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(
        &self,
    ) -> terrars
    ::ListRef<
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations {
    #[doc = "Set the field `effect`.\nMatches a taint effect."]
    pub fn set_effect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\nMatches a taint key (not necessarily unique)."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\nMatches a taint operator."]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nMatches a taint value."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations {}

impl BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations {
    pub fn build(
        self,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationsRef {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effect` after provisioning.\nMatches a taint effect."]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nMatches a taint key (not necessarily unique)."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\nMatches a taint operator."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nMatches a taint value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsDynamic {
    container_resources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources,
        >,
    >,
    pod_tolerations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
    component_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_affinity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_resources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_tolerations: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations>,
    >,
    dynamic: GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsDynamic,
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
    #[doc =
        "Set the field `pod_affinity`.\nPod affinity configuration. Possible values: AFFINITY_UNSPECIFIED, NO_AFFINITY, ANTI_AFFINITY"]
    pub fn set_pod_affinity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pod_affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_count`.\nPod replica count."]
    pub fn set_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_resources`.\n"]
    pub fn set_container_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pod_tolerations`.\n"]
    pub fn set_pod_tolerations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerations,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pod_tolerations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pod_tolerations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
    type O =
        terrars
        ::BlockAssignable<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
    #[doc = "The name for the key in the map for which this object is mapped to in the API"]
    pub component_name: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
    pub fn build(self) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
            component_name: self.component_name,
            pod_affinity: core::default::Default::default(),
            replica_count: core::default::Default::default(),
            container_resources: core::default::Default::default(),
            pod_tolerations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `component_name` after provisioning.\nThe name for the key in the map for which this object is mapped to in the API"]
    pub fn component_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.component_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pod_affinity` after provisioning.\nPod affinity configuration. Possible values: AFFINITY_UNSPECIFIED, NO_AFFINITY, ANTI_AFFINITY"]
    pub fn pod_affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\nPod replica count."]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.base))
    }

    #[doc = "Get a reference to the value of field `container_resources` after provisioning.\n"]
    pub fn container_resources(
        &self,
    ) -> terrars
    ::ListRef<
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_tolerations` after provisioning.\n"]
    pub fn pod_tolerations(
        &self,
    ) -> terrars
    ::ListRef<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_tolerations", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
    bundle_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exempted_namespaces: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
    #[doc = "Set the field `exempted_namespaces`.\nThe set of namespaces to be exempted from the bundle."]
    pub fn set_exempted_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exempted_namespaces = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
    type O =
        terrars
        ::BlockAssignable<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
    #[doc = "The name for the key in the map for which this object is mapped to in the API"]
    pub bundle_name: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
    pub fn build(self) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
            bundle_name: self.bundle_name,
            exempted_namespaces: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bundle_name` after provisioning.\nThe name for the key in the map for which this object is mapped to in the API"]
    pub fn bundle_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exempted_namespaces` after provisioning.\nThe set of namespaces to be exempted from the bundle."]
    pub fn exempted_namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exempted_namespaces", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    installation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
    #[doc =
        "Set the field `installation`.\nConfigures the manner in which the template library is installed on the cluster. Possible values: INSTALLATION_UNSPECIFIED, NOT_INSTALLED, ALL"]
    pub fn set_installation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.installation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
    type O =
        terrars
        ::BlockAssignable<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {}

impl BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
    pub fn build(
        self,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
            installation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `installation` after provisioning.\nConfigures the manner in which the template library is installed on the cluster. Possible values: INSTALLATION_UNSPECIFIED, NOT_INSTALLED, ALL"]
    pub fn installation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.installation", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContentDynamic {
    bundles: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles>,
    >,
    template_library: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary,
        >,
    >,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bundles: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_library: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary>,
    >,
    dynamic: GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContentDynamic,
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent {
    #[doc = "Set the field `bundles`.\n"]
    pub fn set_bundles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bundles = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bundles = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `template_library`.\n"]
    pub fn set_template_library(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.template_library = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.template_library = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent {
    type O =
        terrars::BlockAssignable<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent {}

impl BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent {
    pub fn build(self) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent {
            bundles: core::default::Default::default(),
            template_library: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContentRef {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `template_library` after provisioning.\n"]
    pub fn template_library(
        &self,
    ) -> terrars
    ::ListRef<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_library", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfigDynamic {
    deployment_configs: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs>,
    >,
    monitoring: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring>>,
    policy_content: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent>,
    >,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_interval_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraint_violation_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exemptable_namespaces: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    install_spec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_denies_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mutation_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referential_rules_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_configs: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std::option::Option<std::vec::Vec<GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_content: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent>>,
    dynamic: GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfigDynamic,
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig {
    #[doc =
        "Set the field `audit_interval_seconds`.\nSets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether."]
    pub fn set_audit_interval_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.audit_interval_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `constraint_violation_limit`.\nThe maximum number of audit violations to be stored in a constraint. If not set, the internal default of 20 will be used."]
    pub fn set_constraint_violation_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.constraint_violation_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exemptable_namespaces`.\nThe set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
    pub fn set_exemptable_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exemptable_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `install_spec`.\nConfigures the mode of the Policy Controller installation. Possible values: INSTALL_SPEC_UNSPECIFIED, INSTALL_SPEC_NOT_INSTALLED, INSTALL_SPEC_ENABLED, INSTALL_SPEC_SUSPENDED, INSTALL_SPEC_DETACHED"]
    pub fn set_install_spec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.install_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_denies_enabled`.\nLogs all denies and dry run failures."]
    pub fn set_log_denies_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_denies_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mutation_enabled`.\nEnables the ability to mutate resources using Policy Controller."]
    pub fn set_mutation_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mutation_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `referential_rules_enabled`.\nEnables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
    pub fn set_referential_rules_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.referential_rules_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_configs`.\n"]
    pub fn set_deployment_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deployment_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deployment_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeHubFeatureMembership_Configmanagement_PolicyController_Monitoring>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monitoring = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policy_content`.\n"]
    pub fn set_policy_content(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContent,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_content = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_content = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig {}

impl BuildGkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig {
    pub fn build(self) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig {
            audit_interval_seconds: core::default::Default::default(),
            constraint_violation_limit: core::default::Default::default(),
            exemptable_namespaces: core::default::Default::default(),
            install_spec: core::default::Default::default(),
            log_denies_enabled: core::default::Default::default(),
            mutation_enabled: core::default::Default::default(),
            referential_rules_enabled: core::default::Default::default(),
            deployment_configs: core::default::Default::default(),
            monitoring: core::default::Default::default(),
            policy_content: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfigRef {
        GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audit_interval_seconds` after provisioning.\nSets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether."]
    pub fn audit_interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_interval_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `constraint_violation_limit` after provisioning.\nThe maximum number of audit violations to be stored in a constraint. If not set, the internal default of 20 will be used."]
    pub fn constraint_violation_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constraint_violation_limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exemptable_namespaces` after provisioning.\nThe set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
    pub fn exemptable_namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exemptable_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `install_spec` after provisioning.\nConfigures the mode of the Policy Controller installation. Possible values: INSTALL_SPEC_UNSPECIFIED, INSTALL_SPEC_NOT_INSTALLED, INSTALL_SPEC_ENABLED, INSTALL_SPEC_SUSPENDED, INSTALL_SPEC_DETACHED"]
    pub fn install_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.install_spec", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_denies_enabled` after provisioning.\nLogs all denies and dry run failures."]
    pub fn log_denies_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_denies_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mutation_enabled` after provisioning.\nEnables the ability to mutate resources using Policy Controller."]
    pub fn mutation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mutation_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `referential_rules_enabled` after provisioning.\nEnables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
    pub fn referential_rules_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.referential_rules_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureMembership_Configmanagement_PolicyController_MonitoringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_content` after provisioning.\n"]
    pub fn policy_content(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig_PolicyContentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_content", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembership_PolicycontrollerDynamic {
    policy_controller_hub_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Policycontroller {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_controller_hub_config: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig>>,
    dynamic: GkeHubFeatureMembership_PolicycontrollerDynamic,
}

impl GkeHubFeatureMembership_Policycontroller {
    #[doc =
        "Set the field `version`.\nOptional. Version of Policy Controller to install. Defaults to the latest version."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_controller_hub_config`.\n"]
    pub fn set_policy_controller_hub_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_controller_hub_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_controller_hub_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembership_Policycontroller {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Policycontroller>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Policycontroller {}

impl BuildGkeHubFeatureMembership_Policycontroller {
    pub fn build(self) -> GkeHubFeatureMembership_Policycontroller {
        GkeHubFeatureMembership_Policycontroller {
            version: core::default::Default::default(),
            policy_controller_hub_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_PolicycontrollerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_PolicycontrollerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureMembership_PolicycontrollerRef {
        GkeHubFeatureMembership_PolicycontrollerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_PolicycontrollerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nOptional. Version of Policy Controller to install. Defaults to the latest version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_controller_hub_config` after provisioning.\n"]
    pub fn policy_controller_hub_config(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureMembership_Policycontroller_PolicyControllerHubConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_controller_hub_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembership_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembership_Timeouts {
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

impl terrars::ToListMappable for GkeHubFeatureMembership_Timeouts {
    type O = terrars::BlockAssignable<GkeHubFeatureMembership_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembership_Timeouts {}

impl BuildGkeHubFeatureMembership_Timeouts {
    pub fn build(self) -> GkeHubFeatureMembership_Timeouts {
        GkeHubFeatureMembership_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFeatureMembership_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembership_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureMembership_TimeoutsRef {
        GkeHubFeatureMembership_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembership_TimeoutsRef {
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
