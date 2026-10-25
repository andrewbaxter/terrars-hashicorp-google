#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleRuleData_ {
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
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleRule_Timeouts>,
}

struct ChronicleRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleRuleData_>,
}

#[derive(Clone)]
pub struct ChronicleRule(std::rc::Rc<ChronicleRule_>);

impl ChronicleRule {
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
        "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_rule.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `etag`.\nThe etag for this rule.\nIf this is provided on update, the request will succeed if and only if it\nmatches the server-computed value, and will fail with an ABORTED error\notherwise.\nPopulated in BASIC view and FULL view."]
    pub fn set_etag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().etag = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `rule_id`.\nRule Id is the ID of the Rule."]
    pub fn set_rule_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rule_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scope`.\nResource name of the DataAccessScope bound to this rule.\nPopulated in BASIC view and FULL view.\nIf reference lists are used in the rule, validations will be performed\nagainst this scope to ensure that the reference lists are compatible with\nboth the user's and the rule's scopes.\nThe scope should be in the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope}\"."]
    pub fn set_scope(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\nThe YARA-L content of the rule.\nPopulated in FULL view."]
    pub fn set_text(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleRule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allowed_run_frequencies` after provisioning.\nOutput only. The run frequencies that are allowed for the rule.\nPopulated in BASIC view and FULL view."]
    pub fn allowed_run_frequencies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_run_frequencies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `author` after provisioning.\nOutput only. The author of the rule. Extracted from the meta section of text.\nPopulated in BASIC view and FULL view."]
    pub fn author(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compilation_diagnostics` after provisioning.\nOutput only. A list of a rule's corresponding compilation diagnostic messages\nsuch as compilation errors and compilation warnings.\nPopulated in FULL view."]
    pub fn compilation_diagnostics(&self) -> terrars::ListRef<ChronicleRule_CompilationDiagnosticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compilation_diagnostics", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compilation_state` after provisioning.\nOutput only. The current compilation state of the rule.\nPopulated in FULL view.\nPossible values:\nCOMPILATION_STATE_UNSPECIFIED\nSUCCEEDED\nFAILED"]
    pub fn compilation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compilation_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp of when the rule was created.\nPopulated in FULL view."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_tables` after provisioning.\nOutput only. Resource names of the data tables used in this rule."]
    pub fn data_tables(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_tables", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_rule.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOutput only. Display name of the rule.\nPopulated in BASIC view and FULL view."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag for this rule.\nIf this is provided on update, the request will succeed if and only if it\nmatches the server-computed value, and will fail with an ABORTED error\notherwise.\nPopulated in BASIC view and FULL view."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nOutput only. Additional metadata specified in the meta section of text.\nPopulated in FULL view."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull resource name for the rule. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `near_real_time_live_rule_eligible` after provisioning.\nOutput only. Indicate the rule can run in near real time live rule.\nIf this is true, the rule uses the near real time live rule when the run\nfrequency is set to LIVE."]
    pub fn near_real_time_live_rule_eligible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.near_real_time_live_rule_eligible", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reference_lists` after provisioning.\nOutput only. Resource names of the reference lists used in this rule.\nPopulated in FULL view."]
    pub fn reference_lists(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_lists", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp of when the rule revision was created.\nPopulated in FULL, REVISION_METADATA_ONLY views."]
    pub fn revision_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The revision ID of the rule.\nA new revision is created whenever the rule text is changed in any way.\nFormat: v_{10 digits}_{9 digits}\nPopulated in REVISION_METADATA_ONLY view and FULL view."]
    pub fn revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nRule Id is the ID of the Rule."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nResource name of the DataAccessScope bound to this rule.\nPopulated in BASIC view and FULL view.\nIf reference lists are used in the rule, validations will be performed\nagainst this scope to ensure that the reference lists are compatible with\nboth the user's and the rule's scopes.\nThe scope should be in the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope}\"."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `severity` after provisioning.\nSeverity represents the severity level of the rule."]
    pub fn severity(&self) -> terrars::ListRef<ChronicleRule_SeverityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.severity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nThe YARA-L content of the rule.\nPopulated in FULL view."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nPossible values:\nRULE_TYPE_UNSPECIFIED\nSINGLE_EVENT\nMULTI_EVENT"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleRule_TimeoutsRef {
        ChronicleRule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleRule { }

impl terrars::ToListMappable for ChronicleRule {
    type O = terrars::ListRef<ChronicleRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleRule {
    pub tf_id: std::string::String,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildChronicleRule {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleRule {
        let out = ChronicleRule(std::rc::Rc::new(ChronicleRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                rule_id: core::default::Default::default(),
                scope: core::default::Default::default(),
                text: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_run_frequencies` after provisioning.\nOutput only. The run frequencies that are allowed for the rule.\nPopulated in BASIC view and FULL view."]
    pub fn allowed_run_frequencies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_run_frequencies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `author` after provisioning.\nOutput only. The author of the rule. Extracted from the meta section of text.\nPopulated in BASIC view and FULL view."]
    pub fn author(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compilation_diagnostics` after provisioning.\nOutput only. A list of a rule's corresponding compilation diagnostic messages\nsuch as compilation errors and compilation warnings.\nPopulated in FULL view."]
    pub fn compilation_diagnostics(&self) -> terrars::ListRef<ChronicleRule_CompilationDiagnosticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compilation_diagnostics", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compilation_state` after provisioning.\nOutput only. The current compilation state of the rule.\nPopulated in FULL view.\nPossible values:\nCOMPILATION_STATE_UNSPECIFIED\nSUCCEEDED\nFAILED"]
    pub fn compilation_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compilation_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp of when the rule was created.\nPopulated in FULL view."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_tables` after provisioning.\nOutput only. Resource names of the data tables used in this rule."]
    pub fn data_tables(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_tables", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_rule.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOutput only. Display name of the rule.\nPopulated in BASIC view and FULL view."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag for this rule.\nIf this is provided on update, the request will succeed if and only if it\nmatches the server-computed value, and will fail with an ABORTED error\notherwise.\nPopulated in BASIC view and FULL view."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nOutput only. Additional metadata specified in the meta section of text.\nPopulated in FULL view."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull resource name for the rule. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `near_real_time_live_rule_eligible` after provisioning.\nOutput only. Indicate the rule can run in near real time live rule.\nIf this is true, the rule uses the near real time live rule when the run\nfrequency is set to LIVE."]
    pub fn near_real_time_live_rule_eligible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.near_real_time_live_rule_eligible", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reference_lists` after provisioning.\nOutput only. Resource names of the reference lists used in this rule.\nPopulated in FULL view."]
    pub fn reference_lists(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reference_lists", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision_create_time` after provisioning.\nOutput only. The timestamp of when the rule revision was created.\nPopulated in FULL, REVISION_METADATA_ONLY views."]
    pub fn revision_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `revision_id` after provisioning.\nOutput only. The revision ID of the rule.\nA new revision is created whenever the rule text is changed in any way.\nFormat: v_{10 digits}_{9 digits}\nPopulated in REVISION_METADATA_ONLY view and FULL view."]
    pub fn revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\nRule Id is the ID of the Rule."]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nResource name of the DataAccessScope bound to this rule.\nPopulated in BASIC view and FULL view.\nIf reference lists are used in the rule, validations will be performed\nagainst this scope to ensure that the reference lists are compatible with\nboth the user's and the rule's scopes.\nThe scope should be in the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope}\"."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `severity` after provisioning.\nSeverity represents the severity level of the rule."]
    pub fn severity(&self) -> terrars::ListRef<ChronicleRule_SeverityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.severity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `text` after provisioning.\nThe YARA-L content of the rule.\nPopulated in FULL view."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nPossible values:\nRULE_TYPE_UNSPECIFIED\nSINGLE_EVENT\nMULTI_EVENT"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleRule_TimeoutsRef {
        ChronicleRule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ChronicleRule_CompilationDiagnostics_Position {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_column: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_line: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_column: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_line: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleRule_CompilationDiagnostics_Position {
    #[doc = "Set the field `end_column`.\n"]
    pub fn set_end_column(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.end_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_line`.\n"]
    pub fn set_end_line(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.end_line = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_column`.\n"]
    pub fn set_start_column(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_column = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_line`.\n"]
    pub fn set_start_line(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_line = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleRule_CompilationDiagnostics_Position {
    type O = terrars::BlockAssignable<ChronicleRule_CompilationDiagnostics_Position>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleRule_CompilationDiagnostics_Position {}

impl BuildChronicleRule_CompilationDiagnostics_Position {
    pub fn build(self) -> ChronicleRule_CompilationDiagnostics_Position {
        ChronicleRule_CompilationDiagnostics_Position {
            end_column: core::default::Default::default(),
            end_line: core::default::Default::default(),
            start_column: core::default::Default::default(),
            start_line: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleRule_CompilationDiagnostics_PositionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleRule_CompilationDiagnostics_PositionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleRule_CompilationDiagnostics_PositionRef {
        ChronicleRule_CompilationDiagnostics_PositionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleRule_CompilationDiagnostics_PositionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_column` after provisioning.\n"]
    pub fn end_column(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_column", self.base))
    }

    #[doc = "Get a reference to the value of field `end_line` after provisioning.\n"]
    pub fn end_line(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_line", self.base))
    }

    #[doc = "Get a reference to the value of field `start_column` after provisioning.\n"]
    pub fn start_column(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_column", self.base))
    }

    #[doc = "Get a reference to the value of field `start_line` after provisioning.\n"]
    pub fn start_line(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_line", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleRule_CompilationDiagnostics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    position: std::option::Option<terrars::ListField<ChronicleRule_CompilationDiagnostics_Position>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleRule_CompilationDiagnostics {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `position`.\n"]
    pub fn set_position(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ChronicleRule_CompilationDiagnostics_Position>>,
    ) -> Self {
        self.position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleRule_CompilationDiagnostics {
    type O = terrars::BlockAssignable<ChronicleRule_CompilationDiagnostics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleRule_CompilationDiagnostics {}

impl BuildChronicleRule_CompilationDiagnostics {
    pub fn build(self) -> ChronicleRule_CompilationDiagnostics {
        ChronicleRule_CompilationDiagnostics {
            message: core::default::Default::default(),
            position: core::default::Default::default(),
            severity: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleRule_CompilationDiagnosticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleRule_CompilationDiagnosticsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleRule_CompilationDiagnosticsRef {
        ChronicleRule_CompilationDiagnosticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleRule_CompilationDiagnosticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> terrars::ListRef<ChronicleRule_CompilationDiagnostics_PositionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.position", self.base))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleRule_Severity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleRule_Severity {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleRule_Severity {
    type O = terrars::BlockAssignable<ChronicleRule_Severity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleRule_Severity {}

impl BuildChronicleRule_Severity {
    pub fn build(self) -> ChronicleRule_Severity {
        ChronicleRule_Severity { display_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ChronicleRule_SeverityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleRule_SeverityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleRule_SeverityRef {
        ChronicleRule_SeverityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleRule_SeverityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleRule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleRule_Timeouts {
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

impl terrars::ToListMappable for ChronicleRule_Timeouts {
    type O = terrars::BlockAssignable<ChronicleRule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleRule_Timeouts {}

impl BuildChronicleRule_Timeouts {
    pub fn build(self) -> ChronicleRule_Timeouts {
        ChronicleRule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleRule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleRule_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleRule_TimeoutsRef {
        ChronicleRule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleRule_TimeoutsRef {
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
