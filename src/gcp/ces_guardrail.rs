#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CesGuardrailData_ {
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
    app: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    guardrail_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<CesGuardrail_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_callback: std::option::Option<std::vec::Vec<CesGuardrail_CodeCallback>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_filter: std::option::Option<std::vec::Vec<CesGuardrail_ContentFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_policy: std::option::Option<std::vec::Vec<CesGuardrail_LlmPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_prompt_security: std::option::Option<std::vec::Vec<CesGuardrail_LlmPromptSecurity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_safety: std::option::Option<std::vec::Vec<CesGuardrail_ModelSafety>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CesGuardrail_Timeouts>,
    dynamic: CesGuardrailDynamic,
}

struct CesGuardrail_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CesGuardrailData_>,
}

#[derive(Clone)]
pub struct CesGuardrail(std::rc::Rc<CesGuardrail_>);

impl CesGuardrail {
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

    #[doc = "Set the field `description`.\nDescription of the guardrail."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether the guardrail is enabled."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_Action>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_callback`.\n"]
    pub fn set_code_callback(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_CodeCallback>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().code_callback = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.code_callback = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `content_filter`.\n"]
    pub fn set_content_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_ContentFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().content_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.content_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `llm_policy`.\n"]
    pub fn set_llm_policy(self, v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_LlmPolicy>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().llm_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.llm_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `llm_prompt_security`.\n"]
    pub fn set_llm_prompt_security(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_LlmPromptSecurity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().llm_prompt_security = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.llm_prompt_security = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_safety`.\n"]
    pub fn set_model_safety(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_ModelSafety>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_safety = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_safety = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CesGuardrail_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the guardrail was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the guardrail."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the guardrail."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the guardrail is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `guardrail_id` after provisioning.\nThe ID to use for the guardrail, which will become the final component of\nthe guardrail's resource name. If not provided, a unique ID will be\nautomatically assigned for the guardrail."]
    pub fn guardrail_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the guardrail.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the guardrail was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<CesGuardrail_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_callback` after provisioning.\n"]
    pub fn code_callback(&self) -> terrars::ListRef<CesGuardrail_CodeCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_callback", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_filter` after provisioning.\n"]
    pub fn content_filter(&self) -> terrars::ListRef<CesGuardrail_ContentFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_policy` after provisioning.\n"]
    pub fn llm_policy(&self) -> terrars::ListRef<CesGuardrail_LlmPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_prompt_security` after provisioning.\n"]
    pub fn llm_prompt_security(&self) -> terrars::ListRef<CesGuardrail_LlmPromptSecurityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_prompt_security", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_safety` after provisioning.\n"]
    pub fn model_safety(&self) -> terrars::ListRef<CesGuardrail_ModelSafetyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_safety", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesGuardrail_TimeoutsRef {
        CesGuardrail_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CesGuardrail {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CesGuardrail { }

impl terrars::ToListMappable for CesGuardrail {
    type O = terrars::ListRef<CesGuardrailRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CesGuardrail_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_ces_guardrail".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCesGuardrail {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: terrars::PrimField<std::string::String>,
    #[doc = "Display name of the guardrail."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the guardrail, which will become the final component of\nthe guardrail's resource name. If not provided, a unique ID will be\nautomatically assigned for the guardrail."]
    pub guardrail_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail {
    pub fn build(self, stack: &mut terrars::Stack) -> CesGuardrail {
        let out = CesGuardrail(std::rc::Rc::new(CesGuardrail_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CesGuardrailData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app: self.app,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                enabled: core::default::Default::default(),
                guardrail_id: self.guardrail_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                action: core::default::Default::default(),
                code_callback: core::default::Default::default(),
                content_filter: core::default::Default::default(),
                llm_policy: core::default::Default::default(),
                llm_prompt_security: core::default::Default::default(),
                model_safety: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CesGuardrailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CesGuardrailRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the guardrail was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the guardrail."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the guardrail."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the guardrail is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `guardrail_id` after provisioning.\nThe ID to use for the guardrail, which will become the final component of\nthe guardrail's resource name. If not provided, a unique ID will be\nautomatically assigned for the guardrail."]
    pub fn guardrail_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guardrail_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the guardrail.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the guardrail was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<CesGuardrail_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_callback` after provisioning.\n"]
    pub fn code_callback(&self) -> terrars::ListRef<CesGuardrail_CodeCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_callback", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `content_filter` after provisioning.\n"]
    pub fn content_filter(&self) -> terrars::ListRef<CesGuardrail_ContentFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_policy` after provisioning.\n"]
    pub fn llm_policy(&self) -> terrars::ListRef<CesGuardrail_LlmPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `llm_prompt_security` after provisioning.\n"]
    pub fn llm_prompt_security(&self) -> terrars::ListRef<CesGuardrail_LlmPromptSecurityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_prompt_security", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_safety` after provisioning.\n"]
    pub fn model_safety(&self) -> terrars::ListRef<CesGuardrail_ModelSafetyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_safety", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesGuardrail_TimeoutsRef {
        CesGuardrail_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CesGuardrailDynamic {
    action: std::option::Option<terrars::DynamicBlock<CesGuardrail_Action>>,
    code_callback: std::option::Option<terrars::DynamicBlock<CesGuardrail_CodeCallback>>,
    content_filter: std::option::Option<terrars::DynamicBlock<CesGuardrail_ContentFilter>>,
    llm_policy: std::option::Option<terrars::DynamicBlock<CesGuardrail_LlmPolicy>>,
    llm_prompt_security: std::option::Option<terrars::DynamicBlock<CesGuardrail_LlmPromptSecurity>>,
    model_safety: std::option::Option<terrars::DynamicBlock<CesGuardrail_ModelSafety>>,
}

#[derive(Serialize)]
pub struct CesGuardrail_Action_GenerativeAnswer {
    prompt: terrars::PrimField<std::string::String>,
}

impl CesGuardrail_Action_GenerativeAnswer { }

impl terrars::ToListMappable for CesGuardrail_Action_GenerativeAnswer {
    type O = terrars::BlockAssignable<CesGuardrail_Action_GenerativeAnswer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_Action_GenerativeAnswer {
    #[doc = "The prompt to use for the generative answer."]
    pub prompt: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_Action_GenerativeAnswer {
    pub fn build(self) -> CesGuardrail_Action_GenerativeAnswer {
        CesGuardrail_Action_GenerativeAnswer { prompt: self.prompt }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_Action_GenerativeAnswerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_Action_GenerativeAnswerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_Action_GenerativeAnswerRef {
        CesGuardrail_Action_GenerativeAnswerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_Action_GenerativeAnswerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `prompt` after provisioning.\nThe prompt to use for the generative answer."]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_Action_RespondImmediately_Responses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    text: terrars::PrimField<std::string::String>,
}

impl CesGuardrail_Action_RespondImmediately_Responses {
    #[doc =
        "Set the field `disabled`.\nWhether the response is disabled. Disabled responses are not used by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_Action_RespondImmediately_Responses {
    type O = terrars::BlockAssignable<CesGuardrail_Action_RespondImmediately_Responses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_Action_RespondImmediately_Responses {
    #[doc = "Text for the agent to respond with."]
    pub text: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_Action_RespondImmediately_Responses {
    pub fn build(self) -> CesGuardrail_Action_RespondImmediately_Responses {
        CesGuardrail_Action_RespondImmediately_Responses {
            disabled: core::default::Default::default(),
            text: self.text,
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_Action_RespondImmediately_ResponsesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_Action_RespondImmediately_ResponsesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesGuardrail_Action_RespondImmediately_ResponsesRef {
        CesGuardrail_Action_RespondImmediately_ResponsesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_Action_RespondImmediately_ResponsesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the response is disabled. Disabled responses are not used by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nText for the agent to respond with."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesGuardrail_Action_RespondImmediatelyDynamic {
    responses: std::option::Option<terrars::DynamicBlock<CesGuardrail_Action_RespondImmediately_Responses>>,
}

#[derive(Serialize)]
pub struct CesGuardrail_Action_RespondImmediately {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    responses: std::option::Option<std::vec::Vec<CesGuardrail_Action_RespondImmediately_Responses>>,
    dynamic: CesGuardrail_Action_RespondImmediatelyDynamic,
}

impl CesGuardrail_Action_RespondImmediately {
    #[doc = "Set the field `responses`.\n"]
    pub fn set_responses(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_Action_RespondImmediately_Responses>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.responses = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.responses = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_Action_RespondImmediately {
    type O = terrars::BlockAssignable<CesGuardrail_Action_RespondImmediately>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_Action_RespondImmediately {}

impl BuildCesGuardrail_Action_RespondImmediately {
    pub fn build(self) -> CesGuardrail_Action_RespondImmediately {
        CesGuardrail_Action_RespondImmediately {
            responses: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_Action_RespondImmediatelyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_Action_RespondImmediatelyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_Action_RespondImmediatelyRef {
        CesGuardrail_Action_RespondImmediatelyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_Action_RespondImmediatelyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `responses` after provisioning.\n"]
    pub fn responses(&self) -> terrars::ListRef<CesGuardrail_Action_RespondImmediately_ResponsesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.responses", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_Action_TransferAgent {
    agent: terrars::PrimField<std::string::String>,
}

impl CesGuardrail_Action_TransferAgent { }

impl terrars::ToListMappable for CesGuardrail_Action_TransferAgent {
    type O = terrars::BlockAssignable<CesGuardrail_Action_TransferAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_Action_TransferAgent {
    #[doc =
        "The name of the agent to transfer the conversation to. The agent must be\nin the same app as the current agent.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub agent: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_Action_TransferAgent {
    pub fn build(self) -> CesGuardrail_Action_TransferAgent {
        CesGuardrail_Action_TransferAgent { agent: self.agent }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_Action_TransferAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_Action_TransferAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_Action_TransferAgentRef {
        CesGuardrail_Action_TransferAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_Action_TransferAgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `agent` after provisioning.\nThe name of the agent to transfer the conversation to. The agent must be\nin the same app as the current agent.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesGuardrail_ActionDynamic {
    generative_answer: std::option::Option<terrars::DynamicBlock<CesGuardrail_Action_GenerativeAnswer>>,
    respond_immediately: std::option::Option<terrars::DynamicBlock<CesGuardrail_Action_RespondImmediately>>,
    transfer_agent: std::option::Option<terrars::DynamicBlock<CesGuardrail_Action_TransferAgent>>,
}

#[derive(Serialize)]
pub struct CesGuardrail_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generative_answer: std::option::Option<std::vec::Vec<CesGuardrail_Action_GenerativeAnswer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respond_immediately: std::option::Option<std::vec::Vec<CesGuardrail_Action_RespondImmediately>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_agent: std::option::Option<std::vec::Vec<CesGuardrail_Action_TransferAgent>>,
    dynamic: CesGuardrail_ActionDynamic,
}

impl CesGuardrail_Action {
    #[doc = "Set the field `generative_answer`.\n"]
    pub fn set_generative_answer(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_Action_GenerativeAnswer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generative_answer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generative_answer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `respond_immediately`.\n"]
    pub fn set_respond_immediately(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_Action_RespondImmediately>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.respond_immediately = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.respond_immediately = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transfer_agent`.\n"]
    pub fn set_transfer_agent(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_Action_TransferAgent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transfer_agent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transfer_agent = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_Action {
    type O = terrars::BlockAssignable<CesGuardrail_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_Action {}

impl BuildCesGuardrail_Action {
    pub fn build(self) -> CesGuardrail_Action {
        CesGuardrail_Action {
            generative_answer: core::default::Default::default(),
            respond_immediately: core::default::Default::default(),
            transfer_agent: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_ActionRef {
        CesGuardrail_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `generative_answer` after provisioning.\n"]
    pub fn generative_answer(&self) -> terrars::ListRef<CesGuardrail_Action_GenerativeAnswerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generative_answer", self.base))
    }

    #[doc = "Get a reference to the value of field `respond_immediately` after provisioning.\n"]
    pub fn respond_immediately(&self) -> terrars::ListRef<CesGuardrail_Action_RespondImmediatelyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.respond_immediately", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer_agent` after provisioning.\n"]
    pub fn transfer_agent(&self) -> terrars::ListRef<CesGuardrail_Action_TransferAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_agent", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_CodeCallback_AfterAgentCallback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    python_code: terrars::PrimField<std::string::String>,
}

impl CesGuardrail_CodeCallback_AfterAgentCallback {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proactive_execution_enabled`.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_CodeCallback_AfterAgentCallback {
    type O = terrars::BlockAssignable<CesGuardrail_CodeCallback_AfterAgentCallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_CodeCallback_AfterAgentCallback {
    #[doc = "The python code to execute for the callback."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_CodeCallback_AfterAgentCallback {
    pub fn build(self) -> CesGuardrail_CodeCallback_AfterAgentCallback {
        CesGuardrail_CodeCallback_AfterAgentCallback {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_CodeCallback_AfterAgentCallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_CodeCallback_AfterAgentCallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesGuardrail_CodeCallback_AfterAgentCallbackRef {
        CesGuardrail_CodeCallback_AfterAgentCallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_CodeCallback_AfterAgentCallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_CodeCallback_AfterModelCallback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    python_code: terrars::PrimField<std::string::String>,
}

impl CesGuardrail_CodeCallback_AfterModelCallback {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proactive_execution_enabled`.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_CodeCallback_AfterModelCallback {
    type O = terrars::BlockAssignable<CesGuardrail_CodeCallback_AfterModelCallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_CodeCallback_AfterModelCallback {
    #[doc = "The python code to execute for the callback."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_CodeCallback_AfterModelCallback {
    pub fn build(self) -> CesGuardrail_CodeCallback_AfterModelCallback {
        CesGuardrail_CodeCallback_AfterModelCallback {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_CodeCallback_AfterModelCallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_CodeCallback_AfterModelCallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesGuardrail_CodeCallback_AfterModelCallbackRef {
        CesGuardrail_CodeCallback_AfterModelCallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_CodeCallback_AfterModelCallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_CodeCallback_BeforeAgentCallback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    python_code: terrars::PrimField<std::string::String>,
}

impl CesGuardrail_CodeCallback_BeforeAgentCallback {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proactive_execution_enabled`.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_CodeCallback_BeforeAgentCallback {
    type O = terrars::BlockAssignable<CesGuardrail_CodeCallback_BeforeAgentCallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_CodeCallback_BeforeAgentCallback {
    #[doc = "The python code to execute for the callback."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_CodeCallback_BeforeAgentCallback {
    pub fn build(self) -> CesGuardrail_CodeCallback_BeforeAgentCallback {
        CesGuardrail_CodeCallback_BeforeAgentCallback {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_CodeCallback_BeforeAgentCallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_CodeCallback_BeforeAgentCallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesGuardrail_CodeCallback_BeforeAgentCallbackRef {
        CesGuardrail_CodeCallback_BeforeAgentCallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_CodeCallback_BeforeAgentCallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_CodeCallback_BeforeModelCallback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    python_code: terrars::PrimField<std::string::String>,
}

impl CesGuardrail_CodeCallback_BeforeModelCallback {
    #[doc = "Set the field `description`.\nHuman-readable description of the callback."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proactive_execution_enabled`.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_CodeCallback_BeforeModelCallback {
    type O = terrars::BlockAssignable<CesGuardrail_CodeCallback_BeforeModelCallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_CodeCallback_BeforeModelCallback {
    #[doc = "The python code to execute for the callback."]
    pub python_code: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_CodeCallback_BeforeModelCallback {
    pub fn build(self) -> CesGuardrail_CodeCallback_BeforeModelCallback {
        CesGuardrail_CodeCallback_BeforeModelCallback {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: self.python_code,
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_CodeCallback_BeforeModelCallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_CodeCallback_BeforeModelCallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesGuardrail_CodeCallback_BeforeModelCallbackRef {
        CesGuardrail_CodeCallback_BeforeModelCallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_CodeCallback_BeforeModelCallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the callback."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the callback is disabled. Disabled callbacks are ignored by the\nagent."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\nIf enabled, the callback will also be executed on intermediate model\noutputs. This setting only affects after model callback.\n**ENABLE WITH CAUTION**. Typically after model callback only needs to be\nexecuted after receiving all model responses. Enabling proactive execution\nmay have negative implication on the execution cost and latency, and\nshould only be enabled in rare situations."]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `python_code` after provisioning.\nThe python code to execute for the callback."]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesGuardrail_CodeCallbackDynamic {
    after_agent_callback: std::option::Option<terrars::DynamicBlock<CesGuardrail_CodeCallback_AfterAgentCallback>>,
    after_model_callback: std::option::Option<terrars::DynamicBlock<CesGuardrail_CodeCallback_AfterModelCallback>>,
    before_agent_callback: std::option::Option<terrars::DynamicBlock<CesGuardrail_CodeCallback_BeforeAgentCallback>>,
    before_model_callback: std
    ::option
    ::Option<terrars::DynamicBlock<CesGuardrail_CodeCallback_BeforeModelCallback>>,
}

#[derive(Serialize)]
pub struct CesGuardrail_CodeCallback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_agent_callback: std::option::Option<std::vec::Vec<CesGuardrail_CodeCallback_AfterAgentCallback>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_model_callback: std::option::Option<std::vec::Vec<CesGuardrail_CodeCallback_AfterModelCallback>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_agent_callback: std::option::Option<std::vec::Vec<CesGuardrail_CodeCallback_BeforeAgentCallback>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_model_callback: std::option::Option<std::vec::Vec<CesGuardrail_CodeCallback_BeforeModelCallback>>,
    dynamic: CesGuardrail_CodeCallbackDynamic,
}

impl CesGuardrail_CodeCallback {
    #[doc = "Set the field `after_agent_callback`.\n"]
    pub fn set_after_agent_callback(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_CodeCallback_AfterAgentCallback>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.after_agent_callback = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.after_agent_callback = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `after_model_callback`.\n"]
    pub fn set_after_model_callback(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_CodeCallback_AfterModelCallback>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.after_model_callback = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.after_model_callback = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `before_agent_callback`.\n"]
    pub fn set_before_agent_callback(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_CodeCallback_BeforeAgentCallback>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.before_agent_callback = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.before_agent_callback = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `before_model_callback`.\n"]
    pub fn set_before_model_callback(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_CodeCallback_BeforeModelCallback>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.before_model_callback = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.before_model_callback = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_CodeCallback {
    type O = terrars::BlockAssignable<CesGuardrail_CodeCallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_CodeCallback {}

impl BuildCesGuardrail_CodeCallback {
    pub fn build(self) -> CesGuardrail_CodeCallback {
        CesGuardrail_CodeCallback {
            after_agent_callback: core::default::Default::default(),
            after_model_callback: core::default::Default::default(),
            before_agent_callback: core::default::Default::default(),
            before_model_callback: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_CodeCallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_CodeCallbackRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_CodeCallbackRef {
        CesGuardrail_CodeCallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_CodeCallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `after_agent_callback` after provisioning.\n"]
    pub fn after_agent_callback(&self) -> terrars::ListRef<CesGuardrail_CodeCallback_AfterAgentCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_agent_callback", self.base))
    }

    #[doc = "Get a reference to the value of field `after_model_callback` after provisioning.\n"]
    pub fn after_model_callback(&self) -> terrars::ListRef<CesGuardrail_CodeCallback_AfterModelCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_model_callback", self.base))
    }

    #[doc = "Get a reference to the value of field `before_agent_callback` after provisioning.\n"]
    pub fn before_agent_callback(&self) -> terrars::ListRef<CesGuardrail_CodeCallback_BeforeAgentCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_agent_callback", self.base))
    }

    #[doc = "Get a reference to the value of field `before_model_callback` after provisioning.\n"]
    pub fn before_model_callback(&self) -> terrars::ListRef<CesGuardrail_CodeCallback_BeforeModelCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_model_callback", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_ContentFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banned_contents: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banned_contents_in_agent_response: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banned_contents_in_user_input: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disregard_diacritics: std::option::Option<terrars::PrimField<bool>>,
    match_type: terrars::PrimField<std::string::String>,
}

impl CesGuardrail_ContentFilter {
    #[doc =
        "Set the field `banned_contents`.\nList of banned phrases. Applies to both user inputs and agent responses."]
    pub fn set_banned_contents(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.banned_contents = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `banned_contents_in_agent_response`.\nList of banned phrases. Applies only to agent responses."]
    pub fn set_banned_contents_in_agent_response(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.banned_contents_in_agent_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `banned_contents_in_user_input`.\nList of banned phrases. Applies only to user inputs."]
    pub fn set_banned_contents_in_user_input(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.banned_contents_in_user_input = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disregard_diacritics`.\nIf true, diacritics are ignored during matching."]
    pub fn set_disregard_diacritics(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disregard_diacritics = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_ContentFilter {
    type O = terrars::BlockAssignable<CesGuardrail_ContentFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_ContentFilter {
    #[doc =
        "Match type for the content filter.\nPossible values:\nSIMPLE_STRING_MATCH\nWORD_BOUNDARY_STRING_MATCH\nREGEXP_MATCH"]
    pub match_type: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_ContentFilter {
    pub fn build(self) -> CesGuardrail_ContentFilter {
        CesGuardrail_ContentFilter {
            banned_contents: core::default::Default::default(),
            banned_contents_in_agent_response: core::default::Default::default(),
            banned_contents_in_user_input: core::default::Default::default(),
            disregard_diacritics: core::default::Default::default(),
            match_type: self.match_type,
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_ContentFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_ContentFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_ContentFilterRef {
        CesGuardrail_ContentFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_ContentFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `banned_contents` after provisioning.\nList of banned phrases. Applies to both user inputs and agent responses."]
    pub fn banned_contents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.banned_contents", self.base))
    }

    #[doc =
        "Get a reference to the value of field `banned_contents_in_agent_response` after provisioning.\nList of banned phrases. Applies only to agent responses."]
    pub fn banned_contents_in_agent_response(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.banned_contents_in_agent_response", self.base))
    }

    #[doc =
        "Get a reference to the value of field `banned_contents_in_user_input` after provisioning.\nList of banned phrases. Applies only to user inputs."]
    pub fn banned_contents_in_user_input(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.banned_contents_in_user_input", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disregard_diacritics` after provisioning.\nIf true, diacritics are ignored during matching."]
    pub fn disregard_diacritics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disregard_diacritics", self.base))
    }

    #[doc =
        "Get a reference to the value of field `match_type` after provisioning.\nMatch type for the content filter.\nPossible values:\nSIMPLE_STRING_MATCH\nWORD_BOUNDARY_STRING_MATCH\nREGEXP_MATCH"]
    pub fn match_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_type", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_LlmPolicy_ModelSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thinking_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesGuardrail_LlmPolicy_ModelSettings {
    #[doc =
        "Set the field `model`.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temperature`.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `thinking_level`.\nThe thinking level of the model. Possible values: [\"DEFAULT\", \"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn set_thinking_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.thinking_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_LlmPolicy_ModelSettings {
    type O = terrars::BlockAssignable<CesGuardrail_LlmPolicy_ModelSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_LlmPolicy_ModelSettings {}

impl BuildCesGuardrail_LlmPolicy_ModelSettings {
    pub fn build(self) -> CesGuardrail_LlmPolicy_ModelSettings {
        CesGuardrail_LlmPolicy_ModelSettings {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
            thinking_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_LlmPolicy_ModelSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_LlmPolicy_ModelSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_LlmPolicy_ModelSettingsRef {
        CesGuardrail_LlmPolicy_ModelSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_LlmPolicy_ModelSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `model` after provisioning.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temperature` after provisioning.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thinking_level` after provisioning.\nThe thinking level of the model. Possible values: [\"DEFAULT\", \"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn thinking_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thinking_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesGuardrail_LlmPolicyDynamic {
    model_settings: std::option::Option<terrars::DynamicBlock<CesGuardrail_LlmPolicy_ModelSettings>>,
}

#[derive(Serialize)]
pub struct CesGuardrail_LlmPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_short_utterance: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_conversation_messages: std::option::Option<terrars::PrimField<f64>>,
    policy_scope: terrars::PrimField<std::string::String>,
    prompt: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std::option::Option<std::vec::Vec<CesGuardrail_LlmPolicy_ModelSettings>>,
    dynamic: CesGuardrail_LlmPolicyDynamic,
}

impl CesGuardrail_LlmPolicy {
    #[doc =
        "Set the field `allow_short_utterance`.\nBy default, the LLM policy check is bypassed for short utterances.\nEnabling this setting applies the policy check to all utterances,\nincluding those that would normally be skipped."]
    pub fn set_allow_short_utterance(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_short_utterance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fail_open`.\nIf an error occurs during the policy check, fail open and do not trigger\nthe guardrail."]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_conversation_messages`.\nWhen checking this policy, consider the last 'n' messages in the\nconversation.\nWhen not set a default value of 10 will be used."]
    pub fn set_max_conversation_messages(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_conversation_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_LlmPolicy_ModelSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_LlmPolicy {
    type O = terrars::BlockAssignable<CesGuardrail_LlmPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_LlmPolicy {
    #[doc =
        "Defines when to apply the policy check during the conversation. If set to\n'POLICY_SCOPE_UNSPECIFIED', the policy will be applied to the user input.\nWhen applying the policy to the agent response, additional latency will\nbe introduced before the agent can respond.\nPossible values:\nUSER_QUERY\nAGENT_RESPONSE\nUSER_QUERY_AND_AGENT_RESPONSE Possible values: [\"USER_QUERY\", \"AGENT_RESPONSE\", \"USER_QUERY_AND_AGENT_RESPONSE\"]"]
    pub policy_scope: terrars::PrimField<std::string::String>,
    #[doc = "Policy prompt."]
    pub prompt: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_LlmPolicy {
    pub fn build(self) -> CesGuardrail_LlmPolicy {
        CesGuardrail_LlmPolicy {
            allow_short_utterance: core::default::Default::default(),
            fail_open: core::default::Default::default(),
            max_conversation_messages: core::default::Default::default(),
            policy_scope: self.policy_scope,
            prompt: self.prompt,
            model_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_LlmPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_LlmPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_LlmPolicyRef {
        CesGuardrail_LlmPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_LlmPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_short_utterance` after provisioning.\nBy default, the LLM policy check is bypassed for short utterances.\nEnabling this setting applies the policy check to all utterances,\nincluding those that would normally be skipped."]
    pub fn allow_short_utterance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_short_utterance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fail_open` after provisioning.\nIf an error occurs during the policy check, fail open and do not trigger\nthe guardrail."]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_conversation_messages` after provisioning.\nWhen checking this policy, consider the last 'n' messages in the\nconversation.\nWhen not set a default value of 10 will be used."]
    pub fn max_conversation_messages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_conversation_messages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `policy_scope` after provisioning.\nDefines when to apply the policy check during the conversation. If set to\n'POLICY_SCOPE_UNSPECIFIED', the policy will be applied to the user input.\nWhen applying the policy to the agent response, additional latency will\nbe introduced before the agent can respond.\nPossible values:\nUSER_QUERY\nAGENT_RESPONSE\nUSER_QUERY_AND_AGENT_RESPONSE Possible values: [\"USER_QUERY\", \"AGENT_RESPONSE\", \"USER_QUERY_AND_AGENT_RESPONSE\"]"]
    pub fn policy_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\nPolicy prompt."]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesGuardrail_LlmPolicy_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesGuardrail_LlmPromptSecurity_CustomPolicyDynamic {
    model_settings: std::option::Option<terrars::DynamicBlock<CesGuardrail_LlmPolicy_ModelSettings>>,
}

#[derive(Serialize)]
pub struct CesGuardrail_LlmPromptSecurity_CustomPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_short_utterance: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_conversation_messages: std::option::Option<terrars::PrimField<f64>>,
    policy_scope: terrars::PrimField<std::string::String>,
    prompt: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std::option::Option<std::vec::Vec<CesGuardrail_LlmPolicy_ModelSettings>>,
    dynamic: CesGuardrail_LlmPromptSecurity_CustomPolicyDynamic,
}

impl CesGuardrail_LlmPromptSecurity_CustomPolicy {
    #[doc =
        "Set the field `allow_short_utterance`.\nBy default, the LLM policy check is bypassed for short utterances.\nEnabling this setting applies the policy check to all utterances,\nincluding those that would normally be skipped."]
    pub fn set_allow_short_utterance(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_short_utterance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fail_open`.\nIf an error occurs during the policy check, fail open and do not trigger\nthe guardrail."]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_conversation_messages`.\nWhen checking this policy, consider the last 'n' messages in the\nconversation.\nWhen not set a default value of 10 will be used."]
    pub fn set_max_conversation_messages(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_conversation_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_LlmPolicy_ModelSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_LlmPromptSecurity_CustomPolicy {
    type O = terrars::BlockAssignable<CesGuardrail_LlmPromptSecurity_CustomPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_LlmPromptSecurity_CustomPolicy {
    #[doc =
        "Defines when to apply the policy check during the conversation. If set to\n'POLICY_SCOPE_UNSPECIFIED', the policy will be applied to the user input.\nWhen applying the policy to the agent response, additional latency will\nbe introduced before the agent can respond.\nPossible values:\nUSER_QUERY\nAGENT_RESPONSE\nUSER_QUERY_AND_AGENT_RESPONSE"]
    pub policy_scope: terrars::PrimField<std::string::String>,
    #[doc = "Policy prompt."]
    pub prompt: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_LlmPromptSecurity_CustomPolicy {
    pub fn build(self) -> CesGuardrail_LlmPromptSecurity_CustomPolicy {
        CesGuardrail_LlmPromptSecurity_CustomPolicy {
            allow_short_utterance: core::default::Default::default(),
            fail_open: core::default::Default::default(),
            max_conversation_messages: core::default::Default::default(),
            policy_scope: self.policy_scope,
            prompt: self.prompt,
            model_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_LlmPromptSecurity_CustomPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_LlmPromptSecurity_CustomPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesGuardrail_LlmPromptSecurity_CustomPolicyRef {
        CesGuardrail_LlmPromptSecurity_CustomPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_LlmPromptSecurity_CustomPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_short_utterance` after provisioning.\nBy default, the LLM policy check is bypassed for short utterances.\nEnabling this setting applies the policy check to all utterances,\nincluding those that would normally be skipped."]
    pub fn allow_short_utterance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_short_utterance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fail_open` after provisioning.\nIf an error occurs during the policy check, fail open and do not trigger\nthe guardrail."]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_conversation_messages` after provisioning.\nWhen checking this policy, consider the last 'n' messages in the\nconversation.\nWhen not set a default value of 10 will be used."]
    pub fn max_conversation_messages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_conversation_messages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `policy_scope` after provisioning.\nDefines when to apply the policy check during the conversation. If set to\n'POLICY_SCOPE_UNSPECIFIED', the policy will be applied to the user input.\nWhen applying the policy to the agent response, additional latency will\nbe introduced before the agent can respond.\nPossible values:\nUSER_QUERY\nAGENT_RESPONSE\nUSER_QUERY_AND_AGENT_RESPONSE"]
    pub fn policy_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\nPolicy prompt."]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesGuardrail_LlmPolicy_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_LlmPromptSecurity_DefaultSettings {}

impl CesGuardrail_LlmPromptSecurity_DefaultSettings { }

impl terrars::ToListMappable for CesGuardrail_LlmPromptSecurity_DefaultSettings {
    type O = terrars::BlockAssignable<CesGuardrail_LlmPromptSecurity_DefaultSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_LlmPromptSecurity_DefaultSettings {}

impl BuildCesGuardrail_LlmPromptSecurity_DefaultSettings {
    pub fn build(self) -> CesGuardrail_LlmPromptSecurity_DefaultSettings {
        CesGuardrail_LlmPromptSecurity_DefaultSettings {}
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_LlmPromptSecurity_DefaultSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_LlmPromptSecurity_DefaultSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesGuardrail_LlmPromptSecurity_DefaultSettingsRef {
        CesGuardrail_LlmPromptSecurity_DefaultSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_LlmPromptSecurity_DefaultSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_prompt_template` after provisioning.\nThe default prompt template used by the system.\nThis field is for display purposes to show the user what prompt\nthe system uses by default. It is OUTPUT_ONLY."]
    pub fn default_prompt_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_prompt_template", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesGuardrail_LlmPromptSecurityDynamic {
    custom_policy: std::option::Option<terrars::DynamicBlock<CesGuardrail_LlmPromptSecurity_CustomPolicy>>,
    default_settings: std::option::Option<terrars::DynamicBlock<CesGuardrail_LlmPromptSecurity_DefaultSettings>>,
}

#[derive(Serialize)]
pub struct CesGuardrail_LlmPromptSecurity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_policy: std::option::Option<std::vec::Vec<CesGuardrail_LlmPromptSecurity_CustomPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_settings: std::option::Option<std::vec::Vec<CesGuardrail_LlmPromptSecurity_DefaultSettings>>,
    dynamic: CesGuardrail_LlmPromptSecurityDynamic,
}

impl CesGuardrail_LlmPromptSecurity {
    #[doc =
        "Set the field `fail_open`.\nDetermines the behavior when the guardrail encounters an LLM error.\n- If true: the guardrail is bypassed.\n- If false (default): the guardrail triggers/blocks.\nNote: If a custom policy is provided, this field is ignored in favor of\nthe policy's 'failOpen' configuration."]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_policy`.\n"]
    pub fn set_custom_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_LlmPromptSecurity_CustomPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_settings`.\n"]
    pub fn set_default_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_LlmPromptSecurity_DefaultSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_LlmPromptSecurity {
    type O = terrars::BlockAssignable<CesGuardrail_LlmPromptSecurity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_LlmPromptSecurity {}

impl BuildCesGuardrail_LlmPromptSecurity {
    pub fn build(self) -> CesGuardrail_LlmPromptSecurity {
        CesGuardrail_LlmPromptSecurity {
            fail_open: core::default::Default::default(),
            custom_policy: core::default::Default::default(),
            default_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_LlmPromptSecurityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_LlmPromptSecurityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_LlmPromptSecurityRef {
        CesGuardrail_LlmPromptSecurityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_LlmPromptSecurityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fail_open` after provisioning.\nDetermines the behavior when the guardrail encounters an LLM error.\n- If true: the guardrail is bypassed.\n- If false (default): the guardrail triggers/blocks.\nNote: If a custom policy is provided, this field is ignored in favor of\nthe policy's 'failOpen' configuration."]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_policy` after provisioning.\n"]
    pub fn custom_policy(&self) -> terrars::ListRef<CesGuardrail_LlmPromptSecurity_CustomPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `default_settings` after provisioning.\n"]
    pub fn default_settings(&self) -> terrars::ListRef<CesGuardrail_LlmPromptSecurity_DefaultSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_ModelSafety_SafetySettings {
    category: terrars::PrimField<std::string::String>,
    threshold: terrars::PrimField<std::string::String>,
}

impl CesGuardrail_ModelSafety_SafetySettings { }

impl terrars::ToListMappable for CesGuardrail_ModelSafety_SafetySettings {
    type O = terrars::BlockAssignable<CesGuardrail_ModelSafety_SafetySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_ModelSafety_SafetySettings {
    #[doc =
        "The harm category.\nPossible values:\nHARM_CATEGORY_HATE_SPEECH\nHARM_CATEGORY_DANGEROUS_CONTENT\nHARM_CATEGORY_HARASSMENT\nHARM_CATEGORY_SEXUALLY_EXPLICIT Possible values: [\"HARM_CATEGORY_HATE_SPEECH\", \"HARM_CATEGORY_DANGEROUS_CONTENT\", \"HARM_CATEGORY_HARASSMENT\", \"HARM_CATEGORY_SEXUALLY_EXPLICIT\"]"]
    pub category: terrars::PrimField<std::string::String>,
    #[doc =
        "The harm block threshold.\nPossible values:\nBLOCK_LOW_AND_ABOVE\nBLOCK_MEDIUM_AND_ABOVE\nBLOCK_ONLY_HIGH\nBLOCK_NONE\nOFF Possible values: [\"BLOCK_LOW_AND_ABOVE\", \"BLOCK_MEDIUM_AND_ABOVE\", \"BLOCK_ONLY_HIGH\", \"BLOCK_NONE\", \"OFF\"]"]
    pub threshold: terrars::PrimField<std::string::String>,
}

impl BuildCesGuardrail_ModelSafety_SafetySettings {
    pub fn build(self) -> CesGuardrail_ModelSafety_SafetySettings {
        CesGuardrail_ModelSafety_SafetySettings {
            category: self.category,
            threshold: self.threshold,
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_ModelSafety_SafetySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_ModelSafety_SafetySettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_ModelSafety_SafetySettingsRef {
        CesGuardrail_ModelSafety_SafetySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_ModelSafety_SafetySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `category` after provisioning.\nThe harm category.\nPossible values:\nHARM_CATEGORY_HATE_SPEECH\nHARM_CATEGORY_DANGEROUS_CONTENT\nHARM_CATEGORY_HARASSMENT\nHARM_CATEGORY_SEXUALLY_EXPLICIT Possible values: [\"HARM_CATEGORY_HATE_SPEECH\", \"HARM_CATEGORY_DANGEROUS_CONTENT\", \"HARM_CATEGORY_HARASSMENT\", \"HARM_CATEGORY_SEXUALLY_EXPLICIT\"]"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc =
        "Get a reference to the value of field `threshold` after provisioning.\nThe harm block threshold.\nPossible values:\nBLOCK_LOW_AND_ABOVE\nBLOCK_MEDIUM_AND_ABOVE\nBLOCK_ONLY_HIGH\nBLOCK_NONE\nOFF Possible values: [\"BLOCK_LOW_AND_ABOVE\", \"BLOCK_MEDIUM_AND_ABOVE\", \"BLOCK_ONLY_HIGH\", \"BLOCK_NONE\", \"OFF\"]"]
    pub fn threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesGuardrail_ModelSafetyDynamic {
    safety_settings: std::option::Option<terrars::DynamicBlock<CesGuardrail_ModelSafety_SafetySettings>>,
}

#[derive(Serialize)]
pub struct CesGuardrail_ModelSafety {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    safety_settings: std::option::Option<std::vec::Vec<CesGuardrail_ModelSafety_SafetySettings>>,
    dynamic: CesGuardrail_ModelSafetyDynamic,
}

impl CesGuardrail_ModelSafety {
    #[doc = "Set the field `safety_settings`.\n"]
    pub fn set_safety_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesGuardrail_ModelSafety_SafetySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.safety_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.safety_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesGuardrail_ModelSafety {
    type O = terrars::BlockAssignable<CesGuardrail_ModelSafety>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_ModelSafety {}

impl BuildCesGuardrail_ModelSafety {
    pub fn build(self) -> CesGuardrail_ModelSafety {
        CesGuardrail_ModelSafety {
            safety_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_ModelSafetyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_ModelSafetyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_ModelSafetyRef {
        CesGuardrail_ModelSafetyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_ModelSafetyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `safety_settings` after provisioning.\n"]
    pub fn safety_settings(&self) -> terrars::ListRef<CesGuardrail_ModelSafety_SafetySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.safety_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct CesGuardrail_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesGuardrail_Timeouts {
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

impl terrars::ToListMappable for CesGuardrail_Timeouts {
    type O = terrars::BlockAssignable<CesGuardrail_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesGuardrail_Timeouts {}

impl BuildCesGuardrail_Timeouts {
    pub fn build(self) -> CesGuardrail_Timeouts {
        CesGuardrail_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesGuardrail_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesGuardrail_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesGuardrail_TimeoutsRef {
        CesGuardrail_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesGuardrail_TimeoutsRef {
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
