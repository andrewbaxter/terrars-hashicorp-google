use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeHubMembershipData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    membership_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authority: std::option::Option<std::vec::Vec<GkeHubMembershipAuthorityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<std::vec::Vec<GkeHubMembershipEndpointEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeHubMembershipTimeoutsEl>,
    dynamic: GkeHubMembershipDynamic,
}

struct GkeHubMembership_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeHubMembershipData>,
}

#[derive(Clone)]
pub struct GkeHubMembership(std::rc::Rc<GkeHubMembership_>);

impl GkeHubMembership {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels to apply to this membership.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nLocation of the membership.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authority`.\n"]
    pub fn set_authority(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubMembershipAuthorityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authority = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authority = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(self, v: impl std::convert::Into<terrars::BlockAssignable<GkeHubMembershipEndpointEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeHubMembershipTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this membership.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the membership.\nThe default value is 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `membership_id` after provisioning.\nThe client-provided identifier of the membership."]
    pub fn membership_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the membership."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `authority` after provisioning.\n"]
    pub fn authority(&self) -> terrars::ListRef<GkeHubMembershipAuthorityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<GkeHubMembershipEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubMembershipTimeoutsElRef {
        GkeHubMembershipTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GkeHubMembership {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeHubMembership { }

impl terrars::ToListMappable for GkeHubMembership {
    type O = terrars::ListRef<GkeHubMembershipRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeHubMembership_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gke_hub_membership".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeHubMembership {
    pub tf_id: std::string::String,
    #[doc = "The client-provided identifier of the membership."]
    pub membership_id: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubMembership {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeHubMembership {
        let out = GkeHubMembership(std::rc::Rc::new(GkeHubMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeHubMembershipData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                membership_id: self.membership_id,
                project: core::default::Default::default(),
                authority: core::default::Default::default(),
                endpoint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeHubMembershipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubMembershipRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeHubMembershipRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this membership.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the membership.\nThe default value is 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `membership_id` after provisioning.\nThe client-provided identifier of the membership."]
    pub fn membership_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.membership_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the membership."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `authority` after provisioning.\n"]
    pub fn authority(&self) -> terrars::ListRef<GkeHubMembershipAuthorityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<GkeHubMembershipEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubMembershipTimeoutsElRef {
        GkeHubMembershipTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GkeHubMembershipAuthorityEl {
    issuer: terrars::PrimField<std::string::String>,
}

impl GkeHubMembershipAuthorityEl { }

impl terrars::ToListMappable for GkeHubMembershipAuthorityEl {
    type O = terrars::BlockAssignable<GkeHubMembershipAuthorityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubMembershipAuthorityEl {
    #[doc =
        "A JSON Web Token (JWT) issuer URI. 'issuer' must start with 'https://' and // be a valid\nwith length <2000 characters. For example: 'https://container.googleapis.com/v1/projects/my-project/locations/us-west1/clusters/my-cluster' (must be 'locations' rather than 'zones'). If the cluster is provisioned with Terraform, this is '\"https://container.googleapis.com/v1/${google_container_cluster.my-cluster.id}\"'."]
    pub issuer: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubMembershipAuthorityEl {
    pub fn build(self) -> GkeHubMembershipAuthorityEl {
        GkeHubMembershipAuthorityEl { issuer: self.issuer }
    }
}

pub struct GkeHubMembershipAuthorityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubMembershipAuthorityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubMembershipAuthorityElRef {
        GkeHubMembershipAuthorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubMembershipAuthorityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `issuer` after provisioning.\nA JSON Web Token (JWT) issuer URI. 'issuer' must start with 'https://' and // be a valid\nwith length <2000 characters. For example: 'https://container.googleapis.com/v1/projects/my-project/locations/us-west1/clusters/my-cluster' (must be 'locations' rather than 'zones'). If the cluster is provisioned with Terraform, this is '\"https://container.googleapis.com/v1/${google_container_cluster.my-cluster.id}\"'."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubMembershipEndpointElGkeClusterEl {
    resource_link: terrars::PrimField<std::string::String>,
}

impl GkeHubMembershipEndpointElGkeClusterEl { }

impl terrars::ToListMappable for GkeHubMembershipEndpointElGkeClusterEl {
    type O = terrars::BlockAssignable<GkeHubMembershipEndpointElGkeClusterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubMembershipEndpointElGkeClusterEl {
    #[doc =
        "Self-link of the GCP resource for the GKE cluster.\nFor example: '//container.googleapis.com/projects/my-project/zones/us-west1-a/clusters/my-cluster'.\nIt can be at the most 1000 characters in length. If the cluster is provisioned with Terraform,\nthis can be '\"//container.googleapis.com/${google_container_cluster.my-cluster.id}\"' or\n'google_container_cluster.my-cluster.id'."]
    pub resource_link: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubMembershipEndpointElGkeClusterEl {
    pub fn build(self) -> GkeHubMembershipEndpointElGkeClusterEl {
        GkeHubMembershipEndpointElGkeClusterEl { resource_link: self.resource_link }
    }
}

pub struct GkeHubMembershipEndpointElGkeClusterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubMembershipEndpointElGkeClusterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubMembershipEndpointElGkeClusterElRef {
        GkeHubMembershipEndpointElGkeClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubMembershipEndpointElGkeClusterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_link` after provisioning.\nSelf-link of the GCP resource for the GKE cluster.\nFor example: '//container.googleapis.com/projects/my-project/zones/us-west1-a/clusters/my-cluster'.\nIt can be at the most 1000 characters in length. If the cluster is provisioned with Terraform,\nthis can be '\"//container.googleapis.com/${google_container_cluster.my-cluster.id}\"' or\n'google_container_cluster.my-cluster.id'."]
    pub fn resource_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_link", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubMembershipEndpointElDynamic {
    gke_cluster: std::option::Option<terrars::DynamicBlock<GkeHubMembershipEndpointElGkeClusterEl>>,
}

#[derive(Serialize)]
pub struct GkeHubMembershipEndpointEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_cluster: std::option::Option<std::vec::Vec<GkeHubMembershipEndpointElGkeClusterEl>>,
    dynamic: GkeHubMembershipEndpointElDynamic,
}

impl GkeHubMembershipEndpointEl {
    #[doc = "Set the field `gke_cluster`.\n"]
    pub fn set_gke_cluster(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubMembershipEndpointElGkeClusterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gke_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gke_cluster = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubMembershipEndpointEl {
    type O = terrars::BlockAssignable<GkeHubMembershipEndpointEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubMembershipEndpointEl {}

impl BuildGkeHubMembershipEndpointEl {
    pub fn build(self) -> GkeHubMembershipEndpointEl {
        GkeHubMembershipEndpointEl {
            gke_cluster: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubMembershipEndpointElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubMembershipEndpointElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubMembershipEndpointElRef {
        GkeHubMembershipEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubMembershipEndpointElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gke_cluster` after provisioning.\n"]
    pub fn gke_cluster(&self) -> terrars::ListRef<GkeHubMembershipEndpointElGkeClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_cluster", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubMembershipTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubMembershipTimeoutsEl {
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

impl terrars::ToListMappable for GkeHubMembershipTimeoutsEl {
    type O = terrars::BlockAssignable<GkeHubMembershipTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubMembershipTimeoutsEl {}

impl BuildGkeHubMembershipTimeoutsEl {
    pub fn build(self) -> GkeHubMembershipTimeoutsEl {
        GkeHubMembershipTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GkeHubMembershipTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubMembershipTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubMembershipTimeoutsElRef {
        GkeHubMembershipTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubMembershipTimeoutsElRef {
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
struct GkeHubMembershipDynamic {
    authority: std::option::Option<terrars::DynamicBlock<GkeHubMembershipAuthorityEl>>,
    endpoint: std::option::Option<terrars::DynamicBlock<GkeHubMembershipEndpointEl>>,
}
