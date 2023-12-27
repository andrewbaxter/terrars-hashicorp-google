use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeHubFeatureMembershipData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    feature: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    membership: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    membership_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configmanagement: Option<Vec<GkeHubFeatureMembershipConfigmanagementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mesh: Option<Vec<GkeHubFeatureMembershipMeshEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeHubFeatureMembershipTimeoutsEl>,
    dynamic: GkeHubFeatureMembershipDynamic,
}

struct GkeHubFeatureMembership_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeHubFeatureMembershipData>,
}

#[derive(Clone)]
pub struct GkeHubFeatureMembership(Rc<GkeHubFeatureMembership_>);

impl GkeHubFeatureMembership {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `membership_location`.\nThe location of the membership"]
    pub fn set_membership_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().membership_location = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nThe project of the feature"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `configmanagement`.\n"]
    pub fn set_configmanagement(self, v: impl Into<BlockAssignable<GkeHubFeatureMembershipConfigmanagementEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configmanagement = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configmanagement = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `mesh`.\n"]
    pub fn set_mesh(self, v: impl Into<BlockAssignable<GkeHubFeatureMembershipMeshEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mesh = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mesh = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeHubFeatureMembershipTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `feature` after provisioning.\nThe name of the feature"]
    pub fn feature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location of the feature"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership` after provisioning.\nThe name of the membership"]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership_location` after provisioning.\nThe location of the membership"]
    pub fn membership_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership_location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe project of the feature"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configmanagement` after provisioning.\n"]
    pub fn configmanagement(&self) -> ListRef<GkeHubFeatureMembershipConfigmanagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configmanagement", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mesh` after provisioning.\n"]
    pub fn mesh(&self) -> ListRef<GkeHubFeatureMembershipMeshElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mesh", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFeatureMembershipTimeoutsElRef {
        GkeHubFeatureMembershipTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for GkeHubFeatureMembership {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GkeHubFeatureMembership { }

impl ToListMappable for GkeHubFeatureMembership {
    type O = ListRef<GkeHubFeatureMembershipRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GkeHubFeatureMembership_ {
    fn extract_resource_type(&self) -> String {
        "google_gke_hub_feature_membership".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeHubFeatureMembership {
    pub tf_id: String,
    #[doc= "The name of the feature"]
    pub feature: PrimField<String>,
    #[doc= "The location of the feature"]
    pub location: PrimField<String>,
    #[doc= "The name of the membership"]
    pub membership: PrimField<String>,
}

impl BuildGkeHubFeatureMembership {
    pub fn build(self, stack: &mut Stack) -> GkeHubFeatureMembership {
        let out = GkeHubFeatureMembership(Rc::new(GkeHubFeatureMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeHubFeatureMembershipData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                feature: self.feature,
                id: core::default::Default::default(),
                location: self.location,
                membership: self.membership,
                membership_location: core::default::Default::default(),
                project: core::default::Default::default(),
                configmanagement: core::default::Default::default(),
                mesh: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeHubFeatureMembershipRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeHubFeatureMembershipRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `feature` after provisioning.\nThe name of the feature"]
    pub fn feature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.feature", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location of the feature"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership` after provisioning.\nThe name of the membership"]
    pub fn membership(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `membership_location` after provisioning.\nThe location of the membership"]
    pub fn membership_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.membership_location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe project of the feature"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configmanagement` after provisioning.\n"]
    pub fn configmanagement(&self) -> ListRef<GkeHubFeatureMembershipConfigmanagementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.configmanagement", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mesh` after provisioning.\n"]
    pub fn mesh(&self) -> ListRef<GkeHubFeatureMembershipMeshElRef> {
        ListRef::new(self.shared().clone(), format!("{}.mesh", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFeatureMembershipTimeoutsElRef {
        GkeHubFeatureMembershipTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElBinauthzEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl GkeHubFeatureMembershipConfigmanagementElBinauthzEl {
    #[doc= "Set the field `enabled`.\nWhether binauthz is enabled in this cluster."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for GkeHubFeatureMembershipConfigmanagementElBinauthzEl {
    type O = BlockAssignable<GkeHubFeatureMembershipConfigmanagementElBinauthzEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipConfigmanagementElBinauthzElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureMembershipConfigmanagementElBinauthzElRef {
        GkeHubFeatureMembershipConfigmanagementElBinauthzElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElBinauthzElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether binauthz is enabled in this cluster."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_proxy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_repo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_rev: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_wait_secs: Option<PrimField<String>>,
}

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl {
    #[doc= "Set the field `gcp_service_account_email`.\nThe GCP Service Account Email used for auth when secretType is gcpServiceAccount."]
    pub fn set_gcp_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account_email = Some(v.into());
        self
    }

    #[doc= "Set the field `https_proxy`.\nURL for the HTTPS proxy to be used when communicating with the Git repo."]
    pub fn set_https_proxy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.https_proxy = Some(v.into());
        self
    }

    #[doc= "Set the field `policy_dir`.\nThe path within the Git repository that represents the top level of the repo to sync. Default: the root directory of the repository."]
    pub fn set_policy_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `secret_type`.\nType of secret configured for access to the Git repo. Must be one of ssh, cookiefile, gcenode, token, gcpserviceaccount or none. The validation of this is case-sensitive."]
    pub fn set_secret_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_type = Some(v.into());
        self
    }

    #[doc= "Set the field `sync_branch`.\nThe branch of the repository to sync from. Default: master."]
    pub fn set_sync_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_branch = Some(v.into());
        self
    }

    #[doc= "Set the field `sync_repo`.\nThe URL of the Git repository to use as the source of truth."]
    pub fn set_sync_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_repo = Some(v.into());
        self
    }

    #[doc= "Set the field `sync_rev`.\nGit revision (tag or hash) to check out. Default HEAD."]
    pub fn set_sync_rev(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_rev = Some(v.into());
        self
    }

    #[doc= "Set the field `sync_wait_secs`.\nPeriod in seconds between consecutive syncs. Default: 15."]
    pub fn set_sync_wait_secs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_wait_secs = Some(v.into());
        self
    }
}

impl ToListMappable for GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl {
    type O = BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef {
        GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `gcp_service_account_email` after provisioning.\nThe GCP Service Account Email used for auth when secretType is gcpServiceAccount."]
    pub fn gcp_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account_email", self.base))
    }

    #[doc= "Get a reference to the value of field `https_proxy` after provisioning.\nURL for the HTTPS proxy to be used when communicating with the Git repo."]
    pub fn https_proxy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.https_proxy", self.base))
    }

    #[doc= "Get a reference to the value of field `policy_dir` after provisioning.\nThe path within the Git repository that represents the top level of the repo to sync. Default: the root directory of the repository."]
    pub fn policy_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `secret_type` after provisioning.\nType of secret configured for access to the Git repo. Must be one of ssh, cookiefile, gcenode, token, gcpserviceaccount or none. The validation of this is case-sensitive."]
    pub fn secret_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }

    #[doc= "Get a reference to the value of field `sync_branch` after provisioning.\nThe branch of the repository to sync from. Default: master."]
    pub fn sync_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_branch", self.base))
    }

    #[doc= "Get a reference to the value of field `sync_repo` after provisioning.\nThe URL of the Git repository to use as the source of truth."]
    pub fn sync_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }

    #[doc= "Get a reference to the value of field `sync_rev` after provisioning.\nGit revision (tag or hash) to check out. Default HEAD."]
    pub fn sync_rev(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_rev", self.base))
    }

    #[doc= "Get a reference to the value of field `sync_wait_secs` after provisioning.\nPeriod in seconds between consecutive syncs. Default: 15."]
    pub fn sync_wait_secs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_wait_secs", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_dir: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_repo: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_wait_secs: Option<PrimField<String>>,
}

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl {
    #[doc= "Set the field `gcp_service_account_email`.\nThe GCP Service Account Email used for auth when secret_type is gcpserviceaccount. "]
    pub fn set_gcp_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_service_account_email = Some(v.into());
        self
    }

    #[doc= "Set the field `policy_dir`.\nThe absolute path of the directory that contains the local resources. Default: the root directory of the image."]
    pub fn set_policy_dir(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.policy_dir = Some(v.into());
        self
    }

    #[doc= "Set the field `secret_type`.\nType of secret configured for access to the OCI Image. Must be one of gcenode, gcpserviceaccount or none. The validation of this is case-sensitive."]
    pub fn set_secret_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_type = Some(v.into());
        self
    }

    #[doc= "Set the field `sync_repo`.\nThe OCI image repository URL for the package to sync from. e.g. LOCATION-docker.pkg.dev/PROJECT_ID/REPOSITORY_NAME/PACKAGE_NAME."]
    pub fn set_sync_repo(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_repo = Some(v.into());
        self
    }

    #[doc= "Set the field `sync_wait_secs`.\nPeriod in seconds(int64 format) between consecutive syncs. Default: 15."]
    pub fn set_sync_wait_secs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sync_wait_secs = Some(v.into());
        self
    }
}

impl ToListMappable for GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl {
    type O = BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef {
        GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `gcp_service_account_email` after provisioning.\nThe GCP Service Account Email used for auth when secret_type is gcpserviceaccount. "]
    pub fn gcp_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account_email", self.base))
    }

    #[doc= "Get a reference to the value of field `policy_dir` after provisioning.\nThe absolute path of the directory that contains the local resources. Default: the root directory of the image."]
    pub fn policy_dir(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }

    #[doc= "Get a reference to the value of field `secret_type` after provisioning.\nType of secret configured for access to the OCI Image. Must be one of gcenode, gcpserviceaccount or none. The validation of this is case-sensitive."]
    pub fn secret_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }

    #[doc= "Get a reference to the value of field `sync_repo` after provisioning.\nThe OCI image repository URL for the package to sync from. e.g. LOCATION-docker.pkg.dev/PROJECT_ID/REPOSITORY_NAME/PACKAGE_NAME."]
    pub fn sync_repo(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }

    #[doc= "Get a reference to the value of field `sync_wait_secs` after provisioning.\nPeriod in seconds(int64 format) between consecutive syncs. Default: 15."]
    pub fn sync_wait_secs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sync_wait_secs", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembershipConfigmanagementElConfigSyncElDynamic {
    git: Option<DynamicBlock<GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl>>,
    oci: Option<DynamicBlock<GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElConfigSyncEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics_gcp_service_account_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prevent_drift: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    git: Option<Vec<GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci: Option<Vec<GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl>>,
    dynamic: GkeHubFeatureMembershipConfigmanagementElConfigSyncElDynamic,
}

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncEl {
    #[doc= "Set the field `metrics_gcp_service_account_email`.\nThe Email of the Google Cloud Service Account (GSA) used for exporting Config Sync metrics to Cloud Monitoring. The GSA should have the Monitoring Metric Writer(roles/monitoring.metricWriter) IAM role. The Kubernetes ServiceAccount `default` in the namespace `config-management-monitoring` should be bound to the GSA."]
    pub fn set_metrics_gcp_service_account_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metrics_gcp_service_account_email = Some(v.into());
        self
    }

    #[doc= "Set the field `prevent_drift`.\nSet to true to enable the Config Sync admission webhook to prevent drifts. If set to `false`, disables the Config Sync admission webhook and does not prevent drifts."]
    pub fn set_prevent_drift(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.prevent_drift = Some(v.into());
        self
    }

    #[doc= "Set the field `source_format`.\nSpecifies whether the Config Sync Repo is in \"hierarchical\" or \"unstructured\" mode."]
    pub fn set_source_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_format = Some(v.into());
        self
    }

    #[doc= "Set the field `git`.\n"]
    pub fn set_git(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.git = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.git = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `oci`.\n"]
    pub fn set_oci(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oci = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oci = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GkeHubFeatureMembershipConfigmanagementElConfigSyncEl {
    type O = BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef {
        GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `metrics_gcp_service_account_email` after provisioning.\nThe Email of the Google Cloud Service Account (GSA) used for exporting Config Sync metrics to Cloud Monitoring. The GSA should have the Monitoring Metric Writer(roles/monitoring.metricWriter) IAM role. The Kubernetes ServiceAccount `default` in the namespace `config-management-monitoring` should be bound to the GSA."]
    pub fn metrics_gcp_service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metrics_gcp_service_account_email", self.base))
    }

    #[doc= "Get a reference to the value of field `prevent_drift` after provisioning.\nSet to true to enable the Config Sync admission webhook to prevent drifts. If set to `false`, disables the Config Sync admission webhook and does not prevent drifts."]
    pub fn prevent_drift(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.prevent_drift", self.base))
    }

    #[doc= "Get a reference to the value of field `source_format` after provisioning.\nSpecifies whether the Config Sync Repo is in \"hierarchical\" or \"unstructured\" mode."]
    pub fn source_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_format", self.base))
    }

    #[doc= "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> ListRef<GkeHubFeatureMembershipConfigmanagementElConfigSyncElGitElRef> {
        ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc= "Get a reference to the value of field `oci` after provisioning.\n"]
    pub fn oci(&self) -> ListRef<GkeHubFeatureMembershipConfigmanagementElConfigSyncElOciElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oci", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_hierarchical_resource_quota: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_pod_tree_labels: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl {
    #[doc= "Set the field `enable_hierarchical_resource_quota`.\nWhether hierarchical resource quota is enabled in this cluster."]
    pub fn set_enable_hierarchical_resource_quota(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_hierarchical_resource_quota = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_pod_tree_labels`.\nWhether pod tree labels are enabled in this cluster."]
    pub fn set_enable_pod_tree_labels(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_pod_tree_labels = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nWhether Hierarchy Controller is enabled in this cluster."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl {
    type O = BlockAssignable<GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef {
        GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_hierarchical_resource_quota` after provisioning.\nWhether hierarchical resource quota is enabled in this cluster."]
    pub fn enable_hierarchical_resource_quota(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_hierarchical_resource_quota", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_pod_tree_labels` after provisioning.\nWhether pod tree labels are enabled in this cluster."]
    pub fn enable_pod_tree_labels(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_pod_tree_labels", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether Hierarchy Controller is enabled in this cluster."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backends: Option<ListField<PrimField<String>>>,
}

impl GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl {
    #[doc= "Set the field `backends`.\n Specifies the list of backends Policy Controller will export to. Specifying an empty value `[]` disables metrics export."]
    pub fn set_backends(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.backends = Some(v.into());
        self
    }
}

impl ToListMappable for GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl {
    type O = BlockAssignable<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef {
        GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backends` after provisioning.\n Specifies the list of backends Policy Controller will export to. Specifying an empty value `[]` disables metrics export."]
    pub fn backends(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.backends", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembershipConfigmanagementElPolicyControllerElDynamic {
    monitoring: Option<DynamicBlock<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_interval_seconds: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exemptable_namespaces: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_denies_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutation_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referential_rules_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_library_installed: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    monitoring: Option<Vec<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl>>,
    dynamic: GkeHubFeatureMembershipConfigmanagementElPolicyControllerElDynamic,
}

impl GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl {
    #[doc= "Set the field `audit_interval_seconds`.\nSets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether."]
    pub fn set_audit_interval_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audit_interval_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `enabled`.\nEnables the installation of Policy Controller. If false, the rest of PolicyController fields take no effect."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `exemptable_namespaces`.\nThe set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
    pub fn set_exemptable_namespaces(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exemptable_namespaces = Some(v.into());
        self
    }

    #[doc= "Set the field `log_denies_enabled`.\nLogs all denies and dry run failures."]
    pub fn set_log_denies_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.log_denies_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `mutation_enabled`.\nEnable or disable mutation in policy controller. If true, mutation CRDs, webhook and controller deployment will be deployed to the cluster."]
    pub fn set_mutation_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mutation_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `referential_rules_enabled`.\nEnables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
    pub fn set_referential_rules_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.referential_rules_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `template_library_installed`.\nInstalls the default template library along with Policy Controller."]
    pub fn set_template_library_installed(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.template_library_installed = Some(v.into());
        self
    }

    #[doc= "Set the field `monitoring`.\n"]
    pub fn set_monitoring(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.monitoring = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.monitoring = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl {
    type O = BlockAssignable<GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef {
        GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `audit_interval_seconds` after provisioning.\nSets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether."]
    pub fn audit_interval_seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audit_interval_seconds", self.base))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nEnables the installation of Policy Controller. If false, the rest of PolicyController fields take no effect."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `exemptable_namespaces` after provisioning.\nThe set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster."]
    pub fn exemptable_namespaces(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exemptable_namespaces", self.base))
    }

    #[doc= "Get a reference to the value of field `log_denies_enabled` after provisioning.\nLogs all denies and dry run failures."]
    pub fn log_denies_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_denies_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `mutation_enabled` after provisioning.\nEnable or disable mutation in policy controller. If true, mutation CRDs, webhook and controller deployment will be deployed to the cluster."]
    pub fn mutation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mutation_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `referential_rules_enabled` after provisioning.\nEnables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated."]
    pub fn referential_rules_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.referential_rules_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `template_library_installed` after provisioning.\nInstalls the default template library along with Policy Controller."]
    pub fn template_library_installed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.template_library_installed", self.base))
    }

    #[doc= "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> ListRef<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElMonitoringElRef> {
        ListRef::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembershipConfigmanagementElDynamic {
    binauthz: Option<DynamicBlock<GkeHubFeatureMembershipConfigmanagementElBinauthzEl>>,
    config_sync: Option<DynamicBlock<GkeHubFeatureMembershipConfigmanagementElConfigSyncEl>>,
    hierarchy_controller: Option<DynamicBlock<GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl>>,
    policy_controller: Option<DynamicBlock<GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl>>,
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipConfigmanagementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binauthz: Option<Vec<GkeHubFeatureMembershipConfigmanagementElBinauthzEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_sync: Option<Vec<GkeHubFeatureMembershipConfigmanagementElConfigSyncEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hierarchy_controller: Option<Vec<GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_controller: Option<Vec<GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl>>,
    dynamic: GkeHubFeatureMembershipConfigmanagementElDynamic,
}

impl GkeHubFeatureMembershipConfigmanagementEl {
    #[doc= "Set the field `version`.\nOptional. Version of ACM to install. Defaults to the latest version."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }

    #[doc= "Set the field `binauthz`.\n"]
    pub fn set_binauthz(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureMembershipConfigmanagementElBinauthzEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.binauthz = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.binauthz = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `config_sync`.\n"]
    pub fn set_config_sync(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureMembershipConfigmanagementElConfigSyncEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config_sync = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config_sync = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `hierarchy_controller`.\n"]
    pub fn set_hierarchy_controller(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureMembershipConfigmanagementElHierarchyControllerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hierarchy_controller = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hierarchy_controller = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `policy_controller`.\n"]
    pub fn set_policy_controller(
        mut self,
        v: impl Into<BlockAssignable<GkeHubFeatureMembershipConfigmanagementElPolicyControllerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.policy_controller = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_controller = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GkeHubFeatureMembershipConfigmanagementEl {
    type O = BlockAssignable<GkeHubFeatureMembershipConfigmanagementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct GkeHubFeatureMembershipConfigmanagementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipConfigmanagementElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureMembershipConfigmanagementElRef {
        GkeHubFeatureMembershipConfigmanagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipConfigmanagementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `version` after provisioning.\nOptional. Version of ACM to install. Defaults to the latest version."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc= "Get a reference to the value of field `binauthz` after provisioning.\n"]
    pub fn binauthz(&self) -> ListRef<GkeHubFeatureMembershipConfigmanagementElBinauthzElRef> {
        ListRef::new(self.shared().clone(), format!("{}.binauthz", self.base))
    }

    #[doc= "Get a reference to the value of field `config_sync` after provisioning.\n"]
    pub fn config_sync(&self) -> ListRef<GkeHubFeatureMembershipConfigmanagementElConfigSyncElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config_sync", self.base))
    }

    #[doc= "Get a reference to the value of field `hierarchy_controller` after provisioning.\n"]
    pub fn hierarchy_controller(&self) -> ListRef<GkeHubFeatureMembershipConfigmanagementElHierarchyControllerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hierarchy_controller", self.base))
    }

    #[doc= "Get a reference to the value of field `policy_controller` after provisioning.\n"]
    pub fn policy_controller(&self) -> ListRef<GkeHubFeatureMembershipConfigmanagementElPolicyControllerElRef> {
        ListRef::new(self.shared().clone(), format!("{}.policy_controller", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipMeshEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management: Option<PrimField<String>>,
}

impl GkeHubFeatureMembershipMeshEl {
    #[doc= "Set the field `control_plane`.\n**DEPRECATED** Whether to automatically manage Service Mesh control planes. Possible values: CONTROL_PLANE_MANAGEMENT_UNSPECIFIED, AUTOMATIC, MANUAL"]
    pub fn set_control_plane(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.control_plane = Some(v.into());
        self
    }

    #[doc= "Set the field `management`.\nWhether to automatically manage Service Mesh. Possible values: MANAGEMENT_UNSPECIFIED, MANAGEMENT_AUTOMATIC, MANAGEMENT_MANUAL"]
    pub fn set_management(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.management = Some(v.into());
        self
    }
}

impl ToListMappable for GkeHubFeatureMembershipMeshEl {
    type O = BlockAssignable<GkeHubFeatureMembershipMeshEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipMeshElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureMembershipMeshElRef {
        GkeHubFeatureMembershipMeshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipMeshElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `control_plane` after provisioning.\n**DEPRECATED** Whether to automatically manage Service Mesh control planes. Possible values: CONTROL_PLANE_MANAGEMENT_UNSPECIFIED, AUTOMATIC, MANUAL"]
    pub fn control_plane(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_plane", self.base))
    }

    #[doc= "Get a reference to the value of field `management` after provisioning.\nWhether to automatically manage Service Mesh. Possible values: MANAGEMENT_UNSPECIFIED, MANAGEMENT_AUTOMATIC, MANAGEMENT_MANUAL"]
    pub fn management(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.management", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFeatureMembershipTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl GkeHubFeatureMembershipTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for GkeHubFeatureMembershipTimeoutsEl {
    type O = BlockAssignable<GkeHubFeatureMembershipTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for GkeHubFeatureMembershipTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeHubFeatureMembershipTimeoutsElRef {
        GkeHubFeatureMembershipTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFeatureMembershipTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFeatureMembershipDynamic {
    configmanagement: Option<DynamicBlock<GkeHubFeatureMembershipConfigmanagementEl>>,
    mesh: Option<DynamicBlock<GkeHubFeatureMembershipMeshEl>>,
}
