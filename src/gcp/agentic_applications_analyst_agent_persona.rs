#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AgenticApplicationsAnalystAgentPersonaData_ {
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
    analyst_agent_persona_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_context: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gemini_enterprise_engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    math_rendering_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_examples: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactExamples>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifacts_config: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_data_sources: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ExternalDataSources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mcp_data_sources: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_McpDataSources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skills: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Skills>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tables: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Tables>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AgenticApplicationsAnalystAgentPersona_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_search_config: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_WebSearchConfig>>,
    dynamic: AgenticApplicationsAnalystAgentPersonaDynamic,
}

struct AgenticApplicationsAnalystAgentPersona_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AgenticApplicationsAnalystAgentPersonaData_>,
}

#[derive(Clone)]
pub struct AgenticApplicationsAnalystAgentPersona(std::rc::Rc<AgenticApplicationsAnalystAgentPersona_>);

impl AgenticApplicationsAnalystAgentPersona {
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

    #[doc = "Set the field `customer_context`.\nThe customer-specific context to be used by the agent."]
    pub fn set_customer_context(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().customer_context = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_description`.\nThe description of the persona, shown to users."]
    pub fn set_display_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gemini_enterprise_engine`.\nThe Gemini Enterprise Engine ID associated with this persona.\nIf set, any requests coming from this GE Engine will be routed to this\npersona.\nIf not set, requests from GE will only be routed to this persona if its\nname ends in \"/default\"."]
    pub fn set_gemini_enterprise_engine(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().gemini_enterprise_engine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `math_rendering_mode`.\nThe math rendering mode selected for this persona.\nPossible values:\nMATH_RENDERING_MODE_LATEX\nMATH_RENDERING_MODE_PLAIN_TEXT"]
    pub fn set_math_rendering_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().math_rendering_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_description`.\nThe description of the persona review, used by the model."]
    pub fn set_model_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().model_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `role`.\nPossible values:\nANALYST_ROLE_GENERIC_FINANCE_ANALYST\nANALYST_ROLE_CORPORATE_FINANCE_ANALYST\nANALYST_ROLE_CROSS_ASSET_DERIVATIVES_STRATEGIST\nANALYST_ROLE_KYC_ANALYST\nANALYST_ROLE_SALES_TRADER\nANALYST_ROLE_QUANT_ANALYST\nANALYST_ROLE_EXCHANGE_MANAGER\nANALYST_ROLE_PORTFOLIO_MANAGER\nANALYST_ROLE_WEALTH_MANAGER\nANALYST_ROLE_INSTITUTIONAL_PORTFOLIO_STRATEGIST\nANALYST_ROLE_MNA_EXECUTION_ANALYST\nANALYST_ROLE_ECM_ORIGINATION_STRATEGIST\nANALYST_ROLE_LEVERAGED_FINANCE_SPECIALIST\nANALYST_ROLE_INVESTMENT_RESEARCH_ANALYST\nANALYST_ROLE_CORPORATE_BANKING_ANALYST\nANALYST_ROLE_CREDIT_RISK_STRATEGIST\nANALYST_ROLE_BEHAVIORAL_FINANCIAL_STRATEGIST\nANALYST_ROLE_FUND_ACCOUNTANT\nANALYST_ROLE_MODEL_VALIDATION_AUDITOR\nANALYST_ROLE_PRIVATE_EQUITY_SPECIALIST\nANALYST_ROLE_TREASURY_ANALYST\nANALYST_ROLE_VENTURE_CAPITAL_ANALYST\nANALYST_ROLE_AML_INVESTIGATOR\nANALYST_ROLE_DUE_DILIGENCE_ANALYST\nANALYST_ROLE_INSURANCE_CLAIMS_ANALYST\nANALYST_ROLE_SPECIALTY_LIABILITY_UNDERWRITER\nANALYST_ROLE_CATASTROPHE_EXPOSURE_MODELER"]
    pub fn set_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `artifact_examples`.\n"]
    pub fn set_artifact_examples(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactExamples>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifact_examples = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifact_examples = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `artifacts_config`.\n"]
    pub fn set_artifacts_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().artifacts_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.artifacts_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `external_data_sources`.\n"]
    pub fn set_external_data_sources(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_data_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_data_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mcp_data_sources`.\n"]
    pub fn set_mcp_data_sources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_McpDataSources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mcp_data_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mcp_data_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `skills`.\n"]
    pub fn set_skills(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Skills>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().skills = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.skills = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Tables>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tables = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AgenticApplicationsAnalystAgentPersona_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_search_config`.\n"]
    pub fn set_web_search_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_WebSearchConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().web_search_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.web_search_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `analyst_agent_persona_id` after provisioning.\nId of the requesting object\nIf auto-generating Id server-side, remove this field and\nanalyst_agent_persona_id from the method_signature of Create RPC"]
    pub fn analyst_agent_persona_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analyst_agent_persona_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time stamp."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_context` after provisioning.\nThe customer-specific context to be used by the agent."]
    pub fn customer_context(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_context", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_description` after provisioning.\nThe description of the persona, shown to users."]
    pub fn display_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the persona, shown to users."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gemini_enterprise_engine` after provisioning.\nThe Gemini Enterprise Engine ID associated with this persona.\nIf set, any requests coming from this GE Engine will be routed to this\npersona.\nIf not set, requests from GE will only be routed to this persona if its\nname ends in \"/default\"."]
    pub fn gemini_enterprise_engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gemini_enterprise_engine", self.extract_ref()))
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
        "Get a reference to the value of field `math_rendering_mode` after provisioning.\nThe math rendering mode selected for this persona.\nPossible values:\nMATH_RENDERING_MODE_LATEX\nMATH_RENDERING_MODE_PLAIN_TEXT"]
    pub fn math_rendering_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.math_rendering_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `model_description` after provisioning.\nThe description of the persona review, used by the model."]
    pub fn model_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the analyst agent persona.\nFormat:\nprojects/{project}/locations/{location}/analystAgentPersonas/{analyst_agent_persona}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nPossible values:\nANALYST_ROLE_GENERIC_FINANCE_ANALYST\nANALYST_ROLE_CORPORATE_FINANCE_ANALYST\nANALYST_ROLE_CROSS_ASSET_DERIVATIVES_STRATEGIST\nANALYST_ROLE_KYC_ANALYST\nANALYST_ROLE_SALES_TRADER\nANALYST_ROLE_QUANT_ANALYST\nANALYST_ROLE_EXCHANGE_MANAGER\nANALYST_ROLE_PORTFOLIO_MANAGER\nANALYST_ROLE_WEALTH_MANAGER\nANALYST_ROLE_INSTITUTIONAL_PORTFOLIO_STRATEGIST\nANALYST_ROLE_MNA_EXECUTION_ANALYST\nANALYST_ROLE_ECM_ORIGINATION_STRATEGIST\nANALYST_ROLE_LEVERAGED_FINANCE_SPECIALIST\nANALYST_ROLE_INVESTMENT_RESEARCH_ANALYST\nANALYST_ROLE_CORPORATE_BANKING_ANALYST\nANALYST_ROLE_CREDIT_RISK_STRATEGIST\nANALYST_ROLE_BEHAVIORAL_FINANCIAL_STRATEGIST\nANALYST_ROLE_FUND_ACCOUNTANT\nANALYST_ROLE_MODEL_VALIDATION_AUDITOR\nANALYST_ROLE_PRIVATE_EQUITY_SPECIALIST\nANALYST_ROLE_TREASURY_ANALYST\nANALYST_ROLE_VENTURE_CAPITAL_ANALYST\nANALYST_ROLE_AML_INVESTIGATOR\nANALYST_ROLE_DUE_DILIGENCE_ANALYST\nANALYST_ROLE_INSURANCE_CLAIMS_ANALYST\nANALYST_ROLE_SPECIALTY_LIABILITY_UNDERWRITER\nANALYST_ROLE_CATASTROPHE_EXPOSURE_MODELER"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate time stamp."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifact_examples` after provisioning.\n"]
    pub fn artifact_examples(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactExamplesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifact_examples", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifacts_config` after provisioning.\n"]
    pub fn artifacts_config(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifacts_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_data_sources` after provisioning.\n"]
    pub fn external_data_sources(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ExternalDataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_data_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mcp_data_sources` after provisioning.\n"]
    pub fn mcp_data_sources(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_McpDataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_data_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skills` after provisioning.\n"]
    pub fn skills(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_SkillsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skills", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_TablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AgenticApplicationsAnalystAgentPersona_TimeoutsRef {
        AgenticApplicationsAnalystAgentPersona_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `web_search_config` after provisioning.\n"]
    pub fn web_search_config(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_WebSearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_search_config", self.extract_ref()))
    }
}

impl terrars::Referable for AgenticApplicationsAnalystAgentPersona {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AgenticApplicationsAnalystAgentPersona { }

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona {
    type O = terrars::ListRef<AgenticApplicationsAnalystAgentPersonaRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AgenticApplicationsAnalystAgentPersona_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_agentic_applications_analyst_agent_persona".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona {
    pub tf_id: std::string::String,
    #[doc =
        "Id of the requesting object\nIf auto-generating Id server-side, remove this field and\nanalyst_agent_persona_id from the method_signature of Create RPC"]
    pub analyst_agent_persona_id: terrars::PrimField<std::string::String>,
    #[doc = "The display name of the persona, shown to users."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildAgenticApplicationsAnalystAgentPersona {
    pub fn build(self, stack: &mut terrars::Stack) -> AgenticApplicationsAnalystAgentPersona {
        let out = AgenticApplicationsAnalystAgentPersona(std::rc::Rc::new(AgenticApplicationsAnalystAgentPersona_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AgenticApplicationsAnalystAgentPersonaData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                analyst_agent_persona_id: self.analyst_agent_persona_id,
                customer_context: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_description: core::default::Default::default(),
                display_name: self.display_name,
                gemini_enterprise_engine: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                math_rendering_mode: core::default::Default::default(),
                model_description: core::default::Default::default(),
                project: core::default::Default::default(),
                role: core::default::Default::default(),
                artifact_examples: core::default::Default::default(),
                artifacts_config: core::default::Default::default(),
                external_data_sources: core::default::Default::default(),
                mcp_data_sources: core::default::Default::default(),
                resources: core::default::Default::default(),
                skills: core::default::Default::default(),
                tables: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                web_search_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AgenticApplicationsAnalystAgentPersonaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersonaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AgenticApplicationsAnalystAgentPersonaRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `analyst_agent_persona_id` after provisioning.\nId of the requesting object\nIf auto-generating Id server-side, remove this field and\nanalyst_agent_persona_id from the method_signature of Create RPC"]
    pub fn analyst_agent_persona_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analyst_agent_persona_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time stamp."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_context` after provisioning.\nThe customer-specific context to be used by the agent."]
    pub fn customer_context(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_context", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_description` after provisioning.\nThe description of the persona, shown to users."]
    pub fn display_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the persona, shown to users."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gemini_enterprise_engine` after provisioning.\nThe Gemini Enterprise Engine ID associated with this persona.\nIf set, any requests coming from this GE Engine will be routed to this\npersona.\nIf not set, requests from GE will only be routed to this persona if its\nname ends in \"/default\"."]
    pub fn gemini_enterprise_engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gemini_enterprise_engine", self.extract_ref()))
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
        "Get a reference to the value of field `math_rendering_mode` after provisioning.\nThe math rendering mode selected for this persona.\nPossible values:\nMATH_RENDERING_MODE_LATEX\nMATH_RENDERING_MODE_PLAIN_TEXT"]
    pub fn math_rendering_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.math_rendering_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `model_description` after provisioning.\nThe description of the persona review, used by the model."]
    pub fn model_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the analyst agent persona.\nFormat:\nprojects/{project}/locations/{location}/analystAgentPersonas/{analyst_agent_persona}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nPossible values:\nANALYST_ROLE_GENERIC_FINANCE_ANALYST\nANALYST_ROLE_CORPORATE_FINANCE_ANALYST\nANALYST_ROLE_CROSS_ASSET_DERIVATIVES_STRATEGIST\nANALYST_ROLE_KYC_ANALYST\nANALYST_ROLE_SALES_TRADER\nANALYST_ROLE_QUANT_ANALYST\nANALYST_ROLE_EXCHANGE_MANAGER\nANALYST_ROLE_PORTFOLIO_MANAGER\nANALYST_ROLE_WEALTH_MANAGER\nANALYST_ROLE_INSTITUTIONAL_PORTFOLIO_STRATEGIST\nANALYST_ROLE_MNA_EXECUTION_ANALYST\nANALYST_ROLE_ECM_ORIGINATION_STRATEGIST\nANALYST_ROLE_LEVERAGED_FINANCE_SPECIALIST\nANALYST_ROLE_INVESTMENT_RESEARCH_ANALYST\nANALYST_ROLE_CORPORATE_BANKING_ANALYST\nANALYST_ROLE_CREDIT_RISK_STRATEGIST\nANALYST_ROLE_BEHAVIORAL_FINANCIAL_STRATEGIST\nANALYST_ROLE_FUND_ACCOUNTANT\nANALYST_ROLE_MODEL_VALIDATION_AUDITOR\nANALYST_ROLE_PRIVATE_EQUITY_SPECIALIST\nANALYST_ROLE_TREASURY_ANALYST\nANALYST_ROLE_VENTURE_CAPITAL_ANALYST\nANALYST_ROLE_AML_INVESTIGATOR\nANALYST_ROLE_DUE_DILIGENCE_ANALYST\nANALYST_ROLE_INSURANCE_CLAIMS_ANALYST\nANALYST_ROLE_SPECIALTY_LIABILITY_UNDERWRITER\nANALYST_ROLE_CATASTROPHE_EXPOSURE_MODELER"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate time stamp."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifact_examples` after provisioning.\n"]
    pub fn artifact_examples(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactExamplesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifact_examples", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `artifacts_config` after provisioning.\n"]
    pub fn artifacts_config(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.artifacts_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_data_sources` after provisioning.\n"]
    pub fn external_data_sources(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ExternalDataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_data_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mcp_data_sources` after provisioning.\n"]
    pub fn mcp_data_sources(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_McpDataSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mcp_data_sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skills` after provisioning.\n"]
    pub fn skills(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_SkillsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.skills", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_TablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AgenticApplicationsAnalystAgentPersona_TimeoutsRef {
        AgenticApplicationsAnalystAgentPersona_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `web_search_config` after provisioning.\n"]
    pub fn web_search_config(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_WebSearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_search_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersonaDynamic {
    artifact_examples: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactExamples>>,
    artifacts_config: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig>>,
    external_data_sources: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ExternalDataSources>>,
    mcp_data_sources: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_McpDataSources>>,
    resources: std::option::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources>>,
    skills: std::option::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Skills>>,
    tables: std::option::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Tables>>,
    web_search_config: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_WebSearchConfig>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_table: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_descriptions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource {
    #[doc =
        "Set the field `bigquery_dataset`.\nPoints to a bigquery dataset to use.\n\nExpected Format:\nprojects/{project_id_or_number}/datasets/{dataset_id}"]
    pub fn set_bigquery_dataset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bigquery_dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bigquery_table`.\nPoints to a bigquery table to use.\n\nExpected Format:\nprojects/{project_id_or_number}/datasets/{dataset_id}/tables/{table_id}"]
    pub fn set_bigquery_table(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bigquery_table = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `column_descriptions`.\nA map of column names to column descriptions for the bigquery_table."]
    pub fn set_column_descriptions(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.column_descriptions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource {}

impl BuildAgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource {
        AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource {
            bigquery_dataset: core::default::Default::default(),
            bigquery_table: core::default::Default::default(),
            column_descriptions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResourceRef {
        AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bigquery_dataset` after provisioning.\nPoints to a bigquery dataset to use.\n\nExpected Format:\nprojects/{project_id_or_number}/datasets/{dataset_id}"]
    pub fn bigquery_dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bigquery_dataset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bigquery_table` after provisioning.\nPoints to a bigquery table to use.\n\nExpected Format:\nprojects/{project_id_or_number}/datasets/{dataset_id}/tables/{table_id}"]
    pub fn bigquery_table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bigquery_table", self.base))
    }

    #[doc =
        "Get a reference to the value of field `column_descriptions` after provisioning.\nA map of column names to column descriptions for the bigquery_table."]
    pub fn column_descriptions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.column_descriptions", self.base))
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources_F1Resource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    f1_table: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgenticApplicationsAnalystAgentPersona_Resources_F1Resource {
    #[doc =
        "Set the field `f1_table`.\n## - Points to an f1 table to use.\n\n- Expected Format:\n- {group}.{table_name}"]
    pub fn set_f1_table(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.f1_table = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Resources_F1Resource {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_F1Resource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Resources_F1Resource {}

impl BuildAgenticApplicationsAnalystAgentPersona_Resources_F1Resource {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Resources_F1Resource {
        AgenticApplicationsAnalystAgentPersona_Resources_F1Resource { f1_table: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources_F1ResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_Resources_F1ResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_Resources_F1ResourceRef {
        AgenticApplicationsAnalystAgentPersona_Resources_F1ResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_Resources_F1ResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `f1_table` after provisioning.\n## - Points to an f1 table to use.\n\n- Expected Format:\n- {group}.{table_name}"]
    pub fn f1_table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.f1_table", self.base))
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_extension_restrictions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    google_cloud_storage_object: terrars::PrimField<std::string::String>,
}

impl AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource {
    #[doc =
        "Set the field `file_extension_restrictions`.\nIf non-empty, only files with these extensions are included when\nexpanding the resource.  If empty, all files are included."]
    pub fn set_file_extension_restrictions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_extension_restrictions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource {
    #[doc =
        "The Google Cloud Storage object or folder.\n\nFormat: /\nor: //\n\nNote that to refer to a folder, it _must_ end in a slash."]
    pub google_cloud_storage_object: terrars::PrimField<std::string::String>,
}

impl BuildAgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource {
        AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource {
            file_extension_restrictions: core::default::Default::default(),
            google_cloud_storage_object: self.google_cloud_storage_object,
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResourceRef {
        AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_extension_restrictions` after provisioning.\nIf non-empty, only files with these extensions are included when\nexpanding the resource.  If empty, all files are included."]
    pub fn file_extension_restrictions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_extension_restrictions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `google_cloud_storage_object` after provisioning.\nThe Google Cloud Storage object or folder.\n\nFormat: /\nor: //\n\nNote that to refer to a folder, it _must_ end in a slash."]
    pub fn google_cloud_storage_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_cloud_storage_object", self.base))
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_extension_restrictions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_reference: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource {
    #[doc =
        "Set the field `file_extension_restrictions`.\nIf non-empty, only files with these extensions are included when\nexpanding the resource.  If empty, all files are included."]
    pub fn set_file_extension_restrictions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_extension_restrictions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_reference`.\nPoints to a drive file to use. May refer to workspace files or folders\nas well.  If folder is specifically, all files in the folder\n(recursively) are used.\n\nExpected Format:\nfiles/{file_id}"]
    pub fn set_file_reference(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_reference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource {}

impl BuildAgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource {
        AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource {
            file_extension_restrictions: core::default::Default::default(),
            file_reference: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResourceRef {
        AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_extension_restrictions` after provisioning.\nIf non-empty, only files with these extensions are included when\nexpanding the resource.  If empty, all files are included."]
    pub fn file_extension_restrictions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_extension_restrictions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_reference` after provisioning.\nPoints to a drive file to use. May refer to workspace files or folders\nas well.  If folder is specifically, all files in the folder\n(recursively) are used.\n\nExpected Format:\nfiles/{file_id}"]
    pub fn file_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_reference", self.base))
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource {
    file_content: terrars::PrimField<std::string::String>,
    file_title: terrars::PrimField<std::string::String>,
    mime_type: terrars::PrimField<std::string::String>,
}

impl AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource { }

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Resources_RawFileResource {
    #[doc = "The raw file content."]
    pub file_content: terrars::PrimField<std::string::String>,
    #[doc = "The title of the file."]
    pub file_title: terrars::PrimField<std::string::String>,
    #[doc = "The mime type of the file."]
    pub mime_type: terrars::PrimField<std::string::String>,
}

impl BuildAgenticApplicationsAnalystAgentPersona_Resources_RawFileResource {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource {
        AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource {
            file_content: self.file_content,
            file_title: self.file_title,
            mime_type: self.mime_type,
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources_RawFileResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_Resources_RawFileResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_Resources_RawFileResourceRef {
        AgenticApplicationsAnalystAgentPersona_Resources_RawFileResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_Resources_RawFileResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_content` after provisioning.\nThe raw file content."]
    pub fn file_content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_content", self.base))
    }

    #[doc = "Get a reference to the value of field `file_title` after provisioning.\nThe title of the file."]
    pub fn file_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_title", self.base))
    }

    #[doc = "Get a reference to the value of field `mime_type` after provisioning.\nThe mime type of the file."]
    pub fn mime_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceDynamic {
    bigquery_resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource>>,
    f1_resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources_F1Resource>>,
    google_cloud_storage_resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource>>,
    google_drive_resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource>>,
    raw_file_resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_rag: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_resource: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    f1_resource: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources_F1Resource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_storage_resource: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_drive_resource: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_file_resource: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource>>,
    dynamic: AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource {
    #[doc =
        "Set the field `display_label`.\nA user-friendly name for this resource. This can be shown to the user\nand used by the model."]
    pub fn set_display_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_label = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `model_description`.\nA description of the resource. The model may use this, it will not be\nshown to users."]
    pub fn set_model_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_rag`.\nIf true, use RAG to retrieve relevant information from the resources.\n\nMust only be set for file-based resources."]
    pub fn set_use_rag(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_rag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_resource`.\n"]
    pub fn set_bigquery_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `f1_resource`.\n"]
    pub fn set_f1_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_F1Resource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.f1_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.f1_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_cloud_storage_resource`.\n"]
    pub fn set_google_cloud_storage_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_cloud_storage_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_storage_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_drive_resource`.\n"]
    pub fn set_google_drive_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_drive_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_drive_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `raw_file_resource`.\n"]
    pub fn set_raw_file_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.raw_file_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.raw_file_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource {}

impl BuildAgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource {
        AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource {
            display_label: core::default::Default::default(),
            model_description: core::default::Default::default(),
            use_rag: core::default::Default::default(),
            bigquery_resource: core::default::Default::default(),
            f1_resource: core::default::Default::default(),
            google_cloud_storage_resource: core::default::Default::default(),
            google_drive_resource: core::default::Default::default(),
            raw_file_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceRef {
        AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_label` after provisioning.\nA user-friendly name for this resource. This can be shown to the user\nand used by the model."]
    pub fn display_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_description` after provisioning.\nA description of the resource. The model may use this, it will not be\nshown to users."]
    pub fn model_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_rag` after provisioning.\nIf true, use RAG to retrieve relevant information from the resources.\n\nMust only be set for file-based resources."]
    pub fn use_rag(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_rag", self.base))
    }

    #[doc = "Get a reference to the value of field `bigquery_resource` after provisioning.\n"]
    pub fn bigquery_resource(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `f1_resource` after provisioning.\n"]
    pub fn f1_resource(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Resources_F1ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.f1_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `google_cloud_storage_resource` after provisioning.\n"]
    pub fn google_cloud_storage_resource(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_cloud_storage_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `google_drive_resource` after provisioning.\n"]
    pub fn google_drive_resource(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_drive_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `raw_file_resource` after provisioning.\n"]
    pub fn raw_file_resource(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Resources_RawFileResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.raw_file_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ArtifactExamplesDynamic {
    resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactExamples {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>>,
    dynamic: AgenticApplicationsAnalystAgentPersona_ArtifactExamplesDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactExamples {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ArtifactExamples {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactExamples>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ArtifactExamples {}

impl BuildAgenticApplicationsAnalystAgentPersona_ArtifactExamples {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ArtifactExamples {
        AgenticApplicationsAnalystAgentPersona_ArtifactExamples {
            resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactExamplesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ArtifactExamplesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactExamplesRef {
        AgenticApplicationsAnalystAgentPersona_ArtifactExamplesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactExamplesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamplesDynamic {
    resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>>,
    dynamic: AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamplesDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples {
    type O =
        terrars
        ::BlockAssignable<
            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples {}

impl BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples {
    pub fn build(
        self,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples {
            resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamplesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamplesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamplesRef {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamplesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamplesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptionsDynamic {
    document_examples: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples,
        >,
    >,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_examples: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples>,
    >,
    dynamic: AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptionsDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions {
    #[doc = "Set the field `export_format`.\nFormat for document export.\nPossible values:\nPDF\nDOCX\nGOOGLE_DOCS"]
    pub fn set_export_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.export_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `document_examples`.\n"]
    pub fn set_document_examples(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamples,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.document_examples = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.document_examples = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions {
    type O =
        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions {}

impl BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions {
            export_format: core::default::Default::default(),
            document_examples: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptionsRef {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `export_format` after provisioning.\nFormat for document export.\nPossible values:\nPDF\nDOCX\nGOOGLE_DOCS"]
    pub fn export_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_format", self.base))
    }

    #[doc = "Get a reference to the value of field `document_examples` after provisioning.\n"]
    pub fn document_examples(
        &self,
    ) -> terrars
    ::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions_DocumentExamplesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.document_examples", self.base))
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    append_methodology: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_methodology_artifact: std::option::Option<terrars::PrimField<bool>>,
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions {
    #[doc = "Set the field `append_methodology`.\nIf true, append the detailed methodology to the final response."]
    pub fn set_append_methodology(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.append_methodology = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export_format`.\nFormat for methodology export.\nPossible values:\nMARKDOWN\nHTML\nPDF"]
    pub fn set_export_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.export_format = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `export_methodology_artifact`.\nIf true, export the detailed methodology as a separate artifact."]
    pub fn set_export_methodology_artifact(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.export_methodology_artifact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions {
    type O =
        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions {}

impl BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions {
            append_methodology: core::default::Default::default(),
            export_format: core::default::Default::default(),
            export_methodology_artifact: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptionsRef {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `append_methodology` after provisioning.\nIf true, append the detailed methodology to the final response."]
    pub fn append_methodology(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append_methodology", self.base))
    }

    #[doc =
        "Get a reference to the value of field `export_format` after provisioning.\nFormat for methodology export.\nPossible values:\nMARKDOWN\nHTML\nPDF"]
    pub fn export_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `export_methodology_artifact` after provisioning.\nIf true, export the detailed methodology as a separate artifact."]
    pub fn export_methodology_artifact(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_methodology_artifact", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamplesDynamic {
    resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>>,
    dynamic: AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamplesDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples {
    type O =
        terrars
        ::BlockAssignable<
            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples {}

impl BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples {
    pub fn build(
        self,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples {
            resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamplesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamplesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamplesRef {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamplesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamplesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptionsDynamic {
    slide_examples: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples>,
    >,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slide_examples: std
    ::option
    ::Option<
        std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples>,
    >,
    dynamic: AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptionsDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions {
    #[doc =
        "Set the field `export_format`.\nFormat for slide export.\nPossible values:\nPDF\nPNG\nPPTX\nGOOGLE_SLIDES"]
    pub fn set_export_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.export_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slide_examples`.\n"]
    pub fn set_slide_examples(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamples,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slide_examples = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slide_examples = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions {}

impl BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions {
            export_format: core::default::Default::default(),
            slide_examples: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptionsRef {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `export_format` after provisioning.\nFormat for slide export.\nPossible values:\nPDF\nPNG\nPPTX\nGOOGLE_SLIDES"]
    pub fn export_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export_format", self.base))
    }

    #[doc = "Get a reference to the value of field `slide_examples` after provisioning.\n"]
    pub fn slide_examples(
        &self,
    ) -> terrars
    ::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions_SlideExamplesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slide_examples", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamplesDynamic {
    resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples {
    visualization_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>>,
    dynamic: AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamplesDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_Resource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples {
    type O =
        terrars
        ::BlockAssignable<
            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples {
    #[doc = "The type of the visualization (e.g. \"Bar Chart\", \"Line Chart\")."]
    pub visualization_type: terrars::PrimField<std::string::String>,
}

impl BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples {
    pub fn build(
        self,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples {
            visualization_type: self.visualization_type,
            resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamplesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamplesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamplesRef {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamplesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamplesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `visualization_type` after provisioning.\nThe type of the visualization (e.g. \"Bar Chart\", \"Line Chart\")."]
    pub fn visualization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visualization_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactExamples_ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptionsDynamic {
    visualization_examples: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples,
        >,
    >,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visualization_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visualization_examples: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples>,
    >,
    dynamic: AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptionsDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions {
    #[doc =
        "Set the field `visualization_mode`.\nMode for generating visualizations.\nPossible values:\nVISUALIZATION_MODE_EXPLICIT_ONLY\nVISUALIZATION_MODE_WHEN_NECESSARY\nVISUALIZATION_MODE_WHEN_HELPFUL\nVISUALIZATION_MODE_ALWAYS"]
    pub fn set_visualization_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.visualization_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visualization_examples`.\n"]
    pub fn set_visualization_examples(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamples,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.visualization_examples = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.visualization_examples = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions {}

impl BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions {
            visualization_mode: core::default::Default::default(),
            visualization_examples: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptionsRef {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `visualization_mode` after provisioning.\nMode for generating visualizations.\nPossible values:\nVISUALIZATION_MODE_EXPLICIT_ONLY\nVISUALIZATION_MODE_WHEN_NECESSARY\nVISUALIZATION_MODE_WHEN_HELPFUL\nVISUALIZATION_MODE_ALWAYS"]
    pub fn visualization_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visualization_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `visualization_examples` after provisioning.\n"]
    pub fn visualization_examples(
        &self,
    ) -> terrars
    ::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions_VisualizationExamplesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.visualization_examples", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfigDynamic {
    document_generation_options: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions>>,
    methodology_export_options: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions>>,
    slide_generation_options: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions>>,
    visualization_options: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_generation_options: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methodology_export_options: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slide_generation_options: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visualization_options: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions>>,
    dynamic: AgenticApplicationsAnalystAgentPersona_ArtifactsConfigDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfig {
    #[doc = "Set the field `document_generation_options`.\n"]
    pub fn set_document_generation_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.document_generation_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.document_generation_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `methodology_export_options`.\n"]
    pub fn set_methodology_export_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.methodology_export_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.methodology_export_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `slide_generation_options`.\n"]
    pub fn set_slide_generation_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.slide_generation_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.slide_generation_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `visualization_options`.\n"]
    pub fn set_visualization_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.visualization_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.visualization_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ArtifactsConfig {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig {}

impl BuildAgenticApplicationsAnalystAgentPersona_ArtifactsConfig {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfig {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfig {
            document_generation_options: core::default::Default::default(),
            methodology_export_options: core::default::Default::default(),
            slide_generation_options: core::default::Default::default(),
            visualization_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ArtifactsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ArtifactsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ArtifactsConfigRef {
        AgenticApplicationsAnalystAgentPersona_ArtifactsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ArtifactsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `document_generation_options` after provisioning.\n"]
    pub fn document_generation_options(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_DocumentGenerationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.document_generation_options", self.base))
    }

    #[doc = "Get a reference to the value of field `methodology_export_options` after provisioning.\n"]
    pub fn methodology_export_options(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_MethodologyExportOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.methodology_export_options", self.base))
    }

    #[doc = "Get a reference to the value of field `slide_generation_options` after provisioning.\n"]
    pub fn slide_generation_options(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_SlideGenerationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.slide_generation_options", self.base))
    }

    #[doc = "Get a reference to the value of field `visualization_options` after provisioning.\n"]
    pub fn visualization_options(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ArtifactsConfig_VisualizationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.visualization_options", self.base))
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality {}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality { }

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality {}

impl BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality {}
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQualityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQualityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQualityRef {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQualityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQualityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics {}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics { }

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics {
    type O =
        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics {}

impl BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics {}
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatisticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatisticsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatisticsRef {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatisticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatisticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk {}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk { }

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk {}

impl BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk {}
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_CoindeskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_CoindeskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_CoindeskRef {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_CoindeskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_CoindeskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub {}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub { }

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub {}

impl BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub {}
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FinnhubRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FinnhubRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FinnhubRef {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FinnhubRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FinnhubRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred {}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred { }

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred {}

impl BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred {}
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FredRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FredRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FredRef {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FredRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FredRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar {}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar { }

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar {}

impl BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar {}
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgarRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgarRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgarRef {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgarRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgarRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions {}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions { }

impl terrars
::ToListMappable for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions {
    type O =
        terrars
        ::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions {}

impl BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions {}
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctionsRef {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda {}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda { }

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda {}

impl BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda {}
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources_UsdaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ExternalDataSources_UsdaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources_UsdaRef {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources_UsdaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources_UsdaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ExternalDataSourcesDynamic {
    air_quality: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality>>,
    bureau_labor_statistics: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics>>,
    coindesk: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk>>,
    finnhub: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub>>,
    fred: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred>>,
    sec_edgar: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar>>,
    treasury_securities_auctions: std
    ::option
    ::Option<
        terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions>,
    >,
    usda: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSources {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    air_quality: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bureau_labor_statistics: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coindesk: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finnhub: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fred: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sec_edgar: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    treasury_securities_auctions: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usda: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda>>,
    dynamic: AgenticApplicationsAnalystAgentPersona_ExternalDataSourcesDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSources {
    #[doc = "Set the field `air_quality`.\n"]
    pub fn set_air_quality(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQuality>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.air_quality = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.air_quality = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bureau_labor_statistics`.\n"]
    pub fn set_bureau_labor_statistics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatistics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bureau_labor_statistics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bureau_labor_statistics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `coindesk`.\n"]
    pub fn set_coindesk(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Coindesk>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.coindesk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.coindesk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finnhub`.\n"]
    pub fn set_finnhub(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Finnhub>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finnhub = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finnhub = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fred`.\n"]
    pub fn set_fred(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Fred>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fred = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fred = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sec_edgar`.\n"]
    pub fn set_sec_edgar(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgar>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sec_edgar = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sec_edgar = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `treasury_securities_auctions`.\n"]
    pub fn set_treasury_securities_auctions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.treasury_securities_auctions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.treasury_securities_auctions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `usda`.\n"]
    pub fn set_usda(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_Usda>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.usda = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.usda = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_ExternalDataSources {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_ExternalDataSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources {
    #[doc = "Whether this external data source is enabled for the current analysis."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildAgenticApplicationsAnalystAgentPersona_ExternalDataSources {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSources {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSources {
            enabled: self.enabled,
            air_quality: core::default::Default::default(),
            bureau_labor_statistics: core::default::Default::default(),
            coindesk: core::default::Default::default(),
            finnhub: core::default::Default::default(),
            fred: core::default::Default::default(),
            sec_edgar: core::default::Default::default(),
            treasury_securities_auctions: core::default::Default::default(),
            usda: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ExternalDataSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ExternalDataSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ExternalDataSourcesRef {
        AgenticApplicationsAnalystAgentPersona_ExternalDataSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ExternalDataSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether this external data source is enabled for the current analysis."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `selection_name` after provisioning.\nThe name of the external data source, used for custom org policy\nevaluation. Output-only (populated automatically with the selection case\nname)."]
    pub fn selection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `air_quality` after provisioning.\n"]
    pub fn air_quality(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_AirQualityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.air_quality", self.base))
    }

    #[doc = "Get a reference to the value of field `bureau_labor_statistics` after provisioning.\n"]
    pub fn bureau_labor_statistics(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_BureauLaborStatisticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bureau_labor_statistics", self.base))
    }

    #[doc = "Get a reference to the value of field `coindesk` after provisioning.\n"]
    pub fn coindesk(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_CoindeskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.coindesk", self.base))
    }

    #[doc = "Get a reference to the value of field `finnhub` after provisioning.\n"]
    pub fn finnhub(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FinnhubRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.finnhub", self.base))
    }

    #[doc = "Get a reference to the value of field `fred` after provisioning.\n"]
    pub fn fred(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_FredRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fred", self.base))
    }

    #[doc = "Get a reference to the value of field `sec_edgar` after provisioning.\n"]
    pub fn sec_edgar(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_SecEdgarRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sec_edgar", self.base))
    }

    #[doc = "Get a reference to the value of field `treasury_securities_auctions` after provisioning.\n"]
    pub fn treasury_securities_auctions(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_TreasurySecuritiesAuctionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.treasury_securities_auctions", self.base))
    }

    #[doc = "Get a reference to the value of field `usda` after provisioning.\n"]
    pub fn usda(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_ExternalDataSources_UsdaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.usda", self.base))
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_McpDataSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_header: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    description: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_token_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<terrars::PrimField<std::string::String>>,
    server_url: terrars::PrimField<std::string::String>,
}

impl AgenticApplicationsAnalystAgentPersona_McpDataSources {
    #[doc = "Set the field `api_key`.\nInput only. The API key of the MCP server."]
    pub fn set_api_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `api_key_header`.\nThe HTTP header when the API key is passed in a request header\n(e.g. 'x-api-key', 'api-key', 'X-Auth-Token')."]
    pub fn set_api_key_header(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_name`.\nThe API key parameter name."]
    pub fn set_api_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\nThe client ID for authentication."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nInput only. The client secret for authentication."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_token_url`.\nThe URL to use for retrieving the OAuth token."]
    pub fn set_oauth_token_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oauth_token_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt`.\nThe custom prompt for the MCP agent."]
    pub fn set_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_McpDataSources {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_McpDataSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_McpDataSources {
    #[doc = "The description of the MCP agent."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc =
        "The display name of the MCP server. Must be no longer than 63 characters\nand can only contain letters, numbers, spaces, underscores, and hyphens."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "Whether this external data source is enabled for the current analysis."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The URL of the MCP server."]
    pub server_url: terrars::PrimField<std::string::String>,
}

impl BuildAgenticApplicationsAnalystAgentPersona_McpDataSources {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_McpDataSources {
        AgenticApplicationsAnalystAgentPersona_McpDataSources {
            api_key: core::default::Default::default(),
            api_key_header: core::default::Default::default(),
            api_key_name: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            description: self.description,
            display_name: self.display_name,
            enabled: self.enabled,
            oauth_token_url: core::default::Default::default(),
            prompt: core::default::Default::default(),
            server_url: self.server_url,
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_McpDataSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_McpDataSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_McpDataSourcesRef {
        AgenticApplicationsAnalystAgentPersona_McpDataSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_McpDataSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key` after provisioning.\nInput only. The API key of the MCP server."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `api_key_header` after provisioning.\nThe HTTP header when the API key is passed in a request header\n(e.g. 'x-api-key', 'api-key', 'X-Auth-Token')."]
    pub fn api_key_header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_header", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_name` after provisioning.\nThe API key parameter name."]
    pub fn api_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client ID for authentication."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nInput only. The client secret for authentication."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the MCP agent."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the MCP server. Must be no longer than 63 characters\nand can only contain letters, numbers, spaces, underscores, and hyphens."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether this external data source is enabled for the current analysis."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth_token_url` after provisioning.\nThe URL to use for retrieving the OAuth token."]
    pub fn oauth_token_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_token_url", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\nThe custom prompt for the MCP agent."]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `server_url` after provisioning.\nThe URL of the MCP server."]
    pub fn server_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_ResourcesDynamic {
    bigquery_resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource>>,
    f1_resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources_F1Resource>>,
    google_cloud_storage_resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource>>,
    google_drive_resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource>>,
    raw_file_resource: std
    ::option
    ::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_rag: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_resource: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    f1_resource: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources_F1Resource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_storage_resource: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_drive_resource: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_file_resource: std
    ::option
    ::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource>>,
    dynamic: AgenticApplicationsAnalystAgentPersona_ResourcesDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_Resources {
    #[doc =
        "Set the field `display_label`.\nA user-friendly name for this resource. This can be shown to the user\nand used by the model."]
    pub fn set_display_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_label = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `model_description`.\nA description of the resource. The model may use this, it will not be\nshown to users."]
    pub fn set_model_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_rag`.\nIf true, use RAG to retrieve relevant information from the resources.\n\nMust only be set for file-based resources."]
    pub fn set_use_rag(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_rag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_resource`.\n"]
    pub fn set_bigquery_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `f1_resource`.\n"]
    pub fn set_f1_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_F1Resource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.f1_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.f1_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_cloud_storage_resource`.\n"]
    pub fn set_google_cloud_storage_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_cloud_storage_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_storage_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_drive_resource`.\n"]
    pub fn set_google_drive_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_drive_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_drive_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `raw_file_resource`.\n"]
    pub fn set_raw_file_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources_RawFileResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.raw_file_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.raw_file_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Resources {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Resources {}

impl BuildAgenticApplicationsAnalystAgentPersona_Resources {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Resources {
        AgenticApplicationsAnalystAgentPersona_Resources {
            display_label: core::default::Default::default(),
            model_description: core::default::Default::default(),
            use_rag: core::default::Default::default(),
            bigquery_resource: core::default::Default::default(),
            f1_resource: core::default::Default::default(),
            google_cloud_storage_resource: core::default::Default::default(),
            google_drive_resource: core::default::Default::default(),
            raw_file_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_ResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_ResourcesRef {
        AgenticApplicationsAnalystAgentPersona_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_label` after provisioning.\nA user-friendly name for this resource. This can be shown to the user\nand used by the model."]
    pub fn display_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_label", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_description` after provisioning.\nA description of the resource. The model may use this, it will not be\nshown to users."]
    pub fn model_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_rag` after provisioning.\nIf true, use RAG to retrieve relevant information from the resources.\n\nMust only be set for file-based resources."]
    pub fn use_rag(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_rag", self.base))
    }

    #[doc = "Get a reference to the value of field `bigquery_resource` after provisioning.\n"]
    pub fn bigquery_resource(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Resources_BigqueryResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `f1_resource` after provisioning.\n"]
    pub fn f1_resource(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Resources_F1ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.f1_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `google_cloud_storage_resource` after provisioning.\n"]
    pub fn google_cloud_storage_resource(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Resources_GoogleCloudStorageResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_cloud_storage_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `google_drive_resource` after provisioning.\n"]
    pub fn google_drive_resource(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Resources_GoogleDriveResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_drive_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `raw_file_resource` after provisioning.\n"]
    pub fn raw_file_resource(
        &self,
    ) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Resources_RawFileResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.raw_file_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Skills_References {
    content: terrars::PrimField<std::string::String>,
    reference_id: terrars::PrimField<std::string::String>,
}

impl AgenticApplicationsAnalystAgentPersona_Skills_References { }

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Skills_References {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Skills_References>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Skills_References {
    #[doc = "The content of the reference."]
    pub content: terrars::PrimField<std::string::String>,
    #[doc =
        "The identifier of the reference within the skill. Use a descriptive\nstring that reflects the reference's function."]
    pub reference_id: terrars::PrimField<std::string::String>,
}

impl BuildAgenticApplicationsAnalystAgentPersona_Skills_References {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Skills_References {
        AgenticApplicationsAnalystAgentPersona_Skills_References {
            content: self.content,
            reference_id: self.reference_id,
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_Skills_ReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_Skills_ReferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_Skills_ReferencesRef {
        AgenticApplicationsAnalystAgentPersona_Skills_ReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_Skills_ReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\nThe content of the reference."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reference_id` after provisioning.\nThe identifier of the reference within the skill. Use a descriptive\nstring that reflects the reference's function."]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_SkillsDynamic {
    references: std::option::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Skills_References>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Skills {
    content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    skill_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    references: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Skills_References>>,
    dynamic: AgenticApplicationsAnalystAgentPersona_SkillsDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_Skills {
    #[doc = "Set the field `description`.\nThe description of the skill."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `references`.\n"]
    pub fn set_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Skills_References>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.references = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.references = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Skills {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Skills>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Skills {
    #[doc = "The markdown text content of the skill."]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = "The identifier of the skill. Use a descriptive string that reflects the\nskill's function."]
    pub skill_id: terrars::PrimField<std::string::String>,
}

impl BuildAgenticApplicationsAnalystAgentPersona_Skills {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Skills {
        AgenticApplicationsAnalystAgentPersona_Skills {
            content: self.content,
            description: core::default::Default::default(),
            skill_id: self.skill_id,
            references: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_SkillsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_SkillsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_SkillsRef {
        AgenticApplicationsAnalystAgentPersona_SkillsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_SkillsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe markdown text content of the skill."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the skill."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `skill_id` after provisioning.\nThe identifier of the skill. Use a descriptive string that reflects the\nskill's function."]
    pub fn skill_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skill_id", self.base))
    }

    #[doc = "Get a reference to the value of field `references` after provisioning.\n"]
    pub fn references(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Skills_ReferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.references", self.base))
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Tables_Columns {
    data_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl AgenticApplicationsAnalystAgentPersona_Tables_Columns {
    #[doc = "Set the field `description`.\nThe description of the column."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Tables_Columns {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Tables_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Tables_Columns {
    #[doc =
        "The data type of the column. This should be a GoogleSQL data type.\nParameterized types such as PROTO, ENUM, ARRAY, STRUCT<...>, and\nRANGE are not supported."]
    pub data_type: terrars::PrimField<std::string::String>,
    #[doc = "The name of the column."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAgenticApplicationsAnalystAgentPersona_Tables_Columns {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Tables_Columns {
        AgenticApplicationsAnalystAgentPersona_Tables_Columns {
            data_type: self.data_type,
            description: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_Tables_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_Tables_ColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_Tables_ColumnsRef {
        AgenticApplicationsAnalystAgentPersona_Tables_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_Tables_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe data type of the column. This should be a GoogleSQL data type.\nParameterized types such as PROTO, ENUM, ARRAY, STRUCT<...>, and\nRANGE are not supported."]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the column."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the column."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgenticApplicationsAnalystAgentPersona_TablesDynamic {
    columns: std::option::Option<terrars::DynamicBlock<AgenticApplicationsAnalystAgentPersona_Tables_Columns>>,
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Tables {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<std::vec::Vec<AgenticApplicationsAnalystAgentPersona_Tables_Columns>>,
    dynamic: AgenticApplicationsAnalystAgentPersona_TablesDynamic,
}

impl AgenticApplicationsAnalystAgentPersona_Tables {
    #[doc = "Set the field `description`.\nThe description of the table."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Tables_Columns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Tables {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Tables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Tables {
    #[doc = "The name of the table."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAgenticApplicationsAnalystAgentPersona_Tables {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Tables {
        AgenticApplicationsAnalystAgentPersona_Tables {
            description: core::default::Default::default(),
            name: self.name,
            columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_TablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_TablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_TablesRef {
        AgenticApplicationsAnalystAgentPersona_TablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_TablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the table."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the table."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<AgenticApplicationsAnalystAgentPersona_Tables_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize)]
pub struct AgenticApplicationsAnalystAgentPersona_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgenticApplicationsAnalystAgentPersona_Timeouts {
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

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_Timeouts {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_Timeouts {}

impl BuildAgenticApplicationsAnalystAgentPersona_Timeouts {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_Timeouts {
        AgenticApplicationsAnalystAgentPersona_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_TimeoutsRef {
        AgenticApplicationsAnalystAgentPersona_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_TimeoutsRef {
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
pub struct AgenticApplicationsAnalystAgentPersona_WebSearchConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AgenticApplicationsAnalystAgentPersona_WebSearchConfig {
    #[doc =
        "Set the field `disabled`.\nWhether web search grounding is disabled for the analyst agent.\nDefaults to false if not specified (i.e. web search grounding is enabled)."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `excluded_domains`.\nList of domains to be excluded from Google Search / Enterprise Web Search\ngrounding."]
    pub fn set_excluded_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_domains = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgenticApplicationsAnalystAgentPersona_WebSearchConfig {
    type O = terrars::BlockAssignable<AgenticApplicationsAnalystAgentPersona_WebSearchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgenticApplicationsAnalystAgentPersona_WebSearchConfig {}

impl BuildAgenticApplicationsAnalystAgentPersona_WebSearchConfig {
    pub fn build(self) -> AgenticApplicationsAnalystAgentPersona_WebSearchConfig {
        AgenticApplicationsAnalystAgentPersona_WebSearchConfig {
            disabled: core::default::Default::default(),
            excluded_domains: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgenticApplicationsAnalystAgentPersona_WebSearchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgenticApplicationsAnalystAgentPersona_WebSearchConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgenticApplicationsAnalystAgentPersona_WebSearchConfigRef {
        AgenticApplicationsAnalystAgentPersona_WebSearchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgenticApplicationsAnalystAgentPersona_WebSearchConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether web search grounding is disabled for the analyst agent.\nDefaults to false if not specified (i.e. web search grounding is enabled)."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `excluded_domains` after provisioning.\nList of domains to be excluded from Google Search / Enterprise Web Search\ngrounding."]
    pub fn excluded_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_domains", self.base))
    }
}
