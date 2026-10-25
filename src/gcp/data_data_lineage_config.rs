#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataDataLineageConfigData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
}

struct DataDataLineageConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDataLineageConfigData_>,
}

#[derive(Clone)]
pub struct DataDataLineageConfig(std::rc::Rc<DataDataLineageConfig_>);

impl DataDataLineageConfig {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed for optimistic concurrency control when patching config."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingestion` after provisioning.\nDefines how Lineage should be ingested for this resource."]
    pub fn ingestion(&self) -> terrars::ListRef<DataDataLineageConfig_IngestionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingestion", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe region of the data lineage configuration for integration."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the config.\nFormat: organizations/{organization_id}/locations/{location}/config,\nfolders/{folder_id}/locations/{location}/config,\nprojects/{project_id}/locations/{location}/config,\nor projects/{project_number}/locations/{location}/config."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nParent scope for the config.\nFormat: projects/{project-id|project-number} or folders/{folder-number} or organizations/{organization-number}."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }
}

impl terrars::Referable for DataDataLineageConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDataLineageConfig { }

impl terrars::ToListMappable for DataDataLineageConfig {
    type O = terrars::ListRef<DataDataLineageConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDataLineageConfig_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_data_lineage_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDataLineageConfig {
    pub tf_id: std::string::String,
    #[doc = "The region of the data lineage configuration for integration."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Parent scope for the config.\nFormat: projects/{project-id|project-number} or folders/{folder-number} or organizations/{organization-number}."]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataDataLineageConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDataLineageConfig {
        let out = DataDataLineageConfig(std::rc::Rc::new(DataDataLineageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDataLineageConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDataLineageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataLineageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDataLineageConfigRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed for optimistic concurrency control when patching config."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingestion` after provisioning.\nDefines how Lineage should be ingested for this resource."]
    pub fn ingestion(&self) -> terrars::ListRef<DataDataLineageConfig_IngestionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingestion", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe region of the data lineage configuration for integration."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the config.\nFormat: organizations/{organization_id}/locations/{location}/config,\nfolders/{folder_id}/locations/{location}/config,\nprojects/{project_id}/locations/{location}/config,\nor projects/{project_number}/locations/{location}/config."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nParent scope for the config.\nFormat: projects/{project-id|project-number} or folders/{folder-number} or organizations/{organization-number}."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDataLineageConfig_Ingestion_Rule_IntegrationSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataLineageConfig_Ingestion_Rule_IntegrationSelector {
    #[doc = "Set the field `integration`.\n"]
    pub fn set_integration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.integration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataLineageConfig_Ingestion_Rule_IntegrationSelector {
    type O = terrars::BlockAssignable<DataDataLineageConfig_Ingestion_Rule_IntegrationSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataLineageConfig_Ingestion_Rule_IntegrationSelector {}

impl BuildDataDataLineageConfig_Ingestion_Rule_IntegrationSelector {
    pub fn build(self) -> DataDataLineageConfig_Ingestion_Rule_IntegrationSelector {
        DataDataLineageConfig_Ingestion_Rule_IntegrationSelector { integration: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDataLineageConfig_Ingestion_Rule_IntegrationSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataLineageConfig_Ingestion_Rule_IntegrationSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataLineageConfig_Ingestion_Rule_IntegrationSelectorRef {
        DataDataLineageConfig_Ingestion_Rule_IntegrationSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataLineageConfig_Ingestion_Rule_IntegrationSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataLineageConfig_Ingestion_Rule_LineageEnablement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDataLineageConfig_Ingestion_Rule_LineageEnablement {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataLineageConfig_Ingestion_Rule_LineageEnablement {
    type O = terrars::BlockAssignable<DataDataLineageConfig_Ingestion_Rule_LineageEnablement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataLineageConfig_Ingestion_Rule_LineageEnablement {}

impl BuildDataDataLineageConfig_Ingestion_Rule_LineageEnablement {
    pub fn build(self) -> DataDataLineageConfig_Ingestion_Rule_LineageEnablement {
        DataDataLineageConfig_Ingestion_Rule_LineageEnablement { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDataLineageConfig_Ingestion_Rule_LineageEnablementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataLineageConfig_Ingestion_Rule_LineageEnablementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataLineageConfig_Ingestion_Rule_LineageEnablementRef {
        DataDataLineageConfig_Ingestion_Rule_LineageEnablementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataLineageConfig_Ingestion_Rule_LineageEnablementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataLineageConfig_Ingestion_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_selector: std
    ::option
    ::Option<terrars::ListField<DataDataLineageConfig_Ingestion_Rule_IntegrationSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lineage_enablement: std::option::Option<terrars::ListField<DataDataLineageConfig_Ingestion_Rule_LineageEnablement>>,
}

impl DataDataLineageConfig_Ingestion_Rule {
    #[doc = "Set the field `integration_selector`.\n"]
    pub fn set_integration_selector(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataLineageConfig_Ingestion_Rule_IntegrationSelector>>,
    ) -> Self {
        self.integration_selector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lineage_enablement`.\n"]
    pub fn set_lineage_enablement(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataLineageConfig_Ingestion_Rule_LineageEnablement>>,
    ) -> Self {
        self.lineage_enablement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataLineageConfig_Ingestion_Rule {
    type O = terrars::BlockAssignable<DataDataLineageConfig_Ingestion_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataLineageConfig_Ingestion_Rule {}

impl BuildDataDataLineageConfig_Ingestion_Rule {
    pub fn build(self) -> DataDataLineageConfig_Ingestion_Rule {
        DataDataLineageConfig_Ingestion_Rule {
            integration_selector: core::default::Default::default(),
            lineage_enablement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataLineageConfig_Ingestion_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataLineageConfig_Ingestion_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDataLineageConfig_Ingestion_RuleRef {
        DataDataLineageConfig_Ingestion_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataLineageConfig_Ingestion_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `integration_selector` after provisioning.\n"]
    pub fn integration_selector(&self) -> terrars::ListRef<DataDataLineageConfig_Ingestion_Rule_IntegrationSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `lineage_enablement` after provisioning.\n"]
    pub fn lineage_enablement(&self) -> terrars::ListRef<DataDataLineageConfig_Ingestion_Rule_LineageEnablementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lineage_enablement", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataLineageConfig_Ingestion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<terrars::ListField<DataDataLineageConfig_Ingestion_Rule>>,
}

impl DataDataLineageConfig_Ingestion {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataLineageConfig_Ingestion_Rule>>,
    ) -> Self {
        self.rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataLineageConfig_Ingestion {
    type O = terrars::BlockAssignable<DataDataLineageConfig_Ingestion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataLineageConfig_Ingestion {}

impl BuildDataDataLineageConfig_Ingestion {
    pub fn build(self) -> DataDataLineageConfig_Ingestion {
        DataDataLineageConfig_Ingestion { rule: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDataLineageConfig_IngestionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataLineageConfig_IngestionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDataLineageConfig_IngestionRef {
        DataDataLineageConfig_IngestionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataLineageConfig_IngestionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<DataDataLineageConfig_Ingestion_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}
