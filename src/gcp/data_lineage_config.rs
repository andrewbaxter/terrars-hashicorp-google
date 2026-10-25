#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataLineageConfigData_ {
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
    location: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingestion: std::option::Option<std::vec::Vec<DataLineageConfig_Ingestion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLineageConfig_Timeouts>,
    dynamic: DataLineageConfigDynamic,
}

struct DataLineageConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLineageConfigData_>,
}

#[derive(Clone)]
pub struct DataLineageConfig(std::rc::Rc<DataLineageConfig_>);

impl DataLineageConfig {
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

    #[doc = "Set the field `ingestion`.\n"]
    pub fn set_ingestion(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLineageConfig_Ingestion>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingestion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingestion = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLineageConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `ingestion` after provisioning.\n"]
    pub fn ingestion(&self) -> terrars::ListRef<DataLineageConfig_IngestionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingestion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLineageConfig_TimeoutsRef {
        DataLineageConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataLineageConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataLineageConfig { }

impl terrars::ToListMappable for DataLineageConfig {
    type O = terrars::ListRef<DataLineageConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataLineageConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_data_lineage_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLineageConfig {
    pub tf_id: std::string::String,
    #[doc = "The region of the data lineage configuration for integration."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Parent scope for the config.\nFormat: projects/{project-id|project-number} or folders/{folder-number} or organizations/{organization-number}."]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataLineageConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLineageConfig {
        let out = DataLineageConfig(std::rc::Rc::new(DataLineageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLineageConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
                ingestion: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataLineageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLineageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLineageConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `ingestion` after provisioning.\n"]
    pub fn ingestion(&self) -> terrars::ListRef<DataLineageConfig_IngestionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingestion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLineageConfig_TimeoutsRef {
        DataLineageConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataLineageConfigDynamic {
    ingestion: std::option::Option<terrars::DynamicBlock<DataLineageConfig_Ingestion>>,
}

#[derive(Serialize)]
pub struct DataLineageConfig_Ingestion_Rule_IntegrationSelector {
    integration: terrars::PrimField<std::string::String>,
}

impl DataLineageConfig_Ingestion_Rule_IntegrationSelector { }

impl terrars::ToListMappable for DataLineageConfig_Ingestion_Rule_IntegrationSelector {
    type O = terrars::BlockAssignable<DataLineageConfig_Ingestion_Rule_IntegrationSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLineageConfig_Ingestion_Rule_IntegrationSelector {
    #[doc = "Integration to which the rule applies. Possible values: [\"DATAPROC\", \"LOOKER_CORE\"]"]
    pub integration: terrars::PrimField<std::string::String>,
}

impl BuildDataLineageConfig_Ingestion_Rule_IntegrationSelector {
    pub fn build(self) -> DataLineageConfig_Ingestion_Rule_IntegrationSelector {
        DataLineageConfig_Ingestion_Rule_IntegrationSelector { integration: self.integration }
    }
}

#[allow(dead_code)]
pub struct DataLineageConfig_Ingestion_Rule_IntegrationSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLineageConfig_Ingestion_Rule_IntegrationSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLineageConfig_Ingestion_Rule_IntegrationSelectorRef {
        DataLineageConfig_Ingestion_Rule_IntegrationSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLineageConfig_Ingestion_Rule_IntegrationSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `integration` after provisioning.\nIntegration to which the rule applies. Possible values: [\"DATAPROC\", \"LOOKER_CORE\"]"]
    pub fn integration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLineageConfig_Ingestion_Rule_LineageEnablement {
    enabled: terrars::PrimField<bool>,
}

impl DataLineageConfig_Ingestion_Rule_LineageEnablement { }

impl terrars::ToListMappable for DataLineageConfig_Ingestion_Rule_LineageEnablement {
    type O = terrars::BlockAssignable<DataLineageConfig_Ingestion_Rule_LineageEnablement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLineageConfig_Ingestion_Rule_LineageEnablement {
    #[doc = "Whether ingestion of lineage should be enabled."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildDataLineageConfig_Ingestion_Rule_LineageEnablement {
    pub fn build(self) -> DataLineageConfig_Ingestion_Rule_LineageEnablement {
        DataLineageConfig_Ingestion_Rule_LineageEnablement { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct DataLineageConfig_Ingestion_Rule_LineageEnablementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLineageConfig_Ingestion_Rule_LineageEnablementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLineageConfig_Ingestion_Rule_LineageEnablementRef {
        DataLineageConfig_Ingestion_Rule_LineageEnablementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLineageConfig_Ingestion_Rule_LineageEnablementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether ingestion of lineage should be enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLineageConfig_Ingestion_RuleDynamic {
    integration_selector: std
    ::option
    ::Option<terrars::DynamicBlock<DataLineageConfig_Ingestion_Rule_IntegrationSelector>>,
    lineage_enablement: std
    ::option
    ::Option<terrars::DynamicBlock<DataLineageConfig_Ingestion_Rule_LineageEnablement>>,
}

#[derive(Serialize)]
pub struct DataLineageConfig_Ingestion_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_selector: std::option::Option<std::vec::Vec<DataLineageConfig_Ingestion_Rule_IntegrationSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lineage_enablement: std::option::Option<std::vec::Vec<DataLineageConfig_Ingestion_Rule_LineageEnablement>>,
    dynamic: DataLineageConfig_Ingestion_RuleDynamic,
}

impl DataLineageConfig_Ingestion_Rule {
    #[doc = "Set the field `integration_selector`.\n"]
    pub fn set_integration_selector(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLineageConfig_Ingestion_Rule_IntegrationSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.integration_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.integration_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lineage_enablement`.\n"]
    pub fn set_lineage_enablement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLineageConfig_Ingestion_Rule_LineageEnablement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lineage_enablement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lineage_enablement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLineageConfig_Ingestion_Rule {
    type O = terrars::BlockAssignable<DataLineageConfig_Ingestion_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLineageConfig_Ingestion_Rule {}

impl BuildDataLineageConfig_Ingestion_Rule {
    pub fn build(self) -> DataLineageConfig_Ingestion_Rule {
        DataLineageConfig_Ingestion_Rule {
            integration_selector: core::default::Default::default(),
            lineage_enablement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLineageConfig_Ingestion_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLineageConfig_Ingestion_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLineageConfig_Ingestion_RuleRef {
        DataLineageConfig_Ingestion_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLineageConfig_Ingestion_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `integration_selector` after provisioning.\n"]
    pub fn integration_selector(&self) -> terrars::ListRef<DataLineageConfig_Ingestion_Rule_IntegrationSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `lineage_enablement` after provisioning.\n"]
    pub fn lineage_enablement(&self) -> terrars::ListRef<DataLineageConfig_Ingestion_Rule_LineageEnablementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lineage_enablement", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLineageConfig_IngestionDynamic {
    rule: std::option::Option<terrars::DynamicBlock<DataLineageConfig_Ingestion_Rule>>,
}

#[derive(Serialize)]
pub struct DataLineageConfig_Ingestion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<DataLineageConfig_Ingestion_Rule>>,
    dynamic: DataLineageConfig_IngestionDynamic,
}

impl DataLineageConfig_Ingestion {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLineageConfig_Ingestion_Rule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLineageConfig_Ingestion {
    type O = terrars::BlockAssignable<DataLineageConfig_Ingestion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLineageConfig_Ingestion {}

impl BuildDataLineageConfig_Ingestion {
    pub fn build(self) -> DataLineageConfig_Ingestion {
        DataLineageConfig_Ingestion {
            rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLineageConfig_IngestionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLineageConfig_IngestionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLineageConfig_IngestionRef {
        DataLineageConfig_IngestionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLineageConfig_IngestionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<DataLineageConfig_Ingestion_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLineageConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLineageConfig_Timeouts {
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

impl terrars::ToListMappable for DataLineageConfig_Timeouts {
    type O = terrars::BlockAssignable<DataLineageConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLineageConfig_Timeouts {}

impl BuildDataLineageConfig_Timeouts {
    pub fn build(self) -> DataLineageConfig_Timeouts {
        DataLineageConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLineageConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLineageConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLineageConfig_TimeoutsRef {
        DataLineageConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLineageConfig_TimeoutsRef {
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
