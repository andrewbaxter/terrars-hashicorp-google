#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GeminiDataAnalyticsDataAgentData_ {
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
    data_agent_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_analytics_agent: std::option::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GeminiDataAnalyticsDataAgent_Timeouts>,
    dynamic: GeminiDataAnalyticsDataAgentDynamic,
}

struct GeminiDataAnalyticsDataAgent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GeminiDataAnalyticsDataAgentData_>,
}

#[derive(Clone)]
pub struct GeminiDataAnalyticsDataAgent(std::rc::Rc<GeminiDataAnalyticsDataAgent_>);

impl GeminiDataAnalyticsDataAgent {
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

    #[doc = "Set the field `description`.\nOptional. Description of the agent. Must be between 1 and 1024 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nOptional. User friendly display name. Must be between 1 and 256 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key`.\nOptional. Customer managed encryption key (CMEK) to use for encrypting the\nDataAgent resources. Cloud KMS CryptoKeys must reside in the same location\nas the DataAgent. The expected format is\nprojects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn set_kms_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. Labels to help users filter related agents.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `data_analytics_agent`.\n"]
    pub fn set_data_analytics_agent(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_analytics_agent = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_analytics_agent = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GeminiDataAnalyticsDataAgent_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the data agent was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_agent_id` after provisioning.\nRequired. The ID to use for the DataAgent, which becomes the final component\nof the resource name. The ID must start with a lowercase letter and may\ncontain lowercase letters, digits, underscores, and hyphens, up to 63\ncharacters in total."]
    pub fn data_agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_agent_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. The time the data agent was soft deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the agent. Must be between 1 and 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. User friendly display name. Must be between 1 and 256 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `kms_key` after provisioning.\nOptional. Customer managed encryption key (CMEK) to use for encrypting the\nDataAgent resources. Cloud KMS CryptoKeys must reside in the same location\nas the DataAgent. The expected format is\nprojects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to help users filter related agents.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the DataAgent."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Identifier. The unique resource name of a DataAgent.\nFormat: projects/{project}/locations/{location}/dataAgents/{dataAgentId}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purge_time` after provisioning.\nOutput only. Timestamp in UTC of when this data agent is considered expired."]
    pub fn purge_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purge_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the data agent was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_analytics_agent` after provisioning.\n"]
    pub fn data_analytics_agent(&self) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_analytics_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GeminiDataAnalyticsDataAgent_TimeoutsRef {
        GeminiDataAnalyticsDataAgent_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for GeminiDataAnalyticsDataAgent {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GeminiDataAnalyticsDataAgent { }

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent {
    type O = terrars::ListRef<GeminiDataAnalyticsDataAgentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GeminiDataAnalyticsDataAgent_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gemini_data_analytics_data_agent".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent {
    pub tf_id: std::string::String,
    #[doc =
        "Required. The ID to use for the DataAgent, which becomes the final component\nof the resource name. The ID must start with a lowercase letter and may\ncontain lowercase letters, digits, underscores, and hyphens, up to 63\ncharacters in total."]
    pub data_agent_id: terrars::PrimField<std::string::String>,
    #[doc = "The location of the DataAgent."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildGeminiDataAnalyticsDataAgent {
    pub fn build(self, stack: &mut terrars::Stack) -> GeminiDataAnalyticsDataAgent {
        let out = GeminiDataAnalyticsDataAgent(std::rc::Rc::new(GeminiDataAnalyticsDataAgent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GeminiDataAnalyticsDataAgentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_agent_id: self.data_agent_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                data_analytics_agent: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GeminiDataAnalyticsDataAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GeminiDataAnalyticsDataAgentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the data agent was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_agent_id` after provisioning.\nRequired. The ID to use for the DataAgent, which becomes the final component\nof the resource name. The ID must start with a lowercase letter and may\ncontain lowercase letters, digits, underscores, and hyphens, up to 63\ncharacters in total."]
    pub fn data_agent_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_agent_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. The time the data agent was soft deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the agent. Must be between 1 and 1024 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. User friendly display name. Must be between 1 and 256 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `kms_key` after provisioning.\nOptional. Customer managed encryption key (CMEK) to use for encrypting the\nDataAgent resources. Cloud KMS CryptoKeys must reside in the same location\nas the DataAgent. The expected format is\nprojects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to help users filter related agents.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the DataAgent."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Identifier. The unique resource name of a DataAgent.\nFormat: projects/{project}/locations/{location}/dataAgents/{dataAgentId}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purge_time` after provisioning.\nOutput only. Timestamp in UTC of when this data agent is considered expired."]
    pub fn purge_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purge_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the data agent was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_analytics_agent` after provisioning.\n"]
    pub fn data_analytics_agent(&self) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_analytics_agent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GeminiDataAnalyticsDataAgent_TimeoutsRef {
        GeminiDataAnalyticsDataAgent_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct GeminiDataAnalyticsDataAgentDynamic {
    data_analytics_agent: std
    ::option
    ::Option<terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent>>,
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferences {
    #[doc = "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_id`.\n"]
    pub fn set_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferences {
    type O =
        terrars
        ::BlockAssignable<
            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferences,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferences {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferences {
    pub fn build(
        self,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferences {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferences {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferencesRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_id` after provisioning.\n"]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_references: std
    ::option
    ::Option<
        terrars
        ::ListField<
            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferences,
        >,
    >,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq {
    #[doc = "Set the field `table_references`.\n"]
    pub fn set_table_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferences,
                        >,
                    >,
    ) -> Self {
        self.table_references = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq {
    type O =
        terrars
        ::BlockAssignable<
            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq {
    pub fn build(
        self,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq {
            table_references: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_BqRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_BqRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_BqRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_BqRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_BqRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_references` after provisioning.\n"]
    pub fn table_references(
        &self,
    ) -> terrars
    ::ListRef<
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq_TableReferencesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_references", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bq: std
    ::option
    ::Option<
        terrars
        ::ListField<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq>,
    >,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences {
    #[doc = "Set the field `bq`.\n"]
    pub fn set_bq(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_Bq,
                        >,
                    >,
    ) -> Self {
        self.bq = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences {
    type O =
        terrars
        ::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences {
            bq: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferencesRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bq` after provisioning.\n"]
    pub fn bq(
        &self,
    ) -> terrars
    ::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences_BqRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bq", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datasource_references: std
    ::option
    ::Option<
        terrars::ListField<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_instruction: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext {
    #[doc = "Set the field `datasource_references`.\n"]
    pub fn set_datasource_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferences,
                        >,
                    >,
    ) -> Self {
        self.datasource_references = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_instruction`.\n"]
    pub fn set_system_instruction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.system_instruction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext {
    type O = terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext {
            datasource_references: core::default::Default::default(),
            system_instruction: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContextRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `datasource_references` after provisioning.\n"]
    pub fn datasource_references(
        &self,
    ) -> terrars
    ::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContext_DatasourceReferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasource_references", self.base))
    }

    #[doc = "Get a reference to the value of field `system_instruction` after provisioning.\n"]
    pub fn system_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_instruction", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    table_id: terrars::PrimField<std::string::String>,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences { }

impl terrars
::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences {
    type O =
        terrars
        ::BlockAssignable<
            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences {
    #[doc = "Required. The dataset that the table belongs to."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "Required. The project that the table belongs to."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc = "Required. The table ID."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences {
    pub fn build(
        self,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferencesRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nRequired. The dataset that the table belongs to."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nRequired. The project that the table belongs to."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nRequired. The table ID."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_BqDynamic {
    table_references: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences,
        >,
    >,
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_references: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences>,
    >,
    dynamic: GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_BqDynamic,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq {
    #[doc = "Set the field `table_references`.\n"]
    pub fn set_table_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferences,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table_references = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table_references = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq {
    type O =
        terrars
        ::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq {
            table_references: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_BqRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_BqRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_BqRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_BqRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_BqRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table_references` after provisioning.\n"]
    pub fn table_references(
        &self,
    ) -> terrars
    ::ListRef<
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq_TableReferencesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_references", self.base))
    }
}

#[derive(Serialize, Default)]
struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferencesDynamic {
    bq: std
    ::option
    ::Option<
        terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq>,
    >,
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bq: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq>>,
    dynamic: GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferencesDynamic,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences {
    #[doc = "Set the field `bq`.\n"]
    pub fn set_bq(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_Bq,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bq = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bq = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences {
    type O =
        terrars
        ::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences {
            bq: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferencesRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bq` after provisioning.\n"]
    pub fn bq(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences_BqRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bq", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    natural_language_question: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_query: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries {
    #[doc = "Set the field `natural_language_question`.\nOptional. A natural language question that a user might ask."]
    pub fn set_natural_language_question(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.natural_language_question = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sql_query`.\nOptional. The SQL query that should be generated to answer\nthe natural language question."]
    pub fn set_sql_query(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sql_query = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries {
    type O = terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries {
            natural_language_question: core::default::Default::default(),
            sql_query: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueriesRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `natural_language_question` after provisioning.\nOptional. A natural language question that a user might ask."]
    pub fn natural_language_question(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.natural_language_question", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sql_query` after provisioning.\nOptional. The SQL query that should be generated to answer\nthe natural language question."]
    pub fn sql_query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_query", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms {
    description: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms {
    #[doc = "Set the field `labels`.\nOptional. A list of general purpose labels associated with this term."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms {
    type O = terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms {
    #[doc = "Required. The description or meaning of the term."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "Required. User friendly display name of the glossary term."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms {
            description: self.description,
            display_name: self.display_name,
            labels: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTermsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTermsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTermsRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTermsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTermsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nRequired. The description or meaning of the term."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. User friendly display name of the glossary term."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. A list of general purpose labels associated with this term."]
    pub fn labels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python {
    #[doc = "Set the field `enabled`.\nOptional. Whether to enable Python analysis. Defaults to false."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python {
    type O =
        terrars
        ::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_PythonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_PythonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_PythonRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_PythonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_PythonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nOptional. Whether to enable Python analysis. Defaults to false."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_AnalysisDynamic {
    python: std
    ::option
    ::Option<
        terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python>,
    >,
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python>>,
    dynamic: GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_AnalysisDynamic,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis {
    #[doc = "Set the field `python`.\n"]
    pub fn set_python(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_Python,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.python = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.python = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis {
    type O =
        terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis {
            python: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_AnalysisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_AnalysisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_AnalysisRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_AnalysisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_AnalysisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `python` after provisioning.\n"]
    pub fn python(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis_PythonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    big_query_max_billed_bytes: std::option::Option<terrars::PrimField<f64>>,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource {
    #[doc =
        "Set the field `big_query_max_billed_bytes`.\nOptional. Limits the bytes billed for each BigQuery query job."]
    pub fn set_big_query_max_billed_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.big_query_max_billed_bytes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource {
    type O =
        terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource {
            big_query_max_billed_bytes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_DatasourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_DatasourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_DatasourceRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_DatasourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_DatasourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `big_query_max_billed_bytes` after provisioning.\nOptional. Limits the bytes billed for each BigQuery query job."]
    pub fn big_query_max_billed_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.big_query_max_billed_bytes", self.base))
    }
}

#[derive(Serialize, Default)]
struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_OptionsDynamic {
    analysis: std
    ::option
    ::Option<terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis>>,
    datasource: std
    ::option
    ::Option<
        terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource>,
    >,
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datasource: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource>>,
    dynamic: GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_OptionsDynamic,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options {
    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Analysis,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `datasource`.\n"]
    pub fn set_datasource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_Datasource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datasource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datasource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options {
    type O = terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options {
            analysis: core::default::Default::default(),
            datasource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_OptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_OptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_OptionsRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_OptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_OptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_AnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }

    #[doc = "Get a reference to the value of field `datasource` after provisioning.\n"]
    pub fn datasource(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options_DatasourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasource", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths {
    paths: terrars::ListField<terrars::PrimField<std::string::String>>,
    table_fqn: terrars::PrimField<std::string::String>,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths { }

impl terrars
::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths {
    type O =
        terrars
        ::BlockAssignable<
            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths {
    #[doc = "Required. The ordered list of paths within the table schema."]
    pub paths: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Required. The service-qualified full resource name of the table."]
    pub table_fqn: terrars::PrimField<std::string::String>,
}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths {
    pub fn build(
        self,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths {
            paths: self.paths,
            table_fqn: self.table_fqn,
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPathsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPathsRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPathsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPathsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `paths` after provisioning.\nRequired. The ordered list of paths within the table schema."]
    pub fn paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_fqn` after provisioning.\nRequired. The service-qualified full resource name of the table."]
    pub fn table_fqn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_fqn", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths {
    paths: terrars::ListField<terrars::PrimField<std::string::String>>,
    table_fqn: terrars::PrimField<std::string::String>,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths { }

impl terrars
::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths {
    type O =
        terrars
        ::BlockAssignable<
            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths {
    #[doc = "Required. The ordered list of paths within the table schema."]
    pub paths: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "Required. The service-qualified full resource name of the table."]
    pub table_fqn: terrars::PrimField<std::string::String>,
}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths {
    pub fn build(
        self,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths {
            paths: self.paths,
            table_fqn: self.table_fqn,
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPathsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPathsRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPathsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPathsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `paths` after provisioning.\nRequired. The ordered list of paths within the table schema."]
    pub fn paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_fqn` after provisioning.\nRequired. The service-qualified full resource name of the table."]
    pub fn table_fqn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_fqn", self.base))
    }
}

#[derive(Serialize, Default)]
struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationshipsDynamic {
    left_schema_paths: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths,
        >,
    >,
    right_schema_paths: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths,
        >,
    >,
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence_score: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sources: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    left_schema_paths: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    right_schema_paths: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths>,
    >,
    dynamic: GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationshipsDynamic,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships {
    #[doc = "Set the field `confidence_score`.\nOptional. A confidence score for the suggested relationship."]
    pub fn set_confidence_score(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.confidence_score = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sources`.\nOptional. Sources which generated the schema relation edge.\nPossible values: BIGQUERY_JOB_HISTORY, LLM_SUGGESTED, BIGQUERY_TABLE_CONSTRAINTS."]
    pub fn set_sources(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.sources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `left_schema_paths`.\n"]
    pub fn set_left_schema_paths(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPaths,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.left_schema_paths = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.left_schema_paths = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `right_schema_paths`.\n"]
    pub fn set_right_schema_paths(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPaths,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.right_schema_paths = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.right_schema_paths = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships {
    type O =
        terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships {
            confidence_score: core::default::Default::default(),
            sources: core::default::Default::default(),
            left_schema_paths: core::default::Default::default(),
            right_schema_paths: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationshipsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationshipsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationshipsRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationshipsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationshipsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidence_score` after provisioning.\nOptional. A confidence score for the suggested relationship."]
    pub fn confidence_score(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence_score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sources` after provisioning.\nOptional. Sources which generated the schema relation edge.\nPossible values: BIGQUERY_JOB_HISTORY, LLM_SUGGESTED, BIGQUERY_TABLE_CONSTRAINTS."]
    pub fn sources(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }

    #[doc = "Get a reference to the value of field `left_schema_paths` after provisioning.\n"]
    pub fn left_schema_paths(
        &self,
    ) -> terrars
    ::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_LeftSchemaPathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.left_schema_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `right_schema_paths` after provisioning.\n"]
    pub fn right_schema_paths(
        &self,
    ) -> terrars
    ::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships_RightSchemaPathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.right_schema_paths", self.base))
    }
}

#[derive(Serialize, Default)]
struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContextDynamic {
    datasource_references: std
    ::option
    ::Option<
        terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences>,
    >,
    example_queries: std
    ::option
    ::Option<terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries>>,
    glossary_terms: std
    ::option
    ::Option<terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms>>,
    options: std
    ::option
    ::Option<terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options>>,
    schema_relationships: std
    ::option
    ::Option<
        terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships>,
    >,
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datasource_references: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    example_queries: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glossary_terms: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_relationships: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships>>,
    dynamic: GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContextDynamic,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext {
    #[doc =
        "Set the field `system_instruction`.\nOptional. The basic entry point for data owners creating domain\nknowledge for the agent."]
    pub fn set_system_instruction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.system_instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datasource_references`.\n"]
    pub fn set_datasource_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datasource_references = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datasource_references = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `example_queries`.\n"]
    pub fn set_example_queries(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.example_queries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.example_queries = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `glossary_terms`.\n"]
    pub fn set_glossary_terms(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.glossary_terms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.glossary_terms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_relationships`.\n"]
    pub fn set_schema_relationships(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_relationships = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_relationships = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext {
    type O = terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext {
            system_instruction: core::default::Default::default(),
            datasource_references: core::default::Default::default(),
            example_queries: core::default::Default::default(),
            glossary_terms: core::default::Default::default(),
            options: core::default::Default::default(),
            schema_relationships: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContextRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `system_instruction` after provisioning.\nOptional. The basic entry point for data owners creating domain\nknowledge for the agent."]
    pub fn system_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `datasource_references` after provisioning.\n"]
    pub fn datasource_references(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasource_references", self.base))
    }

    #[doc = "Get a reference to the value of field `example_queries` after provisioning.\n"]
    pub fn example_queries(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.example_queries", self.base))
    }

    #[doc = "Get a reference to the value of field `glossary_terms` after provisioning.\n"]
    pub fn glossary_terms(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTermsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glossary_terms", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_OptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_relationships` after provisioning.\n"]
    pub fn schema_relationships(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationshipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_relationships", self.base))
    }
}

#[derive(Serialize, Default)]
struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContextDynamic {
    datasource_references: std
    ::option
    ::Option<
        terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences>,
    >,
    example_queries: std
    ::option
    ::Option<terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries>>,
    glossary_terms: std
    ::option
    ::Option<terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms>>,
    options: std
    ::option
    ::Option<terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options>>,
    schema_relationships: std
    ::option
    ::Option<
        terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships>,
    >,
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datasource_references: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    example_queries: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glossary_terms: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_relationships: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships>>,
    dynamic: GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContextDynamic,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext {
    #[doc =
        "Set the field `system_instruction`.\nOptional. The basic entry point for data owners creating domain\nknowledge for the agent."]
    pub fn set_system_instruction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.system_instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datasource_references`.\n"]
    pub fn set_datasource_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferences,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.datasource_references = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.datasource_references = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `example_queries`.\n"]
    pub fn set_example_queries(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueries,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.example_queries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.example_queries = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `glossary_terms`.\n"]
    pub fn set_glossary_terms(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTerms,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.glossary_terms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.glossary_terms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_Options>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_relationships`.\n"]
    pub fn set_schema_relationships(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationships,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_relationships = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_relationships = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext {
    type O = terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext {
            system_instruction: core::default::Default::default(),
            datasource_references: core::default::Default::default(),
            example_queries: core::default::Default::default(),
            glossary_terms: core::default::Default::default(),
            options: core::default::Default::default(),
            schema_relationships: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContextRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `system_instruction` after provisioning.\nOptional. The basic entry point for data owners creating domain\nknowledge for the agent."]
    pub fn system_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.system_instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `datasource_references` after provisioning.\n"]
    pub fn datasource_references(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_DatasourceReferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasource_references", self.base))
    }

    #[doc = "Get a reference to the value of field `example_queries` after provisioning.\n"]
    pub fn example_queries(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_ExampleQueriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.example_queries", self.base))
    }

    #[doc = "Get a reference to the value of field `glossary_terms` after provisioning.\n"]
    pub fn glossary_terms(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_GlossaryTermsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glossary_terms", self.base))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_OptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_relationships` after provisioning.\n"]
    pub fn schema_relationships(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext_SchemaRelationshipsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_relationships", self.base))
    }
}

#[derive(Serialize, Default)]
struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgentDynamic {
    published_context: std
    ::option
    ::Option<terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext>>,
    staging_context: std
    ::option
    ::Option<terrars::DynamicBlock<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext>>,
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    published_context: std
    ::option
    ::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    staging_context: std::option::Option<std::vec::Vec<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext>>,
    dynamic: GeminiDataAnalyticsDataAgent_DataAnalyticsAgentDynamic,
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgent {
    #[doc = "Set the field `published_context`.\n"]
    pub fn set_published_context(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContext>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.published_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.published_context = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `staging_context`.\n"]
    pub fn set_staging_context(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContext>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.staging_context = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.staging_context = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_DataAnalyticsAgent {
    type O = terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent {}

impl BuildGeminiDataAnalyticsDataAgent_DataAnalyticsAgent {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgent {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgent {
            published_context: core::default::Default::default(),
            staging_context: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_DataAnalyticsAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_DataAnalyticsAgentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GeminiDataAnalyticsDataAgent_DataAnalyticsAgentRef {
        GeminiDataAnalyticsDataAgent_DataAnalyticsAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_DataAnalyticsAgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `last_published_context` after provisioning.\nOutput only. The last published context for the agent."]
    pub fn last_published_context(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_LastPublishedContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_published_context", self.base))
    }

    #[doc = "Get a reference to the value of field `published_context` after provisioning.\n"]
    pub fn published_context(
        &self,
    ) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_PublishedContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.published_context", self.base))
    }

    #[doc = "Get a reference to the value of field `staging_context` after provisioning.\n"]
    pub fn staging_context(&self) -> terrars::ListRef<GeminiDataAnalyticsDataAgent_DataAnalyticsAgent_StagingContextRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.staging_context", self.base))
    }
}

#[derive(Serialize)]
pub struct GeminiDataAnalyticsDataAgent_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GeminiDataAnalyticsDataAgent_Timeouts {
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

impl terrars::ToListMappable for GeminiDataAnalyticsDataAgent_Timeouts {
    type O = terrars::BlockAssignable<GeminiDataAnalyticsDataAgent_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGeminiDataAnalyticsDataAgent_Timeouts {}

impl BuildGeminiDataAnalyticsDataAgent_Timeouts {
    pub fn build(self) -> GeminiDataAnalyticsDataAgent_Timeouts {
        GeminiDataAnalyticsDataAgent_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GeminiDataAnalyticsDataAgent_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GeminiDataAnalyticsDataAgent_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GeminiDataAnalyticsDataAgent_TimeoutsRef {
        GeminiDataAnalyticsDataAgent_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GeminiDataAnalyticsDataAgent_TimeoutsRef {
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
