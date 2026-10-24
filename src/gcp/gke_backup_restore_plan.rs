use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeBackupRestorePlanData {
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
    backup_plan: terrars::PrimField<std::string::String>,
    cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_config: std::option::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeBackupRestorePlanTimeoutsEl>,
    dynamic: GkeBackupRestorePlanDynamic,
}

struct GkeBackupRestorePlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeBackupRestorePlanData>,
}

#[derive(Clone)]
pub struct GkeBackupRestorePlan(std::rc::Rc<GkeBackupRestorePlan_>);

impl GkeBackupRestorePlan {
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

    #[doc = "Set the field `description`.\nUser specified descriptive string for this RestorePlan."]
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
        "Set the field `labels`.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `restore_config`.\n"]
    pub fn set_restore_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeBackupRestorePlanTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `backup_plan` after provisioning.\nA reference to the BackupPlan from which Backups may be used\nas the source for Restores created via this RestorePlan."]
    pub fn backup_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe source cluster from which Restores will be created via this RestorePlan."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser specified descriptive string for this RestorePlan."]
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
        "Get a reference to the value of field `labels` after provisioning.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe region of the Restore Plan."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full name of the BackupPlan Resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe State of the RestorePlan."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_reason` after provisioning.\nDetailed description of why RestorePlan is in its current state."]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer generated, unique identifier of UUID format."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_config` after provisioning.\n"]
    pub fn restore_config(&self) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeBackupRestorePlanTimeoutsElRef {
        GkeBackupRestorePlanTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GkeBackupRestorePlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeBackupRestorePlan { }

impl terrars::ToListMappable for GkeBackupRestorePlan {
    type O = terrars::ListRef<GkeBackupRestorePlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeBackupRestorePlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gke_backup_restore_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeBackupRestorePlan {
    pub tf_id: std::string::String,
    #[doc =
        "A reference to the BackupPlan from which Backups may be used\nas the source for Restores created via this RestorePlan."]
    pub backup_plan: terrars::PrimField<std::string::String>,
    #[doc = "The source cluster from which Restores will be created via this RestorePlan."]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = "The region of the Restore Plan."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The full name of the BackupPlan Resource."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGkeBackupRestorePlan {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeBackupRestorePlan {
        let out = GkeBackupRestorePlan(std::rc::Rc::new(GkeBackupRestorePlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeBackupRestorePlanData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                backup_plan: self.backup_plan,
                cluster: self.cluster,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                restore_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeBackupRestorePlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeBackupRestorePlanRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_plan` after provisioning.\nA reference to the BackupPlan from which Backups may be used\nas the source for Restores created via this RestorePlan."]
    pub fn backup_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster` after provisioning.\nThe source cluster from which Restores will be created via this RestorePlan."]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser specified descriptive string for this RestorePlan."]
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
        "Get a reference to the value of field `labels` after provisioning.\nDescription: A set of custom labels supplied by the user.\nA list of key->value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe region of the Restore Plan."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full name of the BackupPlan Resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe State of the RestorePlan."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_reason` after provisioning.\nDetailed description of why RestorePlan is in its current state."]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nServer generated, unique identifier of UUID format."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_config` after provisioning.\n"]
    pub fn restore_config(&self) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeBackupRestorePlanTimeoutsElRef {
        GkeBackupRestorePlanTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_kind: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
    #[doc =
        "Set the field `resource_group`.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn set_resource_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_group = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_kind`.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn set_resource_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_kind = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
    type O =
        terrars
        ::BlockAssignable<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {}

impl BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl {
            resource_group: core::default::Default::default(),
            resource_kind: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_group` after provisioning.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn resource_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_kind` after provisioning.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn resource_kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_kind", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_kind: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
    #[doc =
        "Set the field `resource_group`.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn set_resource_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_group = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_kind`.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn set_resource_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_kind = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
    type O =
        terrars
        ::BlockAssignable<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {}

impl BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl {
            resource_group: core::default::Default::default(),
            resource_kind: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_group` after provisioning.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn resource_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_kind` after provisioning.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn resource_kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_kind", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElDynamic {
    excluded_group_kinds: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl>,
    >,
    selected_group_kinds: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl>,
    >,
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_group_kinds: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_group_kinds: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_group_kinds: std
    ::option
    ::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_group_kinds: std
    ::option
    ::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl>>,
    dynamic: GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElDynamic,
}

impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
    #[doc =
        "Set the field `all_group_kinds`.\nIf True, all valid cluster-scoped resources will be restored.\nMutually exclusive to any other field in 'clusterResourceRestoreScope'."]
    pub fn set_all_group_kinds(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_group_kinds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_group_kinds`.\nIf True, no cluster-scoped resources will be restored.\nMutually exclusive to any other field in 'clusterResourceRestoreScope'."]
    pub fn set_no_group_kinds(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_group_kinds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_group_kinds`.\n"]
    pub fn set_excluded_group_kinds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.excluded_group_kinds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.excluded_group_kinds = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selected_group_kinds`.\n"]
    pub fn set_selected_group_kinds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selected_group_kinds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_group_kinds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
    type O = terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {}

impl BuildGkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl {
            all_group_kinds: core::default::Default::default(),
            no_group_kinds: core::default::Default::default(),
            excluded_group_kinds: core::default::Default::default(),
            selected_group_kinds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef {
        GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all_group_kinds` after provisioning.\nIf True, all valid cluster-scoped resources will be restored.\nMutually exclusive to any other field in 'clusterResourceRestoreScope'."]
    pub fn all_group_kinds(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_group_kinds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_group_kinds` after provisioning.\nIf True, no cluster-scoped resources will be restored.\nMutually exclusive to any other field in 'clusterResourceRestoreScope'."]
    pub fn no_group_kinds(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_group_kinds", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_group_kinds` after provisioning.\n"]
    pub fn excluded_group_kinds(
        &self,
    ) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElExcludedGroupKindsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_group_kinds", self.base))
    }

    #[doc = "Get a reference to the value of field `selected_group_kinds` after provisioning.\n"]
    pub fn selected_group_kinds(
        &self,
    ) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElSelectedGroupKindsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selected_group_kinds", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
    namespaces: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl { }

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
    type O = terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
    #[doc = "A list of Kubernetes Namespaces."]
    pub namespaces: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl {
        GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl { namespaces: self.namespaces }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef {
        GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\nA list of Kubernetes Namespaces."]
    pub fn namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.namespaces", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
    name: terrars::PrimField<std::string::String>,
    namespace: terrars::PrimField<std::string::String>,
}

impl GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl { }

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
    type O = terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
    #[doc = "The name of a Kubernetes Resource."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The namespace of a Kubernetes Resource."]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildGkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
        GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl {
            name: self.name,
            namespace: self.namespace,
        }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef {
        GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of a Kubernetes Resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nThe namespace of a Kubernetes Resource."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElDynamic {
    namespaced_names: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl>>,
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespaced_names: std
    ::option
    ::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl>>,
    dynamic: GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElDynamic,
}

impl GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
    #[doc = "Set the field `namespaced_names`.\n"]
    pub fn set_namespaced_names(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.namespaced_names = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.namespaced_names = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
    type O = terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {}

impl BuildGkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
        GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl {
            namespaced_names: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef {
        GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespaced_names` after provisioning.\n"]
    pub fn namespaced_names(
        &self,
    ) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElNamespacedNamesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.namespaced_names", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
    namespaces: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl { }

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
    type O = terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
    #[doc = "A list of Kubernetes Namespaces."]
    pub namespaces: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl {
        GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl { namespaces: self.namespaces }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef {
        GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespaces` after provisioning.\nA list of Kubernetes Namespaces."]
    pub fn namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.namespaces", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_path: std::option::Option<terrars::PrimField<std::string::String>>,
    op: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
    #[doc =
        "Set the field `from_path`.\nA string containing a JSON Pointer value that references the\nlocation in the target document to move the value from."]
    pub fn set_from_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.from_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nA string containing a JSON-Pointer value that references a\nlocation within the target document where the operation is performed."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value`.\nA string that specifies the desired value in string format\nto use for transformation."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
    type O = terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
    #[doc =
        "Specifies the operation to perform. Possible values: [\"REMOVE\", \"MOVE\", \"COPY\", \"ADD\", \"TEST\", \"REPLACE\"]"]
    pub op: terrars::PrimField<std::string::String>,
}

impl BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl {
            from_path: core::default::Default::default(),
            op: self.op,
            path: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `from_path` after provisioning.\nA string containing a JSON Pointer value that references the\nlocation in the target document to move the value from."]
    pub fn from_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `op` after provisioning.\nSpecifies the operation to perform. Possible values: [\"REMOVE\", \"MOVE\", \"COPY\", \"ADD\", \"TEST\", \"REPLACE\"]"]
    pub fn op(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.op", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nA string containing a JSON-Pointer value that references a\nlocation within the target document where the operation is performed."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA string that specifies the desired value in string format\nto use for transformation."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_kind: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
    #[doc =
        "Set the field `resource_group`.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn set_resource_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_group = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_kind`.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn set_resource_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_kind = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
    type O =
        terrars
        ::BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {}

impl BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl {
            resource_group: core::default::Default::default(),
            resource_kind: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_group` after provisioning.\nAPI Group string of a Kubernetes resource, e.g.\n\"apiextensions.k8s.io\", \"storage.k8s.io\", etc.\nUse empty string for core group."]
    pub fn resource_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_kind` after provisioning.\nKind of a Kubernetes resource, e.g.\n\"CustomResourceDefinition\", \"StorageClass\", etc."]
    pub fn resource_kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_kind", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElDynamic {
    group_kinds: std
    ::option
    ::Option<
        terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl>,
    >,
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespaces: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_kinds: std
    ::option
    ::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl>>,
    dynamic: GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElDynamic,
}

impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
    #[doc =
        "Set the field `json_path`.\nThis is a JSONPath expression that matches specific fields of\ncandidate resources and it operates as a filtering parameter\n(resources that are not matched with this expression will not\nbe candidates for transformation)."]
    pub fn set_json_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `namespaces`.\n(Filtering parameter) Any resource subject to transformation must\nbe contained within one of the listed Kubernetes Namespace in the\nBackup. If this field is not provided, no namespace filtering will\nbe performed (all resources in all Namespaces, including all\ncluster-scoped resources, will be candidates for transformation).\nTo mix cluster-scoped and namespaced resources in the same rule,\nuse an empty string (\"\") as one of the target namespaces."]
    pub fn set_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_kinds`.\n"]
    pub fn set_group_kinds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.group_kinds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.group_kinds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
    type O = terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {}

impl BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl {
            json_path: core::default::Default::default(),
            namespaces: core::default::Default::default(),
            group_kinds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `json_path` after provisioning.\nThis is a JSONPath expression that matches specific fields of\ncandidate resources and it operates as a filtering parameter\n(resources that are not matched with this expression will not\nbe candidates for transformation)."]
    pub fn json_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespaces` after provisioning.\n(Filtering parameter) Any resource subject to transformation must\nbe contained within one of the listed Kubernetes Namespace in the\nBackup. If this field is not provided, no namespace filtering will\nbe performed (all resources in all Namespaces, including all\ncluster-scoped resources, will be candidates for transformation).\nTo mix cluster-scoped and namespaced resources in the same rule,\nuse an empty string (\"\") as one of the target namespaces."]
    pub fn namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.namespaces", self.base))
    }

    #[doc = "Get a reference to the value of field `group_kinds` after provisioning.\n"]
    pub fn group_kinds(
        &self,
    ) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElGroupKindsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_kinds", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElDynamic {
    field_actions: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl>>,
    resource_filter: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl>>,
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_actions: std
    ::option
    ::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_filter: std
    ::option
    ::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl>>,
    dynamic: GkeBackupRestorePlanRestoreConfigElTransformationRulesElDynamic,
}

impl GkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
    #[doc =
        "Set the field `description`.\nThe description is a user specified string description\nof the transformation rule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_actions`.\n"]
    pub fn set_field_actions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_filter`.\n"]
    pub fn set_resource_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
    type O = terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesEl {}

impl BuildGkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesEl {
            description: core::default::Default::default(),
            field_actions: core::default::Default::default(),
            resource_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef {
        GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description is a user specified string description\nof the transformation rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `field_actions` after provisioning.\n"]
    pub fn field_actions(
        &self,
    ) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElTransformationRulesElFieldActionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_filter` after provisioning.\n"]
    pub fn resource_filter(
        &self,
    ) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElTransformationRulesElResourceFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeBackupRestorePlanRestoreConfigElDynamic {
    cluster_resource_restore_scope: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl>>,
    excluded_namespaces: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl>>,
    selected_applications: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl>>,
    selected_namespaces: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl>>,
    transformation_rules: std
    ::option
    ::Option<terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigElTransformationRulesEl>>,
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanRestoreConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_namespaces: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_resource_conflict_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespaced_resource_restore_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_namespaces: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_data_restore_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_resource_restore_scope: std
    ::option
    ::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_namespaces: std::option::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_applications: std::option::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_namespaces: std::option::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transformation_rules: std::option::Option<std::vec::Vec<GkeBackupRestorePlanRestoreConfigElTransformationRulesEl>>,
    dynamic: GkeBackupRestorePlanRestoreConfigElDynamic,
}

impl GkeBackupRestorePlanRestoreConfigEl {
    #[doc =
        "Set the field `all_namespaces`.\nIf True, restore all namespaced resources in the Backup.\nSetting this field to False will result in an error."]
    pub fn set_all_namespaces(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cluster_resource_conflict_policy`.\nDefines the behavior for handling the situation where cluster-scoped resources\nbeing restored already exist in the target cluster.\nThis MUST be set to a value other than 'CLUSTER_RESOURCE_CONFLICT_POLICY_UNSPECIFIED'\nif 'clusterResourceRestoreScope' is anyting other than 'noGroupKinds'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#clusterresourceconflictpolicy\nfor more information on each policy option. Possible values: [\"USE_EXISTING_VERSION\", \"USE_BACKUP_VERSION\"]"]
    pub fn set_cluster_resource_conflict_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cluster_resource_conflict_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `namespaced_resource_restore_mode`.\nDefines the behavior for handling the situation where sets of namespaced resources\nbeing restored already exist in the target cluster.\nThis MUST be set to a value other than 'NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED'\nif the 'namespacedResourceRestoreScope' is anything other than 'noNamespaces'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#namespacedresourcerestoremode\nfor more information on each mode. Possible values: [\"DELETE_AND_RESTORE\", \"FAIL_ON_CONFLICT\"]"]
    pub fn set_namespaced_resource_restore_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.namespaced_resource_restore_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `no_namespaces`.\nDo not restore any namespaced resources if set to \"True\".\nSpecifying this field to \"False\" is not allowed."]
    pub fn set_no_namespaces(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `volume_data_restore_policy`.\nSpecifies the mechanism to be used to restore volume data.\nThis should be set to a value other than 'NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED'\nif the 'namespacedResourceRestoreScope' is anything other than 'noNamespaces'.\nIf not specified, it will be treated as 'NO_VOLUME_DATA_RESTORATION'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#VolumeDataRestorePolicy\nfor more information on each policy option. Possible values: [\"RESTORE_VOLUME_DATA_FROM_BACKUP\", \"REUSE_VOLUME_HANDLE_FROM_BACKUP\", \"NO_VOLUME_DATA_RESTORATION\"]"]
    pub fn set_volume_data_restore_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.volume_data_restore_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_resource_restore_scope`.\n"]
    pub fn set_cluster_resource_restore_scope(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cluster_resource_restore_scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cluster_resource_restore_scope = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `excluded_namespaces`.\n"]
    pub fn set_excluded_namespaces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElExcludedNamespacesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.excluded_namespaces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.excluded_namespaces = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selected_applications`.\n"]
    pub fn set_selected_applications(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selected_applications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_applications = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selected_namespaces`.\n"]
    pub fn set_selected_namespaces(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElSelectedNamespacesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selected_namespaces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_namespaces = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transformation_rules`.\n"]
    pub fn set_transformation_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigElTransformationRulesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transformation_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeBackupRestorePlanRestoreConfigEl {
    type O = terrars::BlockAssignable<GkeBackupRestorePlanRestoreConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanRestoreConfigEl {}

impl BuildGkeBackupRestorePlanRestoreConfigEl {
    pub fn build(self) -> GkeBackupRestorePlanRestoreConfigEl {
        GkeBackupRestorePlanRestoreConfigEl {
            all_namespaces: core::default::Default::default(),
            cluster_resource_conflict_policy: core::default::Default::default(),
            namespaced_resource_restore_mode: core::default::Default::default(),
            no_namespaces: core::default::Default::default(),
            volume_data_restore_policy: core::default::Default::default(),
            cluster_resource_restore_scope: core::default::Default::default(),
            excluded_namespaces: core::default::Default::default(),
            selected_applications: core::default::Default::default(),
            selected_namespaces: core::default::Default::default(),
            transformation_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupRestorePlanRestoreConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanRestoreConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeBackupRestorePlanRestoreConfigElRef {
        GkeBackupRestorePlanRestoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanRestoreConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all_namespaces` after provisioning.\nIf True, restore all namespaced resources in the Backup.\nSetting this field to False will result in an error."]
    pub fn all_namespaces(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_resource_conflict_policy` after provisioning.\nDefines the behavior for handling the situation where cluster-scoped resources\nbeing restored already exist in the target cluster.\nThis MUST be set to a value other than 'CLUSTER_RESOURCE_CONFLICT_POLICY_UNSPECIFIED'\nif 'clusterResourceRestoreScope' is anyting other than 'noGroupKinds'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#clusterresourceconflictpolicy\nfor more information on each policy option. Possible values: [\"USE_EXISTING_VERSION\", \"USE_BACKUP_VERSION\"]"]
    pub fn cluster_resource_conflict_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_resource_conflict_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespaced_resource_restore_mode` after provisioning.\nDefines the behavior for handling the situation where sets of namespaced resources\nbeing restored already exist in the target cluster.\nThis MUST be set to a value other than 'NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED'\nif the 'namespacedResourceRestoreScope' is anything other than 'noNamespaces'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#namespacedresourcerestoremode\nfor more information on each mode. Possible values: [\"DELETE_AND_RESTORE\", \"FAIL_ON_CONFLICT\"]"]
    pub fn namespaced_resource_restore_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespaced_resource_restore_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `no_namespaces` after provisioning.\nDo not restore any namespaced resources if set to \"True\".\nSpecifying this field to \"False\" is not allowed."]
    pub fn no_namespaces(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_namespaces", self.base))
    }

    #[doc =
        "Get a reference to the value of field `volume_data_restore_policy` after provisioning.\nSpecifies the mechanism to be used to restore volume data.\nThis should be set to a value other than 'NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED'\nif the 'namespacedResourceRestoreScope' is anything other than 'noNamespaces'.\nIf not specified, it will be treated as 'NO_VOLUME_DATA_RESTORATION'.\nSee https://cloud.google.com/kubernetes-engine/docs/add-on/backup-for-gke/reference/rest/v1/RestoreConfig#VolumeDataRestorePolicy\nfor more information on each policy option. Possible values: [\"RESTORE_VOLUME_DATA_FROM_BACKUP\", \"REUSE_VOLUME_HANDLE_FROM_BACKUP\", \"NO_VOLUME_DATA_RESTORATION\"]"]
    pub fn volume_data_restore_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_data_restore_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_resource_restore_scope` after provisioning.\n"]
    pub fn cluster_resource_restore_scope(
        &self,
    ) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElClusterResourceRestoreScopeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_resource_restore_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_namespaces` after provisioning.\n"]
    pub fn excluded_namespaces(&self) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElExcludedNamespacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_namespaces", self.base))
    }

    #[doc = "Get a reference to the value of field `selected_applications` after provisioning.\n"]
    pub fn selected_applications(
        &self,
    ) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElSelectedApplicationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selected_applications", self.base))
    }

    #[doc = "Get a reference to the value of field `selected_namespaces` after provisioning.\n"]
    pub fn selected_namespaces(&self) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElSelectedNamespacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selected_namespaces", self.base))
    }

    #[doc = "Get a reference to the value of field `transformation_rules` after provisioning.\n"]
    pub fn transformation_rules(&self) -> terrars::ListRef<GkeBackupRestorePlanRestoreConfigElTransformationRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transformation_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeBackupRestorePlanTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeBackupRestorePlanTimeoutsEl {
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

impl terrars::ToListMappable for GkeBackupRestorePlanTimeoutsEl {
    type O = terrars::BlockAssignable<GkeBackupRestorePlanTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeBackupRestorePlanTimeoutsEl {}

impl BuildGkeBackupRestorePlanTimeoutsEl {
    pub fn build(self) -> GkeBackupRestorePlanTimeoutsEl {
        GkeBackupRestorePlanTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct GkeBackupRestorePlanTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeBackupRestorePlanTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeBackupRestorePlanTimeoutsElRef {
        GkeBackupRestorePlanTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeBackupRestorePlanTimeoutsElRef {
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
struct GkeBackupRestorePlanDynamic {
    restore_config: std::option::Option<terrars::DynamicBlock<GkeBackupRestorePlanRestoreConfigEl>>,
}
