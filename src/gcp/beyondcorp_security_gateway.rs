#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BeyondcorpSecurityGatewayData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    security_gateway_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hubs: std::option::Option<std::vec::Vec<BeyondcorpSecurityGateway_Hubs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<std::vec::Vec<BeyondcorpSecurityGateway_Logging>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_protocol_config: std::option::Option<std::vec::Vec<BeyondcorpSecurityGateway_ProxyProtocolConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_discovery: std::option::Option<std::vec::Vec<BeyondcorpSecurityGateway_ServiceDiscovery>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BeyondcorpSecurityGateway_Timeouts>,
    dynamic: BeyondcorpSecurityGatewayDynamic,
}

struct BeyondcorpSecurityGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BeyondcorpSecurityGatewayData_>,
}

#[derive(Clone)]
pub struct BeyondcorpSecurityGateway(std::rc::Rc<BeyondcorpSecurityGateway_>);

impl BeyondcorpSecurityGateway {
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

    #[doc =
        "Set the field `display_name`.\nOptional. An arbitrary user-provided name for the SecurityGateway.\nCannot exceed 64 characters."]
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
        "Set the field `location`.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Must be omitted or set to 'global'."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hubs`.\n"]
    pub fn set_hubs(self, v: impl std::convert::Into<terrars::BlockAssignable<BeyondcorpSecurityGateway_Hubs>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hubs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hubs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BeyondcorpSecurityGateway_Logging>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proxy_protocol_config`.\n"]
    pub fn set_proxy_protocol_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BeyondcorpSecurityGateway_ProxyProtocolConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy_protocol_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy_protocol_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_discovery`.\n"]
    pub fn set_service_discovery(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BeyondcorpSecurityGateway_ServiceDiscovery>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_discovery = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_discovery = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BeyondcorpSecurityGateway_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delegating_service_account` after provisioning.\nService account used for operations that involve resources in consumer projects."]
    pub fn delegating_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delegating_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. An arbitrary user-provided name for the SecurityGateway.\nCannot exceed 64 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_ips` after provisioning.\nOutput only. IP addresses that will be used for establishing\nconnection to the endpoints."]
    pub fn external_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Must be omitted or set to 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_gateway_id` after provisioning.\nOptional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn security_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_gateway_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The operational state of the SecurityGateway.\nPossible values:\nSTATE_UNSPECIFIED\nCREATING\nUPDATING\nDELETING\nRUNNING\nDOWN\nERROR"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the resource was last modified."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<BeyondcorpSecurityGateway_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_protocol_config` after provisioning.\n"]
    pub fn proxy_protocol_config(&self) -> terrars::ListRef<BeyondcorpSecurityGateway_ProxyProtocolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_protocol_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\n"]
    pub fn service_discovery(&self) -> terrars::ListRef<BeyondcorpSecurityGateway_ServiceDiscoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_discovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BeyondcorpSecurityGateway_TimeoutsRef {
        BeyondcorpSecurityGateway_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BeyondcorpSecurityGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BeyondcorpSecurityGateway { }

impl terrars::ToListMappable for BeyondcorpSecurityGateway {
    type O = terrars::ListRef<BeyondcorpSecurityGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BeyondcorpSecurityGateway_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_beyondcorp_security_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBeyondcorpSecurityGateway {
    pub tf_id: std::string::String,
    #[doc =
        "Optional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub security_gateway_id: terrars::PrimField<std::string::String>,
}

impl BuildBeyondcorpSecurityGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> BeyondcorpSecurityGateway {
        let out = BeyondcorpSecurityGateway(std::rc::Rc::new(BeyondcorpSecurityGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BeyondcorpSecurityGatewayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                security_gateway_id: self.security_gateway_id,
                hubs: core::default::Default::default(),
                logging: core::default::Default::default(),
                proxy_protocol_config: core::default::Default::default(),
                service_discovery: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BeyondcorpSecurityGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BeyondcorpSecurityGatewayRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delegating_service_account` after provisioning.\nService account used for operations that involve resources in consumer projects."]
    pub fn delegating_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delegating_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nOptional. An arbitrary user-provided name for the SecurityGateway.\nCannot exceed 64 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_ips` after provisioning.\nOutput only. IP addresses that will be used for establishing\nconnection to the endpoints."]
    pub fn external_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Must be omitted or set to 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_gateway_id` after provisioning.\nOptional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn security_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_gateway_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The operational state of the SecurityGateway.\nPossible values:\nSTATE_UNSPECIFIED\nCREATING\nUPDATING\nDELETING\nRUNNING\nDOWN\nERROR"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the resource was last modified."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<BeyondcorpSecurityGateway_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_protocol_config` after provisioning.\n"]
    pub fn proxy_protocol_config(&self) -> terrars::ListRef<BeyondcorpSecurityGateway_ProxyProtocolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_protocol_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\n"]
    pub fn service_discovery(&self) -> terrars::ListRef<BeyondcorpSecurityGateway_ServiceDiscoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_discovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BeyondcorpSecurityGateway_TimeoutsRef {
        BeyondcorpSecurityGateway_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayDynamic {
    hubs: std::option::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_Hubs>>,
    logging: std::option::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_Logging>>,
    proxy_protocol_config: std::option::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_ProxyProtocolConfig>>,
    service_discovery: std::option::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_ServiceDiscovery>>,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_Hubs_InternetGateway {}

impl BeyondcorpSecurityGateway_Hubs_InternetGateway { }

impl terrars::ToListMappable for BeyondcorpSecurityGateway_Hubs_InternetGateway {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_Hubs_InternetGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_Hubs_InternetGateway {}

impl BuildBeyondcorpSecurityGateway_Hubs_InternetGateway {
    pub fn build(self) -> BeyondcorpSecurityGateway_Hubs_InternetGateway {
        BeyondcorpSecurityGateway_Hubs_InternetGateway {}
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_Hubs_InternetGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_Hubs_InternetGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGateway_Hubs_InternetGatewayRef {
        BeyondcorpSecurityGateway_Hubs_InternetGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_Hubs_InternetGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `assigned_ips` after provisioning.\nOutput only. List of IP addresses assigned to the Cloud NAT."]
    pub fn assigned_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.assigned_ips", self.base))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGateway_HubsDynamic {
    internet_gateway: std::option::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_Hubs_InternetGateway>>,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_Hubs {
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internet_gateway: std::option::Option<std::vec::Vec<BeyondcorpSecurityGateway_Hubs_InternetGateway>>,
    dynamic: BeyondcorpSecurityGateway_HubsDynamic,
}

impl BeyondcorpSecurityGateway_Hubs {
    #[doc = "Set the field `internet_gateway`.\n"]
    pub fn set_internet_gateway(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BeyondcorpSecurityGateway_Hubs_InternetGateway>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.internet_gateway = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.internet_gateway = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGateway_Hubs {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_Hubs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_Hubs {
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildBeyondcorpSecurityGateway_Hubs {
    pub fn build(self) -> BeyondcorpSecurityGateway_Hubs {
        BeyondcorpSecurityGateway_Hubs {
            region: self.region,
            internet_gateway: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_HubsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_HubsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BeyondcorpSecurityGateway_HubsRef {
        BeyondcorpSecurityGateway_HubsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_HubsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `internet_gateway` after provisioning.\n"]
    pub fn internet_gateway(&self) -> terrars::ListRef<BeyondcorpSecurityGateway_Hubs_InternetGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internet_gateway", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_Logging {}

impl BeyondcorpSecurityGateway_Logging { }

impl terrars::ToListMappable for BeyondcorpSecurityGateway_Logging {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_Logging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_Logging {}

impl BuildBeyondcorpSecurityGateway_Logging {
    pub fn build(self) -> BeyondcorpSecurityGateway_Logging {
        BeyondcorpSecurityGateway_Logging {}
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_LoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_LoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BeyondcorpSecurityGateway_LoggingRef {
        BeyondcorpSecurityGateway_LoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_LoggingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
    #[doc =
        "Set the field `output_type`.\nThe output type of the delegated device info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {}

impl BuildBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
    pub fn build(self) -> BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
        BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
            output_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef {
        BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated device info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
    #[doc =
        "Set the field `output_type`.\nThe output type of the delegated group info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {}

impl BuildBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
    pub fn build(self) -> BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
        BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
            output_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef {
        BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated group info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
    #[doc =
        "Set the field `output_type`.\nThe output type of the delegated user info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {}

impl BuildBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
    pub fn build(self) -> BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
        BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
            output_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef {
        BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated user info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersDynamic {
    device_info: std
    ::option
    ::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo>>,
    group_info: std
    ::option
    ::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo>>,
    user_info: std
    ::option
    ::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo>>,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_info: std
    ::option
    ::Option<std::vec::Vec<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_info: std
    ::option
    ::Option<std::vec::Vec<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_info: std
    ::option
    ::Option<std::vec::Vec<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo>>,
    dynamic: BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersDynamic,
}

impl BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
    #[doc =
        "Set the field `output_type`.\nDefault output type for all enabled headers. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_info`.\n"]
    pub fn set_device_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.device_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.device_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `group_info`.\n"]
    pub fn set_group_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.group_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.group_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_info`.\n"]
    pub fn set_user_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {}

impl BuildBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
    pub fn build(self) -> BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
        BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
            output_type: core::default::Default::default(),
            device_info: core::default::Default::default(),
            group_info: core::default::Default::default(),
            user_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef {
        BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `output_type` after provisioning.\nDefault output type for all enabled headers. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }

    #[doc = "Get a reference to the value of field `device_info` after provisioning.\n"]
    pub fn device_info(
        &self,
    ) -> terrars::ListRef<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_info", self.base))
    }

    #[doc = "Get a reference to the value of field `group_info` after provisioning.\n"]
    pub fn group_info(
        &self,
    ) -> terrars::ListRef<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_info", self.base))
    }

    #[doc = "Get a reference to the value of field `user_info` after provisioning.\n"]
    pub fn user_info(
        &self,
    ) -> terrars::ListRef<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_info", self.base))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGateway_ProxyProtocolConfigDynamic {
    contextual_headers: std
    ::option
    ::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders>>,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_ProxyProtocolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_client_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_identity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contextual_headers: std
    ::option
    ::Option<std::vec::Vec<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders>>,
    dynamic: BeyondcorpSecurityGateway_ProxyProtocolConfigDynamic,
}

impl BeyondcorpSecurityGateway_ProxyProtocolConfig {
    #[doc = "Set the field `allowed_client_headers`.\nThe configuration for the proxy."]
    pub fn set_allowed_client_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_client_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_ip`.\nClient IP configuration. The client IP address is included if true."]
    pub fn set_client_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.client_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_identity`.\nGateway identity configuration. Possible values: [\"RESOURCE_NAME\"]"]
    pub fn set_gateway_identity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gateway_identity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metadata_headers`.\nCustom resource specific headers along with the values.\nThe names should conform to RFC 9110:\n> Field names SHOULD constrain themselves to alphanumeric characters, \"-\",\n  and \".\", and SHOULD begin with a letter.\n> Field values SHOULD contain only ASCII printable characters and tab."]
    pub fn set_metadata_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contextual_headers`.\n"]
    pub fn set_contextual_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.contextual_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.contextual_headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGateway_ProxyProtocolConfig {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_ProxyProtocolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_ProxyProtocolConfig {}

impl BuildBeyondcorpSecurityGateway_ProxyProtocolConfig {
    pub fn build(self) -> BeyondcorpSecurityGateway_ProxyProtocolConfig {
        BeyondcorpSecurityGateway_ProxyProtocolConfig {
            allowed_client_headers: core::default::Default::default(),
            client_ip: core::default::Default::default(),
            gateway_identity: core::default::Default::default(),
            metadata_headers: core::default::Default::default(),
            contextual_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_ProxyProtocolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_ProxyProtocolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGateway_ProxyProtocolConfigRef {
        BeyondcorpSecurityGateway_ProxyProtocolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_ProxyProtocolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_client_headers` after provisioning.\nThe configuration for the proxy."]
    pub fn allowed_client_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_client_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_ip` after provisioning.\nClient IP configuration. The client IP address is included if true."]
    pub fn client_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gateway_identity` after provisioning.\nGateway identity configuration. Possible values: [\"RESOURCE_NAME\"]"]
    pub fn gateway_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_identity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata_headers` after provisioning.\nCustom resource specific headers along with the values.\nThe names should conform to RFC 9110:\n> Field names SHOULD constrain themselves to alphanumeric characters, \"-\",\n  and \".\", and SHOULD begin with a letter.\n> Field values SHOULD contain only ASCII printable characters and tab."]
    pub fn metadata_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `contextual_headers` after provisioning.\n"]
    pub fn contextual_headers(
        &self,
    ) -> terrars::ListRef<BeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contextual_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
    #[doc = "Set the field `path`.\nContains uri path fragment where HTTP request is sent."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {}

impl BuildBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
    pub fn build(self) -> BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
        BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef {
        BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nContains uri path fragment where HTTP request is sent."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayDynamic {
    resource_override: std
    ::option
    ::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride>>,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_override: std
    ::option
    ::Option<std::vec::Vec<BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride>>,
    dynamic: BeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayDynamic,
}

impl BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
    #[doc = "Set the field `resource_override`.\n"]
    pub fn set_resource_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {}

impl BuildBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
    pub fn build(self) -> BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
        BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
            resource_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef {
        BeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_override` after provisioning.\n"]
    pub fn resource_override(
        &self,
    ) -> terrars::ListRef<BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_override", self.base))
    }
}

#[derive(Serialize, Default)]
struct BeyondcorpSecurityGateway_ServiceDiscoveryDynamic {
    api_gateway: std::option::Option<terrars::DynamicBlock<BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway>>,
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_ServiceDiscovery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_gateway: std::option::Option<std::vec::Vec<BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway>>,
    dynamic: BeyondcorpSecurityGateway_ServiceDiscoveryDynamic,
}

impl BeyondcorpSecurityGateway_ServiceDiscovery {
    #[doc = "Set the field `api_gateway`.\n"]
    pub fn set_api_gateway(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_gateway = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_gateway = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BeyondcorpSecurityGateway_ServiceDiscovery {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_ServiceDiscovery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_ServiceDiscovery {}

impl BuildBeyondcorpSecurityGateway_ServiceDiscovery {
    pub fn build(self) -> BeyondcorpSecurityGateway_ServiceDiscovery {
        BeyondcorpSecurityGateway_ServiceDiscovery {
            api_gateway: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_ServiceDiscoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_ServiceDiscoveryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BeyondcorpSecurityGateway_ServiceDiscoveryRef {
        BeyondcorpSecurityGateway_ServiceDiscoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_ServiceDiscoveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_gateway` after provisioning.\n"]
    pub fn api_gateway(&self) -> terrars::ListRef<BeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_gateway", self.base))
    }
}

#[derive(Serialize)]
pub struct BeyondcorpSecurityGateway_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BeyondcorpSecurityGateway_Timeouts {
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

impl terrars::ToListMappable for BeyondcorpSecurityGateway_Timeouts {
    type O = terrars::BlockAssignable<BeyondcorpSecurityGateway_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBeyondcorpSecurityGateway_Timeouts {}

impl BuildBeyondcorpSecurityGateway_Timeouts {
    pub fn build(self) -> BeyondcorpSecurityGateway_Timeouts {
        BeyondcorpSecurityGateway_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BeyondcorpSecurityGateway_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BeyondcorpSecurityGateway_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BeyondcorpSecurityGateway_TimeoutsRef {
        BeyondcorpSecurityGateway_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BeyondcorpSecurityGateway_TimeoutsRef {
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
