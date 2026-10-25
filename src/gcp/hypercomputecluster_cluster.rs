#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct HypercomputeclusterClusterData_ {
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
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_resources: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_ComputeResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_resources: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_NetworkResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    orchestrator: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_Orchestrator>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_resources: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<HypercomputeclusterCluster_Timeouts>,
    dynamic: HypercomputeclusterClusterDynamic,
}

struct HypercomputeclusterCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<HypercomputeclusterClusterData_>,
}

#[derive(Clone)]
pub struct HypercomputeclusterCluster(std::rc::Rc<HypercomputeclusterCluster_>);

impl HypercomputeclusterCluster {
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

    #[doc = "Set the field `description`.\nUser-provided description of the cluster."]
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
        "Set the field `labels`.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) applied\nto the cluster. Labels can be used to organize clusters and to filter them\nin queries.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `compute_resources`.\n"]
    pub fn set_compute_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HypercomputeclusterCluster_ComputeResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_resources`.\n"]
    pub fn set_network_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HypercomputeclusterCluster_NetworkResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `orchestrator`.\n"]
    pub fn set_orchestrator(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().orchestrator = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.orchestrator = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_resources`.\n"]
    pub fn set_storage_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<HypercomputeclusterCluster_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nID of the cluster to create. Must start with a lowercase letter,\nuse only lowercase letters and numbers, and be at most 10 characters long."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime that the cluster was originally created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the cluster."]
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
        "Get a reference to the value of field `labels` after provisioning.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) applied\nto the cluster. Labels can be used to organize clusters and to filter them\nin queries.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. [Relative resource name](https://google.aip.dev/122) of the cluster, in the\nformat 'projects/{project}/locations/{location}/clusters/{cluster}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIndicates whether changes to the cluster are currently in flight. If this\nis 'true', then the current state might not match the cluster's intended\nstate."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime that the cluster was most recently updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `orchestrator` after provisioning.\n"]
    pub fn orchestrator(&self) -> terrars::ListRef<HypercomputeclusterCluster_OrchestratorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orchestrator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HypercomputeclusterCluster_TimeoutsRef {
        HypercomputeclusterCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for HypercomputeclusterCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for HypercomputeclusterCluster { }

impl terrars::ToListMappable for HypercomputeclusterCluster {
    type O = terrars::ListRef<HypercomputeclusterClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for HypercomputeclusterCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_hypercomputecluster_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildHypercomputeclusterCluster {
    pub tf_id: std::string::String,
    #[doc =
        "ID of the cluster to create. Must start with a lowercase letter,\nuse only lowercase letters and numbers, and be at most 10 characters long."]
    pub cluster_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> HypercomputeclusterCluster {
        let out = HypercomputeclusterCluster(std::rc::Rc::new(HypercomputeclusterCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(HypercomputeclusterClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_id: self.cluster_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                compute_resources: core::default::Default::default(),
                network_resources: core::default::Default::default(),
                orchestrator: core::default::Default::default(),
                storage_resources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct HypercomputeclusterClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl HypercomputeclusterClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nID of the cluster to create. Must start with a lowercase letter,\nuse only lowercase letters and numbers, and be at most 10 characters long."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime that the cluster was originally created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description of the cluster."]
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
        "Get a reference to the value of field `labels` after provisioning.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) applied\nto the cluster. Labels can be used to organize clusters and to filter them\nin queries.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. [Relative resource name](https://google.aip.dev/122) of the cluster, in the\nformat 'projects/{project}/locations/{location}/clusters/{cluster}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIndicates whether changes to the cluster are currently in flight. If this\nis 'true', then the current state might not match the cluster's intended\nstate."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime that the cluster was most recently updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `orchestrator` after provisioning.\n"]
    pub fn orchestrator(&self) -> terrars::ListRef<HypercomputeclusterCluster_OrchestratorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.orchestrator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HypercomputeclusterCluster_TimeoutsRef {
        HypercomputeclusterCluster_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterClusterDynamic {
    compute_resources: std::option::Option<terrars::DynamicBlock<HypercomputeclusterCluster_ComputeResources>>,
    network_resources: std::option::Option<terrars::DynamicBlock<HypercomputeclusterCluster_NetworkResources>>,
    orchestrator: std::option::Option<terrars::DynamicBlock<HypercomputeclusterCluster_Orchestrator>>,
    storage_resources: std::option::Option<terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances {
    machine_type: terrars::PrimField<std::string::String>,
    max_duration: terrars::PrimField<std::string::String>,
    zone: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances { }

impl terrars::ToListMappable for HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances {
    #[doc =
        "Name of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub machine_type: terrars::PrimField<std::string::String>,
    #[doc =
        "Specifies the time limit for created instances. Instances will be\nterminated at the end of this duration."]
    pub max_duration: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub zone: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances {
    pub fn build(self) -> HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances {
        HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances {
            machine_type: self.machine_type,
            max_duration: self.max_duration,
            zone: self.zone,
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstancesRef {
        HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstancesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nName of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_duration` after provisioning.\nSpecifies the time limit for created instances. Instances will be\nterminated at the end of this duration."]
    pub fn max_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances {
    machine_type: terrars::PrimField<std::string::String>,
    zone: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances { }

impl terrars::ToListMappable for HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances {
    #[doc =
        "Name of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub machine_type: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub zone: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances {
    pub fn build(self) -> HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances {
        HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances {
            machine_type: self.machine_type,
            zone: self.zone,
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstancesRef {
        HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstancesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nName of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances {
    #[doc =
        "Set the field `reservation`.\nName of the reservation from which VM instances should be created, in the\nformat 'projects/{project}/zones/{zone}/reservations/{reservation}'."]
    pub fn set_reservation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reservation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances {}

impl BuildHypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances {
    pub fn build(self) -> HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances {
        HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances {
            reservation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstancesRef {
        HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstancesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `reservation` after provisioning.\nName of the reservation from which VM instances should be created, in the\nformat 'projects/{project}/zones/{zone}/reservations/{reservation}'."]
    pub fn reservation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances {
    machine_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_action: std::option::Option<terrars::PrimField<std::string::String>>,
    zone: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances {
    #[doc =
        "Set the field `termination_action`.\nSpecifies the termination action of the instance\nPossible values:\nSTOP\nDELETE"]
    pub fn set_termination_action(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.termination_action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances {
    #[doc =
        "Name of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub machine_type: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub zone: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances {
    pub fn build(self) -> HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances {
        HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances {
            machine_type: self.machine_type,
            termination_action: core::default::Default::default(),
            zone: self.zone,
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstancesRef {
        HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstancesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nName of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use, e.g.\n'n2-standard-2'."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `termination_action` after provisioning.\nSpecifies the termination action of the instance\nPossible values:\nSTOP\nDELETE"]
    pub fn termination_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.termination_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which VM instances should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_ComputeResources_ConfigDynamic {
    new_flex_start_instances: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances>>,
    new_on_demand_instances: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances>>,
    new_reserved_instances: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances>>,
    new_spot_instances: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_ComputeResources_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_flex_start_instances: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_on_demand_instances: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_reserved_instances: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_spot_instances: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances>>,
    dynamic: HypercomputeclusterCluster_ComputeResources_ConfigDynamic,
}

impl HypercomputeclusterCluster_ComputeResources_Config {
    #[doc = "Set the field `new_flex_start_instances`.\n"]
    pub fn set_new_flex_start_instances(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstances>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.new_flex_start_instances = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.new_flex_start_instances = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `new_on_demand_instances`.\n"]
    pub fn set_new_on_demand_instances(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstances>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.new_on_demand_instances = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.new_on_demand_instances = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `new_reserved_instances`.\n"]
    pub fn set_new_reserved_instances(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstances>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.new_reserved_instances = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.new_reserved_instances = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `new_spot_instances`.\n"]
    pub fn set_new_spot_instances(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstances>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.new_spot_instances = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.new_spot_instances = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_ComputeResources_Config {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_ComputeResources_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_ComputeResources_Config {}

impl BuildHypercomputeclusterCluster_ComputeResources_Config {
    pub fn build(self) -> HypercomputeclusterCluster_ComputeResources_Config {
        HypercomputeclusterCluster_ComputeResources_Config {
            new_flex_start_instances: core::default::Default::default(),
            new_on_demand_instances: core::default::Default::default(),
            new_reserved_instances: core::default::Default::default(),
            new_spot_instances: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_ComputeResources_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_ComputeResources_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_ComputeResources_ConfigRef {
        HypercomputeclusterCluster_ComputeResources_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_ComputeResources_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `new_flex_start_instances` after provisioning.\n"]
    pub fn new_flex_start_instances(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_ComputeResources_Config_NewFlexStartInstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_flex_start_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `new_on_demand_instances` after provisioning.\n"]
    pub fn new_on_demand_instances(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_ComputeResources_Config_NewOnDemandInstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_on_demand_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `new_reserved_instances` after provisioning.\n"]
    pub fn new_reserved_instances(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_ComputeResources_Config_NewReservedInstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_reserved_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `new_spot_instances` after provisioning.\n"]
    pub fn new_spot_instances(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_ComputeResources_Config_NewSpotInstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_spot_instances", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_ComputeResourcesDynamic {
    config: std::option::Option<terrars::DynamicBlock<HypercomputeclusterCluster_ComputeResources_Config>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_ComputeResources {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_ComputeResources_Config>>,
    dynamic: HypercomputeclusterCluster_ComputeResourcesDynamic,
}

impl HypercomputeclusterCluster_ComputeResources {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<HypercomputeclusterCluster_ComputeResources_Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_ComputeResources {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_ComputeResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_ComputeResources {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_ComputeResources {
    pub fn build(self) -> HypercomputeclusterCluster_ComputeResources {
        HypercomputeclusterCluster_ComputeResources {
            id: self.id,
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_ComputeResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_ComputeResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_ComputeResourcesRef {
        HypercomputeclusterCluster_ComputeResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_ComputeResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<HypercomputeclusterCluster_ComputeResources_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_NetworkResources_Network {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HypercomputeclusterCluster_NetworkResources_Network {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_NetworkResources_Network {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_NetworkResources_Network>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_NetworkResources_Network {}

impl BuildHypercomputeclusterCluster_NetworkResources_Network {
    pub fn build(self) -> HypercomputeclusterCluster_NetworkResources_Network {
        HypercomputeclusterCluster_NetworkResources_Network {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_NetworkResources_NetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_NetworkResources_NetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_NetworkResources_NetworkRef {
        HypercomputeclusterCluster_NetworkResources_NetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_NetworkResources_NetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork {
    network: terrars::PrimField<std::string::String>,
    subnetwork: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork { }

impl terrars::ToListMappable for HypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork {
    #[doc = "Name of the network to import, in the format\n'projects/{project}/global/networks/{network}'."]
    pub network: terrars::PrimField<std::string::String>,
    #[doc =
        "Particular subnetwork to use, in the format\n'projects/{project}/regions/{region}/subnetworks/{subnetwork}'."]
    pub subnetwork: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork {
    pub fn build(self) -> HypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork {
        HypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork {
            network: self.network,
            subnetwork: self.subnetwork,
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_NetworkResources_Config_ExistingNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_NetworkResources_Config_ExistingNetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_NetworkResources_Config_ExistingNetworkRef {
        HypercomputeclusterCluster_NetworkResources_Config_ExistingNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_NetworkResources_Config_ExistingNetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nName of the network to import, in the format\n'projects/{project}/global/networks/{network}'."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nParticular subnetwork to use, in the format\n'projects/{project}/regions/{region}/subnetworks/{subnetwork}'."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_NetworkResources_Config_NewNetwork {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    network: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_NetworkResources_Config_NewNetwork {
    #[doc = "Set the field `description`.\nDescription of the network. Maximum of 2048 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_NetworkResources_Config_NewNetwork {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_NetworkResources_Config_NewNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_NetworkResources_Config_NewNetwork {
    #[doc = "Name of the network to create, in the format\n'projects/{project}/global/networks/{network}'."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_NetworkResources_Config_NewNetwork {
    pub fn build(self) -> HypercomputeclusterCluster_NetworkResources_Config_NewNetwork {
        HypercomputeclusterCluster_NetworkResources_Config_NewNetwork {
            description: core::default::Default::default(),
            network: self.network,
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_NetworkResources_Config_NewNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_NetworkResources_Config_NewNetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_NetworkResources_Config_NewNetworkRef {
        HypercomputeclusterCluster_NetworkResources_Config_NewNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_NetworkResources_Config_NewNetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the network. Maximum of 2048 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nName of the network to create, in the format\n'projects/{project}/global/networks/{network}'."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_NetworkResources_ConfigDynamic {
    existing_network: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork>>,
    new_network: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_NetworkResources_Config_NewNetwork>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_NetworkResources_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existing_network: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_network: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_NetworkResources_Config_NewNetwork>>,
    dynamic: HypercomputeclusterCluster_NetworkResources_ConfigDynamic,
}

impl HypercomputeclusterCluster_NetworkResources_Config {
    #[doc = "Set the field `existing_network`.\n"]
    pub fn set_existing_network(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<HypercomputeclusterCluster_NetworkResources_Config_ExistingNetwork>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.existing_network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_network = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `new_network`.\n"]
    pub fn set_new_network(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<HypercomputeclusterCluster_NetworkResources_Config_NewNetwork>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.new_network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.new_network = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_NetworkResources_Config {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_NetworkResources_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_NetworkResources_Config {}

impl BuildHypercomputeclusterCluster_NetworkResources_Config {
    pub fn build(self) -> HypercomputeclusterCluster_NetworkResources_Config {
        HypercomputeclusterCluster_NetworkResources_Config {
            existing_network: core::default::Default::default(),
            new_network: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_NetworkResources_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_NetworkResources_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_NetworkResources_ConfigRef {
        HypercomputeclusterCluster_NetworkResources_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_NetworkResources_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `existing_network` after provisioning.\n"]
    pub fn existing_network(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_NetworkResources_Config_ExistingNetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.existing_network", self.base))
    }

    #[doc = "Get a reference to the value of field `new_network` after provisioning.\n"]
    pub fn new_network(&self) -> terrars::ListRef<HypercomputeclusterCluster_NetworkResources_Config_NewNetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_network", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_NetworkResourcesDynamic {
    config: std::option::Option<terrars::DynamicBlock<HypercomputeclusterCluster_NetworkResources_Config>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_NetworkResources {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_NetworkResources_Config>>,
    dynamic: HypercomputeclusterCluster_NetworkResourcesDynamic,
}

impl HypercomputeclusterCluster_NetworkResources {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<HypercomputeclusterCluster_NetworkResources_Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_NetworkResources {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_NetworkResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_NetworkResources {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_NetworkResources {
    pub fn build(self) -> HypercomputeclusterCluster_NetworkResources {
        HypercomputeclusterCluster_NetworkResources {
            id: self.id,
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_NetworkResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_NetworkResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_NetworkResourcesRef {
        HypercomputeclusterCluster_NetworkResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_NetworkResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nA reference to a [VPC network](https://cloud.google.com/vpc/docs/vpc) in\nGoogle Compute Engine."]
    pub fn network(&self) -> terrars::ListRef<HypercomputeclusterCluster_NetworkResources_NetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<HypercomputeclusterCluster_NetworkResources_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_Instances {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_Instances {
    #[doc = "Set the field `instance`.\n"]
    pub fn set_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_Instances {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_Instances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_Instances {}

impl BuildHypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_Instances {
    pub fn build(self) -> HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_Instances {
        HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_Instances {
            instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_InstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_InstancesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_InstancesRef {
        HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_InstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_InstancesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance` after provisioning.\n"]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk {
    size_gb: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk { }

impl terrars::ToListMappable for HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk {
    #[doc = "Size of the disk in gigabytes. Must be at least 10GB."]
    pub size_gb: terrars::PrimField<std::string::String>,
    #[doc =
        "[Persistent disk\ntype](https://cloud.google.com/compute/docs/disks#disk-types), in the\nformat 'projects/{project}/zones/{zone}/diskTypes/{disk_type}'."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk {
    pub fn build(self) -> HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk {
        HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk {
            size_gb: self.size_gb,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDiskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDiskRef {
        HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDiskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `size_gb` after provisioning.\nSize of the disk in gigabytes. Must be at least 10GB."]
    pub fn size_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\n[Persistent disk\ntype](https://cloud.google.com/compute/docs/disks#disk-types), in the\nformat 'projects/{project}/zones/{zone}/diskTypes/{disk_type}'."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs {
    id: terrars::PrimField<std::string::String>,
    local_mount: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs { }

impl terrars::ToListMappable for HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs {
    #[doc =
        "ID of the storage resource to mount, which must match a key in the\ncluster's [storage_resources](Cluster.storage_resources)."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc =
        "A directory inside the VM instance's file system where the storage resource\nshould be mounted (e.g., '/mnt/share')."]
    pub local_mount: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs {
    pub fn build(self) -> HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs {
        HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs {
            id: self.id,
            local_mount: self.local_mount,
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigsRef {
        HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nID of the storage resource to mount, which must match a key in the\ncluster's [storage_resources](Cluster.storage_resources)."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_mount` after provisioning.\nA directory inside the VM instance's file system where the storage resource\nshould be mounted (e.g., '/mnt/share')."]
    pub fn local_mount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_mount", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodesDynamic {
    boot_disk: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk>>,
    storage_configs: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes {
    count: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_os_login: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_public_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    machine_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_script: std::option::Option<terrars::PrimField<std::string::String>>,
    zone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_configs: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs>>,
    dynamic: HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodesDynamic,
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes {
    #[doc =
        "Set the field `enable_os_login`.\nWhether [OS Login](https://cloud.google.com/compute/docs/oslogin) should be\nenabled on login node instances."]
    pub fn set_enable_os_login(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_os_login = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_public_ips`.\nWhether login node instances should be assigned [external IP\naddresses](https://cloud.google.com/compute/docs/ip-addresses#externaladdresses)."]
    pub fn set_enable_public_ips(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_public_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) that\nshould be applied to each login node instance."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `startup_script`.\n[Startup\nscript](https://cloud.google.com/compute/docs/instances/startup-scripts/linux)\nto be run on each login node instance. Max 256KB.\nThe script must complete within the system-defined default timeout of 5\nminutes. For tasks that require more time, consider running them in the\nbackground using methods such as '&' or 'nohup'."]
    pub fn set_startup_script(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.startup_script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boot_disk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boot_disk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_configs`.\n"]
    pub fn set_storage_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes {
    #[doc = "Number of login node instances to create."]
    pub count: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use for\nlogin nodes, e.g. 'n2-standard-2'."]
    pub machine_type: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the zone in which login nodes should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub zone: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes {
    pub fn build(self) -> HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes {
        HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes {
            count: self.count,
            enable_os_login: core::default::Default::default(),
            enable_public_ips: core::default::Default::default(),
            labels: core::default::Default::default(),
            machine_type: self.machine_type,
            startup_script: core::default::Default::default(),
            zone: self.zone,
            boot_disk: core::default::Default::default(),
            storage_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodesRef {
        HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nNumber of login node instances to create."]
    pub fn count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_os_login` after provisioning.\nWhether [OS Login](https://cloud.google.com/compute/docs/oslogin) should be\nenabled on login node instances."]
    pub fn enable_os_login(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_os_login", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_public_ips` after provisioning.\nWhether login node instances should be assigned [external IP\naddresses](https://cloud.google.com/compute/docs/ip-addresses#externaladdresses)."]
    pub fn enable_public_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_public_ips", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instances` after provisioning.\nInformation about the login node instances that were created in Compute\nEngine."]
    pub fn instances(&self) -> terrars::ListRef<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_InstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) that\nshould be applied to each login node instance."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nName of the Compute Engine [machine\ntype](https://cloud.google.com/compute/docs/machine-resource) to use for\nlogin nodes, e.g. 'n2-standard-2'."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `startup_script` after provisioning.\n[Startup\nscript](https://cloud.google.com/compute/docs/instances/startup-scripts/linux)\nto be run on each login node instance. Max 256KB.\nThe script must complete within the system-defined default timeout of 5\nminutes. For tasks that require more time, consider running them in the\nbackground using methods such as '&' or 'nohup'."]
    pub fn startup_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_script", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which login nodes should run, e.g., 'us-central1-a'.\nMust be in the same region as the cluster, and must match the zone of any\nother resources specified in the cluster."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_configs` after provisioning.\n"]
    pub fn storage_configs(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_configs", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstanceDynamic {
    boot_disk: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk>>,
    dynamic: HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstanceDynamic,
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance {
    #[doc =
        "Set the field `labels`.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) that\nshould be applied to each VM instance in the nodeset."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `startup_script`.\n[Startup\nscript](https://cloud.google.com/compute/docs/instances/startup-scripts/linux)\nto be run on each VM instance in the nodeset. Max 256KB."]
    pub fn set_startup_script(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.startup_script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDisk>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boot_disk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boot_disk = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance {}

impl BuildHypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance {
    pub fn build(self) -> HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance {
        HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance {
            labels: core::default::Default::default(),
            startup_script: core::default::Default::default(),
            boot_disk: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstanceRef {
        HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\n[Labels](https://cloud.google.com/compute/docs/labeling-resources) that\nshould be applied to each VM instance in the nodeset."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `startup_script` after provisioning.\n[Startup\nscript](https://cloud.google.com/compute/docs/instances/startup-scripts/linux)\nto be run on each VM instance in the nodeset. Max 256KB."]
    pub fn startup_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.startup_script", self.base))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes_BootDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_Orchestrator_Slurm_NodeSetsDynamic {
    compute_instance: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance>>,
    storage_configs: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_id: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_dynamic_node_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_node_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_instance: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_configs: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs>>,
    dynamic: HypercomputeclusterCluster_Orchestrator_Slurm_NodeSetsDynamic,
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets {
    #[doc =
        "Set the field `compute_id`.\nID of the compute resource on which this nodeset will run. Must match a key\nin the cluster's [compute_resources](Cluster.compute_resources)."]
    pub fn set_compute_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compute_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_dynamic_node_count`.\nControls how many additional nodes a cluster can bring online to handle\nworkloads. Set this value to enable dynamic node creation and limit the\nnumber of additional nodes the cluster can bring online. Leave empty if you\ndo not want the cluster to create nodes dynamically, and instead rely only\non static nodes."]
    pub fn set_max_dynamic_node_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_dynamic_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `static_node_count`.\nNumber of nodes to be statically created for this nodeset. The cluster will\nattempt to ensure that at least this many nodes exist at all times."]
    pub fn set_static_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.static_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_instance`.\n"]
    pub fn set_compute_instance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstance>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compute_instance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compute_instance = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_configs`.\n"]
    pub fn set_storage_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_Orchestrator_Slurm_NodeSets {
    #[doc =
        "Identifier for the nodeset, which allows it to be referenced by partitions.\nMust conform to\n[RFC-1034](https://datatracker.ietf.org/doc/html/rfc1034) (lower-case,\nalphanumeric, and at most 63 characters)."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_Orchestrator_Slurm_NodeSets {
    pub fn build(self) -> HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets {
        HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets {
            compute_id: core::default::Default::default(),
            id: self.id,
            max_dynamic_node_count: core::default::Default::default(),
            static_node_count: core::default::Default::default(),
            compute_instance: core::default::Default::default(),
            storage_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_NodeSetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_Orchestrator_Slurm_NodeSetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_Orchestrator_Slurm_NodeSetsRef {
        HypercomputeclusterCluster_Orchestrator_Slurm_NodeSetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_NodeSetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compute_id` after provisioning.\nID of the compute resource on which this nodeset will run. Must match a key\nin the cluster's [compute_resources](Cluster.compute_resources)."]
    pub fn compute_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIdentifier for the nodeset, which allows it to be referenced by partitions.\nMust conform to\n[RFC-1034](https://datatracker.ietf.org/doc/html/rfc1034) (lower-case,\nalphanumeric, and at most 63 characters)."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_dynamic_node_count` after provisioning.\nControls how many additional nodes a cluster can bring online to handle\nworkloads. Set this value to enable dynamic node creation and limit the\nnumber of additional nodes the cluster can bring online. Leave empty if you\ndo not want the cluster to create nodes dynamically, and instead rely only\non static nodes."]
    pub fn max_dynamic_node_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_dynamic_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `static_node_count` after provisioning.\nNumber of nodes to be statically created for this nodeset. The cluster will\nattempt to ensure that at least this many nodes exist at all times."]
    pub fn static_node_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_node_count", self.base))
    }

    #[doc = "Get a reference to the value of field `compute_instance` after provisioning.\n"]
    pub fn compute_instance(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_ComputeInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_configs` after provisioning.\n"]
    pub fn storage_configs(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets_StorageConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_Partitions {
    id: terrars::PrimField<std::string::String>,
    node_set_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_Partitions { }

impl terrars::ToListMappable for HypercomputeclusterCluster_Orchestrator_Slurm_Partitions {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_Partitions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_Orchestrator_Slurm_Partitions {
    #[doc =
        "ID of the partition, which is how users will identify it. Must conform to\n[RFC-1034](https://datatracker.ietf.org/doc/html/rfc1034) (lower-case,\nalphanumeric, and at most 63 characters)."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "IDs of the nodesets that make up this partition. Values must match\nSlurmNodeSet.id."]
    pub node_set_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildHypercomputeclusterCluster_Orchestrator_Slurm_Partitions {
    pub fn build(self) -> HypercomputeclusterCluster_Orchestrator_Slurm_Partitions {
        HypercomputeclusterCluster_Orchestrator_Slurm_Partitions {
            id: self.id,
            node_set_ids: self.node_set_ids,
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm_PartitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_Orchestrator_Slurm_PartitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_Orchestrator_Slurm_PartitionsRef {
        HypercomputeclusterCluster_Orchestrator_Slurm_PartitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_Orchestrator_Slurm_PartitionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nID of the partition, which is how users will identify it. Must conform to\n[RFC-1034](https://datatracker.ietf.org/doc/html/rfc1034) (lower-case,\nalphanumeric, and at most 63 characters)."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_set_ids` after provisioning.\nIDs of the nodesets that make up this partition. Values must match\nSlurmNodeSet.id."]
    pub fn node_set_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_set_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_Orchestrator_SlurmDynamic {
    login_nodes: std::option::Option<terrars::DynamicBlock<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes>>,
    node_sets: std::option::Option<terrars::DynamicBlock<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets>>,
    partitions: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_Orchestrator_Slurm_Partitions>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_Orchestrator_Slurm {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_partition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    epilog_bash_scripts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prolog_bash_scripts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_nodes: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_sets: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partitions: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_Orchestrator_Slurm_Partitions>>,
    dynamic: HypercomputeclusterCluster_Orchestrator_SlurmDynamic,
}

impl HypercomputeclusterCluster_Orchestrator_Slurm {
    #[doc =
        "Set the field `default_partition`.\nDefault partition to use for submitted jobs that do not explicitly specify\na partition. Required if and only if there is more than one partition, in\nwhich case it must match the id of one of the partitions."]
    pub fn set_default_partition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_partition = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `epilog_bash_scripts`.\nSlurm [epilog scripts](https://slurm.schedmd.com/prolog_epilog.html), which\nwill be executed by compute nodes whenever a node finishes running a job.\nValues must not be empty."]
    pub fn set_epilog_bash_scripts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.epilog_bash_scripts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prolog_bash_scripts`.\nSlurm [prolog scripts](https://slurm.schedmd.com/prolog_epilog.html), which\nwill be executed by compute nodes before a node begins running a new job.\nValues must not be empty."]
    pub fn set_prolog_bash_scripts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.prolog_bash_scripts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `login_nodes`.\n"]
    pub fn set_login_nodes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.login_nodes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.login_nodes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_sets`.\n"]
    pub fn set_node_sets(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_sets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_sets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `partitions`.\n"]
    pub fn set_partitions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm_Partitions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.partitions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.partitions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_Orchestrator_Slurm {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_Orchestrator_Slurm {}

impl BuildHypercomputeclusterCluster_Orchestrator_Slurm {
    pub fn build(self) -> HypercomputeclusterCluster_Orchestrator_Slurm {
        HypercomputeclusterCluster_Orchestrator_Slurm {
            default_partition: core::default::Default::default(),
            epilog_bash_scripts: core::default::Default::default(),
            prolog_bash_scripts: core::default::Default::default(),
            login_nodes: core::default::Default::default(),
            node_sets: core::default::Default::default(),
            partitions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_Orchestrator_SlurmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_Orchestrator_SlurmRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_Orchestrator_SlurmRef {
        HypercomputeclusterCluster_Orchestrator_SlurmRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_Orchestrator_SlurmRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_partition` after provisioning.\nDefault partition to use for submitted jobs that do not explicitly specify\na partition. Required if and only if there is more than one partition, in\nwhich case it must match the id of one of the partitions."]
    pub fn default_partition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_partition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `epilog_bash_scripts` after provisioning.\nSlurm [epilog scripts](https://slurm.schedmd.com/prolog_epilog.html), which\nwill be executed by compute nodes whenever a node finishes running a job.\nValues must not be empty."]
    pub fn epilog_bash_scripts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.epilog_bash_scripts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prolog_bash_scripts` after provisioning.\nSlurm [prolog scripts](https://slurm.schedmd.com/prolog_epilog.html), which\nwill be executed by compute nodes before a node begins running a new job.\nValues must not be empty."]
    pub fn prolog_bash_scripts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prolog_bash_scripts", self.base))
    }

    #[doc = "Get a reference to the value of field `login_nodes` after provisioning.\n"]
    pub fn login_nodes(&self) -> terrars::ListRef<HypercomputeclusterCluster_Orchestrator_Slurm_LoginNodesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.login_nodes", self.base))
    }

    #[doc = "Get a reference to the value of field `node_sets` after provisioning.\n"]
    pub fn node_sets(&self) -> terrars::ListRef<HypercomputeclusterCluster_Orchestrator_Slurm_NodeSetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_sets", self.base))
    }

    #[doc = "Get a reference to the value of field `partitions` after provisioning.\n"]
    pub fn partitions(&self) -> terrars::ListRef<HypercomputeclusterCluster_Orchestrator_Slurm_PartitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.partitions", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_OrchestratorDynamic {
    slurm: std::option::Option<terrars::DynamicBlock<HypercomputeclusterCluster_Orchestrator_Slurm>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_Orchestrator {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slurm: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_Orchestrator_Slurm>>,
    dynamic: HypercomputeclusterCluster_OrchestratorDynamic,
}

impl HypercomputeclusterCluster_Orchestrator {
    #[doc = "Set the field `slurm`.\n"]
    pub fn set_slurm(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator_Slurm>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slurm = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slurm = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_Orchestrator {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_Orchestrator>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_Orchestrator {}

impl BuildHypercomputeclusterCluster_Orchestrator {
    pub fn build(self) -> HypercomputeclusterCluster_Orchestrator {
        HypercomputeclusterCluster_Orchestrator {
            slurm: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_OrchestratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_OrchestratorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HypercomputeclusterCluster_OrchestratorRef {
        HypercomputeclusterCluster_OrchestratorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_OrchestratorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `slurm` after provisioning.\n"]
    pub fn slurm(&self) -> terrars::ListRef<HypercomputeclusterCluster_Orchestrator_SlurmRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slurm", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Bucket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HypercomputeclusterCluster_StorageResources_Bucket {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Bucket {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Bucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Bucket {}

impl BuildHypercomputeclusterCluster_StorageResources_Bucket {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Bucket {
        HypercomputeclusterCluster_StorageResources_Bucket { bucket: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_BucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_BucketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_BucketRef {
        HypercomputeclusterCluster_StorageResources_BucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_BucketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Filestore {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filestore: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HypercomputeclusterCluster_StorageResources_Filestore {
    #[doc = "Set the field `filestore`.\n"]
    pub fn set_filestore(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filestore = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Filestore {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Filestore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Filestore {}

impl BuildHypercomputeclusterCluster_StorageResources_Filestore {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Filestore {
        HypercomputeclusterCluster_StorageResources_Filestore { filestore: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_FilestoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_FilestoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_FilestoreRef {
        HypercomputeclusterCluster_StorageResources_FilestoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_FilestoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filestore` after provisioning.\n"]
    pub fn filestore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filestore", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Lustre {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lustre: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HypercomputeclusterCluster_StorageResources_Lustre {
    #[doc = "Set the field `lustre`.\n"]
    pub fn set_lustre(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lustre = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Lustre {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Lustre>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Lustre {}

impl BuildHypercomputeclusterCluster_StorageResources_Lustre {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Lustre {
        HypercomputeclusterCluster_StorageResources_Lustre { lustre: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_LustreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_LustreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_LustreRef {
        HypercomputeclusterCluster_StorageResources_LustreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_LustreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lustre` after provisioning.\n"]
    pub fn lustre(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lustre", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Config_ExistingBucket {
    bucket: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_StorageResources_Config_ExistingBucket { }

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Config_ExistingBucket {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_ExistingBucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Config_ExistingBucket {
    #[doc = "Name of the Cloud Storage bucket to import."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_StorageResources_Config_ExistingBucket {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Config_ExistingBucket {
        HypercomputeclusterCluster_StorageResources_Config_ExistingBucket { bucket: self.bucket }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_Config_ExistingBucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_Config_ExistingBucketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_Config_ExistingBucketRef {
        HypercomputeclusterCluster_StorageResources_Config_ExistingBucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_Config_ExistingBucketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nName of the Cloud Storage bucket to import."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Config_ExistingFilestore {
    filestore: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_StorageResources_Config_ExistingFilestore { }

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Config_ExistingFilestore {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_ExistingFilestore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Config_ExistingFilestore {
    #[doc =
        "Name of the Filestore instance to import, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub filestore: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_StorageResources_Config_ExistingFilestore {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Config_ExistingFilestore {
        HypercomputeclusterCluster_StorageResources_Config_ExistingFilestore { filestore: self.filestore }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_Config_ExistingFilestoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_Config_ExistingFilestoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_Config_ExistingFilestoreRef {
        HypercomputeclusterCluster_StorageResources_Config_ExistingFilestoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_Config_ExistingFilestoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filestore` after provisioning.\nName of the Filestore instance to import, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub fn filestore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filestore", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Config_ExistingLustre {
    lustre: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_StorageResources_Config_ExistingLustre { }

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Config_ExistingLustre {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_ExistingLustre>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Config_ExistingLustre {
    #[doc =
        "Name of the Managed Lustre instance to import, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub lustre: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_StorageResources_Config_ExistingLustre {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Config_ExistingLustre {
        HypercomputeclusterCluster_StorageResources_Config_ExistingLustre { lustre: self.lustre }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_Config_ExistingLustreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_Config_ExistingLustreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_Config_ExistingLustreRef {
        HypercomputeclusterCluster_StorageResources_Config_ExistingLustreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_Config_ExistingLustreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `lustre` after provisioning.\nName of the Managed Lustre instance to import, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub fn lustre(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lustre", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminal_storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass {
    #[doc =
        "Set the field `terminal_storage_class`.\nTerminal storage class of the autoclass bucket\nPossible values:\nNEARLINE\nARCHIVE"]
    pub fn set_terminal_storage_class(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.terminal_storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass {
    #[doc = "Enables Auto-class feature."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildHypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass {
        HypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass {
            enabled: self.enabled,
            terminal_storage_class: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewBucket_AutoclassRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_Config_NewBucket_AutoclassRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_Config_NewBucket_AutoclassRef {
        HypercomputeclusterCluster_StorageResources_Config_NewBucket_AutoclassRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_Config_NewBucket_AutoclassRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nEnables Auto-class feature."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `terminal_storage_class` after provisioning.\nTerminal storage class of the autoclass bucket\nPossible values:\nNEARLINE\nARCHIVE"]
    pub fn terminal_storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.terminal_storage_class", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace {
    #[doc = "Set the field `enabled`.\nEnables hierarchical namespace setup for the bucket."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace {
    type O =
        terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace {}

impl BuildHypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace {
        HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespaceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespaceRef {
        HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnables hierarchical namespace setup for the bucket."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_StorageResources_Config_NewBucketDynamic {
    autoclass: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass>>,
    hierarchical_namespace: std
    ::option
    ::Option<
        terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace>,
    >,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewBucket {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoclass: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hierarchical_namespace: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace>>,
    dynamic: HypercomputeclusterCluster_StorageResources_Config_NewBucketDynamic,
}

impl HypercomputeclusterCluster_StorageResources_Config_NewBucket {
    #[doc =
        "Set the field `storage_class`.\nIf set, uses the provided storage class as the bucket's default storage\nclass.\nPossible values:\nSTANDARD\nNEARLINE\nCOLDLINE\nARCHIVE"]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoclass`.\n"]
    pub fn set_autoclass(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewBucket_Autoclass>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoclass = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoclass = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hierarchical_namespace`.\n"]
    pub fn set_hierarchical_namespace(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespace,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hierarchical_namespace = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hierarchical_namespace = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Config_NewBucket {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewBucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Config_NewBucket {
    #[doc = "Name of the Cloud Storage bucket to create."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_StorageResources_Config_NewBucket {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Config_NewBucket {
        HypercomputeclusterCluster_StorageResources_Config_NewBucket {
            bucket: self.bucket,
            storage_class: core::default::Default::default(),
            autoclass: core::default::Default::default(),
            hierarchical_namespace: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewBucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_Config_NewBucketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_Config_NewBucketRef {
        HypercomputeclusterCluster_StorageResources_Config_NewBucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_Config_NewBucketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nName of the Cloud Storage bucket to create."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_class` after provisioning.\nIf set, uses the provided storage class as the bucket's default storage\nclass.\nPossible values:\nSTANDARD\nNEARLINE\nCOLDLINE\nARCHIVE"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }

    #[doc = "Get a reference to the value of field `autoclass` after provisioning.\n"]
    pub fn autoclass(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_Config_NewBucket_AutoclassRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoclass", self.base))
    }

    #[doc = "Get a reference to the value of field `hierarchical_namespace` after provisioning.\n"]
    pub fn hierarchical_namespace(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_Config_NewBucket_HierarchicalNamespaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchical_namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares {
    capacity_gb: terrars::PrimField<std::string::String>,
    file_share: terrars::PrimField<std::string::String>,
}

impl HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares { }

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares {
    #[doc =
        "Size of the filestore in GB. Must be between 1024 and 102400, and must meet\nscalability requirements described at\nhttps://cloud.google.com/filestore/docs/service-tiers."]
    pub capacity_gb: terrars::PrimField<std::string::String>,
    #[doc = "Filestore share location"]
    pub file_share: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares {
        HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares {
            capacity_gb: self.capacity_gb,
            file_share: self.file_share,
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileSharesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileSharesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileSharesRef {
        HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileSharesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileSharesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `capacity_gb` after provisioning.\nSize of the filestore in GB. Must be between 1024 and 102400, and must meet\nscalability requirements described at\nhttps://cloud.google.com/filestore/docs/service-tiers."]
    pub fn capacity_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `file_share` after provisioning.\nFilestore share location"]
    pub fn file_share(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_share", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_StorageResources_Config_NewFilestoreDynamic {
    file_shares: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewFilestore {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    filestore: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    tier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_shares: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares>>,
    dynamic: HypercomputeclusterCluster_StorageResources_Config_NewFilestoreDynamic,
}

impl HypercomputeclusterCluster_StorageResources_Config_NewFilestore {
    #[doc = "Set the field `description`.\nDescription of the instance. Maximum of 2048 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `protocol`.\nAccess protocol to use for all file shares in the instance. Defaults to NFS\nV3 if not set.\nPossible values:\nNFSV3\nNFSV41 Possible values: [\"PROTOCOL_UNSPECIFIED\", \"NFSV3\", \"NFSV41\"]"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_shares`.\n"]
    pub fn set_file_shares(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileShares>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_shares = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_shares = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Config_NewFilestore {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewFilestore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Config_NewFilestore {
    #[doc =
        "Name of the Filestore instance to create, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub filestore: terrars::PrimField<std::string::String>,
    #[doc =
        "Service tier to use for the instance.\nPossible values:\nZONAL\nREGIONAL Possible values: [\"TIER_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub tier: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_StorageResources_Config_NewFilestore {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Config_NewFilestore {
        HypercomputeclusterCluster_StorageResources_Config_NewFilestore {
            description: core::default::Default::default(),
            filestore: self.filestore,
            protocol: core::default::Default::default(),
            tier: self.tier,
            file_shares: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewFilestoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_Config_NewFilestoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_Config_NewFilestoreRef {
        HypercomputeclusterCluster_StorageResources_Config_NewFilestoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_Config_NewFilestoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the instance. Maximum of 2048 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filestore` after provisioning.\nName of the Filestore instance to create, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub fn filestore(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filestore", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nAccess protocol to use for all file shares in the instance. Defaults to NFS\nV3 if not set.\nPossible values:\nNFSV3\nNFSV41 Possible values: [\"PROTOCOL_UNSPECIFIED\", \"NFSV3\", \"NFSV41\"]"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nService tier to use for the instance.\nPossible values:\nZONAL\nREGIONAL Possible values: [\"TIER_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }

    #[doc = "Get a reference to the value of field `file_shares` after provisioning.\n"]
    pub fn file_shares(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_Config_NewFilestore_FileSharesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_shares", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewLustre {
    capacity_gb: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    filesystem: terrars::PrimField<std::string::String>,
    lustre: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_unit_storage_throughput: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HypercomputeclusterCluster_StorageResources_Config_NewLustre {
    #[doc = "Set the field `description`.\nDescription of the Managed Lustre instance. Maximum of 2048 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `per_unit_storage_throughput`.\nThroughput of the instance in MB/s/TiB. Valid values are 125, 250,\n500, 1000. See [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information."]
    pub fn set_per_unit_storage_throughput(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.per_unit_storage_throughput = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Config_NewLustre {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewLustre>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Config_NewLustre {
    #[doc = "Storage capacity of the instance in gibibytes (GiB). Allowed values are\nbetween 18000 and 7632000."]
    pub capacity_gb: terrars::PrimField<std::string::String>,
    #[doc =
        "Filesystem name for this instance. This name is used by client-side tools,\nincluding when mounting the instance. Must be 8 characters or less and can\nonly contain letters and numbers."]
    pub filesystem: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the Managed Lustre instance to create, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub lustre: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_StorageResources_Config_NewLustre {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Config_NewLustre {
        HypercomputeclusterCluster_StorageResources_Config_NewLustre {
            capacity_gb: self.capacity_gb,
            description: core::default::Default::default(),
            filesystem: self.filesystem,
            lustre: self.lustre,
            per_unit_storage_throughput: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_Config_NewLustreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_Config_NewLustreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_Config_NewLustreRef {
        HypercomputeclusterCluster_StorageResources_Config_NewLustreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_Config_NewLustreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `capacity_gb` after provisioning.\nStorage capacity of the instance in gibibytes (GiB). Allowed values are\nbetween 18000 and 7632000."]
    pub fn capacity_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the Managed Lustre instance. Maximum of 2048 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filesystem` after provisioning.\nFilesystem name for this instance. This name is used by client-side tools,\nincluding when mounting the instance. Must be 8 characters or less and can\nonly contain letters and numbers."]
    pub fn filesystem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filesystem", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lustre` after provisioning.\nName of the Managed Lustre instance to create, in the format\n'projects/{project}/locations/{location}/instances/{instance}'"]
    pub fn lustre(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lustre", self.base))
    }

    #[doc =
        "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\nThroughput of the instance in MB/s/TiB. Valid values are 125, 250,\n500, 1000. See [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information."]
    pub fn per_unit_storage_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_unit_storage_throughput", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_StorageResources_ConfigDynamic {
    existing_bucket: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources_Config_ExistingBucket>>,
    existing_filestore: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources_Config_ExistingFilestore>>,
    existing_lustre: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources_Config_ExistingLustre>>,
    new_bucket: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources_Config_NewBucket>>,
    new_filestore: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources_Config_NewFilestore>>,
    new_lustre: std
    ::option
    ::Option<terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources_Config_NewLustre>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existing_bucket: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources_Config_ExistingBucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existing_filestore: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources_Config_ExistingFilestore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    existing_lustre: std
    ::option
    ::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources_Config_ExistingLustre>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_bucket: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources_Config_NewBucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_filestore: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources_Config_NewFilestore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_lustre: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources_Config_NewLustre>>,
    dynamic: HypercomputeclusterCluster_StorageResources_ConfigDynamic,
}

impl HypercomputeclusterCluster_StorageResources_Config {
    #[doc = "Set the field `existing_bucket`.\n"]
    pub fn set_existing_bucket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_ExistingBucket>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.existing_bucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_bucket = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `existing_filestore`.\n"]
    pub fn set_existing_filestore(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_ExistingFilestore>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.existing_filestore = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_filestore = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `existing_lustre`.\n"]
    pub fn set_existing_lustre(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_ExistingLustre>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.existing_lustre = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.existing_lustre = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `new_bucket`.\n"]
    pub fn set_new_bucket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewBucket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.new_bucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.new_bucket = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `new_filestore`.\n"]
    pub fn set_new_filestore(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewFilestore>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.new_filestore = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.new_filestore = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `new_lustre`.\n"]
    pub fn set_new_lustre(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config_NewLustre>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.new_lustre = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.new_lustre = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources_Config {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources_Config {}

impl BuildHypercomputeclusterCluster_StorageResources_Config {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources_Config {
        HypercomputeclusterCluster_StorageResources_Config {
            existing_bucket: core::default::Default::default(),
            existing_filestore: core::default::Default::default(),
            existing_lustre: core::default::Default::default(),
            new_bucket: core::default::Default::default(),
            new_filestore: core::default::Default::default(),
            new_lustre: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResources_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResources_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResources_ConfigRef {
        HypercomputeclusterCluster_StorageResources_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResources_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `existing_bucket` after provisioning.\n"]
    pub fn existing_bucket(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_Config_ExistingBucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.existing_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `existing_filestore` after provisioning.\n"]
    pub fn existing_filestore(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_Config_ExistingFilestoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.existing_filestore", self.base))
    }

    #[doc = "Get a reference to the value of field `existing_lustre` after provisioning.\n"]
    pub fn existing_lustre(
        &self,
    ) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_Config_ExistingLustreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.existing_lustre", self.base))
    }

    #[doc = "Get a reference to the value of field `new_bucket` after provisioning.\n"]
    pub fn new_bucket(&self) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_Config_NewBucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `new_filestore` after provisioning.\n"]
    pub fn new_filestore(&self) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_Config_NewFilestoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_filestore", self.base))
    }

    #[doc = "Get a reference to the value of field `new_lustre` after provisioning.\n"]
    pub fn new_lustre(&self) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_Config_NewLustreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.new_lustre", self.base))
    }
}

#[derive(Serialize, Default)]
struct HypercomputeclusterCluster_StorageResourcesDynamic {
    config: std::option::Option<terrars::DynamicBlock<HypercomputeclusterCluster_StorageResources_Config>>,
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_StorageResources {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<HypercomputeclusterCluster_StorageResources_Config>>,
    dynamic: HypercomputeclusterCluster_StorageResourcesDynamic,
}

impl HypercomputeclusterCluster_StorageResources {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources_Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HypercomputeclusterCluster_StorageResources {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_StorageResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_StorageResources {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildHypercomputeclusterCluster_StorageResources {
    pub fn build(self) -> HypercomputeclusterCluster_StorageResources {
        HypercomputeclusterCluster_StorageResources {
            id: self.id,
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_StorageResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_StorageResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HypercomputeclusterCluster_StorageResourcesRef {
        HypercomputeclusterCluster_StorageResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_StorageResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nA reference to a [Google Cloud Storage](https://cloud.google.com/storage)\nbucket."]
    pub fn bucket(&self) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_BucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filestore` after provisioning.\nA reference to a [Filestore](https://cloud.google.com/filestore) instance."]
    pub fn filestore(&self) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_FilestoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filestore", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `lustre` after provisioning.\nA reference to a [Managed\nLustre](https://cloud.google.com/products/managed-lustre) instance."]
    pub fn lustre(&self) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_LustreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lustre", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<HypercomputeclusterCluster_StorageResources_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize)]
pub struct HypercomputeclusterCluster_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HypercomputeclusterCluster_Timeouts {
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

impl terrars::ToListMappable for HypercomputeclusterCluster_Timeouts {
    type O = terrars::BlockAssignable<HypercomputeclusterCluster_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHypercomputeclusterCluster_Timeouts {}

impl BuildHypercomputeclusterCluster_Timeouts {
    pub fn build(self) -> HypercomputeclusterCluster_Timeouts {
        HypercomputeclusterCluster_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HypercomputeclusterCluster_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HypercomputeclusterCluster_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HypercomputeclusterCluster_TimeoutsRef {
        HypercomputeclusterCluster_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HypercomputeclusterCluster_TimeoutsRef {
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
