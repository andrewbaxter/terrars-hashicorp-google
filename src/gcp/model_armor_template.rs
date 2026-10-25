#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ModelArmorTemplateData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    template_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_config: std::option::Option<std::vec::Vec<ModelArmorTemplate_FilterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_metadata: std::option::Option<std::vec::Vec<ModelArmorTemplate_TemplateMetadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ModelArmorTemplate_Timeouts>,
    dynamic: ModelArmorTemplateDynamic,
}

struct ModelArmorTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ModelArmorTemplateData_>,
}

#[derive(Clone)]
pub struct ModelArmorTemplate(std::rc::Rc<ModelArmorTemplate_>);

impl ModelArmorTemplate {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `filter_config`.\n"]
    pub fn set_filter_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorTemplate_FilterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `template_metadata`.\n"]
    pub fn set_template_metadata(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorTemplate_TemplateMetadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().template_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.template_metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ModelArmorTemplate_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time stamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `template_id` after provisioning.\nId of the requesting object\nIf auto-generating Id server-side, remove this field and\ntemplate_id from the method_signature of Create RPC"]
    pub fn template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate time stamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_config` after provisioning.\n"]
    pub fn filter_config(&self) -> terrars::ListRef<ModelArmorTemplate_FilterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_metadata` after provisioning.\n"]
    pub fn template_metadata(&self) -> terrars::ListRef<ModelArmorTemplate_TemplateMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ModelArmorTemplate_TimeoutsRef {
        ModelArmorTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ModelArmorTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ModelArmorTemplate { }

impl terrars::ToListMappable for ModelArmorTemplate {
    type O = terrars::ListRef<ModelArmorTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ModelArmorTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_model_armor_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildModelArmorTemplate {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Id of the requesting object\nIf auto-generating Id server-side, remove this field and\ntemplate_id from the method_signature of Create RPC"]
    pub template_id: terrars::PrimField<std::string::String>,
}

impl BuildModelArmorTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> ModelArmorTemplate {
        let out = ModelArmorTemplate(std::rc::Rc::new(ModelArmorTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ModelArmorTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                template_id: self.template_id,
                filter_config: core::default::Default::default(),
                template_metadata: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ModelArmorTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ModelArmorTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time stamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `template_id` after provisioning.\nId of the requesting object\nIf auto-generating Id server-side, remove this field and\ntemplate_id from the method_signature of Create RPC"]
    pub fn template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate time stamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_config` after provisioning.\n"]
    pub fn filter_config(&self) -> terrars::ListRef<ModelArmorTemplate_FilterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_metadata` after provisioning.\n"]
    pub fn template_metadata(&self) -> terrars::ListRef<ModelArmorTemplate_TemplateMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ModelArmorTemplate_TimeoutsRef {
        ModelArmorTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ModelArmorTemplateDynamic {
    filter_config: std::option::Option<terrars::DynamicBlock<ModelArmorTemplate_FilterConfig>>,
    template_metadata: std::option::Option<terrars::DynamicBlock<ModelArmorTemplate_TemplateMetadata>>,
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_enforcement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings {
    #[doc =
        "Set the field `filter_enforcement`.\nTells whether the Malicious URI filter is enabled or disabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filter_enforcement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings {
    type O = terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings {}

impl BuildModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings {
    pub fn build(self) -> ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings {
        ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings {
            filter_enforcement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettingsRef {
        ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether the Malicious URI filter is enabled or disabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_enforcement", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_enforcement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings {
    #[doc = "Set the field `confidence_level`.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn set_confidence_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.confidence_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter_enforcement`.\nTells whether Prompt injection and Jailbreak filter is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filter_enforcement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings {
    type O = terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings {}

impl BuildModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings {
    pub fn build(self) -> ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings {
        ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings {
            confidence_level: core::default::Default::default(),
            filter_enforcement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettingsRef {
        ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidence_level` after provisioning.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn confidence_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether Prompt injection and Jailbreak filter is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_enforcement", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence_level: std::option::Option<terrars::PrimField<std::string::String>>,
    filter_type: terrars::PrimField<std::string::String>,
}

impl ModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters {
    #[doc = "Set the field `confidence_level`.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn set_confidence_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.confidence_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters {
    type O = terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters {
    #[doc = "Possible values:\nSEXUALLY_EXPLICIT\nHATE_SPEECH\nHARASSMENT\nDANGEROUS"]
    pub filter_type: terrars::PrimField<std::string::String>,
}

impl BuildModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters {
    pub fn build(self) -> ModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters {
        ModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters {
            confidence_level: core::default::Default::default(),
            filter_type: self.filter_type,
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_FilterConfig_RaiSettings_RaiFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_FilterConfig_RaiSettings_RaiFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorTemplate_FilterConfig_RaiSettings_RaiFiltersRef {
        ModelArmorTemplate_FilterConfig_RaiSettings_RaiFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_FilterConfig_RaiSettings_RaiFiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidence_level` after provisioning.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn confidence_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter_type` after provisioning.\nPossible values:\nSEXUALLY_EXPLICIT\nHATE_SPEECH\nHARASSMENT\nDANGEROUS"]
    pub fn filter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ModelArmorTemplate_FilterConfig_RaiSettingsDynamic {
    rai_filters: std::option::Option<terrars::DynamicBlock<ModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters>>,
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_FilterConfig_RaiSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rai_filters: std::option::Option<std::vec::Vec<ModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters>>,
    dynamic: ModelArmorTemplate_FilterConfig_RaiSettingsDynamic,
}

impl ModelArmorTemplate_FilterConfig_RaiSettings {
    #[doc = "Set the field `rai_filters`.\n"]
    pub fn set_rai_filters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_RaiSettings_RaiFilters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rai_filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rai_filters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ModelArmorTemplate_FilterConfig_RaiSettings {
    type O = terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_RaiSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_FilterConfig_RaiSettings {}

impl BuildModelArmorTemplate_FilterConfig_RaiSettings {
    pub fn build(self) -> ModelArmorTemplate_FilterConfig_RaiSettings {
        ModelArmorTemplate_FilterConfig_RaiSettings {
            rai_filters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_FilterConfig_RaiSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_FilterConfig_RaiSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorTemplate_FilterConfig_RaiSettingsRef {
        ModelArmorTemplate_FilterConfig_RaiSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_FilterConfig_RaiSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rai_filters` after provisioning.\n"]
    pub fn rai_filters(&self) -> terrars::ListRef<ModelArmorTemplate_FilterConfig_RaiSettings_RaiFiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rai_filters", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deidentify_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig {
    #[doc =
        "Set the field `deidentify_template`.\nOptional Sensitive Data Protection Deidentify template resource name.\nIf provided then DeidentifyContent action is performed during Sanitization\nusing this template and inspect template. The De-identified data will\nbe returned in SdpDeidentifyResult.\nNote that all info-types present in the deidentify template must be present\nin inspect template.\ne.g.\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn set_deidentify_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deidentify_template = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `inspect_template`.\nSensitive Data Protection inspect template resource name\nIf only inspect template is provided (de-identify template not provided),\nthen Sensitive Data Protection InspectContent action is performed during\nSanitization. All Sensitive Data Protection findings identified during\ninspection will be returned as SdpFinding in SdpInsepctionResult.\ne.g:-\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn set_inspect_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inspect_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig {
    type O = terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig {}

impl BuildModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig {
    pub fn build(self) -> ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig {
        ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig {
            deidentify_template: core::default::Default::default(),
            inspect_template: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfigRef {
        ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deidentify_template` after provisioning.\nOptional Sensitive Data Protection Deidentify template resource name.\nIf provided then DeidentifyContent action is performed during Sanitization\nusing this template and inspect template. The De-identified data will\nbe returned in SdpDeidentifyResult.\nNote that all info-types present in the deidentify template must be present\nin inspect template.\ne.g.\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn deidentify_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deidentify_template", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inspect_template` after provisioning.\nSensitive Data Protection inspect template resource name\nIf only inspect template is provided (de-identify template not provided),\nthen Sensitive Data Protection InspectContent action is performed during\nSanitization. All Sensitive Data Protection findings identified during\ninspection will be returned as SdpFinding in SdpInsepctionResult.\ne.g:-\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn inspect_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_template", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_enforcement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig {
    #[doc =
        "Set the field `filter_enforcement`.\nTells whether the Sensitive Data Protection basic config is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filter_enforcement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig {
    type O = terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig {}

impl BuildModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig {
    pub fn build(self) -> ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig {
        ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig {
            filter_enforcement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfigRef {
        ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether the Sensitive Data Protection basic config is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_enforcement", self.base))
    }
}

#[derive(Serialize, Default)]
struct ModelArmorTemplate_FilterConfig_SdpSettingsDynamic {
    advanced_config: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig>>,
    basic_config: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig>>,
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_FilterConfig_SdpSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_config: std::option::Option<std::vec::Vec<ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_config: std::option::Option<std::vec::Vec<ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig>>,
    dynamic: ModelArmorTemplate_FilterConfig_SdpSettingsDynamic,
}

impl ModelArmorTemplate_FilterConfig_SdpSettings {
    #[doc = "Set the field `advanced_config`.\n"]
    pub fn set_advanced_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `basic_config`.\n"]
    pub fn set_basic_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ModelArmorTemplate_FilterConfig_SdpSettings {
    type O = terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_SdpSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_FilterConfig_SdpSettings {}

impl BuildModelArmorTemplate_FilterConfig_SdpSettings {
    pub fn build(self) -> ModelArmorTemplate_FilterConfig_SdpSettings {
        ModelArmorTemplate_FilterConfig_SdpSettings {
            advanced_config: core::default::Default::default(),
            basic_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_FilterConfig_SdpSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_FilterConfig_SdpSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorTemplate_FilterConfig_SdpSettingsRef {
        ModelArmorTemplate_FilterConfig_SdpSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_FilterConfig_SdpSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_config` after provisioning.\n"]
    pub fn advanced_config(&self) -> terrars::ListRef<ModelArmorTemplate_FilterConfig_SdpSettings_AdvancedConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_config", self.base))
    }

    #[doc = "Get a reference to the value of field `basic_config` after provisioning.\n"]
    pub fn basic_config(&self) -> terrars::ListRef<ModelArmorTemplate_FilterConfig_SdpSettings_BasicConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ModelArmorTemplate_FilterConfigDynamic {
    malicious_uri_filter_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings>>,
    pi_and_jailbreak_filter_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings>>,
    rai_settings: std::option::Option<terrars::DynamicBlock<ModelArmorTemplate_FilterConfig_RaiSettings>>,
    sdp_settings: std::option::Option<terrars::DynamicBlock<ModelArmorTemplate_FilterConfig_SdpSettings>>,
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_FilterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malicious_uri_filter_settings: std
    ::option
    ::Option<std::vec::Vec<ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pi_and_jailbreak_filter_settings: std
    ::option
    ::Option<std::vec::Vec<ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rai_settings: std::option::Option<std::vec::Vec<ModelArmorTemplate_FilterConfig_RaiSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sdp_settings: std::option::Option<std::vec::Vec<ModelArmorTemplate_FilterConfig_SdpSettings>>,
    dynamic: ModelArmorTemplate_FilterConfigDynamic,
}

impl ModelArmorTemplate_FilterConfig {
    #[doc = "Set the field `malicious_uri_filter_settings`.\n"]
    pub fn set_malicious_uri_filter_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malicious_uri_filter_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malicious_uri_filter_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pi_and_jailbreak_filter_settings`.\n"]
    pub fn set_pi_and_jailbreak_filter_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pi_and_jailbreak_filter_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pi_and_jailbreak_filter_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rai_settings`.\n"]
    pub fn set_rai_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_RaiSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rai_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rai_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sdp_settings`.\n"]
    pub fn set_sdp_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ModelArmorTemplate_FilterConfig_SdpSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sdp_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sdp_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ModelArmorTemplate_FilterConfig {
    type O = terrars::BlockAssignable<ModelArmorTemplate_FilterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_FilterConfig {}

impl BuildModelArmorTemplate_FilterConfig {
    pub fn build(self) -> ModelArmorTemplate_FilterConfig {
        ModelArmorTemplate_FilterConfig {
            malicious_uri_filter_settings: core::default::Default::default(),
            pi_and_jailbreak_filter_settings: core::default::Default::default(),
            rai_settings: core::default::Default::default(),
            sdp_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_FilterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_FilterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ModelArmorTemplate_FilterConfigRef {
        ModelArmorTemplate_FilterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_FilterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `malicious_uri_filter_settings` after provisioning.\n"]
    pub fn malicious_uri_filter_settings(
        &self,
    ) -> terrars::ListRef<ModelArmorTemplate_FilterConfig_MaliciousUriFilterSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.malicious_uri_filter_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `pi_and_jailbreak_filter_settings` after provisioning.\n"]
    pub fn pi_and_jailbreak_filter_settings(
        &self,
    ) -> terrars::ListRef<ModelArmorTemplate_FilterConfig_PiAndJailbreakFilterSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pi_and_jailbreak_filter_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `rai_settings` after provisioning.\n"]
    pub fn rai_settings(&self) -> terrars::ListRef<ModelArmorTemplate_FilterConfig_RaiSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rai_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `sdp_settings` after provisioning.\n"]
    pub fn sdp_settings(&self) -> terrars::ListRef<ModelArmorTemplate_FilterConfig_SdpSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sdp_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_TemplateMetadata_FilterVersionSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorTemplate_TemplateMetadata_FilterVersionSelector {
    #[doc =
        "Set the field `alias`.\nA predefined filter version alias. The template automatically follows the\nversion this alias points to.\nPossible values:\nFILTER_VERSION_ALIAS_STABLE\nFILTER_VERSION_ALIAS_LATEST"]
    pub fn set_alias(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alias = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nPins the template to a specific, immutable filter version. Expected\nformat is a case-sensitive string such as 'v1' or 'v2'."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ModelArmorTemplate_TemplateMetadata_FilterVersionSelector {
    type O = terrars::BlockAssignable<ModelArmorTemplate_TemplateMetadata_FilterVersionSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_TemplateMetadata_FilterVersionSelector {}

impl BuildModelArmorTemplate_TemplateMetadata_FilterVersionSelector {
    pub fn build(self) -> ModelArmorTemplate_TemplateMetadata_FilterVersionSelector {
        ModelArmorTemplate_TemplateMetadata_FilterVersionSelector {
            alias: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_TemplateMetadata_FilterVersionSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_TemplateMetadata_FilterVersionSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorTemplate_TemplateMetadata_FilterVersionSelectorRef {
        ModelArmorTemplate_TemplateMetadata_FilterVersionSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_TemplateMetadata_FilterVersionSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alias` after provisioning.\nA predefined filter version alias. The template automatically follows the\nversion this alias points to.\nPossible values:\nFILTER_VERSION_ALIAS_STABLE\nFILTER_VERSION_ALIAS_LATEST"]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nPins the template to a specific, immutable filter version. Expected\nformat is a case-sensitive string such as 'v1' or 'v2'."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_TemplateMetadata_MultiLanguageDetection {
    enable_multi_language_detection: terrars::PrimField<bool>,
}

impl ModelArmorTemplate_TemplateMetadata_MultiLanguageDetection { }

impl terrars::ToListMappable for ModelArmorTemplate_TemplateMetadata_MultiLanguageDetection {
    type O = terrars::BlockAssignable<ModelArmorTemplate_TemplateMetadata_MultiLanguageDetection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_TemplateMetadata_MultiLanguageDetection {
    #[doc = "If true, multi language detection will be enabled."]
    pub enable_multi_language_detection: terrars::PrimField<bool>,
}

impl BuildModelArmorTemplate_TemplateMetadata_MultiLanguageDetection {
    pub fn build(self) -> ModelArmorTemplate_TemplateMetadata_MultiLanguageDetection {
        ModelArmorTemplate_TemplateMetadata_MultiLanguageDetection {
            enable_multi_language_detection: self.enable_multi_language_detection,
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_TemplateMetadata_MultiLanguageDetectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_TemplateMetadata_MultiLanguageDetectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ModelArmorTemplate_TemplateMetadata_MultiLanguageDetectionRef {
        ModelArmorTemplate_TemplateMetadata_MultiLanguageDetectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_TemplateMetadata_MultiLanguageDetectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_multi_language_detection` after provisioning.\nIf true, multi language detection will be enabled."]
    pub fn enable_multi_language_detection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_multi_language_detection", self.base))
    }
}

#[derive(Serialize, Default)]
struct ModelArmorTemplate_TemplateMetadataDynamic {
    filter_version_selector: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorTemplate_TemplateMetadata_FilterVersionSelector>>,
    multi_language_detection: std
    ::option
    ::Option<terrars::DynamicBlock<ModelArmorTemplate_TemplateMetadata_MultiLanguageDetection>>,
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_TemplateMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_llm_response_safety_error_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_llm_response_safety_error_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_prompt_safety_error_code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_prompt_safety_error_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforcement_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_partial_invocation_failures: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_sanitize_operations: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_template_operations: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modalities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_version_selector: std
    ::option
    ::Option<std::vec::Vec<ModelArmorTemplate_TemplateMetadata_FilterVersionSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_language_detection: std
    ::option
    ::Option<std::vec::Vec<ModelArmorTemplate_TemplateMetadata_MultiLanguageDetection>>,
    dynamic: ModelArmorTemplate_TemplateMetadataDynamic,
}

impl ModelArmorTemplate_TemplateMetadata {
    #[doc =
        "Set the field `custom_llm_response_safety_error_code`.\nIndicates the custom error code set by the user to be returned to the end\nuser if the LLM response trips Model Armor filters."]
    pub fn set_custom_llm_response_safety_error_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.custom_llm_response_safety_error_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_llm_response_safety_error_message`.\nIndicates the custom error message set by the user to be returned to the\nend user if the LLM response trips Model Armor filters."]
    pub fn set_custom_llm_response_safety_error_message(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_llm_response_safety_error_message = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_prompt_safety_error_code`.\nIndicates the custom error code set by the user to be returned to the end\nuser by the service extension if the prompt trips Model Armor filters."]
    pub fn set_custom_prompt_safety_error_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_prompt_safety_error_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_prompt_safety_error_message`.\nIndicates the custom error message set by the user to be returned to the\nend user if the prompt trips Model Armor filters."]
    pub fn set_custom_prompt_safety_error_message(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_prompt_safety_error_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforcement_type`.\nPossible values:\nINSPECT_ONLY\nINSPECT_AND_BLOCK"]
    pub fn set_enforcement_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enforcement_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_partial_invocation_failures`.\nIf true, partial detector failures should be ignored."]
    pub fn set_ignore_partial_invocation_failures(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.ignore_partial_invocation_failures = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_sanitize_operations`.\nIf true, log sanitize operations."]
    pub fn set_log_sanitize_operations(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_sanitize_operations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_template_operations`.\nIf true, log template crud operations."]
    pub fn set_log_template_operations(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_template_operations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `modalities`.\nSpecifies the modalities to scan. If empty, only text modality will be\nscanned. Image screening is in Preview and is only supported in the 'us'\nand 'eu' multi-regions; regional endpoints such as 'us-central1' do not\nsupport it.\nPossible values:\nMODALITY_TEXT\nMODALITY_IMAGE"]
    pub fn set_modalities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.modalities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_version_selector`.\n"]
    pub fn set_filter_version_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ModelArmorTemplate_TemplateMetadata_FilterVersionSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter_version_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_version_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multi_language_detection`.\n"]
    pub fn set_multi_language_detection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ModelArmorTemplate_TemplateMetadata_MultiLanguageDetection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multi_language_detection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_language_detection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ModelArmorTemplate_TemplateMetadata {
    type O = terrars::BlockAssignable<ModelArmorTemplate_TemplateMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_TemplateMetadata {}

impl BuildModelArmorTemplate_TemplateMetadata {
    pub fn build(self) -> ModelArmorTemplate_TemplateMetadata {
        ModelArmorTemplate_TemplateMetadata {
            custom_llm_response_safety_error_code: core::default::Default::default(),
            custom_llm_response_safety_error_message: core::default::Default::default(),
            custom_prompt_safety_error_code: core::default::Default::default(),
            custom_prompt_safety_error_message: core::default::Default::default(),
            enforcement_type: core::default::Default::default(),
            ignore_partial_invocation_failures: core::default::Default::default(),
            log_sanitize_operations: core::default::Default::default(),
            log_template_operations: core::default::Default::default(),
            modalities: core::default::Default::default(),
            filter_version_selector: core::default::Default::default(),
            multi_language_detection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_TemplateMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_TemplateMetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ModelArmorTemplate_TemplateMetadataRef {
        ModelArmorTemplate_TemplateMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_TemplateMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `custom_llm_response_safety_error_code` after provisioning.\nIndicates the custom error code set by the user to be returned to the end\nuser if the LLM response trips Model Armor filters."]
    pub fn custom_llm_response_safety_error_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_llm_response_safety_error_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_llm_response_safety_error_message` after provisioning.\nIndicates the custom error message set by the user to be returned to the\nend user if the LLM response trips Model Armor filters."]
    pub fn custom_llm_response_safety_error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_llm_response_safety_error_message", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `custom_prompt_safety_error_code` after provisioning.\nIndicates the custom error code set by the user to be returned to the end\nuser by the service extension if the prompt trips Model Armor filters."]
    pub fn custom_prompt_safety_error_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_prompt_safety_error_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_prompt_safety_error_message` after provisioning.\nIndicates the custom error message set by the user to be returned to the\nend user if the prompt trips Model Armor filters."]
    pub fn custom_prompt_safety_error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_prompt_safety_error_message", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforcement_type` after provisioning.\nPossible values:\nINSPECT_ONLY\nINSPECT_AND_BLOCK"]
    pub fn enforcement_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcement_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_partial_invocation_failures` after provisioning.\nIf true, partial detector failures should be ignored."]
    pub fn ignore_partial_invocation_failures(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_partial_invocation_failures", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_sanitize_operations` after provisioning.\nIf true, log sanitize operations."]
    pub fn log_sanitize_operations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_sanitize_operations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_template_operations` after provisioning.\nIf true, log template crud operations."]
    pub fn log_template_operations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_template_operations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `modalities` after provisioning.\nSpecifies the modalities to scan. If empty, only text modality will be\nscanned. Image screening is in Preview and is only supported in the 'us'\nand 'eu' multi-regions; regional endpoints such as 'us-central1' do not\nsupport it.\nPossible values:\nMODALITY_TEXT\nMODALITY_IMAGE"]
    pub fn modalities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.modalities", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_version_selector` after provisioning.\n"]
    pub fn filter_version_selector(
        &self,
    ) -> terrars::ListRef<ModelArmorTemplate_TemplateMetadata_FilterVersionSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_version_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_language_detection` after provisioning.\n"]
    pub fn multi_language_detection(
        &self,
    ) -> terrars::ListRef<ModelArmorTemplate_TemplateMetadata_MultiLanguageDetectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_language_detection", self.base))
    }
}

#[derive(Serialize)]
pub struct ModelArmorTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ModelArmorTemplate_Timeouts {
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

impl terrars::ToListMappable for ModelArmorTemplate_Timeouts {
    type O = terrars::BlockAssignable<ModelArmorTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildModelArmorTemplate_Timeouts {}

impl BuildModelArmorTemplate_Timeouts {
    pub fn build(self) -> ModelArmorTemplate_Timeouts {
        ModelArmorTemplate_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ModelArmorTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ModelArmorTemplate_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ModelArmorTemplate_TimeoutsRef {
        ModelArmorTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ModelArmorTemplate_TimeoutsRef {
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
