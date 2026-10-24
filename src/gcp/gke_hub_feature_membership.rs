use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeHubFeatureMembershipData {
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
    configmanagement: std::option::Option<std::vec::Vec<GkeHubFeatureMembershipConfigmanagementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh: std::option::Option<std::vec::Vec<GkeHubFeatureMembershipMeshEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeHubFeatureMembershipTimeoutsEl>,
    dynamic: GkeHubFeatureMembershipDynamic,
}

struct GkeHubFeatureMembership_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeHubFeatureMembershipData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementEl>>,
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
    pub fn set_mesh(self, v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureMembershipMeshEl>>) -> Self {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeHubFeatureMembershipTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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
    pub fn configmanagement(&self) -> terrars::ListRef<GkeHubFeatureMembershipConfigmanagementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configmanagement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh` after provisioning.\n"]
    pub fn mesh(&self) -> terrars::ListRef<GkeHubFeatureMembershipMeshElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFeatureMembershipTimeoutsElRef {
        GkeHubFeatureMembershipTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(GkeHubFeatureMembershipData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                feature: self.feature,
                id: core::default::Default::default(),
                location: self.location,
                membership: self.membership,
                membership_location: core::default::Default::default(),
                project: core::default::Default::default(),
                configmanagement: core::default::Default::default(),
                mesh: core::default::Default::default(),
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
    pub fn configmanagement(&self) -> terrars::ListRef<GkeHubFeatureMembershipConfigmanagementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configmanagement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh` after provisioning.\n"]
    pub fn mesh(&self) -> terrars::ListRef<GkeHubFeatureMembershipMeshElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFeatureMembershipTimeoutsElRef {
        GkeHubFeatureMembershipTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElBinauthzEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeHubFeatureMembershipConfigmanagementElBinauthzEl {
    #[doc = "Set the field `enabled`.\nWhether binauthz is enabled in this cluster."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembershipConfigmanagementElBinauthzEl {
    type O = terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElBinauthzEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembershipConfigmanagementElBinauthzEl {}

impl BuildGkeHubFeatureMembershipConfigmanagementElBinauthzEl {
    pub fn build(self) -> GkeHubFeatureMembershipConfigmanagementElBinauthzEl {
        GkeHubFeatureMembershipConfigmanagementElBinauthzEl { enabled: core::default::Default::default() }
    }
}

pub struct GkeHubFeatureMembershipConfigmanagementElBinauthzElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipConfigmanagementElBinauthzElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembershipConfigmanagementElBinauthzElRef {
        GkeHubFeatureMembershipConfigmanagementElBinauthzElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElBinauthzElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether binauthz is enabled in this cluster."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl {
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

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl {
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

impl terrars::ToListMappable for GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl {
    type O = terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl {}

impl BuildGkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl {
    pub fn build(self) -> GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl {
        GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl {
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

pub struct GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef {
        GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef {
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
pub struct GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl {
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

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl {
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

impl terrars::ToListMappable for GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl {
    type O = terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl {}

impl BuildGkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl {
    pub fn build(self) -> GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl {
        GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl {
            gcp_service_account_email: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: core::default::Default::default(),
            sync_repo: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef {
        GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef {
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
struct GkeHubFeatureMembershipConfigmanagementElConfigSyncElDynamic {
    git: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl>>,
    oci: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElConfigSyncEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics_gcp_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prevent_drift: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci: std::option::Option<std::vec::Vec<GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl>>,
    dynamic: GkeHubFeatureMembershipConfigmanagementElConfigSyncElDynamic,
}

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncEl {
    #[doc =
        "Set the field `metrics_gcp_service_account_email`.\nThe Email of the Google Cloud Service Account (GSA) used for exporting Config Sync metrics to Cloud Monitoring. The GSA should have the Monitoring Metric Writer(roles/monitoring.metricWriter) IAM role. The Kubernetes ServiceAccount `default` in the namespace `config-management-monitoring` should be bound to the GSA."]
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

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl>>,
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
                    ::Into<terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl>>,
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

impl terrars::ToListMappable for GkeHubFeatureMembershipConfigmanagementElConfigSyncEl {
    type O = terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembershipConfigmanagementElConfigSyncEl {}

impl BuildGkeHubFeatureMembershipConfigmanagementElConfigSyncEl {
    pub fn build(self) -> GkeHubFeatureMembershipConfigmanagementElConfigSyncEl {
        GkeHubFeatureMembershipConfigmanagementElConfigSyncEl {
            metrics_gcp_service_account_email: core::default::Default::default(),
            prevent_drift: core::default::Default::default(),
            source_format: core::default::Default::default(),
            git: core::default::Default::default(),
            oci: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef {
        GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `metrics_gcp_service_account_email` after provisioning.\nThe Email of the Google Cloud Service Account (GSA) used for exporting Config Sync metrics to Cloud Monitoring. The GSA should have the Monitoring Metric Writer(roles/monitoring.metricWriter) IAM role. The Kubernetes ServiceAccount `default` in the namespace `config-management-monitoring` should be bound to the GSA."]
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

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `oci` after provisioning.\n"]
    pub fn oci(&self) -> terrars::ListRef<GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oci", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_hierarchical_resource_quota: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_pod_tree_labels: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl {
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

    #[doc = "Set the field `enabled`.\nWhether Hierarchy Controller is enabled in this cluster."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl {
    type O = terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl {}

impl BuildGkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl {
    pub fn build(self) -> GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl {
        GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl {
            enable_hierarchical_resource_quota: core::default::Default::default(),
            enable_pod_tree_labels: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef {
        GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef {
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
        "Get a reference to the value of field `enabled` after provisioning.\nWhether Hierarchy Controller is enabled in this cluster."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backends: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl {
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

impl terrars::ToListMappable for GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl {
    type O = terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl {}

impl BuildGkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl {
    pub fn build(self) -> GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl {
        GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl {
            backends: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef {
        GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef {
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
struct GkeHubFeatureMembershipConfigmanagementElPolicyControllerElDynamic {
    monitoring: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl {
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
    monitoring: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl>>,
    dynamic: GkeHubFeatureMembershipConfigmanagementElPolicyControllerElDynamic,
}

impl GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl {
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
                        terrars
                        ::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl>,
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

impl terrars::ToListMappable for GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl {
    type O = terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembershipConfigmanagementElPolicyControllerEl {}

impl BuildGkeHubFeatureMembershipConfigmanagementElPolicyControllerEl {
    pub fn build(self) -> GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl {
        GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl {
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

pub struct GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef {
        GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef {
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
    ) -> terrars::ListRef<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembershipConfigmanagementElDynamic {
    binauthz: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembershipConfigmanagementElBinauthzEl>>,
    config_sync: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembershipConfigmanagementElConfigSyncEl>>,
    hierarchy_controller: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl>>,
    policy_controller: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binauthz: std::option::Option<std::vec::Vec<GkeHubFeatureMembershipConfigmanagementElBinauthzEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_sync: std::option::Option<std::vec::Vec<GkeHubFeatureMembershipConfigmanagementElConfigSyncEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hierarchy_controller: std
    ::option
    ::Option<std::vec::Vec<GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_controller: std::option::Option<std::vec::Vec<GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl>>,
    dynamic: GkeHubFeatureMembershipConfigmanagementElDynamic,
}

impl GkeHubFeatureMembershipConfigmanagementEl {
    #[doc = "Set the field `version`.\nOptional. Version of ACM to install. Defaults to the latest version."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `binauthz`.\n"]
    pub fn set_binauthz(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElBinauthzEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.binauthz = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.binauthz = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `config_sync`.\n"]
    pub fn set_config_sync(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncEl>>,
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
                    ::Into<terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl>>,
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
                    ::Into<terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl>>,
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

impl terrars::ToListMappable for GkeHubFeatureMembershipConfigmanagementEl {
    type O = terrars::BlockAssignable<GkeHubFeatureMembershipConfigmanagementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembershipConfigmanagementEl {}

impl BuildGkeHubFeatureMembershipConfigmanagementEl {
    pub fn build(self) -> GkeHubFeatureMembershipConfigmanagementEl {
        GkeHubFeatureMembershipConfigmanagementEl {
            version: core::default::Default::default(),
            binauthz: core::default::Default::default(),
            config_sync: core::default::Default::default(),
            hierarchy_controller: core::default::Default::default(),
            policy_controller: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureMembershipConfigmanagementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipConfigmanagementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureMembershipConfigmanagementElRef {
        GkeHubFeatureMembershipConfigmanagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nOptional. Version of ACM to install. Defaults to the latest version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `binauthz` after provisioning.\n"]
    pub fn binauthz(&self) -> terrars::ListRef<GkeHubFeatureMembershipConfigmanagementElBinauthzElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binauthz", self.base))
    }

    #[doc = "Get a reference to the value of field `config_sync` after provisioning.\n"]
    pub fn config_sync(&self) -> terrars::ListRef<GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_sync", self.base))
    }

    #[doc = "Get a reference to the value of field `hierarchy_controller` after provisioning.\n"]
    pub fn hierarchy_controller(
        &self,
    ) -> terrars::ListRef<GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hierarchy_controller", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_controller` after provisioning.\n"]
    pub fn policy_controller(&self) -> terrars::ListRef<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_controller", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipMeshEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembershipMeshEl {
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

impl terrars::ToListMappable for GkeHubFeatureMembershipMeshEl {
    type O = terrars::BlockAssignable<GkeHubFeatureMembershipMeshEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembershipMeshEl {}

impl BuildGkeHubFeatureMembershipMeshEl {
    pub fn build(self) -> GkeHubFeatureMembershipMeshEl {
        GkeHubFeatureMembershipMeshEl {
            control_plane: core::default::Default::default(),
            management: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureMembershipMeshElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipMeshElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureMembershipMeshElRef {
        GkeHubFeatureMembershipMeshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipMeshElRef {
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
pub struct GkeHubFeatureMembershipTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFeatureMembershipTimeoutsEl {
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

impl terrars::ToListMappable for GkeHubFeatureMembershipTimeoutsEl {
    type O = terrars::BlockAssignable<GkeHubFeatureMembershipTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFeatureMembershipTimeoutsEl {}

impl BuildGkeHubFeatureMembershipTimeoutsEl {
    pub fn build(self) -> GkeHubFeatureMembershipTimeoutsEl {
        GkeHubFeatureMembershipTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GkeHubFeatureMembershipTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFeatureMembershipTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFeatureMembershipTimeoutsElRef {
        GkeHubFeatureMembershipTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipTimeoutsElRef {
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
struct GkeHubFeatureMembershipDynamic {
    configmanagement: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembershipConfigmanagementEl>>,
    mesh: std::option::Option<terrars::DynamicBlock<GkeHubFeatureMembershipMeshEl>>,
}
