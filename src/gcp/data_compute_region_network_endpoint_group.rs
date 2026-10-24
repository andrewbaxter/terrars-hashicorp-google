use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeRegionNetworkEndpointGroupData {
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
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeRegionNetworkEndpointGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeRegionNetworkEndpointGroupData>,
}

#[derive(Clone)]
pub struct DataComputeRegionNetworkEndpointGroup(std::rc::Rc<DataComputeRegionNetworkEndpointGroup_>);

impl DataComputeRegionNetworkEndpointGroup {
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
        "Set the field `name`.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nA reference to the region where the Serverless NEGs Reside."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `app_engine` after provisioning.\nOnly valid when networkEndpointType is \"SERVERLESS\".\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn app_engine(&self) -> terrars::ListRef<DataComputeRegionNetworkEndpointGroupAppEngineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_function` after provisioning.\nOnly valid when networkEndpointType is \"SERVERLESS\".\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn cloud_function(&self) -> terrars::ListRef<DataComputeRegionNetworkEndpointGroupCloudFunctionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_function", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_run` after provisioning.\nOnly valid when networkEndpointType is \"SERVERLESS\".\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn cloud_run(&self) -> terrars::ListRef<DataComputeRegionNetworkEndpointGroupCloudRunElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_run", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThis field is only used for PSC.\nThe URL of the network to which all network endpoints in the NEG belong. Uses\n\"default\" project network if unspecified."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_endpoint_type` after provisioning.\nType of network endpoints in this network endpoint group. Defaults to SERVERLESS Default value: \"SERVERLESS\" Possible values: [\"SERVERLESS\", \"PRIVATE_SERVICE_CONNECT\"]"]
    pub fn network_endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_target_service` after provisioning.\nThe target service url used to set up private service connection to\na Google API or a PSC Producer Service Attachment."]
    pub fn psc_target_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_target_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the Serverless NEGs Reside."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThis field is only used for PSC.\nOptional URL of the subnetwork to which all network endpoints in the NEG belong."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeRegionNetworkEndpointGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeRegionNetworkEndpointGroup { }

impl terrars::ToListMappable for DataComputeRegionNetworkEndpointGroup {
    type O = terrars::ListRef<DataComputeRegionNetworkEndpointGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeRegionNetworkEndpointGroup_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_region_network_endpoint_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeRegionNetworkEndpointGroup {
    pub tf_id: std::string::String,
}

impl BuildDataComputeRegionNetworkEndpointGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeRegionNetworkEndpointGroup {
        let out = DataComputeRegionNetworkEndpointGroup(std::rc::Rc::new(DataComputeRegionNetworkEndpointGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeRegionNetworkEndpointGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                self_link: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeRegionNetworkEndpointGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionNetworkEndpointGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeRegionNetworkEndpointGroupRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `app_engine` after provisioning.\nOnly valid when networkEndpointType is \"SERVERLESS\".\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn app_engine(&self) -> terrars::ListRef<DataComputeRegionNetworkEndpointGroupAppEngineElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_function` after provisioning.\nOnly valid when networkEndpointType is \"SERVERLESS\".\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn cloud_function(&self) -> terrars::ListRef<DataComputeRegionNetworkEndpointGroupCloudFunctionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_function", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_run` after provisioning.\nOnly valid when networkEndpointType is \"SERVERLESS\".\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn cloud_run(&self) -> terrars::ListRef<DataComputeRegionNetworkEndpointGroupCloudRunElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_run", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThis field is only used for PSC.\nThe URL of the network to which all network endpoints in the NEG belong. Uses\n\"default\" project network if unspecified."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_endpoint_type` after provisioning.\nType of network endpoints in this network endpoint group. Defaults to SERVERLESS Default value: \"SERVERLESS\" Possible values: [\"SERVERLESS\", \"PRIVATE_SERVICE_CONNECT\"]"]
    pub fn network_endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_target_service` after provisioning.\nThe target service url used to set up private service connection to\na Google API or a PSC Producer Service Attachment."]
    pub fn psc_target_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_target_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the Serverless NEGs Reside."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThis field is only used for PSC.\nOptional URL of the subnetwork to which all network endpoints in the NEG belong."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionNetworkEndpointGroupAppEngineEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_mask: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionNetworkEndpointGroupAppEngineEl {
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url_mask`.\n"]
    pub fn set_url_mask(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url_mask = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionNetworkEndpointGroupAppEngineEl {
    type O = terrars::BlockAssignable<DataComputeRegionNetworkEndpointGroupAppEngineEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionNetworkEndpointGroupAppEngineEl {}

impl BuildDataComputeRegionNetworkEndpointGroupAppEngineEl {
    pub fn build(self) -> DataComputeRegionNetworkEndpointGroupAppEngineEl {
        DataComputeRegionNetworkEndpointGroupAppEngineEl {
            service: core::default::Default::default(),
            url_mask: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataComputeRegionNetworkEndpointGroupAppEngineElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionNetworkEndpointGroupAppEngineElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionNetworkEndpointGroupAppEngineElRef {
        DataComputeRegionNetworkEndpointGroupAppEngineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionNetworkEndpointGroupAppEngineElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `url_mask` after provisioning.\n"]
    pub fn url_mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionNetworkEndpointGroupCloudFunctionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_mask: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionNetworkEndpointGroupCloudFunctionEl {
    #[doc = "Set the field `function`.\n"]
    pub fn set_function(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.function = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url_mask`.\n"]
    pub fn set_url_mask(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url_mask = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionNetworkEndpointGroupCloudFunctionEl {
    type O = terrars::BlockAssignable<DataComputeRegionNetworkEndpointGroupCloudFunctionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionNetworkEndpointGroupCloudFunctionEl {}

impl BuildDataComputeRegionNetworkEndpointGroupCloudFunctionEl {
    pub fn build(self) -> DataComputeRegionNetworkEndpointGroupCloudFunctionEl {
        DataComputeRegionNetworkEndpointGroupCloudFunctionEl {
            function: core::default::Default::default(),
            url_mask: core::default::Default::default(),
        }
    }
}

pub struct DataComputeRegionNetworkEndpointGroupCloudFunctionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionNetworkEndpointGroupCloudFunctionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionNetworkEndpointGroupCloudFunctionElRef {
        DataComputeRegionNetworkEndpointGroupCloudFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionNetworkEndpointGroupCloudFunctionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function", self.base))
    }

    #[doc = "Get a reference to the value of field `url_mask` after provisioning.\n"]
    pub fn url_mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeRegionNetworkEndpointGroupCloudRunEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_mask: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeRegionNetworkEndpointGroupCloudRunEl {
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url_mask`.\n"]
    pub fn set_url_mask(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url_mask = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeRegionNetworkEndpointGroupCloudRunEl {
    type O = terrars::BlockAssignable<DataComputeRegionNetworkEndpointGroupCloudRunEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeRegionNetworkEndpointGroupCloudRunEl {}

impl BuildDataComputeRegionNetworkEndpointGroupCloudRunEl {
    pub fn build(self) -> DataComputeRegionNetworkEndpointGroupCloudRunEl {
        DataComputeRegionNetworkEndpointGroupCloudRunEl {
            service: core::default::Default::default(),
            tag: core::default::Default::default(),
            url_mask: core::default::Default::default(),
        }
    }
}

pub struct DataComputeRegionNetworkEndpointGroupCloudRunElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeRegionNetworkEndpointGroupCloudRunElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeRegionNetworkEndpointGroupCloudRunElRef {
        DataComputeRegionNetworkEndpointGroupCloudRunElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeRegionNetworkEndpointGroupCloudRunElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `url_mask` after provisioning.\n"]
    pub fn url_mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }
}
