#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataOracleDatabaseGoldengateDeploymentEnvironmentsData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOracleDatabaseGoldengateDeploymentEnvironments_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOracleDatabaseGoldengateDeploymentEnvironmentsData_>,
}

#[derive(Clone)]
pub struct DataOracleDatabaseGoldengateDeploymentEnvironments(
    std::rc::Rc<DataOracleDatabaseGoldengateDeploymentEnvironments_>,
);

impl DataOracleDatabaseGoldengateDeploymentEnvironments {
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
        "Set the field `project`.\nThe ID of the project in which the resource is located. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `goldengate_deployment_environments` after provisioning.\n"]
    pub fn goldengate_deployment_environments(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironmentsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.goldengate_deployment_environments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

impl terrars::Referable for DataOracleDatabaseGoldengateDeploymentEnvironments {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOracleDatabaseGoldengateDeploymentEnvironments { }

impl terrars::ToListMappable for DataOracleDatabaseGoldengateDeploymentEnvironments {
    type O = terrars::ListRef<DataOracleDatabaseGoldengateDeploymentEnvironmentsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOracleDatabaseGoldengateDeploymentEnvironments_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_oracle_database_goldengate_deployment_environments".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOracleDatabaseGoldengateDeploymentEnvironments {
    pub tf_id: std::string::String,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataOracleDatabaseGoldengateDeploymentEnvironments {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOracleDatabaseGoldengateDeploymentEnvironments {
        let out =
            DataOracleDatabaseGoldengateDeploymentEnvironments(
                std::rc::Rc::new(DataOracleDatabaseGoldengateDeploymentEnvironments_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataOracleDatabaseGoldengateDeploymentEnvironmentsData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        id: core::default::Default::default(),
                        location: self.location,
                        project: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOracleDatabaseGoldengateDeploymentEnvironmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseGoldengateDeploymentEnvironmentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOracleDatabaseGoldengateDeploymentEnvironmentsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `goldengate_deployment_environments` after provisioning.\n"]
    pub fn goldengate_deployment_environments(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironmentsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.goldengate_deployment_environments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource is located. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    category: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb_per_cpu_core: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_bandwidth_gbps_per_cpu_core: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_usage_limit_gb_per_cpu_core: std::option::Option<terrars::PrimField<f64>>,
}

impl DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironments {
    #[doc = "Set the field `auto_scaling_enabled`.\n"]
    pub fn set_auto_scaling_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_scaling_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_cpu_core_count`.\n"]
    pub fn set_default_cpu_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_type`.\n"]
    pub fn set_environment_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_cpu_core_count`.\n"]
    pub fn set_max_cpu_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb_per_cpu_core`.\n"]
    pub fn set_memory_gb_per_cpu_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb_per_cpu_core = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_cpu_core_count`.\n"]
    pub fn set_min_cpu_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_bandwidth_gbps_per_cpu_core`.\n"]
    pub fn set_network_bandwidth_gbps_per_cpu_core(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.network_bandwidth_gbps_per_cpu_core = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_usage_limit_gb_per_cpu_core`.\n"]
    pub fn set_storage_usage_limit_gb_per_cpu_core(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.storage_usage_limit_gb_per_cpu_core = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironments {
    type O =
        terrars
        ::BlockAssignable<DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironments {}

impl BuildDataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironments {
    pub fn build(self) -> DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironments {
        DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironments {
            auto_scaling_enabled: core::default::Default::default(),
            category: core::default::Default::default(),
            default_cpu_core_count: core::default::Default::default(),
            display_name: core::default::Default::default(),
            environment_type: core::default::Default::default(),
            max_cpu_core_count: core::default::Default::default(),
            memory_gb_per_cpu_core: core::default::Default::default(),
            min_cpu_core_count: core::default::Default::default(),
            name: core::default::Default::default(),
            network_bandwidth_gbps_per_cpu_core: core::default::Default::default(),
            storage_usage_limit_gb_per_cpu_core: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironmentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironmentsRef {
        DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseGoldengateDeploymentEnvironments_GoldengateDeploymentEnvironmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_scaling_enabled` after provisioning.\n"]
    pub fn auto_scaling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `default_cpu_core_count` after provisioning.\n"]
    pub fn default_cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_cpu_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_type` after provisioning.\n"]
    pub fn environment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `max_cpu_core_count` after provisioning.\n"]
    pub fn max_cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_cpu_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gb_per_cpu_core` after provisioning.\n"]
    pub fn memory_gb_per_cpu_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb_per_cpu_core", self.base))
    }

    #[doc = "Get a reference to the value of field `min_cpu_core_count` after provisioning.\n"]
    pub fn min_cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `network_bandwidth_gbps_per_cpu_core` after provisioning.\n"]
    pub fn network_bandwidth_gbps_per_cpu_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_bandwidth_gbps_per_cpu_core", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_usage_limit_gb_per_cpu_core` after provisioning.\n"]
    pub fn storage_usage_limit_gb_per_cpu_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_usage_limit_gb_per_cpu_core", self.base))
    }
}
