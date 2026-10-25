#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataAccessContextManagerSupportedServiceData_ {
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
    service_name: terrars::PrimField<std::string::String>,
}

struct DataAccessContextManagerSupportedService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccessContextManagerSupportedServiceData_>,
}

#[derive(Clone)]
pub struct DataAccessContextManagerSupportedService(std::rc::Rc<DataAccessContextManagerSupportedService_>);

impl DataAccessContextManagerSupportedService {
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
        "Get a reference to the value of field `available_on_restricted_vip` after provisioning.\nTrue if the service is available on the restricted VIP. Services on the restricted VIP typically either support VPC Service Controls or are core infrastructure services required for the functioning of Google Cloud."]
    pub fn available_on_restricted_vip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_on_restricted_vip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `known_limitations` after provisioning.\nTrue if the service is supported with some limitations. Check documentation for details."]
    pub fn known_limitations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.known_limitations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the service to get information about. The names must be in the same format as used in defining a service perimeter, for example, `storage.googleapis.com`."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_support_stage` after provisioning.\nThe support stage of the service. Values are `GA`, `PREVIEW`, and `DEPRECATED`."]
    pub fn service_support_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_support_stage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `support_stage` after provisioning.\nThe support stage of the service."]
    pub fn support_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_stage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_methods` after provisioning.\nThe list of supported methods for this service."]
    pub fn supported_methods(&self) -> terrars::ListRef<DataAccessContextManagerSupportedService_SupportedMethodsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_methods", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nThe name of the supported product, such as 'Cloud Storage'."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccessContextManagerSupportedService {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccessContextManagerSupportedService { }

impl terrars::ToListMappable for DataAccessContextManagerSupportedService {
    type O = terrars::ListRef<DataAccessContextManagerSupportedServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccessContextManagerSupportedService_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_access_context_manager_supported_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccessContextManagerSupportedService {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the service to get information about. The names must be in the same format as used in defining a service perimeter, for example, `storage.googleapis.com`."]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildDataAccessContextManagerSupportedService {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccessContextManagerSupportedService {
        let out =
            DataAccessContextManagerSupportedService(std::rc::Rc::new(DataAccessContextManagerSupportedService_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataAccessContextManagerSupportedServiceData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    id: core::default::Default::default(),
                    service_name: self.service_name,
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccessContextManagerSupportedServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccessContextManagerSupportedServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccessContextManagerSupportedServiceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `available_on_restricted_vip` after provisioning.\nTrue if the service is available on the restricted VIP. Services on the restricted VIP typically either support VPC Service Controls or are core infrastructure services required for the functioning of Google Cloud."]
    pub fn available_on_restricted_vip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_on_restricted_vip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `known_limitations` after provisioning.\nTrue if the service is supported with some limitations. Check documentation for details."]
    pub fn known_limitations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.known_limitations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_name` after provisioning.\nThe name of the service to get information about. The names must be in the same format as used in defining a service perimeter, for example, `storage.googleapis.com`."]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_support_stage` after provisioning.\nThe support stage of the service. Values are `GA`, `PREVIEW`, and `DEPRECATED`."]
    pub fn service_support_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_support_stage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `support_stage` after provisioning.\nThe support stage of the service."]
    pub fn support_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_stage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_methods` after provisioning.\nThe list of supported methods for this service."]
    pub fn supported_methods(&self) -> terrars::ListRef<DataAccessContextManagerSupportedService_SupportedMethodsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_methods", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nThe name of the supported product, such as 'Cloud Storage'."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccessContextManagerSupportedService_SupportedMethods {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccessContextManagerSupportedService_SupportedMethods {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permission`.\n"]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccessContextManagerSupportedService_SupportedMethods {
    type O = terrars::BlockAssignable<DataAccessContextManagerSupportedService_SupportedMethods>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccessContextManagerSupportedService_SupportedMethods {}

impl BuildDataAccessContextManagerSupportedService_SupportedMethods {
    pub fn build(self) -> DataAccessContextManagerSupportedService_SupportedMethods {
        DataAccessContextManagerSupportedService_SupportedMethods {
            method: core::default::Default::default(),
            permission: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccessContextManagerSupportedService_SupportedMethodsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccessContextManagerSupportedService_SupportedMethodsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccessContextManagerSupportedService_SupportedMethodsRef {
        DataAccessContextManagerSupportedService_SupportedMethodsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccessContextManagerSupportedService_SupportedMethodsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }
}
