#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FirebaseAppHostingTrafficData_ {
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
    backend: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_policy: std::option::Option<std::vec::Vec<FirebaseAppHostingTraffic_RolloutPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<FirebaseAppHostingTraffic_Target>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FirebaseAppHostingTraffic_Timeouts>,
    dynamic: FirebaseAppHostingTrafficDynamic,
}

struct FirebaseAppHostingTraffic_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FirebaseAppHostingTrafficData_>,
}

#[derive(Clone)]
pub struct FirebaseAppHostingTraffic(std::rc::Rc<FirebaseAppHostingTraffic_>);

impl FirebaseAppHostingTraffic {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollout_policy`.\n"]
    pub fn set_rollout_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseAppHostingTraffic_RolloutPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rollout_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rollout_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseAppHostingTraffic_Target>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FirebaseAppHostingTraffic_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `backend` after provisioning.\nId of the backend that this Traffic config applies to"]
    pub fn backend(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime at which the backend was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current` after provisioning.\nCurrent state of traffic allocation for the backend.\nWhen setting 'target', this field may differ for some time until the desired state is reached."]
    pub fn current(&self) -> terrars::ListRef<FirebaseAppHostingTraffic_CurrentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nTime at which the backend was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location the Backend that this Traffic config applies to"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the backend traffic config\n\nFormat:\n\n'projects/{project}/locations/{locationId}/backends/{backendId}/traffic'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime at which the backend was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollout_policy` after provisioning.\n"]
    pub fn rollout_policy(&self) -> terrars::ListRef<FirebaseAppHostingTraffic_RolloutPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<FirebaseAppHostingTraffic_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingTraffic_TimeoutsRef {
        FirebaseAppHostingTraffic_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FirebaseAppHostingTraffic {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FirebaseAppHostingTraffic { }

impl terrars::ToListMappable for FirebaseAppHostingTraffic {
    type O = terrars::ListRef<FirebaseAppHostingTrafficRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FirebaseAppHostingTraffic_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_firebase_app_hosting_traffic".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFirebaseAppHostingTraffic {
    pub tf_id: std::string::String,
    #[doc = "Id of the backend that this Traffic config applies to"]
    pub backend: terrars::PrimField<std::string::String>,
    #[doc = "The location the Backend that this Traffic config applies to"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildFirebaseAppHostingTraffic {
    pub fn build(self, stack: &mut terrars::Stack) -> FirebaseAppHostingTraffic {
        let out = FirebaseAppHostingTraffic(std::rc::Rc::new(FirebaseAppHostingTraffic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FirebaseAppHostingTrafficData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                backend: self.backend,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                rollout_policy: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FirebaseAppHostingTrafficRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingTrafficRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FirebaseAppHostingTrafficRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backend` after provisioning.\nId of the backend that this Traffic config applies to"]
    pub fn backend(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backend", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime at which the backend was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current` after provisioning.\nCurrent state of traffic allocation for the backend.\nWhen setting 'target', this field may differ for some time until the desired state is reached."]
    pub fn current(&self) -> terrars::ListRef<FirebaseAppHostingTraffic_CurrentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nTime at which the backend was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location the Backend that this Traffic config applies to"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the backend traffic config\n\nFormat:\n\n'projects/{project}/locations/{locationId}/backends/{backendId}/traffic'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime at which the backend was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollout_policy` after provisioning.\n"]
    pub fn rollout_policy(&self) -> terrars::ListRef<FirebaseAppHostingTraffic_RolloutPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<FirebaseAppHostingTraffic_TargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingTraffic_TimeoutsRef {
        FirebaseAppHostingTraffic_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FirebaseAppHostingTrafficDynamic {
    rollout_policy: std::option::Option<terrars::DynamicBlock<FirebaseAppHostingTraffic_RolloutPolicy>>,
    target: std::option::Option<terrars::DynamicBlock<FirebaseAppHostingTraffic_Target>>,
}

#[derive(Serialize)]
pub struct FirebaseAppHostingTraffic_Current_Splits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl FirebaseAppHostingTraffic_Current_Splits {
    #[doc = "Set the field `build`.\n"]
    pub fn set_build(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.build = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingTraffic_Current_Splits {
    type O = terrars::BlockAssignable<FirebaseAppHostingTraffic_Current_Splits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingTraffic_Current_Splits {}

impl BuildFirebaseAppHostingTraffic_Current_Splits {
    pub fn build(self) -> FirebaseAppHostingTraffic_Current_Splits {
        FirebaseAppHostingTraffic_Current_Splits {
            build: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingTraffic_Current_SplitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingTraffic_Current_SplitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingTraffic_Current_SplitsRef {
        FirebaseAppHostingTraffic_Current_SplitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingTraffic_Current_SplitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `build` after provisioning.\n"]
    pub fn build(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build", self.base))
    }

    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingTraffic_Current {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    splits: std::option::Option<terrars::ListField<FirebaseAppHostingTraffic_Current_Splits>>,
}

impl FirebaseAppHostingTraffic_Current {
    #[doc = "Set the field `splits`.\n"]
    pub fn set_splits(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FirebaseAppHostingTraffic_Current_Splits>>,
    ) -> Self {
        self.splits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingTraffic_Current {
    type O = terrars::BlockAssignable<FirebaseAppHostingTraffic_Current>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingTraffic_Current {}

impl BuildFirebaseAppHostingTraffic_Current {
    pub fn build(self) -> FirebaseAppHostingTraffic_Current {
        FirebaseAppHostingTraffic_Current { splits: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingTraffic_CurrentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingTraffic_CurrentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingTraffic_CurrentRef {
        FirebaseAppHostingTraffic_CurrentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingTraffic_CurrentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `splits` after provisioning.\n"]
    pub fn splits(&self) -> terrars::ListRef<FirebaseAppHostingTraffic_Current_SplitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.splits", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingTraffic_RolloutPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    codebase_branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl FirebaseAppHostingTraffic_RolloutPolicy {
    #[doc =
        "Set the field `codebase_branch`.\nSpecifies a branch that triggers a new build to be started with this\npolicy. If not set, no automatic rollouts will happen."]
    pub fn set_codebase_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.codebase_branch = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nA flag that, if true, prevents rollouts from being created via this RolloutPolicy."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingTraffic_RolloutPolicy {
    type O = terrars::BlockAssignable<FirebaseAppHostingTraffic_RolloutPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingTraffic_RolloutPolicy {}

impl BuildFirebaseAppHostingTraffic_RolloutPolicy {
    pub fn build(self) -> FirebaseAppHostingTraffic_RolloutPolicy {
        FirebaseAppHostingTraffic_RolloutPolicy {
            codebase_branch: core::default::Default::default(),
            disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingTraffic_RolloutPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingTraffic_RolloutPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingTraffic_RolloutPolicyRef {
        FirebaseAppHostingTraffic_RolloutPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingTraffic_RolloutPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `codebase_branch` after provisioning.\nSpecifies a branch that triggers a new build to be started with this\npolicy. If not set, no automatic rollouts will happen."]
    pub fn codebase_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.codebase_branch", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nA flag that, if true, prevents rollouts from being created via this RolloutPolicy."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled_time` after provisioning.\nIf disabled is set, the time at which the rollouts were disabled."]
    pub fn disabled_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_time", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingTraffic_Target_Splits {
    build: terrars::PrimField<std::string::String>,
    percent: terrars::PrimField<f64>,
}

impl FirebaseAppHostingTraffic_Target_Splits { }

impl terrars::ToListMappable for FirebaseAppHostingTraffic_Target_Splits {
    type O = terrars::BlockAssignable<FirebaseAppHostingTraffic_Target_Splits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingTraffic_Target_Splits {
    #[doc = "The build that traffic is being routed to."]
    pub build: terrars::PrimField<std::string::String>,
    #[doc = "The percentage of traffic to send to the build. Currently must be 100 or 0."]
    pub percent: terrars::PrimField<f64>,
}

impl BuildFirebaseAppHostingTraffic_Target_Splits {
    pub fn build(self) -> FirebaseAppHostingTraffic_Target_Splits {
        FirebaseAppHostingTraffic_Target_Splits {
            build: self.build,
            percent: self.percent,
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingTraffic_Target_SplitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingTraffic_Target_SplitsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingTraffic_Target_SplitsRef {
        FirebaseAppHostingTraffic_Target_SplitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingTraffic_Target_SplitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `build` after provisioning.\nThe build that traffic is being routed to."]
    pub fn build(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percent` after provisioning.\nThe percentage of traffic to send to the build. Currently must be 100 or 0."]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirebaseAppHostingTraffic_TargetDynamic {
    splits: std::option::Option<terrars::DynamicBlock<FirebaseAppHostingTraffic_Target_Splits>>,
}

#[derive(Serialize)]
pub struct FirebaseAppHostingTraffic_Target {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    splits: std::option::Option<std::vec::Vec<FirebaseAppHostingTraffic_Target_Splits>>,
    dynamic: FirebaseAppHostingTraffic_TargetDynamic,
}

impl FirebaseAppHostingTraffic_Target {
    #[doc = "Set the field `splits`.\n"]
    pub fn set_splits(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseAppHostingTraffic_Target_Splits>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.splits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.splits = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FirebaseAppHostingTraffic_Target {
    type O = terrars::BlockAssignable<FirebaseAppHostingTraffic_Target>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingTraffic_Target {}

impl BuildFirebaseAppHostingTraffic_Target {
    pub fn build(self) -> FirebaseAppHostingTraffic_Target {
        FirebaseAppHostingTraffic_Target {
            splits: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingTraffic_TargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingTraffic_TargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingTraffic_TargetRef {
        FirebaseAppHostingTraffic_TargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingTraffic_TargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `splits` after provisioning.\n"]
    pub fn splits(&self) -> terrars::ListRef<FirebaseAppHostingTraffic_Target_SplitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.splits", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseAppHostingTraffic_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseAppHostingTraffic_Timeouts {
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

impl terrars::ToListMappable for FirebaseAppHostingTraffic_Timeouts {
    type O = terrars::BlockAssignable<FirebaseAppHostingTraffic_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseAppHostingTraffic_Timeouts {}

impl BuildFirebaseAppHostingTraffic_Timeouts {
    pub fn build(self) -> FirebaseAppHostingTraffic_Timeouts {
        FirebaseAppHostingTraffic_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseAppHostingTraffic_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseAppHostingTraffic_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseAppHostingTraffic_TimeoutsRef {
        FirebaseAppHostingTraffic_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseAppHostingTraffic_TimeoutsRef {
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
