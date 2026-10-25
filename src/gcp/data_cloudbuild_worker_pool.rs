#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudbuildWorkerPoolData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCloudbuildWorkerPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudbuildWorkerPoolData_>,
}

#[derive(Clone)]
pub struct DataCloudbuildWorkerPool(std::rc::Rc<DataCloudbuildWorkerPool_>);

impl DataCloudbuildWorkerPool {
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

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the request to create the `WorkerPool` was received."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. Time at which the request to delete the `WorkerPool` was received."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser-defined name of the `WorkerPool`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_config` after provisioning.\nNetwork configuration for the `WorkerPool`."]
    pub fn network_config(&self) -> terrars::ListRef<DataCloudbuildWorkerPool_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_service_connect` after provisioning.\nPrivate Service Connect configuration for the pool."]
    pub fn private_service_connect(&self) -> terrars::ListRef<DataCloudbuildWorkerPool_PrivateServiceConnectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_service_connect", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. `WorkerPool` state. Possible values: STATE_UNSPECIFIED, PENDING, APPROVED, REJECTED, CANCELLED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. A unique identifier for the `WorkerPool`."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the request to update the `WorkerPool` was received."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `worker_config` after provisioning.\nConfiguration to be used for a creating workers in the `WorkerPool`."]
    pub fn worker_config(&self) -> terrars::ListRef<DataCloudbuildWorkerPool_WorkerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudbuildWorkerPool {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudbuildWorkerPool { }

impl terrars::ToListMappable for DataCloudbuildWorkerPool {
    type O = terrars::ListRef<DataCloudbuildWorkerPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudbuildWorkerPool_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloudbuild_worker_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudbuildWorkerPool {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "User-defined name of the `WorkerPool`."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudbuildWorkerPool {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudbuildWorkerPool {
        let out = DataCloudbuildWorkerPool(std::rc::Rc::new(DataCloudbuildWorkerPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudbuildWorkerPoolData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudbuildWorkerPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildWorkerPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudbuildWorkerPoolRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the request to create the `WorkerPool` was received."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. Time at which the request to delete the `WorkerPool` was received."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser-defined name of the `WorkerPool`."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_config` after provisioning.\nNetwork configuration for the `WorkerPool`."]
    pub fn network_config(&self) -> terrars::ListRef<DataCloudbuildWorkerPool_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_service_connect` after provisioning.\nPrivate Service Connect configuration for the pool."]
    pub fn private_service_connect(&self) -> terrars::ListRef<DataCloudbuildWorkerPool_PrivateServiceConnectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_service_connect", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. `WorkerPool` state. Possible values: STATE_UNSPECIFIED, PENDING, APPROVED, REJECTED, CANCELLED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. A unique identifier for the `WorkerPool`."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the request to update the `WorkerPool` was received."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `worker_config` after provisioning.\nConfiguration to be used for a creating workers in the `WorkerPool`."]
    pub fn worker_config(&self) -> terrars::ListRef<DataCloudbuildWorkerPool_WorkerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildWorkerPool_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peered_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peered_network_ip_range: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudbuildWorkerPool_NetworkConfig {
    #[doc = "Set the field `peered_network`.\n"]
    pub fn set_peered_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peered_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peered_network_ip_range`.\n"]
    pub fn set_peered_network_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.peered_network_ip_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildWorkerPool_NetworkConfig {
    type O = terrars::BlockAssignable<DataCloudbuildWorkerPool_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildWorkerPool_NetworkConfig {}

impl BuildDataCloudbuildWorkerPool_NetworkConfig {
    pub fn build(self) -> DataCloudbuildWorkerPool_NetworkConfig {
        DataCloudbuildWorkerPool_NetworkConfig {
            peered_network: core::default::Default::default(),
            peered_network_ip_range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildWorkerPool_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildWorkerPool_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildWorkerPool_NetworkConfigRef {
        DataCloudbuildWorkerPool_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildWorkerPool_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `peered_network` after provisioning.\n"]
    pub fn peered_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peered_network", self.base))
    }

    #[doc = "Get a reference to the value of field `peered_network_ip_range` after provisioning.\n"]
    pub fn peered_network_ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peered_network_ip_range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildWorkerPool_PrivateServiceConnect {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_all_traffic: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudbuildWorkerPool_PrivateServiceConnect {
    #[doc = "Set the field `network_attachment`.\n"]
    pub fn set_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_all_traffic`.\n"]
    pub fn set_route_all_traffic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.route_all_traffic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildWorkerPool_PrivateServiceConnect {
    type O = terrars::BlockAssignable<DataCloudbuildWorkerPool_PrivateServiceConnect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildWorkerPool_PrivateServiceConnect {}

impl BuildDataCloudbuildWorkerPool_PrivateServiceConnect {
    pub fn build(self) -> DataCloudbuildWorkerPool_PrivateServiceConnect {
        DataCloudbuildWorkerPool_PrivateServiceConnect {
            network_attachment: core::default::Default::default(),
            route_all_traffic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildWorkerPool_PrivateServiceConnectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildWorkerPool_PrivateServiceConnectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudbuildWorkerPool_PrivateServiceConnectRef {
        DataCloudbuildWorkerPool_PrivateServiceConnectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildWorkerPool_PrivateServiceConnectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\n"]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `route_all_traffic` after provisioning.\n"]
    pub fn route_all_traffic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_all_traffic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudbuildWorkerPool_WorkerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_nested_virtualization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_external_ip: std::option::Option<terrars::PrimField<bool>>,
}

impl DataCloudbuildWorkerPool_WorkerConfig {
    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_nested_virtualization`.\n"]
    pub fn set_enable_nested_virtualization(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_external_ip`.\n"]
    pub fn set_no_external_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_external_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudbuildWorkerPool_WorkerConfig {
    type O = terrars::BlockAssignable<DataCloudbuildWorkerPool_WorkerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudbuildWorkerPool_WorkerConfig {}

impl BuildDataCloudbuildWorkerPool_WorkerConfig {
    pub fn build(self) -> DataCloudbuildWorkerPool_WorkerConfig {
        DataCloudbuildWorkerPool_WorkerConfig {
            disk_size_gb: core::default::Default::default(),
            enable_nested_virtualization: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            no_external_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudbuildWorkerPool_WorkerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudbuildWorkerPool_WorkerConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudbuildWorkerPool_WorkerConfigRef {
        DataCloudbuildWorkerPool_WorkerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudbuildWorkerPool_WorkerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\n"]
    pub fn enable_nested_virtualization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `no_external_ip` after provisioning.\n"]
    pub fn no_external_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_external_ip", self.base))
    }
}
