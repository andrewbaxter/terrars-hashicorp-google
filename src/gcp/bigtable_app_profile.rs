#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigtableAppProfileData_ {
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
    app_profile_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_warnings: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_cluster_routing_cluster_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_cluster_routing_use_any: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_affinity: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_boost_isolation_read_only: std::option::Option<std::vec::Vec<BigtableAppProfile_DataBoostIsolationReadOnly>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_cluster_routing: std::option::Option<std::vec::Vec<BigtableAppProfile_SingleClusterRouting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_isolation: std::option::Option<std::vec::Vec<BigtableAppProfile_StandardIsolation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigtableAppProfile_Timeouts>,
    dynamic: BigtableAppProfileDynamic,
}

struct BigtableAppProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigtableAppProfileData_>,
}

#[derive(Clone)]
pub struct BigtableAppProfile(std::rc::Rc<BigtableAppProfile_>);

impl BigtableAppProfile {
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

    #[doc = "Set the field `description`.\nLong form description of the use case for this app profile."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_warnings`.\nIf true, ignore safety checks when deleting/updating the app profile."]
    pub fn set_ignore_warnings(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ignore_warnings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance`.\nThe name of the instance to create the app profile within."]
    pub fn set_instance(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `multi_cluster_routing_cluster_ids`.\nThe set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all clusters are eligible."]
    pub fn set_multi_cluster_routing_cluster_ids(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().multi_cluster_routing_cluster_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `multi_cluster_routing_use_any`.\nIf true, read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available\nin the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes\nconsistency to improve availability."]
    pub fn set_multi_cluster_routing_use_any(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_cluster_routing_use_any = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `row_affinity`.\nMust be used with multi-cluster routing. If true, then this app profile will use row affinity sticky routing. With row affinity, Bigtable will route single row key requests based on the row key, rather than randomly. Instead, each row key will be assigned to a cluster by Cloud Bigtable, and will stick to that cluster. Choosing this option improves read-your-writes consistency for most requests under most circumstances, without sacrificing availability. Consistency is not guaranteed, as requests may still fail over between clusters in the event of errors or latency."]
    pub fn set_row_affinity(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().row_affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_boost_isolation_read_only`.\n"]
    pub fn set_data_boost_isolation_read_only(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigtableAppProfile_DataBoostIsolationReadOnly>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_boost_isolation_read_only = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_boost_isolation_read_only = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `single_cluster_routing`.\n"]
    pub fn set_single_cluster_routing(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigtableAppProfile_SingleClusterRouting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().single_cluster_routing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.single_cluster_routing = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `standard_isolation`.\n"]
    pub fn set_standard_isolation(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigtableAppProfile_StandardIsolation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().standard_isolation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.standard_isolation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigtableAppProfile_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `app_profile_id` after provisioning.\nThe unique name of the app profile in the form '[_a-zA-Z0-9][-_.a-zA-Z0-9]*'."]
    pub fn app_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_profile_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nLong form description of the use case for this app profile."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_warnings` after provisioning.\nIf true, ignore safety checks when deleting/updating the app profile."]
    pub fn ignore_warnings(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_warnings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe name of the instance to create the app profile within."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `multi_cluster_routing_cluster_ids` after provisioning.\nThe set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all clusters are eligible."]
    pub fn multi_cluster_routing_cluster_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.multi_cluster_routing_cluster_ids", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `multi_cluster_routing_use_any` after provisioning.\nIf true, read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available\nin the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes\nconsistency to improve availability."]
    pub fn multi_cluster_routing_use_any(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_cluster_routing_use_any", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique name of the requested app profile. Values are of the form 'projects/<project>/instances/<instance>/appProfiles/<appProfileId>'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `row_affinity` after provisioning.\nMust be used with multi-cluster routing. If true, then this app profile will use row affinity sticky routing. With row affinity, Bigtable will route single row key requests based on the row key, rather than randomly. Instead, each row key will be assigned to a cluster by Cloud Bigtable, and will stick to that cluster. Choosing this option improves read-your-writes consistency for most requests under most circumstances, without sacrificing availability. Consistency is not guaranteed, as requests may still fail over between clusters in the event of errors or latency."]
    pub fn row_affinity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_affinity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_boost_isolation_read_only` after provisioning.\n"]
    pub fn data_boost_isolation_read_only(&self) -> terrars::ListRef<BigtableAppProfile_DataBoostIsolationReadOnlyRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_boost_isolation_read_only", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `single_cluster_routing` after provisioning.\n"]
    pub fn single_cluster_routing(&self) -> terrars::ListRef<BigtableAppProfile_SingleClusterRoutingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_cluster_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standard_isolation` after provisioning.\n"]
    pub fn standard_isolation(&self) -> terrars::ListRef<BigtableAppProfile_StandardIsolationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_isolation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableAppProfile_TimeoutsRef {
        BigtableAppProfile_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BigtableAppProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigtableAppProfile { }

impl terrars::ToListMappable for BigtableAppProfile {
    type O = terrars::ListRef<BigtableAppProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigtableAppProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigtable_app_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigtableAppProfile {
    pub tf_id: std::string::String,
    #[doc = "The unique name of the app profile in the form '[_a-zA-Z0-9][-_.a-zA-Z0-9]*'."]
    pub app_profile_id: terrars::PrimField<std::string::String>,
}

impl BuildBigtableAppProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> BigtableAppProfile {
        let out = BigtableAppProfile(std::rc::Rc::new(BigtableAppProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigtableAppProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_profile_id: self.app_profile_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_warnings: core::default::Default::default(),
                instance: core::default::Default::default(),
                multi_cluster_routing_cluster_ids: core::default::Default::default(),
                multi_cluster_routing_use_any: core::default::Default::default(),
                project: core::default::Default::default(),
                row_affinity: core::default::Default::default(),
                data_boost_isolation_read_only: core::default::Default::default(),
                single_cluster_routing: core::default::Default::default(),
                standard_isolation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigtableAppProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableAppProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigtableAppProfileRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_profile_id` after provisioning.\nThe unique name of the app profile in the form '[_a-zA-Z0-9][-_.a-zA-Z0-9]*'."]
    pub fn app_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_profile_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nLong form description of the use case for this app profile."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_warnings` after provisioning.\nIf true, ignore safety checks when deleting/updating the app profile."]
    pub fn ignore_warnings(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_warnings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe name of the instance to create the app profile within."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `multi_cluster_routing_cluster_ids` after provisioning.\nThe set of clusters to route to. The order is ignored; clusters will be tried in order of distance. If left empty, all clusters are eligible."]
    pub fn multi_cluster_routing_cluster_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.multi_cluster_routing_cluster_ids", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `multi_cluster_routing_use_any` after provisioning.\nIf true, read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available\nin the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes\nconsistency to improve availability."]
    pub fn multi_cluster_routing_use_any(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_cluster_routing_use_any", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique name of the requested app profile. Values are of the form 'projects/<project>/instances/<instance>/appProfiles/<appProfileId>'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `row_affinity` after provisioning.\nMust be used with multi-cluster routing. If true, then this app profile will use row affinity sticky routing. With row affinity, Bigtable will route single row key requests based on the row key, rather than randomly. Instead, each row key will be assigned to a cluster by Cloud Bigtable, and will stick to that cluster. Choosing this option improves read-your-writes consistency for most requests under most circumstances, without sacrificing availability. Consistency is not guaranteed, as requests may still fail over between clusters in the event of errors or latency."]
    pub fn row_affinity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.row_affinity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_boost_isolation_read_only` after provisioning.\n"]
    pub fn data_boost_isolation_read_only(&self) -> terrars::ListRef<BigtableAppProfile_DataBoostIsolationReadOnlyRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.data_boost_isolation_read_only", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `single_cluster_routing` after provisioning.\n"]
    pub fn single_cluster_routing(&self) -> terrars::ListRef<BigtableAppProfile_SingleClusterRoutingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_cluster_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standard_isolation` after provisioning.\n"]
    pub fn standard_isolation(&self) -> terrars::ListRef<BigtableAppProfile_StandardIsolationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_isolation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigtableAppProfile_TimeoutsRef {
        BigtableAppProfile_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BigtableAppProfileDynamic {
    data_boost_isolation_read_only: std
    ::option
    ::Option<terrars::DynamicBlock<BigtableAppProfile_DataBoostIsolationReadOnly>>,
    single_cluster_routing: std::option::Option<terrars::DynamicBlock<BigtableAppProfile_SingleClusterRouting>>,
    standard_isolation: std::option::Option<terrars::DynamicBlock<BigtableAppProfile_StandardIsolation>>,
}

#[derive(Serialize)]
pub struct BigtableAppProfile_DataBoostIsolationReadOnly {
    compute_billing_owner: terrars::PrimField<std::string::String>,
}

impl BigtableAppProfile_DataBoostIsolationReadOnly { }

impl terrars::ToListMappable for BigtableAppProfile_DataBoostIsolationReadOnly {
    type O = terrars::BlockAssignable<BigtableAppProfile_DataBoostIsolationReadOnly>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableAppProfile_DataBoostIsolationReadOnly {
    #[doc = "The Compute Billing Owner for this Data Boost App Profile. Possible values: [\"HOST_PAYS\"]"]
    pub compute_billing_owner: terrars::PrimField<std::string::String>,
}

impl BuildBigtableAppProfile_DataBoostIsolationReadOnly {
    pub fn build(self) -> BigtableAppProfile_DataBoostIsolationReadOnly {
        BigtableAppProfile_DataBoostIsolationReadOnly { compute_billing_owner: self.compute_billing_owner }
    }
}

#[allow(dead_code)]
pub struct BigtableAppProfile_DataBoostIsolationReadOnlyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableAppProfile_DataBoostIsolationReadOnlyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigtableAppProfile_DataBoostIsolationReadOnlyRef {
        BigtableAppProfile_DataBoostIsolationReadOnlyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableAppProfile_DataBoostIsolationReadOnlyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compute_billing_owner` after provisioning.\nThe Compute Billing Owner for this Data Boost App Profile. Possible values: [\"HOST_PAYS\"]"]
    pub fn compute_billing_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_billing_owner", self.base))
    }
}

#[derive(Serialize)]
pub struct BigtableAppProfile_SingleClusterRouting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_transactional_writes: std::option::Option<terrars::PrimField<bool>>,
    cluster_id: terrars::PrimField<std::string::String>,
}

impl BigtableAppProfile_SingleClusterRouting {
    #[doc =
        "Set the field `allow_transactional_writes`.\nIf true, CheckAndMutateRow and ReadModifyWriteRow requests are allowed by this app profile.\nIt is unsafe to send these requests to the same table/row/column in multiple clusters."]
    pub fn set_allow_transactional_writes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_transactional_writes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigtableAppProfile_SingleClusterRouting {
    type O = terrars::BlockAssignable<BigtableAppProfile_SingleClusterRouting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableAppProfile_SingleClusterRouting {
    #[doc = "The cluster to which read/write requests should be routed."]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildBigtableAppProfile_SingleClusterRouting {
    pub fn build(self) -> BigtableAppProfile_SingleClusterRouting {
        BigtableAppProfile_SingleClusterRouting {
            allow_transactional_writes: core::default::Default::default(),
            cluster_id: self.cluster_id,
        }
    }
}

#[allow(dead_code)]
pub struct BigtableAppProfile_SingleClusterRoutingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableAppProfile_SingleClusterRoutingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigtableAppProfile_SingleClusterRoutingRef {
        BigtableAppProfile_SingleClusterRoutingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableAppProfile_SingleClusterRoutingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_transactional_writes` after provisioning.\nIf true, CheckAndMutateRow and ReadModifyWriteRow requests are allowed by this app profile.\nIt is unsafe to send these requests to the same table/row/column in multiple clusters."]
    pub fn allow_transactional_writes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_transactional_writes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nThe cluster to which read/write requests should be routed."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BigtableAppProfile_StandardIsolation {
    priority: terrars::PrimField<std::string::String>,
}

impl BigtableAppProfile_StandardIsolation { }

impl terrars::ToListMappable for BigtableAppProfile_StandardIsolation {
    type O = terrars::BlockAssignable<BigtableAppProfile_StandardIsolation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableAppProfile_StandardIsolation {
    #[doc =
        "The priority of requests sent using this app profile. Possible values: [\"PRIORITY_LOW\", \"PRIORITY_MEDIUM\", \"PRIORITY_HIGH\"]"]
    pub priority: terrars::PrimField<std::string::String>,
}

impl BuildBigtableAppProfile_StandardIsolation {
    pub fn build(self) -> BigtableAppProfile_StandardIsolation {
        BigtableAppProfile_StandardIsolation { priority: self.priority }
    }
}

#[allow(dead_code)]
pub struct BigtableAppProfile_StandardIsolationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableAppProfile_StandardIsolationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigtableAppProfile_StandardIsolationRef {
        BigtableAppProfile_StandardIsolationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableAppProfile_StandardIsolationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nThe priority of requests sent using this app profile. Possible values: [\"PRIORITY_LOW\", \"PRIORITY_MEDIUM\", \"PRIORITY_HIGH\"]"]
    pub fn priority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}

#[derive(Serialize)]
pub struct BigtableAppProfile_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigtableAppProfile_Timeouts {
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

impl terrars::ToListMappable for BigtableAppProfile_Timeouts {
    type O = terrars::BlockAssignable<BigtableAppProfile_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigtableAppProfile_Timeouts {}

impl BuildBigtableAppProfile_Timeouts {
    pub fn build(self) -> BigtableAppProfile_Timeouts {
        BigtableAppProfile_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigtableAppProfile_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigtableAppProfile_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigtableAppProfile_TimeoutsRef {
        BigtableAppProfile_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigtableAppProfile_TimeoutsRef {
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
