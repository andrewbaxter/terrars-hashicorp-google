#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBeyondcorpSecurityGatewayData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    security_gateway_id: terrars::PrimField<std::string::String>,
}

struct DataBeyondcorpSecurityGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBeyondcorpSecurityGatewayData_>,
}

#[derive(Clone)]
pub struct DataBeyondcorpSecurityGateway(std::rc::Rc<DataBeyondcorpSecurityGateway_>);

impl DataBeyondcorpSecurityGateway {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
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

    #[doc =
        "Get a reference to the value of field `hubs` after provisioning.\nOptional. Map of Hubs that represents regional data path deployment with GCP region\nas a key."]
    pub fn hubs(&self) -> terrars::SetRef<DataBeyondcorpSecurityGateway_HubsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hubs", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `logging` after provisioning.\nSettings related to Cloud Logging."]
    pub fn logging(&self) -> terrars::ListRef<DataBeyondcorpSecurityGateway_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
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
        "Get a reference to the value of field `proxy_protocol_config` after provisioning.\nShared proxy configuration for all apps."]
    pub fn proxy_protocol_config(&self) -> terrars::ListRef<DataBeyondcorpSecurityGateway_ProxyProtocolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_protocol_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_gateway_id` after provisioning.\nOptional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn security_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_gateway_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_discovery` after provisioning.\nSettings related to the Service Discovery."]
    pub fn service_discovery(&self) -> terrars::ListRef<DataBeyondcorpSecurityGateway_ServiceDiscoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_discovery", self.extract_ref()))
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
}

impl terrars::Referable for DataBeyondcorpSecurityGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBeyondcorpSecurityGateway { }

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway {
    type O = terrars::ListRef<DataBeyondcorpSecurityGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBeyondcorpSecurityGateway_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_beyondcorp_security_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBeyondcorpSecurityGateway {
    pub tf_id: std::string::String,
    #[doc =
        "Optional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub security_gateway_id: terrars::PrimField<std::string::String>,
}

impl BuildDataBeyondcorpSecurityGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBeyondcorpSecurityGateway {
        let out = DataBeyondcorpSecurityGateway(std::rc::Rc::new(DataBeyondcorpSecurityGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBeyondcorpSecurityGatewayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                security_gateway_id: self.security_gateway_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBeyondcorpSecurityGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBeyondcorpSecurityGatewayRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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

    #[doc =
        "Get a reference to the value of field `hubs` after provisioning.\nOptional. Map of Hubs that represents regional data path deployment with GCP region\nas a key."]
    pub fn hubs(&self) -> terrars::SetRef<DataBeyondcorpSecurityGateway_HubsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.hubs", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `logging` after provisioning.\nSettings related to Cloud Logging."]
    pub fn logging(&self) -> terrars::ListRef<DataBeyondcorpSecurityGateway_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
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
        "Get a reference to the value of field `proxy_protocol_config` after provisioning.\nShared proxy configuration for all apps."]
    pub fn proxy_protocol_config(&self) -> terrars::ListRef<DataBeyondcorpSecurityGateway_ProxyProtocolConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_protocol_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_gateway_id` after provisioning.\nOptional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn security_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_gateway_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_discovery` after provisioning.\nSettings related to the Service Discovery."]
    pub fn service_discovery(&self) -> terrars::ListRef<DataBeyondcorpSecurityGateway_ServiceDiscoveryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_discovery", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_Hubs_InternetGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assigned_ips: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataBeyondcorpSecurityGateway_Hubs_InternetGateway {
    #[doc = "Set the field `assigned_ips`.\n"]
    pub fn set_assigned_ips(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.assigned_ips = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_Hubs_InternetGateway {
    type O = terrars::BlockAssignable<DataBeyondcorpSecurityGateway_Hubs_InternetGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_Hubs_InternetGateway {}

impl BuildDataBeyondcorpSecurityGateway_Hubs_InternetGateway {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_Hubs_InternetGateway {
        DataBeyondcorpSecurityGateway_Hubs_InternetGateway { assigned_ips: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_Hubs_InternetGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_Hubs_InternetGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBeyondcorpSecurityGateway_Hubs_InternetGatewayRef {
        DataBeyondcorpSecurityGateway_Hubs_InternetGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_Hubs_InternetGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assigned_ips` after provisioning.\n"]
    pub fn assigned_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.assigned_ips", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_Hubs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internet_gateway: std::option::Option<terrars::ListField<DataBeyondcorpSecurityGateway_Hubs_InternetGateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBeyondcorpSecurityGateway_Hubs {
    #[doc = "Set the field `internet_gateway`.\n"]
    pub fn set_internet_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBeyondcorpSecurityGateway_Hubs_InternetGateway>>,
    ) -> Self {
        self.internet_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_Hubs {
    type O = terrars::BlockAssignable<DataBeyondcorpSecurityGateway_Hubs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_Hubs {}

impl BuildDataBeyondcorpSecurityGateway_Hubs {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_Hubs {
        DataBeyondcorpSecurityGateway_Hubs {
            internet_gateway: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_HubsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_HubsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBeyondcorpSecurityGateway_HubsRef {
        DataBeyondcorpSecurityGateway_HubsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_HubsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `internet_gateway` after provisioning.\n"]
    pub fn internet_gateway(&self) -> terrars::ListRef<DataBeyondcorpSecurityGateway_Hubs_InternetGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internet_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_Logging {}

impl DataBeyondcorpSecurityGateway_Logging { }

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_Logging {
    type O = terrars::BlockAssignable<DataBeyondcorpSecurityGateway_Logging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_Logging {}

impl BuildDataBeyondcorpSecurityGateway_Logging {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_Logging {
        DataBeyondcorpSecurityGateway_Logging {}
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_LoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_LoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBeyondcorpSecurityGateway_LoggingRef {
        DataBeyondcorpSecurityGateway_LoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_LoggingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
    #[doc = "Set the field `output_type`.\n"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
    type O =
        terrars::BlockAssignable<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {}

impl BuildDataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
        DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo {
            output_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef {
        DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
    #[doc = "Set the field `output_type`.\n"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
    type O = terrars::BlockAssignable<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {}

impl BuildDataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
        DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo {
            output_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef {
        DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
    #[doc = "Set the field `output_type`.\n"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
    type O = terrars::BlockAssignable<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {}

impl BuildDataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
        DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo {
            output_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef {
        DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_info: std
    ::option
    ::Option<terrars::ListField<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_info: std
    ::option
    ::Option<terrars::ListField<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_info: std
    ::option
    ::Option<terrars::ListField<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo>>,
}

impl DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
    #[doc = "Set the field `device_info`.\n"]
    pub fn set_device_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfo>,
                    >,
    ) -> Self {
        self.device_info = std::option::Option::Some(v.into());
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
                        ::ListField<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfo>,
                    >,
    ) -> Self {
        self.group_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_type`.\n"]
    pub fn set_output_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.output_type = std::option::Option::Some(v.into());
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
                        ::ListField<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfo>,
                    >,
    ) -> Self {
        self.user_info = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
    type O = terrars::BlockAssignable<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {}

impl BuildDataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
        DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders {
            device_info: core::default::Default::default(),
            group_info: core::default::Default::default(),
            output_type: core::default::Default::default(),
            user_info: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef {
        DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_info` after provisioning.\n"]
    pub fn device_info(
        &self,
    ) -> terrars::ListRef<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_DeviceInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_info", self.base))
    }

    #[doc = "Get a reference to the value of field `group_info` after provisioning.\n"]
    pub fn group_info(
        &self,
    ) -> terrars::ListRef<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_GroupInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_info", self.base))
    }

    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }

    #[doc = "Get a reference to the value of field `user_info` after provisioning.\n"]
    pub fn user_info(
        &self,
    ) -> terrars::ListRef<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders_UserInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_ProxyProtocolConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_client_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contextual_headers: std
    ::option
    ::Option<terrars::ListField<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_identity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataBeyondcorpSecurityGateway_ProxyProtocolConfig {
    #[doc = "Set the field `allowed_client_headers`.\n"]
    pub fn set_allowed_client_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_client_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_ip`.\n"]
    pub fn set_client_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.client_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contextual_headers`.\n"]
    pub fn set_contextual_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeaders>>,
    ) -> Self {
        self.contextual_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_identity`.\n"]
    pub fn set_gateway_identity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gateway_identity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_headers`.\n"]
    pub fn set_metadata_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata_headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_ProxyProtocolConfig {
    type O = terrars::BlockAssignable<DataBeyondcorpSecurityGateway_ProxyProtocolConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_ProxyProtocolConfig {}

impl BuildDataBeyondcorpSecurityGateway_ProxyProtocolConfig {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_ProxyProtocolConfig {
        DataBeyondcorpSecurityGateway_ProxyProtocolConfig {
            allowed_client_headers: core::default::Default::default(),
            client_ip: core::default::Default::default(),
            contextual_headers: core::default::Default::default(),
            gateway_identity: core::default::Default::default(),
            metadata_headers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_ProxyProtocolConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_ProxyProtocolConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBeyondcorpSecurityGateway_ProxyProtocolConfigRef {
        DataBeyondcorpSecurityGateway_ProxyProtocolConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_ProxyProtocolConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_client_headers` after provisioning.\n"]
    pub fn allowed_client_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_client_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `client_ip` after provisioning.\n"]
    pub fn client_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `contextual_headers` after provisioning.\n"]
    pub fn contextual_headers(
        &self,
    ) -> terrars::ListRef<DataBeyondcorpSecurityGateway_ProxyProtocolConfig_ContextualHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contextual_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_identity` after provisioning.\n"]
    pub fn gateway_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_identity", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_headers` after provisioning.\n"]
    pub fn metadata_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
    type O = terrars::BlockAssignable<DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {}

impl BuildDataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
        DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride {
            path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef {
        DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_override: std
    ::option
    ::Option<terrars::ListField<DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride>>,
}

impl DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
    #[doc = "Set the field `resource_override`.\n"]
    pub fn set_resource_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverride>,
                    >,
    ) -> Self {
        self.resource_override = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
    type O = terrars::BlockAssignable<DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {}

impl BuildDataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
        DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway {
            resource_override: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef {
        DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_override` after provisioning.\n"]
    pub fn resource_override(
        &self,
    ) -> terrars::ListRef<DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway_ResourceOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_override", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGateway_ServiceDiscovery {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_gateway: std::option::Option<terrars::ListField<DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway>>,
}

impl DataBeyondcorpSecurityGateway_ServiceDiscovery {
    #[doc = "Set the field `api_gateway`.\n"]
    pub fn set_api_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGateway>>,
    ) -> Self {
        self.api_gateway = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBeyondcorpSecurityGateway_ServiceDiscovery {
    type O = terrars::BlockAssignable<DataBeyondcorpSecurityGateway_ServiceDiscovery>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBeyondcorpSecurityGateway_ServiceDiscovery {}

impl BuildDataBeyondcorpSecurityGateway_ServiceDiscovery {
    pub fn build(self) -> DataBeyondcorpSecurityGateway_ServiceDiscovery {
        DataBeyondcorpSecurityGateway_ServiceDiscovery { api_gateway: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBeyondcorpSecurityGateway_ServiceDiscoveryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBeyondcorpSecurityGateway_ServiceDiscoveryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBeyondcorpSecurityGateway_ServiceDiscoveryRef {
        DataBeyondcorpSecurityGateway_ServiceDiscoveryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBeyondcorpSecurityGateway_ServiceDiscoveryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_gateway` after provisioning.\n"]
    pub fn api_gateway(&self) -> terrars::ListRef<DataBeyondcorpSecurityGateway_ServiceDiscovery_ApiGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_gateway", self.base))
    }
}
