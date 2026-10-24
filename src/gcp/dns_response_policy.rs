use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DnsResponsePolicyData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    response_policy_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_clusters: std::option::Option<std::vec::Vec<DnsResponsePolicyGkeClustersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    networks: std::option::Option<std::vec::Vec<DnsResponsePolicyNetworksEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DnsResponsePolicyTimeoutsEl>,
    dynamic: DnsResponsePolicyDynamic,
}

struct DnsResponsePolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DnsResponsePolicyData>,
}

#[derive(Clone)]
pub struct DnsResponsePolicy(std::rc::Rc<DnsResponsePolicy_>);

impl DnsResponsePolicy {
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

    #[doc = "Set the field `description`.\nThe description of the response policy, such as 'My new response policy'."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gke_clusters`.\n"]
    pub fn set_gke_clusters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsResponsePolicyGkeClustersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gke_clusters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gke_clusters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `networks`.\n"]
    pub fn set_networks(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsResponsePolicyNetworksEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DnsResponsePolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the response policy, such as 'My new response policy'."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `response_policy_name` after provisioning.\nThe user assigned name for this Response Policy, such as 'myresponsepolicy'."]
    pub fn response_policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_policy_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gke_clusters` after provisioning.\n"]
    pub fn gke_clusters(&self) -> terrars::ListRef<DnsResponsePolicyGkeClustersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> terrars::ListRef<DnsResponsePolicyNetworksElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsResponsePolicyTimeoutsElRef {
        DnsResponsePolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DnsResponsePolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DnsResponsePolicy { }

impl terrars::ToListMappable for DnsResponsePolicy {
    type O = terrars::ListRef<DnsResponsePolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DnsResponsePolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dns_response_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDnsResponsePolicy {
    pub tf_id: std::string::String,
    #[doc = "The user assigned name for this Response Policy, such as 'myresponsepolicy'."]
    pub response_policy_name: terrars::PrimField<std::string::String>,
}

impl BuildDnsResponsePolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DnsResponsePolicy {
        let out = DnsResponsePolicy(std::rc::Rc::new(DnsResponsePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DnsResponsePolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                response_policy_name: self.response_policy_name,
                gke_clusters: core::default::Default::default(),
                networks: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DnsResponsePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DnsResponsePolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the response policy, such as 'My new response policy'."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `response_policy_name` after provisioning.\nThe user assigned name for this Response Policy, such as 'myresponsepolicy'."]
    pub fn response_policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_policy_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gke_clusters` after provisioning.\n"]
    pub fn gke_clusters(&self) -> terrars::ListRef<DnsResponsePolicyGkeClustersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> terrars::ListRef<DnsResponsePolicyNetworksElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsResponsePolicyTimeoutsElRef {
        DnsResponsePolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DnsResponsePolicyGkeClustersEl {
    gke_cluster_name: terrars::PrimField<std::string::String>,
}

impl DnsResponsePolicyGkeClustersEl { }

impl terrars::ToListMappable for DnsResponsePolicyGkeClustersEl {
    type O = terrars::BlockAssignable<DnsResponsePolicyGkeClustersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsResponsePolicyGkeClustersEl {
    #[doc =
        "The resource name of the cluster to bind this ManagedZone to.\nThis should be specified in the format like\n'projects/*/locations/*/clusters/*'"]
    pub gke_cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildDnsResponsePolicyGkeClustersEl {
    pub fn build(self) -> DnsResponsePolicyGkeClustersEl {
        DnsResponsePolicyGkeClustersEl { gke_cluster_name: self.gke_cluster_name }
    }
}

pub struct DnsResponsePolicyGkeClustersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyGkeClustersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsResponsePolicyGkeClustersElRef {
        DnsResponsePolicyGkeClustersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsResponsePolicyGkeClustersElRef {
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
pub struct DnsResponsePolicyNetworksEl {
    network_url: terrars::PrimField<std::string::String>,
}

impl DnsResponsePolicyNetworksEl { }

impl terrars::ToListMappable for DnsResponsePolicyNetworksEl {
    type O = terrars::BlockAssignable<DnsResponsePolicyNetworksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsResponsePolicyNetworksEl {
    #[doc =
        "The fully qualified URL of the VPC network to bind to.\nThis should be formatted like\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub network_url: terrars::PrimField<std::string::String>,
}

impl BuildDnsResponsePolicyNetworksEl {
    pub fn build(self) -> DnsResponsePolicyNetworksEl {
        DnsResponsePolicyNetworksEl { network_url: self.network_url }
    }
}

pub struct DnsResponsePolicyNetworksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyNetworksElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsResponsePolicyNetworksElRef {
        DnsResponsePolicyNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsResponsePolicyNetworksElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_url` after provisioning.\nThe fully qualified URL of the VPC network to bind to.\nThis should be formatted like\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub fn network_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsResponsePolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsResponsePolicyTimeoutsEl {
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

impl terrars::ToListMappable for DnsResponsePolicyTimeoutsEl {
    type O = terrars::BlockAssignable<DnsResponsePolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsResponsePolicyTimeoutsEl {}

impl BuildDnsResponsePolicyTimeoutsEl {
    pub fn build(self) -> DnsResponsePolicyTimeoutsEl {
        DnsResponsePolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DnsResponsePolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsResponsePolicyTimeoutsElRef {
        DnsResponsePolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsResponsePolicyTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct DnsResponsePolicyDynamic {
    gke_clusters: std::option::Option<terrars::DynamicBlock<DnsResponsePolicyGkeClustersEl>>,
    networks: std::option::Option<terrars::DynamicBlock<DnsResponsePolicyNetworksEl>>,
}
