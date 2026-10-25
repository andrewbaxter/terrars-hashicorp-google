#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataOracleDatabaseGoldengateDeploymentVersionsData_ {
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

struct DataOracleDatabaseGoldengateDeploymentVersions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOracleDatabaseGoldengateDeploymentVersionsData_>,
}

#[derive(Clone)]
pub struct DataOracleDatabaseGoldengateDeploymentVersions(
    std::rc::Rc<DataOracleDatabaseGoldengateDeploymentVersions_>,
);

impl DataOracleDatabaseGoldengateDeploymentVersions {
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

    #[doc = "Get a reference to the value of field `goldengate_deployment_versions` after provisioning.\n"]
    pub fn goldengate_deployment_versions(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.goldengate_deployment_versions", self.extract_ref()),
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

impl terrars::Referable for DataOracleDatabaseGoldengateDeploymentVersions {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOracleDatabaseGoldengateDeploymentVersions { }

impl terrars::ToListMappable for DataOracleDatabaseGoldengateDeploymentVersions {
    type O = terrars::ListRef<DataOracleDatabaseGoldengateDeploymentVersionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOracleDatabaseGoldengateDeploymentVersions_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_oracle_database_goldengate_deployment_versions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOracleDatabaseGoldengateDeploymentVersions {
    pub tf_id: std::string::String,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataOracleDatabaseGoldengateDeploymentVersions {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOracleDatabaseGoldengateDeploymentVersions {
        let out =
            DataOracleDatabaseGoldengateDeploymentVersions(
                std::rc::Rc::new(DataOracleDatabaseGoldengateDeploymentVersions_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataOracleDatabaseGoldengateDeploymentVersionsData_ {
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

pub struct DataOracleDatabaseGoldengateDeploymentVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseGoldengateDeploymentVersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOracleDatabaseGoldengateDeploymentVersionsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `goldengate_deployment_versions` after provisioning.\n"]
    pub fn goldengate_deployment_versions(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.goldengate_deployment_versions", self.extract_ref()),
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
pub struct DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ogg_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_fix: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    support_end_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_Properties {
    #[doc = "Set the field `deployment_type`.\n"]
    pub fn set_deployment_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployment_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ogg_version`.\n"]
    pub fn set_ogg_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ogg_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `release_time`.\n"]
    pub fn set_release_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.release_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `release_type`.\n"]
    pub fn set_release_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.release_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_fix`.\n"]
    pub fn set_security_fix(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.security_fix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `support_end_time`.\n"]
    pub fn set_support_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.support_end_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_Properties {
    type O =
        terrars
        ::BlockAssignable<DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_Properties {}

impl BuildDataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_Properties {
    pub fn build(self) -> DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_Properties {
        DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_Properties {
            deployment_type: core::default::Default::default(),
            ogg_version: core::default::Default::default(),
            release_time: core::default::Default::default(),
            release_type: core::default::Default::default(),
            security_fix: core::default::Default::default(),
            support_end_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_PropertiesRef {
        DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ogg_version` after provisioning.\n"]
    pub fn ogg_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ogg_version", self.base))
    }

    #[doc = "Get a reference to the value of field `release_time` after provisioning.\n"]
    pub fn release_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_time", self.base))
    }

    #[doc = "Get a reference to the value of field `release_type` after provisioning.\n"]
    pub fn release_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_type", self.base))
    }

    #[doc = "Get a reference to the value of field `security_fix` after provisioning.\n"]
    pub fn security_fix(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_fix", self.base))
    }

    #[doc = "Get a reference to the value of field `support_end_time` after provisioning.\n"]
    pub fn support_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.support_end_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std
    ::option
    ::Option<
        terrars::ListField<DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_Properties>,
    >,
}

impl DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ocid`.\n"]
    pub fn set_ocid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ocid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_Properties,
                        >,
                    >,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions {
    type O = terrars::BlockAssignable<DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions {}

impl BuildDataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions {
    pub fn build(self) -> DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions {
        DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions {
            name: core::default::Default::default(),
            ocid: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersionsRef {
        DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `ocid` after provisioning.\n"]
    pub fn ocid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocid", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(
        &self,
    ) -> terrars::ListRef<DataOracleDatabaseGoldengateDeploymentVersions_GoldengateDeploymentVersions_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}
