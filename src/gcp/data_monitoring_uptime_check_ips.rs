#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataMonitoringUptimeCheckIpsData_ {
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

struct DataMonitoringUptimeCheckIps_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMonitoringUptimeCheckIpsData_>,
}

#[derive(Clone)]
pub struct DataMonitoringUptimeCheckIps(std::rc::Rc<DataMonitoringUptimeCheckIps_>);

impl DataMonitoringUptimeCheckIps {
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

    #[doc = "Get a reference to the value of field `uptime_check_ips` after provisioning.\n"]
    pub fn uptime_check_ips(&self) -> terrars::ListRef<DataMonitoringUptimeCheckIps_UptimeCheckIpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uptime_check_ips", self.extract_ref()))
    }
}

impl terrars::Referable for DataMonitoringUptimeCheckIps {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMonitoringUptimeCheckIps { }

impl terrars::ToListMappable for DataMonitoringUptimeCheckIps {
    type O = terrars::ListRef<DataMonitoringUptimeCheckIpsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMonitoringUptimeCheckIps_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_monitoring_uptime_check_ips".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMonitoringUptimeCheckIps {
    pub tf_id: std::string::String,
}

impl BuildDataMonitoringUptimeCheckIps {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMonitoringUptimeCheckIps {
        let out = DataMonitoringUptimeCheckIps(std::rc::Rc::new(DataMonitoringUptimeCheckIps_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMonitoringUptimeCheckIpsData_ {
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

pub struct DataMonitoringUptimeCheckIpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMonitoringUptimeCheckIpsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMonitoringUptimeCheckIpsRef {
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

    #[doc = "Get a reference to the value of field `uptime_check_ips` after provisioning.\n"]
    pub fn uptime_check_ips(&self) -> terrars::ListRef<DataMonitoringUptimeCheckIps_UptimeCheckIpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uptime_check_ips", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMonitoringUptimeCheckIps_UptimeCheckIps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMonitoringUptimeCheckIps_UptimeCheckIps {
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMonitoringUptimeCheckIps_UptimeCheckIps {
    type O = terrars::BlockAssignable<DataMonitoringUptimeCheckIps_UptimeCheckIps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMonitoringUptimeCheckIps_UptimeCheckIps {}

impl BuildDataMonitoringUptimeCheckIps_UptimeCheckIps {
    pub fn build(self) -> DataMonitoringUptimeCheckIps_UptimeCheckIps {
        DataMonitoringUptimeCheckIps_UptimeCheckIps {
            ip_address: core::default::Default::default(),
            location: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMonitoringUptimeCheckIps_UptimeCheckIpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMonitoringUptimeCheckIps_UptimeCheckIpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMonitoringUptimeCheckIps_UptimeCheckIpsRef {
        DataMonitoringUptimeCheckIps_UptimeCheckIpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMonitoringUptimeCheckIps_UptimeCheckIpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
