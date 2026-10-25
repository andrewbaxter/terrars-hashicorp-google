#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataLossPreventionContentPolicyData_ {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_action: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_DefaultAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failed_to_scan_supported_file_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_FailedToScanSupportedFileType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_too_large: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InputTooLarge>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_config: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_configs: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_LoggingConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_Rules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLossPreventionContentPolicy_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unsupported_file_type: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_UnsupportedFileType>>,
    dynamic: DataLossPreventionContentPolicyDynamic,
}

struct DataLossPreventionContentPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLossPreventionContentPolicyData_>,
}

#[derive(Clone)]
pub struct DataLossPreventionContentPolicy(std::rc::Rc<DataLossPreventionContentPolicy_>);

impl DataLossPreventionContentPolicy {
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

    #[doc = "Set the field `display_name`.\nDisplay name (max 63 chars)."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_action`.\n"]
    pub fn set_default_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_DefaultAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failed_to_scan_supported_file_type`.\n"]
    pub fn set_failed_to_scan_supported_file_type(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_FailedToScanSupportedFileType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().failed_to_scan_supported_file_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.failed_to_scan_supported_file_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `input_too_large`.\n"]
    pub fn set_input_too_large(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_InputTooLarge>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_too_large = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_too_large = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inspect_config`.\n"]
    pub fn set_inspect_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inspect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inspect_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_configs`.\n"]
    pub fn set_logging_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_LoggingConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_Rules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLossPreventionContentPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unsupported_file_type`.\n"]
    pub fn set_unsupported_file_type(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_UnsupportedFileType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().unsupported_file_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.unsupported_file_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation timestamp of a ContentPolicy."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name (max 63 chars)."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `errors` after provisioning.\nOutput only. A stream of errors encountered when the policy was applied. Output only field. Will return the last 100 errors."]
    pub fn errors(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the content policy. Set by the server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the content policy in any of the following formats:\n\n* 'projects/{{project}}/locations/{{location}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last update timestamp of a ContentPolicy."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_DefaultActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failed_to_scan_supported_file_type` after provisioning.\n"]
    pub fn failed_to_scan_supported_file_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_FailedToScanSupportedFileTypeRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.failed_to_scan_supported_file_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `input_too_large` after provisioning.\n"]
    pub fn input_too_large(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InputTooLargeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_too_large", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inspect_config` after provisioning.\n"]
    pub fn inspect_config(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configs` after provisioning.\n"]
    pub fn logging_configs(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_LoggingConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionContentPolicy_TimeoutsRef {
        DataLossPreventionContentPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `unsupported_file_type` after provisioning.\n"]
    pub fn unsupported_file_type(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_UnsupportedFileTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unsupported_file_type", self.extract_ref()))
    }
}

impl terrars::Referable for DataLossPreventionContentPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataLossPreventionContentPolicy { }

impl terrars::ToListMappable for DataLossPreventionContentPolicy {
    type O = terrars::ListRef<DataLossPreventionContentPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataLossPreventionContentPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_data_loss_prevention_content_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLossPreventionContentPolicy {
    pub tf_id: std::string::String,
    #[doc =
        "The parent of the content policy in any of the following formats:\n\n* 'projects/{{project}}/locations/{{location}}'"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLossPreventionContentPolicy {
        let out = DataLossPreventionContentPolicy(std::rc::Rc::new(DataLossPreventionContentPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLossPreventionContentPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                default_action: core::default::Default::default(),
                failed_to_scan_supported_file_type: core::default::Default::default(),
                input_too_large: core::default::Default::default(),
                inspect_config: core::default::Default::default(),
                logging_configs: core::default::Default::default(),
                rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                unsupported_file_type: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataLossPreventionContentPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLossPreventionContentPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation timestamp of a ContentPolicy."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name (max 63 chars)."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `errors` after provisioning.\nOutput only. A stream of errors encountered when the policy was applied. Output only field. Will return the last 100 errors."]
    pub fn errors(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the content policy. Set by the server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the content policy in any of the following formats:\n\n* 'projects/{{project}}/locations/{{location}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last update timestamp of a ContentPolicy."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_DefaultActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failed_to_scan_supported_file_type` after provisioning.\n"]
    pub fn failed_to_scan_supported_file_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_FailedToScanSupportedFileTypeRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.failed_to_scan_supported_file_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `input_too_large` after provisioning.\n"]
    pub fn input_too_large(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InputTooLargeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_too_large", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inspect_config` after provisioning.\n"]
    pub fn inspect_config(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configs` after provisioning.\n"]
    pub fn logging_configs(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_LoggingConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionContentPolicy_TimeoutsRef {
        DataLossPreventionContentPolicy_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `unsupported_file_type` after provisioning.\n"]
    pub fn unsupported_file_type(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_UnsupportedFileTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unsupported_file_type", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicyDynamic {
    default_action: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_DefaultAction>>,
    failed_to_scan_supported_file_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_FailedToScanSupportedFileType>>,
    input_too_large: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InputTooLarge>>,
    inspect_config: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig>>,
    logging_configs: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_LoggingConfigs>>,
    rules: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_Rules>>,
    unsupported_file_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_UnsupportedFileType>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_Errors_Details {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionContentPolicy_Errors_Details {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_Errors_Details {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_Errors_Details>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_Errors_Details {}

impl BuildDataLossPreventionContentPolicy_Errors_Details {
    pub fn build(self) -> DataLossPreventionContentPolicy_Errors_Details {
        DataLossPreventionContentPolicy_Errors_Details {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_Errors_DetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_Errors_DetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_Errors_DetailsRef {
        DataLossPreventionContentPolicy_Errors_DetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_Errors_DetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_Errors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<DataLossPreventionContentPolicy_Errors_Details>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_info: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamps: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLossPreventionContentPolicy_Errors {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLossPreventionContentPolicy_Errors_Details>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extra_info`.\n"]
    pub fn set_extra_info(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.extra_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamps`.\n"]
    pub fn set_timestamps(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.timestamps = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_Errors {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_Errors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_Errors {}

impl BuildDataLossPreventionContentPolicy_Errors {
    pub fn build(self) -> DataLossPreventionContentPolicy_Errors {
        DataLossPreventionContentPolicy_Errors {
            details: core::default::Default::default(),
            extra_info: core::default::Default::default(),
            timestamps: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_ErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_ErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionContentPolicy_ErrorsRef {
        DataLossPreventionContentPolicy_ErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_ErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_Errors_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `extra_info` after provisioning.\n"]
    pub fn extra_info(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extra_info", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamps` after provisioning.\n"]
    pub fn timestamps(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamps", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_DefaultAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_verdict: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionContentPolicy_DefaultAction {
    #[doc =
        "Set the field `return_verdict`.\nIf set, the verdict will be returned to the user.\nPossible values: [\"ALLOW\", \"BLOCK\"] Possible values: [\"ALLOW\", \"BLOCK\"]"]
    pub fn set_return_verdict(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.return_verdict = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_DefaultAction {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_DefaultAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_DefaultAction {}

impl BuildDataLossPreventionContentPolicy_DefaultAction {
    pub fn build(self) -> DataLossPreventionContentPolicy_DefaultAction {
        DataLossPreventionContentPolicy_DefaultAction { return_verdict: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_DefaultActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_DefaultActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_DefaultActionRef {
        DataLossPreventionContentPolicy_DefaultActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_DefaultActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_verdict` after provisioning.\nIf set, the verdict will be returned to the user.\nPossible values: [\"ALLOW\", \"BLOCK\"] Possible values: [\"ALLOW\", \"BLOCK\"]"]
    pub fn return_verdict(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_verdict", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_FailedToScanSupportedFileType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_verdict: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionContentPolicy_FailedToScanSupportedFileType {
    #[doc =
        "Set the field `return_verdict`.\nIf set, the verdict will be returned to the user.\nPossible values: [\"ALLOW\", \"BLOCK\"] Possible values: [\"ALLOW\", \"BLOCK\"]"]
    pub fn set_return_verdict(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.return_verdict = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_FailedToScanSupportedFileType {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_FailedToScanSupportedFileType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_FailedToScanSupportedFileType {}

impl BuildDataLossPreventionContentPolicy_FailedToScanSupportedFileType {
    pub fn build(self) -> DataLossPreventionContentPolicy_FailedToScanSupportedFileType {
        DataLossPreventionContentPolicy_FailedToScanSupportedFileType {
            return_verdict: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_FailedToScanSupportedFileTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_FailedToScanSupportedFileTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_FailedToScanSupportedFileTypeRef {
        DataLossPreventionContentPolicy_FailedToScanSupportedFileTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_FailedToScanSupportedFileTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_verdict` after provisioning.\nIf set, the verdict will be returned to the user.\nPossible values: [\"ALLOW\", \"BLOCK\"] Possible values: [\"ALLOW\", \"BLOCK\"]"]
    pub fn return_verdict(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_verdict", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InputTooLarge {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_verdict: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionContentPolicy_InputTooLarge {
    #[doc =
        "Set the field `return_verdict`.\nIf set, the verdict will be returned to the user.\nPossible values: [\"ALLOW\", \"BLOCK\"] Possible values: [\"ALLOW\", \"BLOCK\"]"]
    pub fn set_return_verdict(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.return_verdict = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InputTooLarge {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InputTooLarge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InputTooLarge {}

impl BuildDataLossPreventionContentPolicy_InputTooLarge {
    pub fn build(self) -> DataLossPreventionContentPolicy_InputTooLarge {
        DataLossPreventionContentPolicy_InputTooLarge { return_verdict: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InputTooLargeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InputTooLargeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InputTooLargeRef {
        DataLossPreventionContentPolicy_InputTooLargeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InputTooLargeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_verdict` after provisioning.\nIf set, the verdict will be returned to the user.\nPossible values: [\"ALLOW\", \"BLOCK\"] Possible values: [\"ALLOW\", \"BLOCK\"]"]
    pub fn return_verdict(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_verdict", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    #[doc =
        "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.group_indexes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    #[doc =
        "Pattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group_indexes` after provisioning.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."]
    pub fn group_indexes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_indexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nPattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relative_likelihood: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment {
    #[doc =
        "Set the field `fixed_likelihood`.\nSet the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_fixed_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fixed_likelihood = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `relative_likelihood`.\nIncrease or decrease the likelihood by the specified number of levels."]
    pub fn set_relative_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.relative_likelihood = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment {
            fixed_likelihood: core::default::Default::default(),
            relative_likelihood: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustmentRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fixed_likelihood` after provisioning.\nSet the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn fixed_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_likelihood", self.base))
    }

    #[doc =
        "Get a reference to the value of field `relative_likelihood` after provisioning.\nIncrease or decrease the likelihood by the specified number of levels."]
    pub fn relative_likelihood(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relative_likelihood", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_before: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity {
    #[doc = "Set the field `window_after`.\nNumber of characters after the finding to consider."]
    pub fn set_window_after(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window_before`.\nNumber of characters before the finding to consider."]
    pub fn set_window_before(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window_before = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity {
            window_after: core::default::Default::default(),
            window_before: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_ProximityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_ProximityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_ProximityRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_ProximityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_ProximityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `window_after` after provisioning.\nNumber of characters after the finding to consider."]
    pub fn window_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `window_before` after provisioning.\nNumber of characters before the finding to consider."]
    pub fn window_before(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_before", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRuleDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>,
    >,
    likelihood_adjustment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment,
        >,
    >,
    proximity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood_adjustment: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity>,
    >,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRuleDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule {
    #[doc = "Set the field `hotword_regex`.\n"]
    pub fn set_hotword_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `likelihood_adjustment`.\n"]
    pub fn set_likelihood_adjustment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.likelihood_adjustment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.likelihood_adjustment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proximity`.\n"]
    pub fn set_proximity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proximity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proximity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule {
            hotword_regex: core::default::Default::default(),
            likelihood_adjustment: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRuleRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `likelihood_adjustment` after provisioning.\n"]
    pub fn likelihood_adjustment(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_LikelihoodAdjustmentRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.likelihood_adjustment", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_ProximityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRulesDynamic {
    hotword_rule: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRulesDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules {
    #[doc = "Set the field `hotword_rule`.\n"]
    pub fn set_hotword_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules {
            hotword_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRulesRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_rule` after provisioning.\n"]
    pub fn hotword_rule(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath { }

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath { path: self.path }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nA url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList { }

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    type O =
        terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    #[doc =
        "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList { words: self.words }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `words` after provisioning.\nWords or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub fn words(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.words", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DictionaryDynamic {
    cloud_storage_path: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath>,
    >,
    word_list: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_path: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    word_list: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DictionaryDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary {
    #[doc = "Set the field `cloud_storage_path`.\n"]
    pub fn set_cloud_storage_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `word_list`.\n"]
    pub fn set_word_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.word_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.word_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DictionaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DictionaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DictionaryRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DictionaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DictionaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]
    pub fn cloud_storage_path(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_path", self.base))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary_WordListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch {
    id: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch {

}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch {
    #[doc = "The identifier of the Label Field."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = "The value of the Label Field to match."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch {
            id: self.id,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatchRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe identifier of the Label Field."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the Label Field to match."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabelDynamic {
    label_fields_to_match: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel {
    label_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_fields_to_match: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch,
        >,
    >,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabelDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel {
    #[doc = "Set the field `label_fields_to_match`.\n"]
    pub fn set_label_fields_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatch,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_fields_to_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_fields_to_match = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel {
    #[doc = "The label ID of the Google Drive label."]
    pub label_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel {
            label_id: self.label_id,
            label_fields_to_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabelRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `label_id` after provisioning.\nThe label ID of the Google Drive label."]
    pub fn label_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_id", self.base))
    }

    #[doc = "Get a reference to the value of field `label_fields_to_match` after provisioning.\n"]
    pub fn label_fields_to_match(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel_LabelFieldsToMatchRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_fields_to_match", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel {
    guid: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel { }

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel {
    #[doc = "The GUID of the sensitivity label."]
    pub guid: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel {
            guid: self.guid,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabelRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `guid` after provisioning.\nThe GUID of the sensitivity label."]
    pub fn guid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guid", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoTypeDynamic {
    google_drive_label: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel,
        >,
    >,
    sensitivity_label: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_drive_label: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_label: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel>,
    >,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoTypeDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType {
    #[doc = "Set the field `google_drive_label`.\n"]
    pub fn set_google_drive_label(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabel,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_drive_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_drive_label = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sensitivity_label`.\n"]
    pub fn set_sensitivity_label(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabel,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_label = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType {
    type O =
        terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType {
            google_drive_label: core::default::Default::default(),
            sensitivity_label: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoTypeRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `google_drive_label` after provisioning.\n"]
    pub fn google_drive_label(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_GoogleDriveLabelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_drive_label", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_label` after provisioning.\n"]
    pub fn sensitivity_label(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType_SensitivityLabelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_label", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore { }

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore {
        DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore { score: self.score }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef {
        DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoTypeDynamic {
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoTypeDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names\nlisted at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoTypeRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names\nlisted at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression {
    key_regex: terrars::PrimField<std::string::String>,
    value_regex: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression { }

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression {
    #[doc = "The regular expression for the key."]
    pub key_regex: terrars::PrimField<std::string::String>,
    #[doc = "The regular expression for the value."]
    pub value_regex: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression {
            key_regex: self.key_regex,
            value_regex: self.value_regex,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpressionRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key_regex` after provisioning.\nThe regular expression for the key."]
    pub fn key_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value_regex` after provisioning.\nThe regular expression for the value."]
    pub fn value_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex {
    #[doc =
        "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.group_indexes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex {
    #[doc =
        "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_RegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_RegexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_RegexRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_RegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_RegexRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group_indexes` after provisioning.\nThe index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub fn group_indexes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_indexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nPattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType { }

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType {
    #[doc =
        "Resource name of the requested StoredInfoType, for example 'organizations/433245324/storedInfoTypes/432452342'\nor 'projects/project-id/storedInfoTypes/432452342'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredTypeRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp indicating when the version of the StoredInfoType used for inspection was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name of the requested StoredInfoType, for example 'organizations/433245324/storedInfoTypes/432452342'\nor 'projects/project-id/storedInfoTypes/432452342'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType {}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType { }

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypesDynamic {
    detection_rules: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules>>,
    dictionary: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary>>,
    file_label_info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType>>,
    info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType>>,
    metadata_key_value_expression: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression>,
    >,
    regex: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex>>,
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
    stored_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType>>,
    surrogate_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detection_rules: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_label_info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_key_value_expression: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surrogate_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypesDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes {
    #[doc =
        "Set the field `exclusion_type`.\nIf set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching. Possible values: [\"EXCLUSION_TYPE_EXCLUDE\"]"]
    pub fn set_exclusion_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exclusion_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `likelihood`.\nLikelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria\nspecified by the rule. Default value: \"VERY_LIKELY\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.likelihood = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detection_rules`.\n"]
    pub fn set_detection_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.detection_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.detection_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dictionary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dictionary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_label_info_type`.\n"]
    pub fn set_file_label_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_label_info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_label_info_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `info_type`.\n"]
    pub fn set_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoType>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata_key_value_expression`.\n"]
    pub fn set_metadata_key_value_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpression,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata_key_value_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_key_value_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stored_type`.\n"]
    pub fn set_stored_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredType>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stored_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stored_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `surrogate_type`.\n"]
    pub fn set_surrogate_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateType>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.surrogate_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.surrogate_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes {
            exclusion_type: core::default::Default::default(),
            likelihood: core::default::Default::default(),
            detection_rules: core::default::Default::default(),
            dictionary: core::default::Default::default(),
            file_label_info_type: core::default::Default::default(),
            info_type: core::default::Default::default(),
            metadata_key_value_expression: core::default::Default::default(),
            regex: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            stored_type: core::default::Default::default(),
            surrogate_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypesRef {
        DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclusion_type` after provisioning.\nIf set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching. Possible values: [\"EXCLUSION_TYPE_EXCLUDE\"]"]
    pub fn exclusion_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclusion_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `likelihood` after provisioning.\nLikelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria\nspecified by the rule. Default value: \"VERY_LIKELY\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.likelihood", self.base))
    }

    #[doc = "Get a reference to the value of field `detection_rules` after provisioning.\n"]
    pub fn detection_rules(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.detection_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DictionaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }

    #[doc = "Get a reference to the value of field `file_label_info_type` after provisioning.\n"]
    pub fn file_label_info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_FileLabelInfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_label_info_type", self.base))
    }

    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_InfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_key_value_expression` after provisioning.\n"]
    pub fn metadata_key_value_expression(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_MetadataKeyValueExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_key_value_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_RegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }

    #[doc = "Get a reference to the value of field `stored_type` after provisioning.\n"]
    pub fn stored_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_StoredTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stored_type", self.base))
    }

    #[doc = "Get a reference to the value of field `surrogate_type` after provisioning.\n"]
    pub fn surrogate_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_SurrogateTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.surrogate_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_InfoTypesDynamic {
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_InfoTypes {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_InfoTypesDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_InfoTypes {
    #[doc =
        "Set the field `version`.\nVersion of the information type to use. By default, the version is set to stable"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_InfoTypes {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_InfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_InfoTypes {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_InfoTypes {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_InfoTypes {
        DataLossPreventionContentPolicy_InspectConfig_InfoTypes {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_InfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_InfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_InfoTypesRef {
        DataLossPreventionContentPolicy_InspectConfig_InfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_InfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the information type to use. By default, the version is set to stable"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoTypeDynamic {
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoTypeDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType {
    type O =
        terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType {
        DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef {
        DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoTypeDynamic {
    info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType {
    max_findings: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoTypeDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType {
    #[doc = "Set the field `info_type`.\n"]
    pub fn set_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType {
    #[doc = "Max findings limit for the given infoType."]
    pub max_findings: terrars::PrimField<f64>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType {
        DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType {
            max_findings: self.max_findings,
            info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
        DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_findings` after provisioning.\nMax findings limit for the given infoType."]
    pub fn max_findings(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_findings", self.base))
    }

    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_LimitsDynamic {
    max_findings_per_info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_Limits {
    max_findings_per_item: terrars::PrimField<f64>,
    max_findings_per_request: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings_per_info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_LimitsDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_Limits {
    #[doc = "Set the field `max_findings_per_info_type`.\n"]
    pub fn set_max_findings_per_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoType>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.max_findings_per_info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.max_findings_per_info_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_Limits {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_Limits {
    #[doc = "Max number of findings that will be returned for each item scanned. The maximum returned is 2000."]
    pub max_findings_per_item: terrars::PrimField<f64>,
    #[doc = "Max number of findings that will be returned per request/job. The maximum returned is 2000."]
    pub max_findings_per_request: terrars::PrimField<f64>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_Limits {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_Limits {
        DataLossPreventionContentPolicy_InspectConfig_Limits {
            max_findings_per_item: self.max_findings_per_item,
            max_findings_per_request: self.max_findings_per_request,
            max_findings_per_info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_LimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_LimitsRef {
        DataLossPreventionContentPolicy_InspectConfig_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_findings_per_item` after provisioning.\nMax number of findings that will be returned for each item scanned. The maximum returned is 2000."]
    pub fn max_findings_per_item(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_findings_per_item", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_findings_per_request` after provisioning.\nMax number of findings that will be returned per request/job. The maximum returned is 2000."]
    pub fn max_findings_per_request(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_findings_per_request", self.base))
    }

    #[doc = "Get a reference to the value of field `max_findings_per_info_type` after provisioning.\n"]
    pub fn max_findings_per_info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_Limits_MaxFindingsPerInfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_findings_per_info_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoTypeDynamic {
    info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType {
    min_likelihood: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoTypeDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType {
    #[doc = "Set the field `info_type`.\n"]
    pub fn set_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType {
    #[doc =
        "Only returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub min_likelihood: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType {
        DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType {
            min_likelihood: self.min_likelihood,
            info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoTypeRef {
        DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `min_likelihood` after provisioning.\nOnly returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn min_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_likelihood", self.base))
    }

    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypesDynamic {
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypesDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypesRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses {}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses {

}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_EnclosesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_EnclosesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_EnclosesRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_EnclosesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_EnclosesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside {}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside {

}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInsideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInsideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInsideRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInsideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInsideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps {}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps {

}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_OverlapsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_OverlapsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_OverlapsRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_OverlapsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_OverlapsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentTypeDynamic {
    encloses: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses,
        >,
    >,
    fully_inside: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside,
        >,
    >,
    overlaps: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encloses: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fully_inside: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overlaps: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps,
        >,
    >,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentTypeDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType {
    #[doc = "Set the field `encloses`.\n"]
    pub fn set_encloses(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Encloses,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encloses = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encloses = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fully_inside`.\n"]
    pub fn set_fully_inside(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInside,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fully_inside = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fully_inside = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `overlaps`.\n"]
    pub fn set_overlaps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_Overlaps,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.overlaps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.overlaps = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType {
            encloses: core::default::Default::default(),
            fully_inside: core::default::Default::default(),
            overlaps: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentTypeRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encloses` after provisioning.\n"]
    pub fn encloses(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_EnclosesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encloses", self.base))
    }

    #[doc = "Get a reference to the value of field `fully_inside` after provisioning.\n"]
    pub fn fully_inside(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_FullyInsideRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fully_inside", self.base))
    }

    #[doc = "Get a reference to the value of field `overlaps` after provisioning.\n"]
    pub fn overlaps(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType_OverlapsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.overlaps", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypesDynamic {
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypesDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScore>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes {
    #[doc = "Name of the information type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypesRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the information type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindingsDynamic {
    image_containment_type: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType,
        >,
    >,
    info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings {
    min_likelihood: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_containment_type: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes,
        >,
    >,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindingsDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings {
    #[doc = "Set the field `image_containment_type`.\n"]
    pub fn set_image_containment_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_containment_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_containment_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings {
    #[doc =
        "Minimum likelihood of the adjustByImageFindings infoTypes finding. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub min_likelihood: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings {
            min_likelihood: self.min_likelihood,
            image_containment_type: core::default::Default::default(),
            info_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindingsRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `min_likelihood` after provisioning.\nMinimum likelihood of the adjustByImageFindings infoTypes finding. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn min_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_likelihood", self.base))
    }

    #[doc = "Get a reference to the value of field `image_containment_type` after provisioning.\n"]
    pub fn image_containment_type(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentTypeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_containment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypesDynamic {
    info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes {
    matching_type: terrars::PrimField<std::string::String>,
    min_likelihood: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes,
        >,
    >,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypesDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes {
    #[doc =
        "How the adjustment rule is applied. Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\"]"]
    pub matching_type: terrars::PrimField<std::string::String>,
    #[doc =
        "Minimum likelihood of the adjustByMatchingInfoTypes infoTypes finding. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub min_likelihood: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes {
            matching_type: self.matching_type,
            min_likelihood: self.min_likelihood,
            info_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypesRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `matching_type` after provisioning.\nHow the adjustment rule is applied. Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\"]"]
    pub fn matching_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.matching_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_likelihood` after provisioning.\nMinimum likelihood of the adjustByMatchingInfoTypes infoTypes finding. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn min_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_likelihood", self.base))
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment {
    fixed_likelihood: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment { }

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment {
    #[doc =
        "Set the likelihood of a finding to a fixed value. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fixed_likelihood: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment {
            fixed_likelihood: self.fixed_likelihood,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustmentRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fixed_likelihood` after provisioning.\nSet the likelihood of a finding to a fixed value. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn fixed_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_likelihood", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRuleDynamic {
    adjust_by_image_findings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings,
        >,
    >,
    adjust_by_matching_info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes,
        >,
    >,
    likelihood_adjustment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adjust_by_image_findings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adjust_by_matching_info_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood_adjustment: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment>,
    >,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRuleDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule {
    #[doc = "Set the field `adjust_by_image_findings`.\n"]
    pub fn set_adjust_by_image_findings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.adjust_by_image_findings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.adjust_by_image_findings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `adjust_by_matching_info_types`.\n"]
    pub fn set_adjust_by_matching_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.adjust_by_matching_info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.adjust_by_matching_info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `likelihood_adjustment`.\n"]
    pub fn set_likelihood_adjustment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.likelihood_adjustment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.likelihood_adjustment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule {
            adjust_by_image_findings: core::default::Default::default(),
            adjust_by_matching_info_types: core::default::Default::default(),
            likelihood_adjustment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRuleRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adjust_by_image_findings` after provisioning.\n"]
    pub fn adjust_by_image_findings(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.adjust_by_image_findings", self.base))
    }

    #[doc = "Get a reference to the value of field `adjust_by_matching_info_types` after provisioning.\n"]
    pub fn adjust_by_matching_info_types(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByMatchingInfoTypesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.adjust_by_matching_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `likelihood_adjustment` after provisioning.\n"]
    pub fn likelihood_adjustment(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_LikelihoodAdjustmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.likelihood_adjustment", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>,
    >,
    proximity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity>,
    >,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    #[doc = "Set the field `hotword_regex`.\n"]
    pub fn set_hotword_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proximity`.\n"]
    pub fn set_proximity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_Proximity,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proximity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proximity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
            hotword_regex: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DetectionRules_HotwordRule_ProximityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindingsDynamic {
    image_containment_type: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType,
        >,
    >,
    info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_containment_type: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes,
        >,
    >,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindingsDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings {
    #[doc = "Set the field `image_containment_type`.\n"]
    pub fn set_image_containment_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image_containment_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image_containment_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings {
            image_containment_type: core::default::Default::default(),
            info_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindingsRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image_containment_type` after provisioning.\n"]
    pub fn image_containment_type(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_ImageContainmentTypeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image_containment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule_AdjustByImageFindings_InfoTypesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesDynamic {
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
            info_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRuleDynamic {
    dictionary: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary>>,
    exclude_by_hotword: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword>,
    >,
    exclude_by_image_findings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings,
        >,
    >,
    exclude_info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes>,
    >,
    regex: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule {
    matching_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_by_hotword: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_by_image_findings: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_info_types: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRuleDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule {
    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Dictionary>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dictionary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dictionary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude_by_hotword`.\n"]
    pub fn set_exclude_by_hotword(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_by_hotword = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_by_hotword = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude_by_image_findings`.\n"]
    pub fn set_exclude_by_image_findings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_by_image_findings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_by_image_findings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude_info_types`.\n"]
    pub fn set_exclude_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_Regex>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule {
    #[doc =
        "How the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\", \"MATCHING_TYPE_RULE_SPECIFIC\"]"]
    pub matching_type: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule {
            matching_type: self.matching_type,
            dictionary: core::default::Default::default(),
            exclude_by_hotword: core::default::Default::default(),
            exclude_by_image_findings: core::default::Default::default(),
            exclude_info_types: core::default::Default::default(),
            regex: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `matching_type` after provisioning.\nHow the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\", \"MATCHING_TYPE_RULE_SPECIFIC\"]"]
    pub fn matching_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.matching_type", self.base))
    }

    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_DictionaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_by_hotword` after provisioning.\n"]
    pub fn exclude_by_hotword(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_by_hotword", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_by_image_findings` after provisioning.\n"]
    pub fn exclude_by_image_findings(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByImageFindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_by_image_findings", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_info_types` after provisioning.\n"]
    pub fn exclude_info_types(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes_RegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relative_likelihood: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    #[doc =
        "Set the field `fixed_likelihood`.\nSet the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_fixed_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fixed_likelihood = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `relative_likelihood`.\nIncrease or decrease the likelihood by the specified number of levels. For example,\nif a finding would be POSSIBLE without the detection rule and relativeLikelihood is 1,\nthen it is upgraded to LIKELY, while a value of -1 would downgrade it to UNLIKELY.\nLikelihood may never drop below VERY_UNLIKELY or exceed VERY_LIKELY, so applying an\nadjustment of 1 followed by an adjustment of -1 when base likelihood is VERY_LIKELY\nwill result in a final likelihood of LIKELY. Either this or fixed_likelihood can be set."]
    pub fn set_relative_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.relative_likelihood = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    pub fn build(
        self,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
            fixed_likelihood: core::default::Default::default(),
            relative_likelihood: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fixed_likelihood` after provisioning.\nSet the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn fixed_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_likelihood", self.base))
    }

    #[doc =
        "Get a reference to the value of field `relative_likelihood` after provisioning.\nIncrease or decrease the likelihood by the specified number of levels. For example,\nif a finding would be POSSIBLE without the detection rule and relativeLikelihood is 1,\nthen it is upgraded to LIKELY, while a value of -1 would downgrade it to UNLIKELY.\nLikelihood may never drop below VERY_UNLIKELY or exceed VERY_LIKELY, so applying an\nadjustment of 1 followed by an adjustment of -1 when base likelihood is VERY_LIKELY\nwill result in a final likelihood of LIKELY. Either this or fixed_likelihood can be set."]
    pub fn relative_likelihood(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relative_likelihood", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_before: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    #[doc =
        "Set the field `window_after`.\nNumber of characters after the finding to consider. Either this or window_before must be specified"]
    pub fn set_window_after(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window_after = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `window_before`.\nNumber of characters before the finding to consider. Either this or window_after must be specified"]
    pub fn set_window_before(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window_before = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    type O =
        terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
            window_after: core::default::Default::default(),
            window_before: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `window_after` after provisioning.\nNumber of characters after the finding to consider. Either this or window_before must be specified"]
    pub fn window_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `window_before` after provisioning.\nNumber of characters before the finding to consider. Either this or window_after must be specified"]
    pub fn window_before(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_before", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRuleDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>,
    >,
    likelihood_adjustment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment>,
    >,
    proximity: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood_adjustment: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRuleDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule {
    #[doc = "Set the field `hotword_regex`.\n"]
    pub fn set_hotword_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `likelihood_adjustment`.\n"]
    pub fn set_likelihood_adjustment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.likelihood_adjustment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.likelihood_adjustment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proximity`.\n"]
    pub fn set_proximity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_Proximity,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proximity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proximity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule {
            hotword_regex: core::default::Default::default(),
            likelihood_adjustment: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRuleRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `likelihood_adjustment` after provisioning.\n"]
    pub fn likelihood_adjustment(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.likelihood_adjustment", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_RulesDynamic {
    adjustment_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule>>,
    exclusion_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule>>,
    hotword_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adjustment_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSet_RulesDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules {
    #[doc = "Set the field `adjustment_rule`.\n"]
    pub fn set_adjustment_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRule>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.adjustment_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.adjustment_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclusion_rule`.\n"]
    pub fn set_exclusion_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRule>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hotword_rule`.\n"]
    pub fn set_hotword_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRule>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules {
            adjustment_rule: core::default::Default::default(),
            exclusion_rule: core::default::Default::default(),
            hotword_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSet_RulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet_RulesRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adjustment_rule` after provisioning.\n"]
    pub fn adjustment_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_AdjustmentRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.adjustment_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusion_rule` after provisioning.\n"]
    pub fn exclusion_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_ExclusionRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `hotword_rule` after provisioning.\n"]
    pub fn hotword_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules_HotwordRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfig_RuleSetDynamic {
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes>>,
    rules: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfig_RuleSetDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSet {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet_Rules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig_RuleSet {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet {}

impl BuildDataLossPreventionContentPolicy_InspectConfig_RuleSet {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig_RuleSet {
        DataLossPreventionContentPolicy_InspectConfig_RuleSet {
            info_types: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfig_RuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfig_RuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfig_RuleSetRef {
        DataLossPreventionContentPolicy_InspectConfig_RuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfig_RuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_InfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSet_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_InspectConfigDynamic {
    custom_info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes>>,
    info_types: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_InfoTypes>>,
    limits: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_Limits>>,
    min_likelihood_per_info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType>>,
    rule_set: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_InspectConfig_RuleSet>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_InspectConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_info_types: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_quote: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_info_types: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_InfoTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_Limits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_likelihood_per_info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_set: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_InspectConfig_RuleSet>>,
    dynamic: DataLossPreventionContentPolicy_InspectConfigDynamic,
}

impl DataLossPreventionContentPolicy_InspectConfig {
    #[doc =
        "Set the field `content_options`.\nList of options defining data content to scan. If empty, text, images, and other content will be included. Possible values: [\"CONTENT_TEXT\", \"CONTENT_IMAGE\"]"]
    pub fn set_content_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.content_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_info_types`.\nWhen true, excludes type information of the findings."]
    pub fn set_exclude_info_types(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_info_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_quote`.\nWhen true, a contextual quote from the data that triggered a finding is included in the response."]
    pub fn set_include_quote(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_quote = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_likelihood`.\nOnly returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info Default value: \"POSSIBLE\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_min_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_likelihood = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_info_types`.\n"]
    pub fn set_custom_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_InfoTypes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_Limits>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `min_likelihood_per_info_type`.\n"]
    pub fn set_min_likelihood_per_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoType>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.min_likelihood_per_info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.min_likelihood_per_info_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_set`.\n"]
    pub fn set_rule_set(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig_RuleSet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_InspectConfig {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_InspectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_InspectConfig {}

impl BuildDataLossPreventionContentPolicy_InspectConfig {
    pub fn build(self) -> DataLossPreventionContentPolicy_InspectConfig {
        DataLossPreventionContentPolicy_InspectConfig {
            content_options: core::default::Default::default(),
            exclude_info_types: core::default::Default::default(),
            include_quote: core::default::Default::default(),
            min_likelihood: core::default::Default::default(),
            custom_info_types: core::default::Default::default(),
            info_types: core::default::Default::default(),
            limits: core::default::Default::default(),
            min_likelihood_per_info_type: core::default::Default::default(),
            rule_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_InspectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_InspectConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_InspectConfigRef {
        DataLossPreventionContentPolicy_InspectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_InspectConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content_options` after provisioning.\nList of options defining data content to scan. If empty, text, images, and other content will be included. Possible values: [\"CONTENT_TEXT\", \"CONTENT_IMAGE\"]"]
    pub fn content_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_info_types` after provisioning.\nWhen true, excludes type information of the findings."]
    pub fn exclude_info_types(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_info_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_quote` after provisioning.\nWhen true, a contextual quote from the data that triggered a finding is included in the response."]
    pub fn include_quote(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_quote", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_likelihood` after provisioning.\nOnly returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info Default value: \"POSSIBLE\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn min_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_likelihood", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_info_types` after provisioning.\n"]
    pub fn custom_info_types(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_CustomInfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_InfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_LimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `min_likelihood_per_info_type` after provisioning.\n"]
    pub fn min_likelihood_per_info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_MinLikelihoodPerInfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.min_likelihood_per_info_type", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_set` after provisioning.\n"]
    pub fn rule_set(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_InspectConfig_RuleSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_set", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    table_id: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery { }

impl terrars::ToListMappable for DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery {
    #[doc = "The dataset ID of the BigQuery table to log to."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The project ID of the BigQuery table to log to."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc = "The table ID of the BigQuery table to log to."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery {
    pub fn build(self) -> DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery {
        DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQueryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQueryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQueryRef {
        DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQueryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQueryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset ID of the BigQuery table to log to."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe project ID of the BigQuery table to log to."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_id` after provisioning.\nThe table ID of the BigQuery table to log to."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_LoggingConfigsDynamic {
    log_to_big_query: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_LoggingConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_to_big_query: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery>>,
    dynamic: DataLossPreventionContentPolicy_LoggingConfigsDynamic,
}

impl DataLossPreventionContentPolicy_LoggingConfigs {
    #[doc = "Set the field `log_to_big_query`.\n"]
    pub fn set_log_to_big_query(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQuery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_to_big_query = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_to_big_query = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_LoggingConfigs {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_LoggingConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_LoggingConfigs {}

impl BuildDataLossPreventionContentPolicy_LoggingConfigs {
    pub fn build(self) -> DataLossPreventionContentPolicy_LoggingConfigs {
        DataLossPreventionContentPolicy_LoggingConfigs {
            log_to_big_query: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_LoggingConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_LoggingConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_LoggingConfigsRef {
        DataLossPreventionContentPolicy_LoggingConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_LoggingConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_to_big_query` after provisioning.\n"]
    pub fn log_to_big_query(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_LoggingConfigs_LogToBigQueryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_to_big_query", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_Rules_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_verdict: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionContentPolicy_Rules_Action {
    #[doc =
        "Set the field `return_verdict`.\nIf set, the verdict will be returned to the user.\nPossible values: [\"ALLOW\", \"BLOCK\"] Possible values: [\"ALLOW\", \"BLOCK\"]"]
    pub fn set_return_verdict(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.return_verdict = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_Rules_Action {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_Rules_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_Rules_Action {}

impl BuildDataLossPreventionContentPolicy_Rules_Action {
    pub fn build(self) -> DataLossPreventionContentPolicy_Rules_Action {
        DataLossPreventionContentPolicy_Rules_Action { return_verdict: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_Rules_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_Rules_ActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_Rules_ActionRef {
        DataLossPreventionContentPolicy_Rules_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_Rules_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_verdict` after provisioning.\nIf set, the verdict will be returned to the user.\nPossible values: [\"ALLOW\", \"BLOCK\"] Possible values: [\"ALLOW\", \"BLOCK\"]"]
    pub fn return_verdict(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_verdict", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType {}

impl DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType { }

impl terrars::ToListMappable for DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType {
    type O =
        terrars::BlockAssignable<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType {}

impl BuildDataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType {
    pub fn build(self) -> DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType {
        DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoTypeRef {
        DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes {
    info_type_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes { }

impl terrars::ToListMappable for DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes {
    #[doc = "List of info type names."]
    pub info_type_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes {
    pub fn build(self) -> DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes {
        DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes {
            info_type_names: self.info_type_names,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypesRef {
        DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_type_names` after provisioning.\nList of info type names."]
    pub fn info_type_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type_names", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeConditionDynamic {
    any_info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType>>,
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes>>,
    dynamic: DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeConditionDynamic,
}

impl DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition {
    #[doc =
        "Set the field `min_count`.\nThe minimum number of findings required for this condition to be met. Defaults to 1."]
    pub fn set_min_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `any_info_type`.\n"]
    pub fn set_any_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoType,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.any_info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.any_info_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition {}

impl BuildDataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition {
    pub fn build(self) -> DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition {
        DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition {
            min_count: core::default::Default::default(),
            any_info_type: core::default::Default::default(),
            info_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeConditionRef {
        DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `min_count` after provisioning.\nThe minimum number of findings required for this condition to be met. Defaults to 1."]
    pub fn min_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_count", self.base))
    }

    #[doc = "Get a reference to the value of field `any_info_type` after provisioning.\n"]
    pub fn any_info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_AnyInfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.any_info_type", self.base))
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition_InfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_Rules_ConditionsDynamic {
    info_type_condition: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_Rules_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type_condition: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition>>,
    dynamic: DataLossPreventionContentPolicy_Rules_ConditionsDynamic,
}

impl DataLossPreventionContentPolicy_Rules_Conditions {
    #[doc = "Set the field `info_type_condition`.\n"]
    pub fn set_info_type_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeCondition>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_type_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_type_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_Rules_Conditions {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_Rules_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_Rules_Conditions {}

impl BuildDataLossPreventionContentPolicy_Rules_Conditions {
    pub fn build(self) -> DataLossPreventionContentPolicy_Rules_Conditions {
        DataLossPreventionContentPolicy_Rules_Conditions {
            info_type_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_Rules_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_Rules_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_Rules_ConditionsRef {
        DataLossPreventionContentPolicy_Rules_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_Rules_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_type_condition` after provisioning.\n"]
    pub fn info_type_condition(
        &self,
    ) -> terrars::ListRef<DataLossPreventionContentPolicy_Rules_Conditions_InfoTypeConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionContentPolicy_RulesDynamic {
    action: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_Rules_Action>>,
    conditions: std::option::Option<terrars::DynamicBlock<DataLossPreventionContentPolicy_Rules_Conditions>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_Rules_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<DataLossPreventionContentPolicy_Rules_Conditions>>,
    dynamic: DataLossPreventionContentPolicy_RulesDynamic,
}

impl DataLossPreventionContentPolicy_Rules {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_Rules_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionContentPolicy_Rules_Conditions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_Rules {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_Rules {}

impl BuildDataLossPreventionContentPolicy_Rules {
    pub fn build(self) -> DataLossPreventionContentPolicy_Rules {
        DataLossPreventionContentPolicy_Rules {
            action: core::default::Default::default(),
            conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionContentPolicy_RulesRef {
        DataLossPreventionContentPolicy_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_Rules_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<DataLossPreventionContentPolicy_Rules_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionContentPolicy_Timeouts {
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

impl terrars::ToListMappable for DataLossPreventionContentPolicy_Timeouts {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_Timeouts {}

impl BuildDataLossPreventionContentPolicy_Timeouts {
    pub fn build(self) -> DataLossPreventionContentPolicy_Timeouts {
        DataLossPreventionContentPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionContentPolicy_TimeoutsRef {
        DataLossPreventionContentPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_TimeoutsRef {
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

#[derive(Serialize)]
pub struct DataLossPreventionContentPolicy_UnsupportedFileType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_verdict: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionContentPolicy_UnsupportedFileType {
    #[doc =
        "Set the field `return_verdict`.\nIf set, the verdict will be returned to the user.\nPossible values: [\"ALLOW\", \"BLOCK\"] Possible values: [\"ALLOW\", \"BLOCK\"]"]
    pub fn set_return_verdict(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.return_verdict = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionContentPolicy_UnsupportedFileType {
    type O = terrars::BlockAssignable<DataLossPreventionContentPolicy_UnsupportedFileType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionContentPolicy_UnsupportedFileType {}

impl BuildDataLossPreventionContentPolicy_UnsupportedFileType {
    pub fn build(self) -> DataLossPreventionContentPolicy_UnsupportedFileType {
        DataLossPreventionContentPolicy_UnsupportedFileType { return_verdict: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionContentPolicy_UnsupportedFileTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionContentPolicy_UnsupportedFileTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionContentPolicy_UnsupportedFileTypeRef {
        DataLossPreventionContentPolicy_UnsupportedFileTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionContentPolicy_UnsupportedFileTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `return_verdict` after provisioning.\nIf set, the verdict will be returned to the user.\nPossible values: [\"ALLOW\", \"BLOCK\"] Possible values: [\"ALLOW\", \"BLOCK\"]"]
    pub fn return_verdict(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_verdict", self.base))
    }
}
