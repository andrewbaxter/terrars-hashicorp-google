#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiSemanticGovernancePolicyEngineData_ {
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
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_configs: std::option::Option<std::vec::Vec<VertexAiSemanticGovernancePolicyEngine_GatewayConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiSemanticGovernancePolicyEngine_Timeouts>,
    dynamic: VertexAiSemanticGovernancePolicyEngineDynamic,
}

struct VertexAiSemanticGovernancePolicyEngine_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiSemanticGovernancePolicyEngineData_>,
}

#[derive(Clone)]
pub struct VertexAiSemanticGovernancePolicyEngine(std::rc::Rc<VertexAiSemanticGovernancePolicyEngine_>);

impl VertexAiSemanticGovernancePolicyEngine {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region of the SemanticGovernancePolicyEngine, e.g. 'us-central1'."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_configs`.\n"]
    pub fn set_gateway_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiSemanticGovernancePolicyEngine_GatewayConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gateway_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gateway_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiSemanticGovernancePolicyEngine_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the SemanticGovernancePolicyEngine was created, in RFC3339\nUTC \"Zulu\" format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address allocated for the SGPE's managed PSC endpoint."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the SemanticGovernancePolicyEngine, in the form\n'projects/{project}/locations/{region}/semanticGovernancePolicyEngine'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_forwarding_rule` after provisioning.\nThe Private Service Connect forwarding rule URI for the SGPE's\nmanaged endpoint."]
    pub fn psc_forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_forwarding_rule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_service_attachment` after provisioning.\nThe Private Service Connect service attachment URI for the SGPE's\nmanaged endpoint."]
    pub fn psc_service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_service_attachment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the SemanticGovernancePolicyEngine, e.g. 'us-central1'."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the SemanticGovernancePolicyEngine. One of:\nSTATE_UNSPECIFIED, PROVISIONING, ACTIVE, FAILED, DEPROVISIONING,\nINACTIVE. 'FAILED' indicates provisioning did not succeed; recover by\ndestroying the resource (deprovision) or re-applying (re-provision)."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the SemanticGovernancePolicyEngine was last updated, in\nRFC3339 UTC \"Zulu\" format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiSemanticGovernancePolicyEngine_TimeoutsRef {
        VertexAiSemanticGovernancePolicyEngine_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VertexAiSemanticGovernancePolicyEngine {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiSemanticGovernancePolicyEngine { }

impl terrars::ToListMappable for VertexAiSemanticGovernancePolicyEngine {
    type O = terrars::ListRef<VertexAiSemanticGovernancePolicyEngineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiSemanticGovernancePolicyEngine_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_semantic_governance_policy_engine".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiSemanticGovernancePolicyEngine {
    pub tf_id: std::string::String,
}

impl BuildVertexAiSemanticGovernancePolicyEngine {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiSemanticGovernancePolicyEngine {
        let out = VertexAiSemanticGovernancePolicyEngine(std::rc::Rc::new(VertexAiSemanticGovernancePolicyEngine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiSemanticGovernancePolicyEngineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                gateway_configs: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiSemanticGovernancePolicyEngineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiSemanticGovernancePolicyEngineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiSemanticGovernancePolicyEngineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the SemanticGovernancePolicyEngine was created, in RFC3339\nUTC \"Zulu\" format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address allocated for the SGPE's managed PSC endpoint."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the SemanticGovernancePolicyEngine, in the form\n'projects/{project}/locations/{region}/semanticGovernancePolicyEngine'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_forwarding_rule` after provisioning.\nThe Private Service Connect forwarding rule URI for the SGPE's\nmanaged endpoint."]
    pub fn psc_forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_forwarding_rule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_service_attachment` after provisioning.\nThe Private Service Connect service attachment URI for the SGPE's\nmanaged endpoint."]
    pub fn psc_service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_service_attachment", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the SemanticGovernancePolicyEngine, e.g. 'us-central1'."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the SemanticGovernancePolicyEngine. One of:\nSTATE_UNSPECIFIED, PROVISIONING, ACTIVE, FAILED, DEPROVISIONING,\nINACTIVE. 'FAILED' indicates provisioning did not succeed; recover by\ndestroying the resource (deprovision) or re-applying (re-provision)."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the SemanticGovernancePolicyEngine was last updated, in\nRFC3339 UTC \"Zulu\" format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiSemanticGovernancePolicyEngine_TimeoutsRef {
        VertexAiSemanticGovernancePolicyEngine_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VertexAiSemanticGovernancePolicyEngineDynamic {
    gateway_configs: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiSemanticGovernancePolicyEngine_GatewayConfigs>>,
}

#[derive(Serialize)]
pub struct VertexAiSemanticGovernancePolicyEngine_GatewayConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_projects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_zone_name: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiSemanticGovernancePolicyEngine_GatewayConfigs {
    #[doc =
        "Set the field `allowed_projects`.\nAdditional consumer projects permitted to attach their own PSC endpoint\nto this gateway's ServiceAttachment. This is the \"decoupled\" mode, where\nthe customer creates the PSC endpoint in a project other than this\ngateway's network project. Each listed project is VPC-SC enforced: it\nmust be within the caller's service perimeter. The owning\nSemanticGovernancePolicyEngine's own project is always permitted\nimplicitly and need not be listed. Format: projects/{project} (ID or number)."]
    pub fn set_allowed_projects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_projects = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dns_zone_name`.\nThe name of the private Cloud DNS managed zone in which the backend\ncreates the DNS record set for this gateway's PSC endpoint. This is the\nmanaged-zone resource name, not a fully-qualified domain name. The zone\nmust already exist and be attached to the gateway's VPC at provision\ntime. The name must match '^[a-z0-9.-]{1,63}$'. Must be set together\nwith 'network' and 'subnetwork' (all three or none)."]
    pub fn set_dns_zone_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_zone_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe URI of the network resource where the gateway's PSC endpoint is\nprovisioned. Format: projects/{project}/global/networks/{network}.\n'network', 'subnetwork', and 'dns_zone_name' must all be set together\nor all omitted; setting only some is rejected by the API."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nThe URI of the subnetwork resource where the gateway's PSC endpoint is\nprovisioned. Format:\nprojects/{project}/regions/{region}/subnetworks/{subnetwork}. Must be\nset together with 'network' and 'dns_zone_name' (all three or none)."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiSemanticGovernancePolicyEngine_GatewayConfigs {
    type O = terrars::BlockAssignable<VertexAiSemanticGovernancePolicyEngine_GatewayConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiSemanticGovernancePolicyEngine_GatewayConfigs {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiSemanticGovernancePolicyEngine_GatewayConfigs {
    pub fn build(self) -> VertexAiSemanticGovernancePolicyEngine_GatewayConfigs {
        VertexAiSemanticGovernancePolicyEngine_GatewayConfigs {
            allowed_projects: core::default::Default::default(),
            dns_zone_name: core::default::Default::default(),
            name: self.name,
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiSemanticGovernancePolicyEngine_GatewayConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiSemanticGovernancePolicyEngine_GatewayConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiSemanticGovernancePolicyEngine_GatewayConfigsRef {
        VertexAiSemanticGovernancePolicyEngine_GatewayConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiSemanticGovernancePolicyEngine_GatewayConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_projects` after provisioning.\nAdditional consumer projects permitted to attach their own PSC endpoint\nto this gateway's ServiceAttachment. This is the \"decoupled\" mode, where\nthe customer creates the PSC endpoint in a project other than this\ngateway's network project. Each listed project is VPC-SC enforced: it\nmust be within the caller's service perimeter. The owning\nSemanticGovernancePolicyEngine's own project is always permitted\nimplicitly and need not be listed. Format: projects/{project} (ID or number)."]
    pub fn allowed_projects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_projects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_record` after provisioning.\nThe fully qualified record name of the A-record the backend writes into\n'dns_zone_name' for this gateway. Populated after the gateway reaches\n'ACTIVE'; empty until then."]
    pub fn dns_record(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_record", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dns_zone_name` after provisioning.\nThe name of the private Cloud DNS managed zone in which the backend\ncreates the DNS record set for this gateway's PSC endpoint. This is the\nmanaged-zone resource name, not a fully-qualified domain name. The zone\nmust already exist and be attached to the gateway's VPC at provision\ntime. The name must match '^[a-z0-9.-]{1,63}$'. Must be set together\nwith 'network' and 'subnetwork' (all three or none)."]
    pub fn dns_zone_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_zone_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe private IP address of the PSC endpoint. This field is currently\nalways empty and is slated for deprecation; do not depend on it."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe URI of the network resource where the gateway's PSC endpoint is\nprovisioned. Format: projects/{project}/global/networks/{network}.\n'network', 'subnetwork', and 'dns_zone_name' must all be set together\nor all omitted; setting only some is rejected by the API."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psc_endpoint` after provisioning.\nThe self-link or name of the Private Service Connect endpoint forwarding\nrule."]
    pub fn psc_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the Gateway configuration. One of: STATE_UNSPECIFIED,\nPROVISIONING, ACTIVE, DEPROVISIONING, INACTIVE, FAILED. A 'FAILED'\ngateway is surfaced here without a provider error; the engine as a\nwhole may still be 'ACTIVE'."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe URI of the subnetwork resource where the gateway's PSC endpoint is\nprovisioned. Format:\nprojects/{project}/regions/{region}/subnetworks/{subnetwork}. Must be\nset together with 'network' and 'dns_zone_name' (all three or none)."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiSemanticGovernancePolicyEngine_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiSemanticGovernancePolicyEngine_Timeouts {
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

impl terrars::ToListMappable for VertexAiSemanticGovernancePolicyEngine_Timeouts {
    type O = terrars::BlockAssignable<VertexAiSemanticGovernancePolicyEngine_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiSemanticGovernancePolicyEngine_Timeouts {}

impl BuildVertexAiSemanticGovernancePolicyEngine_Timeouts {
    pub fn build(self) -> VertexAiSemanticGovernancePolicyEngine_Timeouts {
        VertexAiSemanticGovernancePolicyEngine_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiSemanticGovernancePolicyEngine_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiSemanticGovernancePolicyEngine_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiSemanticGovernancePolicyEngine_TimeoutsRef {
        VertexAiSemanticGovernancePolicyEngine_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiSemanticGovernancePolicyEngine_TimeoutsRef {
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
