use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OsConfigPatchDeploymentData {
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
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    patch_deployment_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_filter: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentInstanceFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    one_time_schedule: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentOneTimeScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch_config: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_schedule: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentRecurringScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentRolloutEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OsConfigPatchDeploymentTimeoutsEl>,
    dynamic: OsConfigPatchDeploymentDynamic,
}

struct OsConfigPatchDeployment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OsConfigPatchDeploymentData>,
}

#[derive(Clone)]
pub struct OsConfigPatchDeployment(std::rc::Rc<OsConfigPatchDeployment_>);

impl OsConfigPatchDeployment {
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
        "Set the field `description`.\nDescription of the patch deployment. Length of the description is limited to 1024 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `duration`.\nDuration of the patch. After the duration ends, the patch times out.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\""]
    pub fn set_duration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().duration = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `instance_filter`.\n"]
    pub fn set_instance_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentInstanceFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `one_time_schedule`.\n"]
    pub fn set_one_time_schedule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentOneTimeScheduleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().one_time_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.one_time_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `patch_config`.\n"]
    pub fn set_patch_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().patch_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.patch_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recurring_schedule`.\n"]
    pub fn set_recurring_schedule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recurring_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recurring_schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rollout`.\n"]
    pub fn set_rollout(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentRolloutEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rollout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rollout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OsConfigPatchDeploymentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the patch deployment was created. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the patch deployment. Length of the description is limited to 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nDuration of the patch. After the duration ends, the patch times out.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\""]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_execute_time` after provisioning.\nThe last time a patch job was started by this deployment. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_execute_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_execute_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUnique name for the patch deployment resource in a project.\nThe patch deployment name is in the form: projects/{project_id}/patchDeployments/{patchDeploymentId}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `patch_deployment_id` after provisioning.\nA name for the patch deployment in the project. When creating a name the following rules apply:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project."]
    pub fn patch_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patch_deployment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the patch deployment was last updated. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"]
    pub fn instance_filter(&self) -> terrars::ListRef<OsConfigPatchDeploymentInstanceFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `one_time_schedule` after provisioning.\n"]
    pub fn one_time_schedule(&self) -> terrars::ListRef<OsConfigPatchDeploymentOneTimeScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.one_time_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `patch_config` after provisioning.\n"]
    pub fn patch_config(&self) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.patch_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recurring_schedule` after provisioning.\n"]
    pub fn recurring_schedule(&self) -> terrars::ListRef<OsConfigPatchDeploymentRecurringScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurring_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> terrars::ListRef<OsConfigPatchDeploymentRolloutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigPatchDeploymentTimeoutsElRef {
        OsConfigPatchDeploymentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for OsConfigPatchDeployment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OsConfigPatchDeployment { }

impl terrars::ToListMappable for OsConfigPatchDeployment {
    type O = terrars::ListRef<OsConfigPatchDeploymentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OsConfigPatchDeployment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_os_config_patch_deployment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOsConfigPatchDeployment {
    pub tf_id: std::string::String,
    #[doc =
        "A name for the patch deployment in the project. When creating a name the following rules apply:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project."]
    pub patch_deployment_id: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigPatchDeployment {
    pub fn build(self, stack: &mut terrars::Stack) -> OsConfigPatchDeployment {
        let out = OsConfigPatchDeployment(std::rc::Rc::new(OsConfigPatchDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OsConfigPatchDeploymentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                duration: core::default::Default::default(),
                id: core::default::Default::default(),
                patch_deployment_id: self.patch_deployment_id,
                project: core::default::Default::default(),
                instance_filter: core::default::Default::default(),
                one_time_schedule: core::default::Default::default(),
                patch_config: core::default::Default::default(),
                recurring_schedule: core::default::Default::default(),
                rollout: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OsConfigPatchDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OsConfigPatchDeploymentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the patch deployment was created. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the patch deployment. Length of the description is limited to 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nDuration of the patch. After the duration ends, the patch times out.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\""]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_execute_time` after provisioning.\nThe last time a patch job was started by this deployment. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_execute_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_execute_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUnique name for the patch deployment resource in a project.\nThe patch deployment name is in the form: projects/{project_id}/patchDeployments/{patchDeploymentId}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `patch_deployment_id` after provisioning.\nA name for the patch deployment in the project. When creating a name the following rules apply:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the project."]
    pub fn patch_deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patch_deployment_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the patch deployment was last updated. Timestamp is in RFC3339 text format.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"]
    pub fn instance_filter(&self) -> terrars::ListRef<OsConfigPatchDeploymentInstanceFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `one_time_schedule` after provisioning.\n"]
    pub fn one_time_schedule(&self) -> terrars::ListRef<OsConfigPatchDeploymentOneTimeScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.one_time_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `patch_config` after provisioning.\n"]
    pub fn patch_config(&self) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.patch_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recurring_schedule` after provisioning.\n"]
    pub fn recurring_schedule(&self) -> terrars::ListRef<OsConfigPatchDeploymentRecurringScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurring_schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> terrars::ListRef<OsConfigPatchDeploymentRolloutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigPatchDeploymentTimeoutsElRef {
        OsConfigPatchDeploymentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
    labels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl { }

impl terrars::ToListMappable for OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
    #[doc = "Compute Engine instance labels that must be present for a VM instance to be targeted by this filter"]
    pub labels: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildOsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
    pub fn build(self) -> OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl {
        OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl { labels: self.labels }
    }
}

pub struct OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef {
        OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nCompute Engine instance labels that must be present for a VM instance to be targeted by this filter"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentInstanceFilterElDynamic {
    group_labels: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl>>,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentInstanceFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_name_prefixes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instances: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zones: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_labels: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl>>,
    dynamic: OsConfigPatchDeploymentInstanceFilterElDynamic,
}

impl OsConfigPatchDeploymentInstanceFilterEl {
    #[doc = "Set the field `all`.\nTarget all VM instances in the project. If true, no other criteria is permitted."]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instance_name_prefixes`.\nTargets VMs whose name starts with one of these prefixes. Similar to labels, this is another way to group\nVMs when targeting configs, for example prefix=\"prod-\"."]
    pub fn set_instance_name_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_name_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instances`.\nTargets any of the VM instances specified. Instances are specified by their URI in the 'form zones/{{zone}}/instances/{{instance_name}}',\n'projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}', or\n'https://www.googleapis.com/compute/v1/projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}'"]
    pub fn set_instances(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zones`.\nTargets VM instances in ANY of these zones. Leave empty to target VM instances in any zone."]
    pub fn set_zones(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_labels`.\n"]
    pub fn set_group_labels(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentInstanceFilterElGroupLabelsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.group_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.group_labels = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentInstanceFilterEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentInstanceFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentInstanceFilterEl {}

impl BuildOsConfigPatchDeploymentInstanceFilterEl {
    pub fn build(self) -> OsConfigPatchDeploymentInstanceFilterEl {
        OsConfigPatchDeploymentInstanceFilterEl {
            all: core::default::Default::default(),
            instance_name_prefixes: core::default::Default::default(),
            instances: core::default::Default::default(),
            zones: core::default::Default::default(),
            group_labels: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentInstanceFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentInstanceFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigPatchDeploymentInstanceFilterElRef {
        OsConfigPatchDeploymentInstanceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentInstanceFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all` after provisioning.\nTarget all VM instances in the project. If true, no other criteria is permitted."]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_name_prefixes` after provisioning.\nTargets VMs whose name starts with one of these prefixes. Similar to labels, this is another way to group\nVMs when targeting configs, for example prefix=\"prod-\"."]
    pub fn instance_name_prefixes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_name_prefixes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instances` after provisioning.\nTargets any of the VM instances specified. Instances are specified by their URI in the 'form zones/{{zone}}/instances/{{instance_name}}',\n'projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}', or\n'https://www.googleapis.com/compute/v1/projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}'"]
    pub fn instances(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zones` after provisioning.\nTargets VM instances in ANY of these zones. Leave empty to target VM instances in any zone."]
    pub fn zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zones", self.base))
    }

    #[doc = "Get a reference to the value of field `group_labels` after provisioning.\n"]
    pub fn group_labels(&self) -> terrars::ListRef<OsConfigPatchDeploymentInstanceFilterElGroupLabelsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_labels", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentOneTimeScheduleEl {
    execute_time: terrars::PrimField<std::string::String>,
}

impl OsConfigPatchDeploymentOneTimeScheduleEl { }

impl terrars::ToListMappable for OsConfigPatchDeploymentOneTimeScheduleEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentOneTimeScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentOneTimeScheduleEl {
    #[doc =
        "The desired patch job execution time. A timestamp in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub execute_time: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigPatchDeploymentOneTimeScheduleEl {
    pub fn build(self) -> OsConfigPatchDeploymentOneTimeScheduleEl {
        OsConfigPatchDeploymentOneTimeScheduleEl { execute_time: self.execute_time }
    }
}

pub struct OsConfigPatchDeploymentOneTimeScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentOneTimeScheduleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigPatchDeploymentOneTimeScheduleElRef {
        OsConfigPatchDeploymentOneTimeScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentOneTimeScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `execute_time` after provisioning.\nThe desired patch job execution time. A timestamp in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn execute_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execute_time", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElAptEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusive_packages: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigPatchDeploymentPatchConfigElAptEl {
    #[doc = "Set the field `excludes`.\nList of packages to exclude from update. These packages will be excluded."]
    pub fn set_excludes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclusive_packages`.\nAn exclusive list of packages to be updated. These are the only packages that will be updated.\nIf these packages are not installed, they will be ignored. This field cannot be specified with\nany other patch configuration fields."]
    pub fn set_exclusive_packages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusive_packages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nBy changing the type to DIST, the patching is performed using apt-get dist-upgrade instead. Possible values: [\"DIST\", \"UPGRADE\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElAptEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElAptEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElAptEl {}

impl BuildOsConfigPatchDeploymentPatchConfigElAptEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElAptEl {
        OsConfigPatchDeploymentPatchConfigElAptEl {
            excludes: core::default::Default::default(),
            exclusive_packages: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElAptElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElAptElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigPatchDeploymentPatchConfigElAptElRef {
        OsConfigPatchDeploymentPatchConfigElAptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElAptElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `excludes` after provisioning.\nList of packages to exclude from update. These packages will be excluded."]
    pub fn excludes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclusive_packages` after provisioning.\nAn exclusive list of packages to be updated. These are the only packages that will be updated.\nIf these packages are not installed, they will be ignored. This field cannot be specified with\nany other patch configuration fields."]
    pub fn exclusive_packages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusive_packages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nBy changing the type to DIST, the patching is performed using apt-get dist-upgrade instead. Possible values: [\"DIST\", \"UPGRADE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElGooEl {
    enabled: terrars::PrimField<bool>,
}

impl OsConfigPatchDeploymentPatchConfigElGooEl { }

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElGooEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElGooEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElGooEl {
    #[doc = "goo update settings. Use this setting to override the default goo patch rules."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildOsConfigPatchDeploymentPatchConfigElGooEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElGooEl {
        OsConfigPatchDeploymentPatchConfigElGooEl { enabled: self.enabled }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElGooElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElGooElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigPatchDeploymentPatchConfigElGooElRef {
        OsConfigPatchDeploymentPatchConfigElGooElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElGooElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\ngoo update settings. Use this setting to override the default goo patch rules."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
    bucket: terrars::PrimField<std::string::String>,
    generation_number: terrars::PrimField<std::string::String>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl { }

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
    type O =
        terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc =
        "Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub generation_number: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
        OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl {
            bucket: self.bucket,
            generation_number: self.generation_number,
            object: self.object,
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef {
        OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation_number` after provisioning.\nGeneration number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub fn generation_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation_number", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElDynamic {
    gcs_object: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl>>,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_success_codes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interpreter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_object: std
    ::option
    ::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElDynamic,
}

impl OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
    #[doc =
        "Set the field `allowed_success_codes`.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn set_allowed_success_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.allowed_success_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interpreter`.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn set_interpreter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interpreter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nAn absolute path to the executable on the VM."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_object`.\n"]
    pub fn set_gcs_object(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_object = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_object = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {}

impl BuildOsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
        OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl {
            allowed_success_codes: core::default::Default::default(),
            interpreter: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs_object: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef {
        OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_success_codes` after provisioning.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn allowed_success_codes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_success_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nAn absolute path to the executable on the VM."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_object` after provisioning.\n"]
    pub fn gcs_object(
        &self,
    ) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElGcsObjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_object", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
    bucket: terrars::PrimField<std::string::String>,
    generation_number: terrars::PrimField<std::string::String>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl { }

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
    type O =
        terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc =
        "Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub generation_number: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
        OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl {
            bucket: self.bucket,
            generation_number: self.generation_number,
            object: self.object,
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef {
        OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation_number` after provisioning.\nGeneration number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub fn generation_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation_number", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElDynamic {
    gcs_object: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_success_codes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interpreter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_object: std
    ::option
    ::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElDynamic,
}

impl OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
    #[doc =
        "Set the field `allowed_success_codes`.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn set_allowed_success_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.allowed_success_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interpreter`.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn set_interpreter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interpreter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nAn absolute path to the executable on the VM."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_object`.\n"]
    pub fn set_gcs_object(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_object = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_object = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {}

impl BuildOsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
        OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl {
            allowed_success_codes: core::default::Default::default(),
            interpreter: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs_object: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef {
        OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_success_codes` after provisioning.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn allowed_success_codes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_success_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nAn absolute path to the executable on the VM."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_object` after provisioning.\n"]
    pub fn gcs_object(
        &self,
    ) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElGcsObjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_object", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPostStepElDynamic {
    linux_exec_step_config: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl>>,
    windows_exec_step_config: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl>>,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPostStepEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_exec_step_config: std
    ::option
    ::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    windows_exec_step_config: std
    ::option
    ::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElPostStepElDynamic,
}

impl OsConfigPatchDeploymentPatchConfigElPostStepEl {
    #[doc = "Set the field `linux_exec_step_config`.\n"]
    pub fn set_linux_exec_step_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.linux_exec_step_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.linux_exec_step_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `windows_exec_step_config`.\n"]
    pub fn set_windows_exec_step_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.windows_exec_step_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.windows_exec_step_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElPostStepEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElPostStepEl {}

impl BuildOsConfigPatchDeploymentPatchConfigElPostStepEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPostStepEl {
        OsConfigPatchDeploymentPatchConfigElPostStepEl {
            linux_exec_step_config: core::default::Default::default(),
            windows_exec_step_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElPostStepElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElPostStepElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElPostStepElRef {
        OsConfigPatchDeploymentPatchConfigElPostStepElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElPostStepElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `linux_exec_step_config` after provisioning.\n"]
    pub fn linux_exec_step_config(
        &self,
    ) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElPostStepElLinuxExecStepConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_exec_step_config", self.base))
    }

    #[doc = "Get a reference to the value of field `windows_exec_step_config` after provisioning.\n"]
    pub fn windows_exec_step_config(
        &self,
    ) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElPostStepElWindowsExecStepConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.windows_exec_step_config", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
    bucket: terrars::PrimField<std::string::String>,
    generation_number: terrars::PrimField<std::string::String>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl { }

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc =
        "Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub generation_number: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
        OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl {
            bucket: self.bucket,
            generation_number: self.generation_number,
            object: self.object,
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef {
        OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation_number` after provisioning.\nGeneration number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub fn generation_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation_number", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElDynamic {
    gcs_object: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl>>,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_success_codes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interpreter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_object: std
    ::option
    ::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElDynamic,
}

impl OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
    #[doc =
        "Set the field `allowed_success_codes`.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn set_allowed_success_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.allowed_success_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interpreter`.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn set_interpreter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interpreter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nAn absolute path to the executable on the VM."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_object`.\n"]
    pub fn set_gcs_object(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_object = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_object = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {}

impl BuildOsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
        OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl {
            allowed_success_codes: core::default::Default::default(),
            interpreter: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs_object: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef {
        OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_success_codes` after provisioning.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn allowed_success_codes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_success_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nAn absolute path to the executable on the VM."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_object` after provisioning.\n"]
    pub fn gcs_object(
        &self,
    ) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElGcsObjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_object", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
    bucket: terrars::PrimField<std::string::String>,
    generation_number: terrars::PrimField<std::string::String>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl { }

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
    type O =
        terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc =
        "Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub generation_number: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
        OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl {
            bucket: self.bucket,
            generation_number: self.generation_number,
            object: self.object,
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef {
        OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation_number` after provisioning.\nGeneration number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change."]
    pub fn generation_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation_number", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElDynamic {
    gcs_object: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl>>,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_success_codes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interpreter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_object: std
    ::option
    ::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElDynamic,
}

impl OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
    #[doc =
        "Set the field `allowed_success_codes`.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn set_allowed_success_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.allowed_success_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interpreter`.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn set_interpreter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interpreter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nAn absolute path to the executable on the VM."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_object`.\n"]
    pub fn set_gcs_object(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_object = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_object = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {}

impl BuildOsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
        OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl {
            allowed_success_codes: core::default::Default::default(),
            interpreter: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs_object: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef {
        OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_success_codes` after provisioning.\nDefaults to [0]. A list of possible return values that the execution can return to indicate a success."]
    pub fn allowed_success_codes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_success_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use to run the script. If no interpreter is specified the script will\nbe executed directly, which will likely only succeed for scripts with shebang lines. Possible values: [\"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nAn absolute path to the executable on the VM."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_object` after provisioning.\n"]
    pub fn gcs_object(
        &self,
    ) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElGcsObjectElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_object", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElPreStepElDynamic {
    linux_exec_step_config: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl>>,
    windows_exec_step_config: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl>>,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElPreStepEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linux_exec_step_config: std
    ::option
    ::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    windows_exec_step_config: std
    ::option
    ::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElPreStepElDynamic,
}

impl OsConfigPatchDeploymentPatchConfigElPreStepEl {
    #[doc = "Set the field `linux_exec_step_config`.\n"]
    pub fn set_linux_exec_step_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.linux_exec_step_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.linux_exec_step_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `windows_exec_step_config`.\n"]
    pub fn set_windows_exec_step_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.windows_exec_step_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.windows_exec_step_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElPreStepEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElPreStepEl {}

impl BuildOsConfigPatchDeploymentPatchConfigElPreStepEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElPreStepEl {
        OsConfigPatchDeploymentPatchConfigElPreStepEl {
            linux_exec_step_config: core::default::Default::default(),
            windows_exec_step_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElPreStepElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElPreStepElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElPreStepElRef {
        OsConfigPatchDeploymentPatchConfigElPreStepElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElPreStepElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `linux_exec_step_config` after provisioning.\n"]
    pub fn linux_exec_step_config(
        &self,
    ) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElPreStepElLinuxExecStepConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linux_exec_step_config", self.base))
    }

    #[doc = "Get a reference to the value of field `windows_exec_step_config` after provisioning.\n"]
    pub fn windows_exec_step_config(
        &self,
    ) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElPreStepElWindowsExecStepConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.windows_exec_step_config", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classifications: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusive_patches: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
    #[doc =
        "Set the field `classifications`.\nOnly apply updates of these windows update classifications. If empty, all updates are applied. Possible values: [\"CRITICAL\", \"SECURITY\", \"DEFINITION\", \"DRIVER\", \"FEATURE_PACK\", \"SERVICE_PACK\", \"TOOL\", \"UPDATE_ROLLUP\", \"UPDATE\"]"]
    pub fn set_classifications(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.classifications = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excludes`.\nList of KBs to exclude from update."]
    pub fn set_excludes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclusive_patches`.\nAn exclusive list of kbs to be updated. These are the only patches that will be updated.\nThis field must not be used with other patch configurations."]
    pub fn set_exclusive_patches(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusive_patches = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {}

impl BuildOsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
        OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl {
            classifications: core::default::Default::default(),
            excludes: core::default::Default::default(),
            exclusive_patches: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef {
        OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `classifications` after provisioning.\nOnly apply updates of these windows update classifications. If empty, all updates are applied. Possible values: [\"CRITICAL\", \"SECURITY\", \"DEFINITION\", \"DRIVER\", \"FEATURE_PACK\", \"SERVICE_PACK\", \"TOOL\", \"UPDATE_ROLLUP\", \"UPDATE\"]"]
    pub fn classifications(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.classifications", self.base))
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\nList of KBs to exclude from update."]
    pub fn excludes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclusive_patches` after provisioning.\nAn exclusive list of kbs to be updated. These are the only patches that will be updated.\nThis field must not be used with other patch configurations."]
    pub fn exclusive_patches(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusive_patches", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElYumEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusive_packages: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimal: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security: std::option::Option<terrars::PrimField<bool>>,
}

impl OsConfigPatchDeploymentPatchConfigElYumEl {
    #[doc = "Set the field `excludes`.\nList of packages to exclude from update. These packages will be excluded."]
    pub fn set_excludes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclusive_packages`.\nAn exclusive list of packages to be updated. These are the only packages that will be updated.\nIf these packages are not installed, they will be ignored. This field cannot be specified with\nany other patch configuration fields."]
    pub fn set_exclusive_packages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusive_packages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimal`.\nWill cause patch to run yum update-minimal instead."]
    pub fn set_minimal(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.minimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security`.\nAdds the --security flag to yum update. Not supported on all platforms."]
    pub fn set_security(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.security = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElYumEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElYumEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElYumEl {}

impl BuildOsConfigPatchDeploymentPatchConfigElYumEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElYumEl {
        OsConfigPatchDeploymentPatchConfigElYumEl {
            excludes: core::default::Default::default(),
            exclusive_packages: core::default::Default::default(),
            minimal: core::default::Default::default(),
            security: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElYumElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElYumElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigPatchDeploymentPatchConfigElYumElRef {
        OsConfigPatchDeploymentPatchConfigElYumElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElYumElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `excludes` after provisioning.\nList of packages to exclude from update. These packages will be excluded."]
    pub fn excludes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclusive_packages` after provisioning.\nAn exclusive list of packages to be updated. These are the only packages that will be updated.\nIf these packages are not installed, they will be ignored. This field cannot be specified with\nany other patch configuration fields."]
    pub fn exclusive_packages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusive_packages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimal` after provisioning.\nWill cause patch to run yum update-minimal instead."]
    pub fn minimal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security` after provisioning.\nAdds the --security flag to yum update. Not supported on all platforms."]
    pub fn security(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigElZypperEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    categories: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusive_patches: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    with_optional: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    with_update: std::option::Option<terrars::PrimField<bool>>,
}

impl OsConfigPatchDeploymentPatchConfigElZypperEl {
    #[doc =
        "Set the field `categories`.\nInstall only patches with these categories. Common categories include security, recommended, and feature."]
    pub fn set_categories(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.categories = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excludes`.\nList of packages to exclude from update."]
    pub fn set_excludes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excludes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exclusive_patches`.\nAn exclusive list of patches to be updated. These are the only patches that will be installed using 'zypper patch patch:' command.\nThis field must not be used with any other patch configuration fields."]
    pub fn set_exclusive_patches(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclusive_patches = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `severities`.\nInstall only patches with these severities. Common severities include critical, important, moderate, and low."]
    pub fn set_severities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.severities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `with_optional`.\nAdds the --with-optional flag to zypper patch."]
    pub fn set_with_optional(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.with_optional = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `with_update`.\nAdds the --with-update flag, to zypper patch."]
    pub fn set_with_update(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.with_update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigElZypperEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElZypperEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigElZypperEl {}

impl BuildOsConfigPatchDeploymentPatchConfigElZypperEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigElZypperEl {
        OsConfigPatchDeploymentPatchConfigElZypperEl {
            categories: core::default::Default::default(),
            excludes: core::default::Default::default(),
            exclusive_patches: core::default::Default::default(),
            severities: core::default::Default::default(),
            with_optional: core::default::Default::default(),
            with_update: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElZypperElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElZypperElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentPatchConfigElZypperElRef {
        OsConfigPatchDeploymentPatchConfigElZypperElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElZypperElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `categories` after provisioning.\nInstall only patches with these categories. Common categories include security, recommended, and feature."]
    pub fn categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categories", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excludes` after provisioning.\nList of packages to exclude from update."]
    pub fn excludes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclusive_patches` after provisioning.\nAn exclusive list of patches to be updated. These are the only patches that will be installed using 'zypper patch patch:' command.\nThis field must not be used with any other patch configuration fields."]
    pub fn exclusive_patches(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusive_patches", self.base))
    }

    #[doc =
        "Get a reference to the value of field `severities` after provisioning.\nInstall only patches with these severities. Common severities include critical, important, moderate, and low."]
    pub fn severities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.severities", self.base))
    }

    #[doc =
        "Get a reference to the value of field `with_optional` after provisioning.\nAdds the --with-optional flag to zypper patch."]
    pub fn with_optional(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_optional", self.base))
    }

    #[doc =
        "Get a reference to the value of field `with_update` after provisioning.\nAdds the --with-update flag, to zypper patch."]
    pub fn with_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_update", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentPatchConfigElDynamic {
    apt: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElAptEl>>,
    goo: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElGooEl>>,
    post_step: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElPostStepEl>>,
    pre_step: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElPreStepEl>>,
    windows_update: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl>>,
    yum: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElYumEl>>,
    zypper: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigElZypperEl>>,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentPatchConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mig_instances_allowed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reboot_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apt: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElAptEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    goo: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElGooEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_step: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElPostStepEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_step: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElPreStepEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    windows_update: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    yum: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElYumEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zypper: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentPatchConfigElZypperEl>>,
    dynamic: OsConfigPatchDeploymentPatchConfigElDynamic,
}

impl OsConfigPatchDeploymentPatchConfigEl {
    #[doc = "Set the field `mig_instances_allowed`.\nAllows the patch job to run on Managed instance groups (MIGs)."]
    pub fn set_mig_instances_allowed(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mig_instances_allowed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reboot_config`.\nPost-patch reboot settings. Possible values: [\"DEFAULT\", \"ALWAYS\", \"NEVER\"]"]
    pub fn set_reboot_config(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reboot_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `apt`.\n"]
    pub fn set_apt(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElAptEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.apt = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.apt = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `goo`.\n"]
    pub fn set_goo(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElGooEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.goo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.goo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `post_step`.\n"]
    pub fn set_post_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPostStepEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.post_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.post_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pre_step`.\n"]
    pub fn set_pre_step(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElPreStepEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pre_step = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_step = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `windows_update`.\n"]
    pub fn set_windows_update(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElWindowsUpdateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.windows_update = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.windows_update = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `yum`.\n"]
    pub fn set_yum(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElYumEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.yum = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.yum = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zypper`.\n"]
    pub fn set_zypper(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigElZypperEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zypper = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zypper = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentPatchConfigEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentPatchConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentPatchConfigEl {}

impl BuildOsConfigPatchDeploymentPatchConfigEl {
    pub fn build(self) -> OsConfigPatchDeploymentPatchConfigEl {
        OsConfigPatchDeploymentPatchConfigEl {
            mig_instances_allowed: core::default::Default::default(),
            reboot_config: core::default::Default::default(),
            apt: core::default::Default::default(),
            goo: core::default::Default::default(),
            post_step: core::default::Default::default(),
            pre_step: core::default::Default::default(),
            windows_update: core::default::Default::default(),
            yum: core::default::Default::default(),
            zypper: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentPatchConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentPatchConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigPatchDeploymentPatchConfigElRef {
        OsConfigPatchDeploymentPatchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentPatchConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mig_instances_allowed` after provisioning.\nAllows the patch job to run on Managed instance groups (MIGs)."]
    pub fn mig_instances_allowed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mig_instances_allowed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reboot_config` after provisioning.\nPost-patch reboot settings. Possible values: [\"DEFAULT\", \"ALWAYS\", \"NEVER\"]"]
    pub fn reboot_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reboot_config", self.base))
    }

    #[doc = "Get a reference to the value of field `apt` after provisioning.\n"]
    pub fn apt(&self) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElAptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apt", self.base))
    }

    #[doc = "Get a reference to the value of field `goo` after provisioning.\n"]
    pub fn goo(&self) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElGooElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.goo", self.base))
    }

    #[doc = "Get a reference to the value of field `post_step` after provisioning.\n"]
    pub fn post_step(&self) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElPostStepElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_step", self.base))
    }

    #[doc = "Get a reference to the value of field `pre_step` after provisioning.\n"]
    pub fn pre_step(&self) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElPreStepElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pre_step", self.base))
    }

    #[doc = "Get a reference to the value of field `windows_update` after provisioning.\n"]
    pub fn windows_update(&self) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElWindowsUpdateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.windows_update", self.base))
    }

    #[doc = "Get a reference to the value of field `yum` after provisioning.\n"]
    pub fn yum(&self) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElYumElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yum", self.base))
    }

    #[doc = "Get a reference to the value of field `zypper` after provisioning.\n"]
    pub fn zypper(&self) -> terrars::ListRef<OsConfigPatchDeploymentPatchConfigElZypperElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zypper", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
    day_of_week: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_offset: std::option::Option<terrars::PrimField<f64>>,
    week_ordinal: terrars::PrimField<f64>,
}

impl OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
    #[doc =
        "Set the field `day_offset`.\nRepresents the number of days before or after the given week day of month that the patch deployment is scheduled for."]
    pub fn set_day_offset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day_offset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
    #[doc =
        "A day of the week. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: terrars::PrimField<std::string::String>,
    #[doc =
        "Week number in a month. 1-4 indicates the 1st to 4th week of the month. -1 indicates the last week of the month."]
    pub week_ordinal: terrars::PrimField<f64>,
}

impl BuildOsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
        OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl {
            day_of_week: self.day_of_week,
            day_offset: core::default::Default::default(),
            week_ordinal: self.week_ordinal,
        }
    }
}

pub struct OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef {
        OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day_of_week` after provisioning.\nA day of the week. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc =
        "Get a reference to the value of field `day_offset` after provisioning.\nRepresents the number of days before or after the given week day of month that the patch deployment is scheduled for."]
    pub fn day_offset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_offset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `week_ordinal` after provisioning.\nWeek number in a month. 1-4 indicates the 1st to 4th week of the month. -1 indicates the last week of the month."]
    pub fn week_ordinal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.week_ordinal", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentRecurringScheduleElMonthlyElDynamic {
    week_day_of_month: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl>>,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month_day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    week_day_of_month: std
    ::option
    ::Option<std::vec::Vec<OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl>>,
    dynamic: OsConfigPatchDeploymentRecurringScheduleElMonthlyElDynamic,
}

impl OsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
    #[doc =
        "Set the field `month_day`.\nOne day of the month. 1-31 indicates the 1st to the 31st day. -1 indicates the last day of the month.\nMonths without the target day will be skipped. For example, a schedule to run \"every month on the 31st\"\nwill not run in February, April, June, etc."]
    pub fn set_month_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `week_day_of_month`.\n"]
    pub fn set_week_day_of_month(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.week_day_of_month = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.week_day_of_month = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElMonthlyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentRecurringScheduleElMonthlyEl {}

impl BuildOsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
        OsConfigPatchDeploymentRecurringScheduleElMonthlyEl {
            month_day: core::default::Default::default(),
            week_day_of_month: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef {
        OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `month_day` after provisioning.\nOne day of the month. 1-31 indicates the 1st to the 31st day. -1 indicates the last day of the month.\nMonths without the target day will be skipped. For example, a schedule to run \"every month on the 31st\"\nwill not run in February, April, June, etc."]
    pub fn month_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month_day", self.base))
    }

    #[doc = "Get a reference to the value of field `week_day_of_month` after provisioning.\n"]
    pub fn week_day_of_month(
        &self,
    ) -> terrars::ListRef<OsConfigPatchDeploymentRecurringScheduleElMonthlyElWeekDayOfMonthElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.week_day_of_month", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
    #[doc =
        "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {}

impl BuildOsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
        OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef {
        OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23.\nAn API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
    #[doc = "Set the field `version`.\nIANA Time Zone Database version number, e.g. \"2019a\"."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
    #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
        OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl {
            id: self.id,
            version: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef {
        OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nIANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nIANA Time Zone Database version number, e.g. \"2019a\"."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
    day_of_week: terrars::PrimField<std::string::String>,
}

impl OsConfigPatchDeploymentRecurringScheduleElWeeklyEl { }

impl terrars::ToListMappable for OsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElWeeklyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
    #[doc =
        "IANA Time Zone Database time zone, e.g. \"America/New_York\". Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleElWeeklyEl {
        OsConfigPatchDeploymentRecurringScheduleElWeeklyEl { day_of_week: self.day_of_week }
    }
}

pub struct OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef {
        OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day_of_week` after provisioning.\nIANA Time Zone Database time zone, e.g. \"America/New_York\". Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentRecurringScheduleElDynamic {
    monthly: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentRecurringScheduleElMonthlyEl>>,
    time_of_day: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl>>,
    time_zone: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl>>,
    weekly: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentRecurringScheduleElWeeklyEl>>,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRecurringScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monthly: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentRecurringScheduleElMonthlyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_of_day: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentRecurringScheduleElWeeklyEl>>,
    dynamic: OsConfigPatchDeploymentRecurringScheduleElDynamic,
}

impl OsConfigPatchDeploymentRecurringScheduleEl {
    #[doc =
        "Set the field `end_time`.\nThe end time at which a recurring patch deployment schedule is no longer active.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_time`.\nThe time that the recurring schedule becomes effective. Defaults to createTime of the patch deployment.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monthly`.\n"]
    pub fn set_monthly(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElMonthlyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monthly = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monthly = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_of_day`.\n"]
    pub fn set_time_of_day(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElTimeOfDayEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_of_day = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_of_day = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElTimeZoneEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_zone = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_zone = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weekly`.\n"]
    pub fn set_weekly(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleElWeeklyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weekly = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentRecurringScheduleEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentRecurringScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentRecurringScheduleEl {}

impl BuildOsConfigPatchDeploymentRecurringScheduleEl {
    pub fn build(self) -> OsConfigPatchDeploymentRecurringScheduleEl {
        OsConfigPatchDeploymentRecurringScheduleEl {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            monthly: core::default::Default::default(),
            time_of_day: core::default::Default::default(),
            time_zone: core::default::Default::default(),
            weekly: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentRecurringScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentRecurringScheduleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigPatchDeploymentRecurringScheduleElRef {
        OsConfigPatchDeploymentRecurringScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentRecurringScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nThe end time at which a recurring patch deployment schedule is no longer active.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_execute_time` after provisioning.\nThe time the last patch job ran successfully.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn last_execute_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_execute_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `next_execute_time` after provisioning.\nThe time the next patch job is scheduled to run.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn next_execute_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_execute_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nThe time that the recurring schedule becomes effective. Defaults to createTime of the patch deployment.\nA timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `monthly` after provisioning.\n"]
    pub fn monthly(&self) -> terrars::ListRef<OsConfigPatchDeploymentRecurringScheduleElMonthlyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monthly", self.base))
    }

    #[doc = "Get a reference to the value of field `time_of_day` after provisioning.\n"]
    pub fn time_of_day(&self) -> terrars::ListRef<OsConfigPatchDeploymentRecurringScheduleElTimeOfDayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::ListRef<OsConfigPatchDeploymentRecurringScheduleElTimeZoneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly` after provisioning.\n"]
    pub fn weekly(&self) -> terrars::ListRef<OsConfigPatchDeploymentRecurringScheduleElWeeklyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl OsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
    #[doc = "Set the field `fixed`.\nSpecifies a fixed value."]
    pub fn set_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `percentage`.\nSpecifies the relative value defined as a percentage, which will be multiplied by a reference value."]
    pub fn set_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentRolloutElDisruptionBudgetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentRolloutElDisruptionBudgetEl {}

impl BuildOsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
    pub fn build(self) -> OsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
        OsConfigPatchDeploymentRolloutElDisruptionBudgetEl {
            fixed: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef {
        OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed` after provisioning.\nSpecifies a fixed value."]
    pub fn fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percentage` after provisioning.\nSpecifies the relative value defined as a percentage, which will be multiplied by a reference value."]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentRolloutElDynamic {
    disruption_budget: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigPatchDeploymentRolloutElDisruptionBudgetEl>>,
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentRolloutEl {
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disruption_budget: std::option::Option<std::vec::Vec<OsConfigPatchDeploymentRolloutElDisruptionBudgetEl>>,
    dynamic: OsConfigPatchDeploymentRolloutElDynamic,
}

impl OsConfigPatchDeploymentRolloutEl {
    #[doc = "Set the field `disruption_budget`.\n"]
    pub fn set_disruption_budget(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigPatchDeploymentRolloutElDisruptionBudgetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disruption_budget = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disruption_budget = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigPatchDeploymentRolloutEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentRolloutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentRolloutEl {
    #[doc = "Mode of the patch rollout. Possible values: [\"ZONE_BY_ZONE\", \"CONCURRENT_ZONES\"]"]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigPatchDeploymentRolloutEl {
    pub fn build(self) -> OsConfigPatchDeploymentRolloutEl {
        OsConfigPatchDeploymentRolloutEl {
            mode: self.mode,
            disruption_budget: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentRolloutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentRolloutElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigPatchDeploymentRolloutElRef {
        OsConfigPatchDeploymentRolloutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentRolloutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nMode of the patch rollout. Possible values: [\"ZONE_BY_ZONE\", \"CONCURRENT_ZONES\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `disruption_budget` after provisioning.\n"]
    pub fn disruption_budget(&self) -> terrars::ListRef<OsConfigPatchDeploymentRolloutElDisruptionBudgetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disruption_budget", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigPatchDeploymentTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigPatchDeploymentTimeoutsEl {
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
}

impl terrars::ToListMappable for OsConfigPatchDeploymentTimeoutsEl {
    type O = terrars::BlockAssignable<OsConfigPatchDeploymentTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigPatchDeploymentTimeoutsEl {}

impl BuildOsConfigPatchDeploymentTimeoutsEl {
    pub fn build(self) -> OsConfigPatchDeploymentTimeoutsEl {
        OsConfigPatchDeploymentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct OsConfigPatchDeploymentTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigPatchDeploymentTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigPatchDeploymentTimeoutsElRef {
        OsConfigPatchDeploymentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigPatchDeploymentTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct OsConfigPatchDeploymentDynamic {
    instance_filter: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentInstanceFilterEl>>,
    one_time_schedule: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentOneTimeScheduleEl>>,
    patch_config: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentPatchConfigEl>>,
    recurring_schedule: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentRecurringScheduleEl>>,
    rollout: std::option::Option<terrars::DynamicBlock<OsConfigPatchDeploymentRolloutEl>>,
}
