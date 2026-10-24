use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OsConfigOsPolicyAssignmentData {
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
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_await_rollout: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_filter: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentInstanceFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_policies: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentRolloutEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OsConfigOsPolicyAssignmentTimeoutsEl>,
    dynamic: OsConfigOsPolicyAssignmentDynamic,
}

struct OsConfigOsPolicyAssignment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OsConfigOsPolicyAssignmentData>,
}

#[derive(Clone)]
pub struct OsConfigOsPolicyAssignment(std::rc::Rc<OsConfigOsPolicyAssignment_>);

impl OsConfigOsPolicyAssignment {
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
        "Set the field `description`.\nOS policy assignment description. Length of the description is limited to 1024 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_await_rollout`.\nSet to true to skip awaiting rollout during resource creation and update."]
    pub fn set_skip_await_rollout(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_await_rollout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_filter`.\n"]
    pub fn set_instance_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterEl>>,
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

    #[doc = "Set the field `os_policies`.\n"]
    pub fn set_os_policies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().os_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.os_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rollout`.\n"]
    pub fn set_rollout(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigOsPolicyAssignmentRolloutEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<OsConfigOsPolicyAssignmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `baseline` after provisioning.\nOutput only. Indicates that this revision has been successfully rolled out in this zone and new VMs will be assigned OS policies from this revision.\nFor a given OS policy assignment, there is only one revision with a value of 'true' for this field."]
    pub fn baseline(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.baseline", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deleted` after provisioning.\nOutput only. Indicates that this revision deletes the OS policy assignment."]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOS policy assignment description. Length of the description is limited to 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag for this OS policy assignment. If this is provided on update, it must match the server's etag."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Indicates that reconciliation is in progress for the revision. This value is 'true' when the 'rollout_state' is one of:\n* IN_PROGRESS\n* CANCELLING"]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp that the revision was created."]
    pub fn revision_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The assignment revision ID A new revision is committed whenever a rollout is triggered for a OS policy assignment"]
    pub fn revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rollout_state` after provisioning.\nOutput only. OS policy assignment rollout state"]
    pub fn rollout_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_await_rollout` after provisioning.\nSet to true to skip awaiting rollout during resource creation and update."]
    pub fn skip_await_rollout(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_await_rollout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server generated unique id for the OS policy assignment resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"]
    pub fn instance_filter(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentInstanceFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_policies` after provisioning.\n"]
    pub fn os_policies(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.os_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentRolloutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigOsPolicyAssignmentTimeoutsElRef {
        OsConfigOsPolicyAssignmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for OsConfigOsPolicyAssignment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OsConfigOsPolicyAssignment { }

impl terrars::ToListMappable for OsConfigOsPolicyAssignment {
    type O = terrars::ListRef<OsConfigOsPolicyAssignmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OsConfigOsPolicyAssignment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_os_config_os_policy_assignment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOsConfigOsPolicyAssignment {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Resource name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignment {
    pub fn build(self, stack: &mut terrars::Stack) -> OsConfigOsPolicyAssignment {
        let out = OsConfigOsPolicyAssignment(std::rc::Rc::new(OsConfigOsPolicyAssignment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OsConfigOsPolicyAssignmentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                skip_await_rollout: core::default::Default::default(),
                instance_filter: core::default::Default::default(),
                os_policies: core::default::Default::default(),
                rollout: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OsConfigOsPolicyAssignmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OsConfigOsPolicyAssignmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `baseline` after provisioning.\nOutput only. Indicates that this revision has been successfully rolled out in this zone and new VMs will be assigned OS policies from this revision.\nFor a given OS policy assignment, there is only one revision with a value of 'true' for this field."]
    pub fn baseline(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.baseline", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deleted` after provisioning.\nOutput only. Indicates that this revision deletes the OS policy assignment."]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOS policy assignment description. Length of the description is limited to 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag for this OS policy assignment. If this is provided on update, it must match the server's etag."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nOutput only. Indicates that reconciliation is in progress for the revision. This value is 'true' when the 'rollout_state' is one of:\n* IN_PROGRESS\n* CANCELLING"]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp that the revision was created."]
    pub fn revision_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The assignment revision ID A new revision is committed whenever a rollout is triggered for a OS policy assignment"]
    pub fn revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rollout_state` after provisioning.\nOutput only. OS policy assignment rollout state"]
    pub fn rollout_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_await_rollout` after provisioning.\nSet to true to skip awaiting rollout during resource creation and update."]
    pub fn skip_await_rollout(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_await_rollout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. Server generated unique id for the OS policy assignment resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_filter` after provisioning.\n"]
    pub fn instance_filter(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentInstanceFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_policies` after provisioning.\n"]
    pub fn os_policies(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.os_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rollout` after provisioning.\n"]
    pub fn rollout(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentRolloutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OsConfigOsPolicyAssignmentTimeoutsElRef {
        OsConfigOsPolicyAssignmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
    #[doc =
        "Set the field `labels`.\nLabels are identified by key/value pairs in this map. A VM should contain all the key/value pairs specified in this map to be selected."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {}

impl BuildOsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl {
        OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl { labels: core::default::Default::default() }
    }
}

pub struct OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef {
        OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels are identified by key/value pairs in this map. A VM should contain all the key/value pairs specified in this map to be selected."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
    #[doc =
        "Set the field `labels`.\nLabels are identified by key/value pairs in this map. A VM should contain all the key/value pairs specified in this map to be selected."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {}

impl BuildOsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl {
        OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl { labels: core::default::Default::default() }
    }
}

pub struct OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef {
        OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels are identified by key/value pairs in this map. A VM should contain all the key/value pairs specified in this map to be selected."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
    os_short_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
    #[doc =
        "Set the field `os_version`.\nThe OS version Prefix matches are supported if asterisk(*) is provided as the last character. For example, to match all versions with a major version of '7', specify the following value for this field '7.*' An empty string matches all OS versions."]
    pub fn set_os_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.os_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
    #[doc = "The OS short name"]
    pub os_short_name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
        OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl {
            os_short_name: self.os_short_name,
            os_version: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef {
        OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `os_short_name` after provisioning.\nThe OS short name"]
    pub fn os_short_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_short_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_version` after provisioning.\nThe OS version Prefix matches are supported if asterisk(*) is provided as the last character. For example, to match all versions with a major version of '7', specify the following value for this field '7.*' An empty string matches all OS versions."]
    pub fn os_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentInstanceFilterElDynamic {
    exclusion_labels: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl>>,
    inclusion_labels: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl>>,
    inventories: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl>>,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentInstanceFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_labels: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusion_labels: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inventories: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl>>,
    dynamic: OsConfigOsPolicyAssignmentInstanceFilterElDynamic,
}

impl OsConfigOsPolicyAssignmentInstanceFilterEl {
    #[doc = "Set the field `all`.\nTarget all VMs in the project. If true, no other criteria is permitted."]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclusion_labels`.\n"]
    pub fn set_exclusion_labels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inclusion_labels`.\n"]
    pub fn set_inclusion_labels(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inclusion_labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inclusion_labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inventories`.\n"]
    pub fn set_inventories(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterElInventoriesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inventories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inventories = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentInstanceFilterEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentInstanceFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentInstanceFilterEl {}

impl BuildOsConfigOsPolicyAssignmentInstanceFilterEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentInstanceFilterEl {
        OsConfigOsPolicyAssignmentInstanceFilterEl {
            all: core::default::Default::default(),
            exclusion_labels: core::default::Default::default(),
            inclusion_labels: core::default::Default::default(),
            inventories: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentInstanceFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentInstanceFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigOsPolicyAssignmentInstanceFilterElRef {
        OsConfigOsPolicyAssignmentInstanceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentInstanceFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all` after provisioning.\nTarget all VMs in the project. If true, no other criteria is permitted."]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusion_labels` after provisioning.\n"]
    pub fn exclusion_labels(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentInstanceFilterElExclusionLabelsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion_labels` after provisioning.\n"]
    pub fn inclusion_labels(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentInstanceFilterElInclusionLabelsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inclusion_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `inventories` after provisioning.\n"]
    pub fn inventories(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentInstanceFilterElInventoriesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inventories", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
    os_short_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
    #[doc =
        "Set the field `os_version`.\nThe OS version\nPrefix matches are supported if asterisk(*) is provided as the last character. For example, to match all versions with a major version of '7', specify the following value for this field '7.*'\nAn empty string matches all OS versions."]
    pub fn set_os_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.os_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
    #[doc = "The OS short name"]
    pub os_short_name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl {
            os_short_name: self.os_short_name,
            os_version: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `os_short_name` after provisioning.\nThe OS short name"]
    pub fn os_short_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_short_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `os_version` after provisioning.\nThe OS version\nPrefix matches are supported if asterisk(*) is provided as the last character. For example, to match all versions with a major version of '7', specify the following value for this field '7.*'\nAn empty string matches all OS versions."]
    pub fn os_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_version", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<f64>>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256_checksum: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256_checksum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl {
    #[doc =
        "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl {
            sha256_checksum: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_checksum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElDynamic {
    gcs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl>,
    >,
    remote: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs: std
    ::option
    ::Option<
        std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std
    ::option
    ::Option<
        std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl {
    #[doc =
        "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type: Remote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl {
    type O =
        terrars
        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type: Remote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> terrars
    ::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElGcsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(
        &self,
    ) -> terrars
    ::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRemoteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElDynamic {
    file: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    interpreter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_file_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
    #[doc = "Set the field `args`.\nOptional arguments to pass to the source during execution."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_file_path`.\nOnly recorded for enforce Exec. Path to an output file (that is created by this Exec) whose content will be recorded in OSPolicyResourceCompliance after a successful run. Absence or failure to read this file will result in this ExecResource being non-compliant. Output file size is limited to 100K bytes."]
    pub fn set_output_file_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_file_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `script`.\nAn inline script. The size of the script is limited to 1024 characters."]
    pub fn set_script(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
    type O =
        terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
    #[doc =
        "The script interpreter to use. Possible values: [\"INTERPRETER_UNSPECIFIED\", \"NONE\", \"SHELL\", \"POWERSHELL\"]"]
    pub interpreter: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl {
            args: core::default::Default::default(),
            interpreter: self.interpreter,
            output_file_path: core::default::Default::default(),
            script: core::default::Default::default(),
            file: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nOptional arguments to pass to the source during execution."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use. Possible values: [\"INTERPRETER_UNSPECIFIED\", \"NONE\", \"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_file_path` after provisioning.\nOnly recorded for enforce Exec. Path to an output file (that is created by this Exec) whose content will be recorded in OSPolicyResourceCompliance after a successful run. Absence or failure to read this file will result in this ExecResource being non-compliant. Output file size is limited to 100K bytes."]
    pub fn output_file_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_file_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script` after provisioning.\nAn inline script. The size of the script is limited to 1024 characters."]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<f64>>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256_checksum: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256_checksum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl {
    #[doc =
        "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl {
            sha256_checksum: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_checksum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElDynamic {
    gcs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl>,
    >,
    remote: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs: std
    ::option
    ::Option<
        std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std
    ::option
    ::Option<
        std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl {
    #[doc =
        "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl {
    type O =
        terrars
        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> terrars
    ::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElGcsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(
        &self,
    ) -> terrars
    ::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRemoteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElDynamic {
    file: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    interpreter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_file_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
    #[doc = "Set the field `args`.\nOptional arguments to pass to the source during execution."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_file_path`.\nOnly recorded for enforce Exec. Path to an output file (that is created by this Exec) whose content will be recorded in OSPolicyResourceCompliance after a successful run. Absence or failure to read this file will result in this ExecResource being non-compliant. Output file size is limited to 100K bytes."]
    pub fn set_output_file_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_file_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `script`.\nAn inline script. The size of the script is limited to 1024 characters."]
    pub fn set_script(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
    type O =
        terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
    #[doc =
        "The script interpreter to use. Possible values: [\"INTERPRETER_UNSPECIFIED\", \"NONE\", \"SHELL\", \"POWERSHELL\"]"]
    pub interpreter: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl {
            args: core::default::Default::default(),
            interpreter: self.interpreter,
            output_file_path: core::default::Default::default(),
            script: core::default::Default::default(),
            file: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nOptional arguments to pass to the source during execution."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interpreter` after provisioning.\nThe script interpreter to use. Possible values: [\"INTERPRETER_UNSPECIFIED\", \"NONE\", \"SHELL\", \"POWERSHELL\"]"]
    pub fn interpreter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpreter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `output_file_path` after provisioning.\nOnly recorded for enforce Exec. Path to an output file (that is created by this Exec) whose content will be recorded in OSPolicyResourceCompliance after a successful run. Absence or failure to read this file will result in this ExecResource being non-compliant. Output file size is limited to 100K bytes."]
    pub fn output_file_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_file_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script` after provisioning.\nAn inline script. The size of the script is limited to 1024 characters."]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElDynamic {
    enforce: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl>>,
    validate: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validate: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
    #[doc = "Set the field `enforce`.\n"]
    pub fn set_enforce(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enforce = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enforce = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validate`.\n"]
    pub fn set_validate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.validate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.validate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl {
            enforce: core::default::Default::default(),
            validate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforce` after provisioning.\n"]
    pub fn enforce(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElEnforceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enforce", self.base))
    }

    #[doc = "Get a reference to the value of field `validate` after provisioning.\n"]
    pub fn validate(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElValidateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validate", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<f64>>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
    type O =
        terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256_checksum: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256_checksum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
    type O =
        terrars
        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
    #[doc =
        "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl {
            sha256_checksum: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_checksum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElDynamic {
    gcs: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl>,
    >,
    remote: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
    #[doc =
        "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type: Remote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type: Remote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElGcsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRemoteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElDynamic {
    file: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl>>,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    path: terrars::PrimField<std::string::String>,
    state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
    #[doc =
        "Set the field `content`.\nA a file with this content. The size of the content is limited to 1024 characters."]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
    #[doc = "The absolute path of the file within the VM."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc =
        "Desired state of the file. Possible values: [\"DESIRED_STATE_UNSPECIFIED\", \"PRESENT\", \"ABSENT\", \"CONTENTS_MATCH\"]"]
    pub state: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl {
            content: core::default::Default::default(),
            path: self.path,
            state: self.state,
            file: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nA a file with this content. The size of the content is limited to 1024 characters."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe absolute path of the file within the VM."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `permissions` after provisioning.\nConsists of three octal digits which represent, in order, the permissions of the owner, group, and other users for the file (similarly to the numeric mode used in the linux chmod utility). Each digit represents a three bit number with the 4 bit corresponding to the read permissions, the 2 bit corresponds to the write bit, and the one bit corresponds to the execute permission. Default behavior is 755.\nBelow are some examples of permissions and their associated values: read, write, and execute: 7 read and execute: 5 read and write: 6 read only: 4"]
    pub fn permissions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permissions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nDesired state of the file. Possible values: [\"DESIRED_STATE_UNSPECIFIED\", \"PRESENT\", \"ABSENT\", \"CONTENTS_MATCH\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {
    name: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { }

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {
    #[doc = "Package name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl { name: self.name }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nPackage name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<f64>>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl {
    type O =
        terrars
        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256_checksum: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256_checksum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl {
    #[doc =
        "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl {
            sha256_checksum: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_checksum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElDynamic {
    gcs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl>,
    >,
    remote: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std
    ::option
    ::Option<
        std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {
    #[doc =
        "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {
    type O =
        terrars
        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> terrars
    ::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElGcsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(
        &self,
    ) -> terrars
    ::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRemoteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElDynamic {
    source: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_deps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
    #[doc =
        "Set the field `pull_deps`.\nWhether dependencies should also be installed. - install when false: 'dpkg -i package' - install when true: 'apt-get update && apt-get -y install package.deb'"]
    pub fn set_pull_deps(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pull_deps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl {
            pull_deps: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pull_deps` after provisioning.\nWhether dependencies should also be installed. - install when false: 'dpkg -i package' - install when true: 'apt-get update && apt-get -y install package.deb'"]
    pub fn pull_deps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pull_deps", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {
    name: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { }

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {
    type O =
        terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {
    #[doc = "Package name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl { name: self.name }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nPackage name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<f64>>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl {
    type O =
        terrars
        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256_checksum: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256_checksum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl {
    #[doc =
        "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl {
            sha256_checksum: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_checksum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElDynamic {
    gcs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl>,
    >,
    remote: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std
    ::option
    ::Option<
        std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {
    #[doc =
        "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {
    type O =
        terrars
        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> terrars
    ::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElGcsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(
        &self,
    ) -> terrars
    ::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRemoteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElDynamic {
    source: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
    #[doc =
        "Set the field `properties`.\nAdditional properties to use during installation. This should be in the format of Property=Setting. Appended to the defaults of 'ACTION=INSTALL REBOOT=ReallySuppress'."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl {
            properties: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nAdditional properties to use during installation. This should be in the format of Property=Setting. Appended to the defaults of 'ACTION=INSTALL REBOOT=ReallySuppress'."]
    pub fn properties(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<f64>>,
    object: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl {
    #[doc = "Set the field `generation`.\nGeneration number of the Cloud Storage object."]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl {
    type O =
        terrars
        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl {
    #[doc = "Bucket of the Cloud Storage object."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Cloud Storage object."]
    pub object: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl {
            bucket: self.bucket,
            generation: core::default::Default::default(),
            object: self.object,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nBucket of the Cloud Storage object."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGeneration number of the Cloud Storage object."]
    pub fn generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\nName of the Cloud Storage object."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha256_checksum: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl {
    #[doc = "Set the field `sha256_checksum`.\nSHA256 checksum of the remote file."]
    pub fn set_sha256_checksum(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha256_checksum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl {
    type O =
        terrars
        ::BlockAssignable<
            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl {
    #[doc =
        "URI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl {
    pub fn build(
        self,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl {
            sha256_checksum: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `sha256_checksum` after provisioning.\nSHA256 checksum of the remote file."]
    pub fn sha256_checksum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256_checksum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nURI from which to fetch the object. It should contain both the protocol and path following the format '{protocol}://{location}'."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElDynamic {
    gcs: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl>,
    >,
    remote: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_insecure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote: std
    ::option
    ::Option<
        std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl>,
    >,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {
    #[doc =
        "Set the field `allow_insecure`.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn set_allow_insecure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_insecure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_path`.\nA local path within the VM to use."]
    pub fn set_local_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs`.\n"]
    pub fn set_gcs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote`.\n"]
    pub fn set_remote(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.remote = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.remote = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {
    type O =
        terrars
        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl {
            allow_insecure: core::default::Default::default(),
            local_path: core::default::Default::default(),
            gcs: core::default::Default::default(),
            remote: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_insecure` after provisioning.\nDefaults to false. When false, files are subject to validations based on the file type:\nRemote: A checksum must be specified. Cloud Storage: An object generation number must be specified."]
    pub fn allow_insecure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_insecure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_path` after provisioning.\nA local path within the VM to use."]
    pub fn local_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_path", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs` after provisioning.\n"]
    pub fn gcs(
        &self,
    ) -> terrars
    ::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElGcsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs", self.base))
    }

    #[doc = "Get a reference to the value of field `remote` after provisioning.\n"]
    pub fn remote(
        &self,
    ) -> terrars
    ::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRemoteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElDynamic {
    source: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pull_deps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
    #[doc =
        "Set the field `pull_deps`.\nWhether dependencies should also be installed. - install when false: 'rpm --upgrade --replacepkgs package.rpm' - install when true: 'yum -y install package.rpm' or 'zypper -y install package.rpm'"]
    pub fn set_pull_deps(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pull_deps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl {
            pull_deps: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pull_deps` after provisioning.\nWhether dependencies should also be installed. - install when false: 'rpm --upgrade --replacepkgs package.rpm' - install when true: 'yum -y install package.rpm' or 'zypper -y install package.rpm'"]
    pub fn pull_deps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pull_deps", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {
    name: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { }

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {
    #[doc = "Package name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl { name: self.name }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nPackage name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {
    name: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { }

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {
    type O =
        terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {
    #[doc = "Package name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl { name: self.name }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nPackage name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDynamic {
    apt: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl>>,
    deb: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl>>,
    googet: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl>>,
    msi: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl>>,
    rpm: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl>>,
    yum: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl>>,
    zypper: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl>>,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
    desired_state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apt: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deb: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    googet: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    msi: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rpm: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    yum: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zypper: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
    #[doc = "Set the field `apt`.\n"]
    pub fn set_apt(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptEl>,
                    >,
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

    #[doc = "Set the field `deb`.\n"]
    pub fn set_deb(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `googet`.\n"]
    pub fn set_googet(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.googet = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.googet = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `msi`.\n"]
    pub fn set_msi(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.msi = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.msi = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rpm`.\n"]
    pub fn set_rpm(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rpm = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rpm = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `yum`.\n"]
    pub fn set_yum(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumEl>,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperEl,
                        >,
                    >,
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

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
    #[doc =
        "The desired state the agent should maintain for this package. Possible values: [\"DESIRED_STATE_UNSPECIFIED\", \"INSTALLED\", \"REMOVED\"]"]
    pub desired_state: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl {
            desired_state: self.desired_state,
            apt: core::default::Default::default(),
            deb: core::default::Default::default(),
            googet: core::default::Default::default(),
            msi: core::default::Default::default(),
            rpm: core::default::Default::default(),
            yum: core::default::Default::default(),
            zypper: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nThe desired state the agent should maintain for this package. Possible values: [\"DESIRED_STATE_UNSPECIFIED\", \"INSTALLED\", \"REMOVED\"]"]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.base))
    }

    #[doc = "Get a reference to the value of field `apt` after provisioning.\n"]
    pub fn apt(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElAptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apt", self.base))
    }

    #[doc = "Get a reference to the value of field `deb` after provisioning.\n"]
    pub fn deb(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElDebElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deb", self.base))
    }

    #[doc = "Get a reference to the value of field `googet` after provisioning.\n"]
    pub fn googet(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElGoogetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.googet", self.base))
    }

    #[doc = "Get a reference to the value of field `msi` after provisioning.\n"]
    pub fn msi(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElMsiElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.msi", self.base))
    }

    #[doc = "Get a reference to the value of field `rpm` after provisioning.\n"]
    pub fn rpm(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRpmElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rpm", self.base))
    }

    #[doc = "Get a reference to the value of field `yum` after provisioning.\n"]
    pub fn yum(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElYumElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yum", self.base))
    }

    #[doc = "Get a reference to the value of field `zypper` after provisioning.\n"]
    pub fn zypper(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElZypperElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zypper", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
    archive_type: terrars::PrimField<std::string::String>,
    components: terrars::ListField<terrars::PrimField<std::string::String>>,
    distribution: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpg_key: std::option::Option<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
    #[doc =
        "Set the field `gpg_key`.\nURI of the key file for this repository. The agent maintains a keyring at '/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg'."]
    pub fn set_gpg_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gpg_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
    type O =
        terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
    #[doc =
        "Type of archive files in this repository. Possible values: [\"ARCHIVE_TYPE_UNSPECIFIED\", \"DEB\", \"DEB_SRC\"]"]
    pub archive_type: terrars::PrimField<std::string::String>,
    #[doc = "List of components for this repository. Must contain at least one item."]
    pub components: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Distribution of this repository."]
    pub distribution: terrars::PrimField<std::string::String>,
    #[doc = "URI for this repository."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl {
            archive_type: self.archive_type,
            components: self.components,
            distribution: self.distribution,
            gpg_key: core::default::Default::default(),
            uri: self.uri,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_type` after provisioning.\nType of archive files in this repository. Possible values: [\"ARCHIVE_TYPE_UNSPECIFIED\", \"DEB\", \"DEB_SRC\"]"]
    pub fn archive_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.archive_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `components` after provisioning.\nList of components for this repository. Must contain at least one item."]
    pub fn components(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.components", self.base))
    }

    #[doc =
        "Get a reference to the value of field `distribution` after provisioning.\nDistribution of this repository."]
    pub fn distribution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distribution", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gpg_key` after provisioning.\nURI of the key file for this repository. The agent maintains a keyring at '/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg'."]
    pub fn gpg_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpg_key", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI for this repository."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
    name: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl { }

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
    type O =
        terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
    #[doc = "The name of the repository."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The url of the repository."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl {
            name: self.name,
            url: self.url,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the repository."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe url of the repository."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
    base_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpg_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    id: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
    #[doc = "Set the field `display_name`.\nThe display name of the repository."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpg_keys`.\nURIs of GPG keys."]
    pub fn set_gpg_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.gpg_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
    type O =
        terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
    #[doc = "The location of the repository directory."]
    pub base_url: terrars::PrimField<std::string::String>,
    #[doc =
        "A one word, unique name for this repository. This is the 'repo id' in the yum config file and also the 'display_name' if 'display_name' is omitted. This id is also used as the unique identifier when checking for resource conflicts."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl {
            base_url: self.base_url,
            display_name: core::default::Default::default(),
            gpg_keys: core::default::Default::default(),
            id: self.id,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `base_url` after provisioning.\nThe location of the repository directory."]
    pub fn base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the repository."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `gpg_keys` after provisioning.\nURIs of GPG keys."]
    pub fn gpg_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpg_keys", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nA one word, unique name for this repository. This is the 'repo id' in the yum config file and also the 'display_name' if 'display_name' is omitted. This id is also used as the unique identifier when checking for resource conflicts."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
    base_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpg_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    id: terrars::PrimField<std::string::String>,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
    #[doc = "Set the field `display_name`.\nThe display name of the repository."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpg_keys`.\nURIs of GPG keys."]
    pub fn set_gpg_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.gpg_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
    type O =
        terrars
        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
    #[doc = "The location of the repository directory."]
    pub base_url: terrars::PrimField<std::string::String>,
    #[doc =
        "A one word, unique name for this repository. This is the 'repo id' in the zypper config file and also the 'display_name' if 'display_name' is omitted. This id is also used as the unique identifier when checking for GuestPolicy conflicts."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl {
            base_url: self.base_url,
            display_name: core::default::Default::default(),
            gpg_keys: core::default::Default::default(),
            id: self.id,
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `base_url` after provisioning.\nThe location of the repository directory."]
    pub fn base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the repository."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `gpg_keys` after provisioning.\nURIs of GPG keys."]
    pub fn gpg_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpg_keys", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nA one word, unique name for this repository. This is the 'repo id' in the zypper config file and also the 'display_name' if 'display_name' is omitted. This id is also used as the unique identifier when checking for GuestPolicy conflicts."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElDynamic {
    apt: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl>,
    >,
    goo: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl>,
    >,
    yum: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl>,
    >,
    zypper: std
    ::option
    ::Option<
        terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl>,
    >,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    apt: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    goo: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    yum: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zypper: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
    #[doc = "Set the field `apt`.\n"]
    pub fn set_apt(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptEl,
                        >,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooEl,
                        >,
                    >,
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

    #[doc = "Set the field `yum`.\n"]
    pub fn set_yum(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumEl,
                        >,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperEl,
                        >,
                    >,
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

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl {
            apt: core::default::Default::default(),
            goo: core::default::Default::default(),
            yum: core::default::Default::default(),
            zypper: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apt` after provisioning.\n"]
    pub fn apt(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElAptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.apt", self.base))
    }

    #[doc = "Get a reference to the value of field `goo` after provisioning.\n"]
    pub fn goo(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElGooElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.goo", self.base))
    }

    #[doc = "Get a reference to the value of field `yum` after provisioning.\n"]
    pub fn yum(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElYumElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yum", self.base))
    }

    #[doc = "Get a reference to the value of field `zypper` after provisioning.\n"]
    pub fn zypper(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElZypperElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zypper", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElDynamic {
    exec: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl>>,
    file: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl>>,
    pkg: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl>>,
    repository: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl>>,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exec: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pkg: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
    #[doc = "Set the field `exec`.\n"]
    pub fn set_exec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file`.\n"]
    pub fn set_file(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pkg`.\n"]
    pub fn set_pkg(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pkg = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pkg = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `repository`.\n"]
    pub fn set_repository(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repository = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repository = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
    #[doc =
        "The id of the resource with the following restrictions:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the OS policy."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl {
            id: self.id,
            exec: core::default::Default::default(),
            file: core::default::Default::default(),
            pkg: core::default::Default::default(),
            repository: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe id of the resource with the following restrictions:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the OS policy."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `exec` after provisioning.\n"]
    pub fn exec(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElExecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exec", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElFileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `pkg` after provisioning.\n"]
    pub fn pkg(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElPkgElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pkg", self.base))
    }

    #[doc = "Get a reference to the value of field `repository` after provisioning.\n"]
    pub fn repository(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRepositoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElDynamic {
    inventory_filters: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl>>,
    resources: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl>>,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inventory_filters: std
    ::option
    ::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
    #[doc = "Set the field `inventory_filters`.\n"]
    pub fn set_inventory_filters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inventory_filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inventory_filters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl {
            inventory_filters: core::default::Default::default(),
            resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inventory_filters` after provisioning.\n"]
    pub fn inventory_filters(
        &self,
    ) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElInventoryFiltersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inventory_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentOsPoliciesElDynamic {
    resource_groups: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl>>,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentOsPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_no_resource_group_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_groups: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl>>,
    dynamic: OsConfigOsPolicyAssignmentOsPoliciesElDynamic,
}

impl OsConfigOsPolicyAssignmentOsPoliciesEl {
    #[doc =
        "Set the field `allow_no_resource_group_match`.\nThis flag determines the OS policy compliance status when none of the resource groups within the policy are applicable for a VM. Set this value to 'true' if the policy needs to be reported as compliant even if the policy has nothing to validate or enforce."]
    pub fn set_allow_no_resource_group_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_no_resource_group_match = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nPolicy description. Length of the description is limited to 1024 characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_groups`.\n"]
    pub fn set_resource_groups(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_groups = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentOsPoliciesEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentOsPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentOsPoliciesEl {
    #[doc =
        "The id of the OS policy with the following restrictions:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the assignment."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "Policy mode Possible values: [\"MODE_UNSPECIFIED\", \"VALIDATION\", \"ENFORCEMENT\"]"]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentOsPoliciesEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentOsPoliciesEl {
        OsConfigOsPolicyAssignmentOsPoliciesEl {
            allow_no_resource_group_match: core::default::Default::default(),
            description: core::default::Default::default(),
            id: self.id,
            mode: self.mode,
            resource_groups: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentOsPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentOsPoliciesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigOsPolicyAssignmentOsPoliciesElRef {
        OsConfigOsPolicyAssignmentOsPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentOsPoliciesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_no_resource_group_match` after provisioning.\nThis flag determines the OS policy compliance status when none of the resource groups within the policy are applicable for a VM. Set this value to 'true' if the policy needs to be reported as compliant even if the policy has nothing to validate or enforce."]
    pub fn allow_no_resource_group_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_no_resource_group_match", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nPolicy description. Length of the description is limited to 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe id of the OS policy with the following restrictions:\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter.\n* Must be unique within the assignment."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nPolicy mode Possible values: [\"MODE_UNSPECIFIED\", \"VALIDATION\", \"ENFORCEMENT\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_groups` after provisioning.\n"]
    pub fn resource_groups(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentOsPoliciesElResourceGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_groups", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
    #[doc = "Set the field `fixed`.\nSpecifies a fixed value."]
    pub fn set_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `percent`.\nSpecifies the relative value defined as a percentage, which will be multiplied by a reference value."]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {}

impl BuildOsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
        OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef {
        OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed` after provisioning.\nSpecifies a fixed value."]
    pub fn fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percent` after provisioning.\nSpecifies the relative value defined as a percentage, which will be multiplied by a reference value."]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct OsConfigOsPolicyAssignmentRolloutElDynamic {
    disruption_budget: std
    ::option
    ::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl>>,
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentRolloutEl {
    min_wait_duration: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disruption_budget: std::option::Option<std::vec::Vec<OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl>>,
    dynamic: OsConfigOsPolicyAssignmentRolloutElDynamic,
}

impl OsConfigOsPolicyAssignmentRolloutEl {
    #[doc = "Set the field `disruption_budget`.\n"]
    pub fn set_disruption_budget(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetEl>>,
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

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentRolloutEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentRolloutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentRolloutEl {
    #[doc =
        "This determines the minimum duration of time to wait after the configuration changes are applied through the current rollout. A VM continues to count towards the 'disruption_budget' at least until this duration of time has passed after configuration changes are applied."]
    pub min_wait_duration: terrars::PrimField<std::string::String>,
}

impl BuildOsConfigOsPolicyAssignmentRolloutEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentRolloutEl {
        OsConfigOsPolicyAssignmentRolloutEl {
            min_wait_duration: self.min_wait_duration,
            disruption_budget: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentRolloutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentRolloutElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigOsPolicyAssignmentRolloutElRef {
        OsConfigOsPolicyAssignmentRolloutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentRolloutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `min_wait_duration` after provisioning.\nThis determines the minimum duration of time to wait after the configuration changes are applied through the current rollout. A VM continues to count towards the 'disruption_budget' at least until this duration of time has passed after configuration changes are applied."]
    pub fn min_wait_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_wait_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `disruption_budget` after provisioning.\n"]
    pub fn disruption_budget(&self) -> terrars::ListRef<OsConfigOsPolicyAssignmentRolloutElDisruptionBudgetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disruption_budget", self.base))
    }
}

#[derive(Serialize)]
pub struct OsConfigOsPolicyAssignmentTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OsConfigOsPolicyAssignmentTimeoutsEl {
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

impl terrars::ToListMappable for OsConfigOsPolicyAssignmentTimeoutsEl {
    type O = terrars::BlockAssignable<OsConfigOsPolicyAssignmentTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOsConfigOsPolicyAssignmentTimeoutsEl {}

impl BuildOsConfigOsPolicyAssignmentTimeoutsEl {
    pub fn build(self) -> OsConfigOsPolicyAssignmentTimeoutsEl {
        OsConfigOsPolicyAssignmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct OsConfigOsPolicyAssignmentTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OsConfigOsPolicyAssignmentTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OsConfigOsPolicyAssignmentTimeoutsElRef {
        OsConfigOsPolicyAssignmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OsConfigOsPolicyAssignmentTimeoutsElRef {
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
struct OsConfigOsPolicyAssignmentDynamic {
    instance_filter: std::option::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentInstanceFilterEl>>,
    os_policies: std::option::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentOsPoliciesEl>>,
    rollout: std::option::Option<terrars::DynamicBlock<OsConfigOsPolicyAssignmentRolloutEl>>,
}
