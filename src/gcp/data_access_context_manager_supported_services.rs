#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataAccessContextManagerSupportedServicesData_ {
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
}

struct DataAccessContextManagerSupportedServices_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccessContextManagerSupportedServicesData_>,
}

#[derive(Clone)]
pub struct DataAccessContextManagerSupportedServices(std::rc::Rc<DataAccessContextManagerSupportedServices_>);

impl DataAccessContextManagerSupportedServices {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_services` after provisioning.\nThe list of VPC-SC supported services."]
    pub fn supported_services(&self) -> terrars::ListRef<DataAccessContextManagerSupportedServices_SupportedServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_services", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccessContextManagerSupportedServices {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccessContextManagerSupportedServices { }

impl terrars::ToListMappable for DataAccessContextManagerSupportedServices {
    type O = terrars::ListRef<DataAccessContextManagerSupportedServicesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccessContextManagerSupportedServices_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_access_context_manager_supported_services".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccessContextManagerSupportedServices {
    pub tf_id: std::string::String,
}

impl BuildDataAccessContextManagerSupportedServices {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccessContextManagerSupportedServices {
        let out =
            DataAccessContextManagerSupportedServices(std::rc::Rc::new(DataAccessContextManagerSupportedServices_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataAccessContextManagerSupportedServicesData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    id: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccessContextManagerSupportedServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccessContextManagerSupportedServicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccessContextManagerSupportedServicesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_services` after provisioning.\nThe list of VPC-SC supported services."]
    pub fn supported_services(&self) -> terrars::ListRef<DataAccessContextManagerSupportedServices_SupportedServicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_services", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccessContextManagerSupportedServices_SupportedServices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_on_restricted_vip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    known_limitations: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_support_stage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_stage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccessContextManagerSupportedServices_SupportedServices {
    #[doc = "Set the field `available_on_restricted_vip`.\n"]
    pub fn set_available_on_restricted_vip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.available_on_restricted_vip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `known_limitations`.\n"]
    pub fn set_known_limitations(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.known_limitations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_support_stage`.\n"]
    pub fn set_service_support_stage(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_support_stage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `support_stage`.\n"]
    pub fn set_support_stage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.support_stage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccessContextManagerSupportedServices_SupportedServices {
    type O = terrars::BlockAssignable<DataAccessContextManagerSupportedServices_SupportedServices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccessContextManagerSupportedServices_SupportedServices {}

impl BuildDataAccessContextManagerSupportedServices_SupportedServices {
    pub fn build(self) -> DataAccessContextManagerSupportedServices_SupportedServices {
        DataAccessContextManagerSupportedServices_SupportedServices {
            available_on_restricted_vip: core::default::Default::default(),
            known_limitations: core::default::Default::default(),
            name: core::default::Default::default(),
            service_support_stage: core::default::Default::default(),
            support_stage: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccessContextManagerSupportedServices_SupportedServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccessContextManagerSupportedServices_SupportedServicesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAccessContextManagerSupportedServices_SupportedServicesRef {
        DataAccessContextManagerSupportedServices_SupportedServicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccessContextManagerSupportedServices_SupportedServicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_on_restricted_vip` after provisioning.\n"]
    pub fn available_on_restricted_vip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_on_restricted_vip", self.base))
    }

    #[doc = "Get a reference to the value of field `known_limitations` after provisioning.\n"]
    pub fn known_limitations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.known_limitations", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_support_stage` after provisioning.\n"]
    pub fn service_support_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_support_stage", self.base))
    }

    #[doc = "Get a reference to the value of field `support_stage` after provisioning.\n"]
    pub fn support_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_stage", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
