#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SecurityposturePostureData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    posture_id: terrars::PrimField<std::string::String>,
    state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_sets: std::option::Option<std::vec::Vec<SecurityposturePosture_PolicySets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SecurityposturePosture_Timeouts>,
    dynamic: SecurityposturePostureDynamic,
}

struct SecurityposturePosture_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecurityposturePostureData_>,
}

#[derive(Clone)]
pub struct SecurityposturePosture(std::rc::Rc<SecurityposturePosture_>);

impl SecurityposturePosture {
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

    #[doc = "Set the field `description`.\nDescription of the posture."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_sets`.\n"]
    pub fn set_policy_sets(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityposturePosture_PolicySets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy_sets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy_sets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SecurityposturePosture_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Posture was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the posture."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the resource, eg: global."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the posture."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource, an organization. Format should be 'organizations/{organization_id}'."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `posture_id` after provisioning.\nId of the posture. It is an immutable field."]
    pub fn posture_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.posture_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the posture."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nRevision_id of the posture."]
    pub fn revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nState of the posture. Update to state field should not be triggered along with\nwith other field updates. Possible values: [\"DEPRECATED\", \"DRAFT\", \"ACTIVE\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Posture was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_sets` after provisioning.\n"]
    pub fn policy_sets(&self) -> terrars::ListRef<SecurityposturePosture_PolicySetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_sets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecurityposturePosture_TimeoutsRef {
        SecurityposturePosture_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SecurityposturePosture {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecurityposturePosture { }

impl terrars::ToListMappable for SecurityposturePosture {
    type O = terrars::ListRef<SecurityposturePostureRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecurityposturePosture_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_securityposture_posture".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityposturePosture {
    pub tf_id: std::string::String,
    #[doc = "Location of the resource, eg: global."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The parent of the resource, an organization. Format should be 'organizations/{organization_id}'."]
    pub parent: terrars::PrimField<std::string::String>,
    #[doc = "Id of the posture. It is an immutable field."]
    pub posture_id: terrars::PrimField<std::string::String>,
    #[doc =
        "State of the posture. Update to state field should not be triggered along with\nwith other field updates. Possible values: [\"DEPRECATED\", \"DRAFT\", \"ACTIVE\"]"]
    pub state: terrars::PrimField<std::string::String>,
}

impl BuildSecurityposturePosture {
    pub fn build(self, stack: &mut terrars::Stack) -> SecurityposturePosture {
        let out = SecurityposturePosture(std::rc::Rc::new(SecurityposturePosture_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecurityposturePostureData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
                posture_id: self.posture_id,
                state: self.state,
                policy_sets: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityposturePostureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePostureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecurityposturePostureRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Posture was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the posture."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nFor Resource freshness validation (https://google.aip.dev/154)"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation of the resource, eg: global."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the posture."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource, an organization. Format should be 'organizations/{organization_id}'."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `posture_id` after provisioning.\nId of the posture. It is an immutable field."]
    pub fn posture_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.posture_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIf set, there are currently changes in flight to the posture."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision_id` after provisioning.\nRevision_id of the posture."]
    pub fn revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nState of the posture. Update to state field should not be triggered along with\nwith other field updates. Possible values: [\"DEPRECATED\", \"DRAFT\", \"ACTIVE\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Posture was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_sets` after provisioning.\n"]
    pub fn policy_sets(&self) -> terrars::ListRef<SecurityposturePosture_PolicySetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_sets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecurityposturePosture_TimeoutsRef {
        SecurityposturePosture_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePostureDynamic {
    policy_sets: std::option::Option<terrars::DynamicBlock<SecurityposturePosture_PolicySets>>,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_ComplianceStandards {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityposturePosture_PolicySets_Policies_ComplianceStandards {
    #[doc = "Set the field `control`.\nMapping of security controls for the policy."]
    pub fn set_control(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standard`.\nMapping of compliance standards for the policy."]
    pub fn set_standard(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.standard = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityposturePosture_PolicySets_Policies_ComplianceStandards {
    type O = terrars::BlockAssignable<SecurityposturePosture_PolicySets_Policies_ComplianceStandards>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_ComplianceStandards {}

impl BuildSecurityposturePosture_PolicySets_Policies_ComplianceStandards {
    pub fn build(self) -> SecurityposturePosture_PolicySets_Policies_ComplianceStandards {
        SecurityposturePosture_PolicySets_Policies_ComplianceStandards {
            control: core::default::Default::default(),
            standard: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_ComplianceStandardsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_PolicySets_Policies_ComplianceStandardsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_ComplianceStandardsRef {
        SecurityposturePosture_PolicySets_Policies_ComplianceStandardsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_ComplianceStandardsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `control` after provisioning.\nMapping of security controls for the policy."]
    pub fn control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `standard` after provisioning.\nMapping of compliance standards for the policy."]
    pub fn standard(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standard", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition {
    #[doc = "Set the field `description`.\nDescription of the expression"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition {
    type O =
        terrars
        ::BlockAssignable<
            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition {
    pub fn build(
        self,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ConditionRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    denied_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values {
    #[doc = "Set the field `allowed_values`.\nList of values allowed at this resource."]
    pub fn set_allowed_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `denied_values`.\nList of values denied at this resource."]
    pub fn set_denied_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.denied_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values {
    type O =
        terrars
        ::BlockAssignable<
            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values {}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values {
    pub fn build(
        self,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values {
            allowed_values: core::default::Default::default(),
            denied_values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ValuesRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_values` after provisioning.\nList of values allowed at this resource."]
    pub fn allowed_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `denied_values` after provisioning.\nList of values denied at this resource."]
    pub fn denied_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.denied_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRulesDynamic {
    condition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition,
        >,
    >,
    values: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values>,
    >,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std
    ::option
    ::Option<
        std::vec::Vec<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std
    ::option
    ::Option<
        std::vec::Vec<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values>,
    >,
    dynamic: SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRulesDynamic,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules {
    #[doc =
        "Set the field `allow_all`.\nSetting this to true means that all values are allowed. This field can be set only in policies for list constraints."]
    pub fn set_allow_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_all = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deny_all`.\nSetting this to true means that all values are denied. This field can be set only in policies for list constraints."]
    pub fn set_deny_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deny_all = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforce`.\nIf 'true', then the policy is enforced. If 'false', then any configuration is acceptable.\nThis field can be set only in policies for boolean constraints."]
    pub fn set_enforce(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforce = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Condition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_Values,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules {
    type O =
        terrars
        ::BlockAssignable<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules {}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules {
    pub fn build(self) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules {
            allow_all: core::default::Default::default(),
            deny_all: core::default::Default::default(),
            enforce: core::default::Default::default(),
            condition: core::default::Default::default(),
            values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRulesRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_all` after provisioning.\nSetting this to true means that all values are allowed. This field can be set only in policies for list constraints."]
    pub fn allow_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deny_all` after provisioning.\nSetting this to true means that all values are denied. This field can be set only in policies for list constraints."]
    pub fn deny_all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deny_all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce` after provisioning.\nIf 'true', then the policy is enforced. If 'false', then any configuration is acceptable.\nThis field can be set only in policies for boolean constraints."]
    pub fn enforce(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(
        &self,
    ) -> terrars
    ::ListRef<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(
        &self,
    ) -> terrars
    ::ListRef<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules_ValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintDynamic {
    policy_rules: std
    ::option
    ::Option<
        terrars::DynamicBlock<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules>,
    >,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint {
    canned_constraint_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_rules: std
    ::option
    ::Option<std::vec::Vec<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules>>,
    dynamic: SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintDynamic,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint {
    #[doc = "Set the field `policy_rules`.\n"]
    pub fn set_policy_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint {
    type O = terrars::BlockAssignable<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint {
    #[doc = "Organization policy canned constraint Id"]
    pub canned_constraint_id: terrars::PrimField<std::string::String>,
}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint {
    pub fn build(self) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint {
            canned_constraint_id: self.canned_constraint_id,
            policy_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `canned_constraint_id` after provisioning.\nOrganization policy canned constraint Id"]
    pub fn canned_constraint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canned_constraint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_rules` after provisioning.\n"]
    pub fn policy_rules(
        &self,
    ) -> terrars::ListRef<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint {
    action_type: terrars::PrimField<std::string::String>,
    condition: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    method_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    resource_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint {
    #[doc =
        "Set the field `description`.\nA human-friendly description of the constraint to display as an error message when the policy is violated."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nA human-friendly name for the constraint."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint {
    type O =
        terrars
        ::BlockAssignable<
            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint {
    #[doc = "The action to take if the condition is met. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub action_type: terrars::PrimField<std::string::String>,
    #[doc =
        "A CEL condition that refers to a supported service resource, for example 'resource.management.autoUpgrade == false'. For details about CEL usage, see [Common Expression Language](https://docs.cloud.google.com/resource-manager/docs/organization-policy/creating-managing-custom-constraints#common_expression_language)."]
    pub condition: terrars::PrimField<std::string::String>,
    #[doc =
        "A list of RESTful methods for which to enforce the constraint. Can be 'CREATE', 'UPDATE', or both. Not all Google Cloud services support both methods. To see supported methods for each service, find the service in [Supported services](https://docs.cloud.google.com/resource-manager/docs/organization-policy/custom-constraint-supported-services)."]
    pub method_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Immutable. The name of the custom constraint. This is unique within the organization."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Immutable. The fully qualified name of the Google Cloud REST resource containing the object and field you want to restrict. For example, 'container.googleapis.com/NodePool'."]
    pub resource_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint {
    pub fn build(
        self,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint {
            action_type: self.action_type,
            condition: self.condition,
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            method_types: self.method_types,
            name: self.name,
            resource_types: self.resource_types,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraintRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraintRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraintRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action_type` after provisioning.\nThe action to take if the condition is met. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub fn action_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `condition` after provisioning.\nA CEL condition that refers to a supported service resource, for example 'resource.management.autoUpgrade == false'. For details about CEL usage, see [Common Expression Language](https://docs.cloud.google.com/resource-manager/docs/organization-policy/creating-managing-custom-constraints#common_expression_language)."]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-friendly description of the constraint to display as an error message when the policy is violated."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA human-friendly name for the constraint."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method_types` after provisioning.\nA list of RESTful methods for which to enforce the constraint. Can be 'CREATE', 'UPDATE', or both. Not all Google Cloud services support both methods. To see supported methods for each service, find the service in [Supported services](https://docs.cloud.google.com/resource-manager/docs/organization-policy/custom-constraint-supported-services)."]
    pub fn method_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. The name of the custom constraint. This is unique within the organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_types` after provisioning.\nImmutable. The fully qualified name of the Google Cloud REST resource containing the object and field you want to restrict. For example, 'container.googleapis.com/NodePool'."]
    pub fn resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustomDynamic {
    custom_constraint: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint,
        >,
    >,
    policy_rules: std
    ::option
    ::Option<
        terrars::DynamicBlock<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules>,
    >,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_constraint: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_rules: std
    ::option
    ::Option<std::vec::Vec<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules>>,
    dynamic: SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustomDynamic,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom {
    #[doc = "Set the field `custom_constraint`.\n"]
    pub fn set_custom_constraint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_constraint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_constraint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `policy_rules`.\n"]
    pub fn set_policy_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRules,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom {
    type O =
        terrars::BlockAssignable<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom {}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom {
    pub fn build(self) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom {
            custom_constraint: core::default::Default::default(),
            policy_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustomRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustomRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustomRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustomRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustomRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_constraint` after provisioning.\n"]
    pub fn custom_constraint(
        &self,
    ) -> terrars
    ::ListRef<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom_CustomConstraintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_constraint", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_rules` after provisioning.\n"]
    pub fn policy_rules(
        &self,
    ) -> terrars::ListRef<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint_PolicyRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression {
    #[doc = "Set the field `description`.\nDescription of the expression"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression {
    type O =
        terrars
        ::BlockAssignable<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression {
    pub fn build(
        self,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpressionRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_PropertiesDynamic {
    value_expression: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression,
        >,
    >,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_expression: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression,
        >,
    >,
    dynamic: SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_PropertiesDynamic,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties {
    #[doc = "Set the field `value_expression`.\n"]
    pub fn set_value_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpression,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value_expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties {
    type O =
        terrars
        ::BlockAssignable<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties {
    #[doc = "Name of the property for the custom output."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties {
    pub fn build(
        self,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties {
            name: self.name,
            value_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_PropertiesRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the property for the custom output."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value_expression` after provisioning.\n"]
    pub fn value_expression(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties_ValueExpressionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutputDynamic {
    properties: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties,
        >,
    >,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties,
        >,
    >,
    dynamic: SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutputDynamic,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_Properties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput {
    type O =
        terrars
        ::BlockAssignable<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput {}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput {
    pub fn build(
        self,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput {
            properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutputRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutputRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutputRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutputRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput_PropertiesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate {
    #[doc = "Set the field `description`.\nDescription of the expression"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate {
    type O =
        terrars
        ::BlockAssignable<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate {
    pub fn build(
        self,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_PredicateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_PredicateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_PredicateRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_PredicateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_PredicateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector {
    resource_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector {

}

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector {
    type O =
        terrars
        ::BlockAssignable<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector {
    #[doc = "The resource types to run the detector on."]
    pub resource_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector {
    pub fn build(
        self,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector {
            resource_types: self.resource_types,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelectorRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_types` after provisioning.\nThe resource types to run the detector on."]
    pub fn resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_ConfigDynamic {
    custom_output: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput,
        >,
    >,
    predicate: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate,
        >,
    >,
    resource_selector: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector,
        >,
    >,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recommendation: std::option::Option<terrars::PrimField<std::string::String>>,
    severity: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_output: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predicate: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_selector: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector,
        >,
    >,
    dynamic: SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_ConfigDynamic,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config {
    #[doc =
        "Set the field `description`.\nText that describes the vulnerability or misconfiguration that the custom\nmodule detects."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `recommendation`.\nAn explanation of the recommended steps that security teams can take to\nresolve the detected issue"]
    pub fn set_recommendation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.recommendation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_output`.\n"]
    pub fn set_custom_output(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutput,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_output = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_output = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `predicate`.\n"]
    pub fn set_predicate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_Predicate,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.predicate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.predicate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_selector`.\n"]
    pub fn set_resource_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelector,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_selector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config {
    type O =
        terrars
        ::BlockAssignable<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config {
    #[doc =
        "The severity to assign to findings generated by the module. Possible values: [\"SEVERITY_UNSPECIFIED\", \"CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub severity: terrars::PrimField<std::string::String>,
}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config {
    pub fn build(
        self,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config {
            description: core::default::Default::default(),
            recommendation: core::default::Default::default(),
            severity: self.severity,
            custom_output: core::default::Default::default(),
            predicate: core::default::Default::default(),
            resource_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_ConfigRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nText that describes the vulnerability or misconfiguration that the custom\nmodule detects."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `recommendation` after provisioning.\nAn explanation of the recommended steps that security teams can take to\nresolve the detected issue"]
    pub fn recommendation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recommendation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `severity` after provisioning.\nThe severity to assign to findings generated by the module. Possible values: [\"SEVERITY_UNSPECIFIED\", \"CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_output` after provisioning.\n"]
    pub fn custom_output(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_CustomOutputRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_output", self.base))
    }

    #[doc = "Get a reference to the value of field `predicate` after provisioning.\n"]
    pub fn predicate(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_PredicateRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predicate", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_selector` after provisioning.\n"]
    pub fn resource_selector(
        &self,
    ) -> terrars
    ::ListRef<
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config_ResourceSelectorRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_selector", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModuleDynamic {
    config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config,
        >,
    >,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    module_enablement_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config>,
    >,
    dynamic: SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModuleDynamic,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule {
    #[doc =
        "Set the field `display_name`.\nThe display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `module_enablement_state`.\nThe state of enablement for the module at its level of the resource hierarchy. Possible values: [\"ENABLEMENT_STATE_UNSPECIFIED\", \"ENABLED\", \"DISABLED\"]"]
    pub fn set_module_enablement_state(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.module_enablement_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_Config,
                        >,
                    >,
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

impl terrars
::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule {
    type O =
        terrars
        ::BlockAssignable<SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule {}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule {
    pub fn build(self) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule {
            display_name: core::default::Default::default(),
            module_enablement_state: core::default::Default::default(),
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModuleRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Security Health Analytics custom module. This\ndisplay name becomes the finding category for all findings that are\nreturned by this custom module."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nA server generated id of custom module."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `module_enablement_state` after provisioning.\nThe state of enablement for the module at its level of the resource hierarchy. Possible values: [\"ENABLEMENT_STATE_UNSPECIFIED\", \"ENABLED\", \"DISABLED\"]"]
    pub fn module_enablement_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.module_enablement_state", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(
        &self,
    ) -> terrars
    ::ListRef<SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    module_enablement_state: std::option::Option<terrars::PrimField<std::string::String>>,
    module_name: terrars::PrimField<std::string::String>,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule {
    #[doc =
        "Set the field `module_enablement_state`.\nThe state of enablement for the module at its level of the resource hierarchy. Possible values: [\"ENABLEMENT_STATE_UNSPECIFIED\", \"ENABLED\", \"DISABLED\"]"]
    pub fn set_module_enablement_state(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.module_enablement_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule {
    type O =
        terrars
        ::BlockAssignable<SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule {
    #[doc = "The name of the module eg: BIGQUERY_TABLE_CMEK_DISABLED."]
    pub module_name: terrars::PrimField<std::string::String>,
}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule {
    pub fn build(self) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule {
            module_enablement_state: core::default::Default::default(),
            module_name: self.module_name,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModuleRef {
        SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `module_enablement_state` after provisioning.\nThe state of enablement for the module at its level of the resource hierarchy. Possible values: [\"ENABLEMENT_STATE_UNSPECIFIED\", \"ENABLED\", \"DISABLED\"]"]
    pub fn module_enablement_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.module_enablement_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `module_name` after provisioning.\nThe name of the module eg: BIGQUERY_TABLE_CMEK_DISABLED."]
    pub fn module_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.module_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePosture_PolicySets_Policies_ConstraintDynamic {
    org_policy_constraint: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint>>,
    org_policy_constraint_custom: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom>>,
    security_health_analytics_custom_module: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule>,
    >,
    security_health_analytics_module: std
    ::option
    ::Option<
        terrars::DynamicBlock<SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule>,
    >,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies_Constraint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    org_policy_constraint: std
    ::option
    ::Option<std::vec::Vec<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    org_policy_constraint_custom: std
    ::option
    ::Option<std::vec::Vec<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_health_analytics_custom_module: std
    ::option
    ::Option<
        std::vec::Vec<SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_health_analytics_module: std
    ::option
    ::Option<std::vec::Vec<SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule>>,
    dynamic: SecurityposturePosture_PolicySets_Policies_ConstraintDynamic,
}

impl SecurityposturePosture_PolicySets_Policies_Constraint {
    #[doc = "Set the field `org_policy_constraint`.\n"]
    pub fn set_org_policy_constraint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.org_policy_constraint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.org_policy_constraint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `org_policy_constraint_custom`.\n"]
    pub fn set_org_policy_constraint_custom(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustom,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.org_policy_constraint_custom = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.org_policy_constraint_custom = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_health_analytics_custom_module`.\n"]
    pub fn set_security_health_analytics_custom_module(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_health_analytics_custom_module = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_health_analytics_custom_module = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_health_analytics_module`.\n"]
    pub fn set_security_health_analytics_module(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_health_analytics_module = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_health_analytics_module = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityposturePosture_PolicySets_Policies_Constraint {
    type O = terrars::BlockAssignable<SecurityposturePosture_PolicySets_Policies_Constraint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies_Constraint {}

impl BuildSecurityposturePosture_PolicySets_Policies_Constraint {
    pub fn build(self) -> SecurityposturePosture_PolicySets_Policies_Constraint {
        SecurityposturePosture_PolicySets_Policies_Constraint {
            org_policy_constraint: core::default::Default::default(),
            org_policy_constraint_custom: core::default::Default::default(),
            security_health_analytics_custom_module: core::default::Default::default(),
            security_health_analytics_module: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_Policies_ConstraintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_PolicySets_Policies_ConstraintRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityposturePosture_PolicySets_Policies_ConstraintRef {
        SecurityposturePosture_PolicySets_Policies_ConstraintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_Policies_ConstraintRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `org_policy_constraint` after provisioning.\n"]
    pub fn org_policy_constraint(
        &self,
    ) -> terrars::ListRef<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.org_policy_constraint", self.base))
    }

    #[doc = "Get a reference to the value of field `org_policy_constraint_custom` after provisioning.\n"]
    pub fn org_policy_constraint_custom(
        &self,
    ) -> terrars::ListRef<SecurityposturePosture_PolicySets_Policies_Constraint_OrgPolicyConstraintCustomRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.org_policy_constraint_custom", self.base))
    }

    #[doc = "Get a reference to the value of field `security_health_analytics_custom_module` after provisioning.\n"]
    pub fn security_health_analytics_custom_module(
        &self,
    ) -> terrars
    ::ListRef<SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsCustomModuleRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.security_health_analytics_custom_module", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_health_analytics_module` after provisioning.\n"]
    pub fn security_health_analytics_module(
        &self,
    ) -> terrars::ListRef<SecurityposturePosture_PolicySets_Policies_Constraint_SecurityHealthAnalyticsModuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_health_analytics_module", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePosture_PolicySets_PoliciesDynamic {
    compliance_standards: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityposturePosture_PolicySets_Policies_ComplianceStandards>>,
    constraint: std::option::Option<terrars::DynamicBlock<SecurityposturePosture_PolicySets_Policies_Constraint>>,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets_Policies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    policy_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_standards: std
    ::option
    ::Option<std::vec::Vec<SecurityposturePosture_PolicySets_Policies_ComplianceStandards>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraint: std::option::Option<std::vec::Vec<SecurityposturePosture_PolicySets_Policies_Constraint>>,
    dynamic: SecurityposturePosture_PolicySets_PoliciesDynamic,
}

impl SecurityposturePosture_PolicySets_Policies {
    #[doc = "Set the field `description`.\nDescription of the policy."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compliance_standards`.\n"]
    pub fn set_compliance_standards(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityposturePosture_PolicySets_Policies_ComplianceStandards>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_standards = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_standards = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `constraint`.\n"]
    pub fn set_constraint(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityposturePosture_PolicySets_Policies_Constraint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.constraint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.constraint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityposturePosture_PolicySets_Policies {
    type O = terrars::BlockAssignable<SecurityposturePosture_PolicySets_Policies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets_Policies {
    #[doc = "ID of the policy."]
    pub policy_id: terrars::PrimField<std::string::String>,
}

impl BuildSecurityposturePosture_PolicySets_Policies {
    pub fn build(self) -> SecurityposturePosture_PolicySets_Policies {
        SecurityposturePosture_PolicySets_Policies {
            description: core::default::Default::default(),
            policy_id: self.policy_id,
            compliance_standards: core::default::Default::default(),
            constraint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySets_PoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_PolicySets_PoliciesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityposturePosture_PolicySets_PoliciesRef {
        SecurityposturePosture_PolicySets_PoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySets_PoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the policy."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nID of the policy."]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `compliance_standards` after provisioning.\n"]
    pub fn compliance_standards(
        &self,
    ) -> terrars::ListRef<SecurityposturePosture_PolicySets_Policies_ComplianceStandardsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compliance_standards", self.base))
    }

    #[doc = "Get a reference to the value of field `constraint` after provisioning.\n"]
    pub fn constraint(&self) -> terrars::ListRef<SecurityposturePosture_PolicySets_Policies_ConstraintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.constraint", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityposturePosture_PolicySetsDynamic {
    policies: std::option::Option<terrars::DynamicBlock<SecurityposturePosture_PolicySets_Policies>>,
}

#[derive(Serialize)]
pub struct SecurityposturePosture_PolicySets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    policy_set_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policies: std::option::Option<std::vec::Vec<SecurityposturePosture_PolicySets_Policies>>,
    dynamic: SecurityposturePosture_PolicySetsDynamic,
}

impl SecurityposturePosture_PolicySets {
    #[doc = "Set the field `description`.\nDescription of the policy set."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policies`.\n"]
    pub fn set_policies(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityposturePosture_PolicySets_Policies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policies = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityposturePosture_PolicySets {
    type O = terrars::BlockAssignable<SecurityposturePosture_PolicySets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_PolicySets {
    #[doc = "ID of the policy set."]
    pub policy_set_id: terrars::PrimField<std::string::String>,
}

impl BuildSecurityposturePosture_PolicySets {
    pub fn build(self) -> SecurityposturePosture_PolicySets {
        SecurityposturePosture_PolicySets {
            description: core::default::Default::default(),
            policy_set_id: self.policy_set_id,
            policies: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_PolicySetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_PolicySetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityposturePosture_PolicySetsRef {
        SecurityposturePosture_PolicySetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_PolicySetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the policy set."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_set_id` after provisioning.\nID of the policy set."]
    pub fn policy_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_set_id", self.base))
    }

    #[doc = "Get a reference to the value of field `policies` after provisioning.\n"]
    pub fn policies(&self) -> terrars::ListRef<SecurityposturePosture_PolicySets_PoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policies", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityposturePosture_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityposturePosture_Timeouts {
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

impl terrars::ToListMappable for SecurityposturePosture_Timeouts {
    type O = terrars::BlockAssignable<SecurityposturePosture_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityposturePosture_Timeouts {}

impl BuildSecurityposturePosture_Timeouts {
    pub fn build(self) -> SecurityposturePosture_Timeouts {
        SecurityposturePosture_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityposturePosture_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityposturePosture_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityposturePosture_TimeoutsRef {
        SecurityposturePosture_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityposturePosture_TimeoutsRef {
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
