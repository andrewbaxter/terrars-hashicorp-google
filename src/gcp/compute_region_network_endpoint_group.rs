#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionNetworkEndpointGroupData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_endpoint_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_target_service: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_engine: std::option::Option<std::vec::Vec<ComputeRegionNetworkEndpointGroup_AppEngine>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_function: std::option::Option<std::vec::Vec<ComputeRegionNetworkEndpointGroup_CloudFunction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_run: std::option::Option<std::vec::Vec<ComputeRegionNetworkEndpointGroup_CloudRun>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_data: std::option::Option<std::vec::Vec<ComputeRegionNetworkEndpointGroup_PscData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRegionNetworkEndpointGroup_Timeouts>,
    dynamic: ComputeRegionNetworkEndpointGroupDynamic,
}

struct ComputeRegionNetworkEndpointGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRegionNetworkEndpointGroupData_>,
}

#[derive(Clone)]
pub struct ComputeRegionNetworkEndpointGroup(std::rc::Rc<ComputeRegionNetworkEndpointGroup_>);

impl ComputeRegionNetworkEndpointGroup {
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
        "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe URL of the network to which all network endpoints in the NEG belong. Uses\n\"default\" project network if unspecified."]
    pub fn set_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_endpoint_type`.\nType of network endpoints in this network endpoint group. Defaults to SERVERLESS. Default value: \"SERVERLESS\" Possible values: [\"SERVERLESS\", \"PRIVATE_SERVICE_CONNECT\", \"INTERNET_IP_PORT\", \"INTERNET_FQDN_PORT\", \"GCE_VM_IP_PORTMAP\"]"]
    pub fn set_network_endpoint_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_endpoint_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `psc_target_service`.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe target service url used to set up private service connection to\na Google API or a PSC Producer Service Attachment."]
    pub fn set_psc_target_service(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().psc_target_service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nThis field is only used for PSC NEGs.\n\nOptional URL of the subnetwork to which all network endpoints in the NEG belong."]
    pub fn set_subnetwork(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app_engine`.\n"]
    pub fn set_app_engine(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionNetworkEndpointGroup_AppEngine>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().app_engine = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.app_engine = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_function`.\n"]
    pub fn set_cloud_function(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionNetworkEndpointGroup_CloudFunction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_function = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_function = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_run`.\n"]
    pub fn set_cloud_run(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionNetworkEndpointGroup_CloudRun>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_run = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_run = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `psc_data`.\n"]
    pub fn set_psc_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionNetworkEndpointGroup_PscData>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRegionNetworkEndpointGroup_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `network` after provisioning.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe URL of the network to which all network endpoints in the NEG belong. Uses\n\"default\" project network if unspecified."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_endpoint_type` after provisioning.\nType of network endpoints in this network endpoint group. Defaults to SERVERLESS. Default value: \"SERVERLESS\" Possible values: [\"SERVERLESS\", \"PRIVATE_SERVICE_CONNECT\", \"INTERNET_IP_PORT\", \"INTERNET_FQDN_PORT\", \"GCE_VM_IP_PORTMAP\"]"]
    pub fn network_endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_target_service` after provisioning.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe target service url used to set up private service connection to\na Google API or a PSC Producer Service Attachment."]
    pub fn psc_target_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_target_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the regional NEGs reside."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThis field is only used for PSC NEGs.\n\nOptional URL of the subnetwork to which all network endpoints in the NEG belong."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_engine` after provisioning.\n"]
    pub fn app_engine(&self) -> terrars::ListRef<ComputeRegionNetworkEndpointGroup_AppEngineRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\n"]
    pub fn cloud_function(&self) -> terrars::ListRef<ComputeRegionNetworkEndpointGroup_CloudFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_run` after provisioning.\n"]
    pub fn cloud_run(&self) -> terrars::ListRef<ComputeRegionNetworkEndpointGroup_CloudRunRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_data` after provisioning.\n"]
    pub fn psc_data(&self) -> terrars::ListRef<ComputeRegionNetworkEndpointGroup_PscDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionNetworkEndpointGroup_TimeoutsRef {
        ComputeRegionNetworkEndpointGroup_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ComputeRegionNetworkEndpointGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeRegionNetworkEndpointGroup { }

impl terrars::ToListMappable for ComputeRegionNetworkEndpointGroup {
    type O = terrars::ListRef<ComputeRegionNetworkEndpointGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeRegionNetworkEndpointGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_region_network_endpoint_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRegionNetworkEndpointGroup {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "A reference to the region where the regional NEGs reside."]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionNetworkEndpointGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeRegionNetworkEndpointGroup {
        let out = ComputeRegionNetworkEndpointGroup(std::rc::Rc::new(ComputeRegionNetworkEndpointGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeRegionNetworkEndpointGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                network_endpoint_type: core::default::Default::default(),
                project: core::default::Default::default(),
                psc_target_service: core::default::Default::default(),
                region: self.region,
                subnetwork: core::default::Default::default(),
                app_engine: core::default::Default::default(),
                cloud_function: core::default::Default::default(),
                cloud_run: core::default::Default::default(),
                psc_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRegionNetworkEndpointGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionNetworkEndpointGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRegionNetworkEndpointGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `network` after provisioning.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe URL of the network to which all network endpoints in the NEG belong. Uses\n\"default\" project network if unspecified."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_endpoint_type` after provisioning.\nType of network endpoints in this network endpoint group. Defaults to SERVERLESS. Default value: \"SERVERLESS\" Possible values: [\"SERVERLESS\", \"PRIVATE_SERVICE_CONNECT\", \"INTERNET_IP_PORT\", \"INTERNET_FQDN_PORT\", \"GCE_VM_IP_PORTMAP\"]"]
    pub fn network_endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_target_service` after provisioning.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe target service url used to set up private service connection to\na Google API or a PSC Producer Service Attachment."]
    pub fn psc_target_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_target_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the regional NEGs reside."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThis field is only used for PSC NEGs.\n\nOptional URL of the subnetwork to which all network endpoints in the NEG belong."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `app_engine` after provisioning.\n"]
    pub fn app_engine(&self) -> terrars::ListRef<ComputeRegionNetworkEndpointGroup_AppEngineRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\n"]
    pub fn cloud_function(&self) -> terrars::ListRef<ComputeRegionNetworkEndpointGroup_CloudFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_function", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_run` after provisioning.\n"]
    pub fn cloud_run(&self) -> terrars::ListRef<ComputeRegionNetworkEndpointGroup_CloudRunRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_run", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_data` after provisioning.\n"]
    pub fn psc_data(&self) -> terrars::ListRef<ComputeRegionNetworkEndpointGroup_PscDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionNetworkEndpointGroup_TimeoutsRef {
        ComputeRegionNetworkEndpointGroup_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionNetworkEndpointGroupDynamic {
    app_engine: std::option::Option<terrars::DynamicBlock<ComputeRegionNetworkEndpointGroup_AppEngine>>,
    cloud_function: std::option::Option<terrars::DynamicBlock<ComputeRegionNetworkEndpointGroup_CloudFunction>>,
    cloud_run: std::option::Option<terrars::DynamicBlock<ComputeRegionNetworkEndpointGroup_CloudRun>>,
    psc_data: std::option::Option<terrars::DynamicBlock<ComputeRegionNetworkEndpointGroup_PscData>>,
}

#[derive(Serialize)]
pub struct ComputeRegionNetworkEndpointGroup_AppEngine {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_mask: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionNetworkEndpointGroup_AppEngine {
    #[doc =
        "Set the field `service`.\nOptional serving service.\nThe service name must be 1-63 characters long, and comply with RFC1035.\nExample value: \"default\", \"my-service\"."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `url_mask`.\nA template to parse service and version fields from a request URL.\nURL mask allows for routing to multiple App Engine services without\nhaving to create multiple Network Endpoint Groups and backend services.\n\nFor example, the request URLs \"foo1-dot-appname.appspot.com/v1\" and\n\"foo1-dot-appname.appspot.com/v2\" can be backed by the same Serverless NEG with\nURL mask \"-dot-appname.appspot.com/\". The URL mask will parse\nthem to { service = \"foo1\", version = \"v1\" } and { service = \"foo1\", version = \"v2\" } respectively."]
    pub fn set_url_mask(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url_mask = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nOptional serving version.\nThe version must be 1-63 characters long, and comply with RFC1035.\nExample value: \"v1\", \"v2\"."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionNetworkEndpointGroup_AppEngine {
    type O = terrars::BlockAssignable<ComputeRegionNetworkEndpointGroup_AppEngine>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionNetworkEndpointGroup_AppEngine {}

impl BuildComputeRegionNetworkEndpointGroup_AppEngine {
    pub fn build(self) -> ComputeRegionNetworkEndpointGroup_AppEngine {
        ComputeRegionNetworkEndpointGroup_AppEngine {
            service: core::default::Default::default(),
            url_mask: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionNetworkEndpointGroup_AppEngineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionNetworkEndpointGroup_AppEngineRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionNetworkEndpointGroup_AppEngineRef {
        ComputeRegionNetworkEndpointGroup_AppEngineRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionNetworkEndpointGroup_AppEngineRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nOptional serving service.\nThe service name must be 1-63 characters long, and comply with RFC1035.\nExample value: \"default\", \"my-service\"."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url_mask` after provisioning.\nA template to parse service and version fields from a request URL.\nURL mask allows for routing to multiple App Engine services without\nhaving to create multiple Network Endpoint Groups and backend services.\n\nFor example, the request URLs \"foo1-dot-appname.appspot.com/v1\" and\n\"foo1-dot-appname.appspot.com/v2\" can be backed by the same Serverless NEG with\nURL mask \"-dot-appname.appspot.com/\". The URL mask will parse\nthem to { service = \"foo1\", version = \"v1\" } and { service = \"foo1\", version = \"v2\" } respectively."]
    pub fn url_mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nOptional serving version.\nThe version must be 1-63 characters long, and comply with RFC1035.\nExample value: \"v1\", \"v2\"."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionNetworkEndpointGroup_CloudFunction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_mask: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionNetworkEndpointGroup_CloudFunction {
    #[doc =
        "Set the field `function`.\nA user-defined name of the Cloud Function.\nThe function name is case-sensitive and must be 1-63 characters long.\nExample value: \"func1\"."]
    pub fn set_function(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.function = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `url_mask`.\nA template to parse function field from a request URL. URL mask allows\nfor routing to multiple Cloud Functions without having to create\nmultiple Network Endpoint Groups and backend services.\n\nFor example, request URLs \"mydomain.com/function1\" and \"mydomain.com/function2\"\ncan be backed by the same Serverless NEG with URL mask \"/\". The URL mask\nwill parse them to { function = \"function1\" } and { function = \"function2\" } respectively."]
    pub fn set_url_mask(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url_mask = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionNetworkEndpointGroup_CloudFunction {
    type O = terrars::BlockAssignable<ComputeRegionNetworkEndpointGroup_CloudFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionNetworkEndpointGroup_CloudFunction {}

impl BuildComputeRegionNetworkEndpointGroup_CloudFunction {
    pub fn build(self) -> ComputeRegionNetworkEndpointGroup_CloudFunction {
        ComputeRegionNetworkEndpointGroup_CloudFunction {
            function: core::default::Default::default(),
            url_mask: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionNetworkEndpointGroup_CloudFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionNetworkEndpointGroup_CloudFunctionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionNetworkEndpointGroup_CloudFunctionRef {
        ComputeRegionNetworkEndpointGroup_CloudFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionNetworkEndpointGroup_CloudFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `function` after provisioning.\nA user-defined name of the Cloud Function.\nThe function name is case-sensitive and must be 1-63 characters long.\nExample value: \"func1\"."]
    pub fn function(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url_mask` after provisioning.\nA template to parse function field from a request URL. URL mask allows\nfor routing to multiple Cloud Functions without having to create\nmultiple Network Endpoint Groups and backend services.\n\nFor example, request URLs \"mydomain.com/function1\" and \"mydomain.com/function2\"\ncan be backed by the same Serverless NEG with URL mask \"/\". The URL mask\nwill parse them to { function = \"function1\" } and { function = \"function2\" } respectively."]
    pub fn url_mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionNetworkEndpointGroup_CloudRun {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_mask: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionNetworkEndpointGroup_CloudRun {
    #[doc =
        "Set the field `service`.\nCloud Run service is the main resource of Cloud Run.\nThe service must be 1-63 characters long, and comply with RFC1035.\nExample value: \"run-service\"."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nCloud Run tag represents the \"named-revision\" to provide\nadditional fine-grained traffic routing information.\nThe tag must be 1-63 characters long, and comply with RFC1035.\nExample value: \"revision-0010\"."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `url_mask`.\nA template to parse service and tag fields from a request URL.\nURL mask allows for routing to multiple Run services without having\nto create multiple network endpoint groups and backend services.\n\nFor example, request URLs \"foo1.domain.com/bar1\" and \"foo1.domain.com/bar2\"\nan be backed by the same Serverless Network Endpoint Group (NEG) with\nURL mask \".domain.com/\". The URL mask will parse them to { service=\"bar1\", tag=\"foo1\" }\nand { service=\"bar2\", tag=\"foo2\" } respectively."]
    pub fn set_url_mask(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url_mask = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionNetworkEndpointGroup_CloudRun {
    type O = terrars::BlockAssignable<ComputeRegionNetworkEndpointGroup_CloudRun>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionNetworkEndpointGroup_CloudRun {}

impl BuildComputeRegionNetworkEndpointGroup_CloudRun {
    pub fn build(self) -> ComputeRegionNetworkEndpointGroup_CloudRun {
        ComputeRegionNetworkEndpointGroup_CloudRun {
            service: core::default::Default::default(),
            tag: core::default::Default::default(),
            url_mask: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionNetworkEndpointGroup_CloudRunRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionNetworkEndpointGroup_CloudRunRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionNetworkEndpointGroup_CloudRunRef {
        ComputeRegionNetworkEndpointGroup_CloudRunRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionNetworkEndpointGroup_CloudRunRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nCloud Run service is the main resource of Cloud Run.\nThe service must be 1-63 characters long, and comply with RFC1035.\nExample value: \"run-service\"."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nCloud Run tag represents the \"named-revision\" to provide\nadditional fine-grained traffic routing information.\nThe tag must be 1-63 characters long, and comply with RFC1035.\nExample value: \"revision-0010\"."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url_mask` after provisioning.\nA template to parse service and tag fields from a request URL.\nURL mask allows for routing to multiple Run services without having\nto create multiple network endpoint groups and backend services.\n\nFor example, request URLs \"foo1.domain.com/bar1\" and \"foo1.domain.com/bar2\"\nan be backed by the same Serverless Network Endpoint Group (NEG) with\nURL mask \".domain.com/\". The URL mask will parse them to { service=\"bar1\", tag=\"foo1\" }\nand { service=\"bar2\", tag=\"foo2\" } respectively."]
    pub fn url_mask(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionNetworkEndpointGroup_PscData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    producer_port: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionNetworkEndpointGroup_PscData {
    #[doc =
        "Set the field `producer_port`.\nThe PSC producer port to use when consumer PSC NEG connects to a producer. If\nthis flag isn't specified for a PSC NEG with endpoint type\nprivate-service-connect, then PSC NEG will be connected to a first port in the\navailable PSC producer port range."]
    pub fn set_producer_port(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.producer_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionNetworkEndpointGroup_PscData {
    type O = terrars::BlockAssignable<ComputeRegionNetworkEndpointGroup_PscData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionNetworkEndpointGroup_PscData {}

impl BuildComputeRegionNetworkEndpointGroup_PscData {
    pub fn build(self) -> ComputeRegionNetworkEndpointGroup_PscData {
        ComputeRegionNetworkEndpointGroup_PscData { producer_port: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionNetworkEndpointGroup_PscDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionNetworkEndpointGroup_PscDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionNetworkEndpointGroup_PscDataRef {
        ComputeRegionNetworkEndpointGroup_PscDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionNetworkEndpointGroup_PscDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `producer_port` after provisioning.\nThe PSC producer port to use when consumer PSC NEG connects to a producer. If\nthis flag isn't specified for a PSC NEG with endpoint type\nprivate-service-connect, then PSC NEG will be connected to a first port in the\navailable PSC producer port range."]
    pub fn producer_port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.producer_port", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionNetworkEndpointGroup_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionNetworkEndpointGroup_Timeouts {
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
}

impl terrars::ToListMappable for ComputeRegionNetworkEndpointGroup_Timeouts {
    type O = terrars::BlockAssignable<ComputeRegionNetworkEndpointGroup_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionNetworkEndpointGroup_Timeouts {}

impl BuildComputeRegionNetworkEndpointGroup_Timeouts {
    pub fn build(self) -> ComputeRegionNetworkEndpointGroup_Timeouts {
        ComputeRegionNetworkEndpointGroup_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionNetworkEndpointGroup_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionNetworkEndpointGroup_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionNetworkEndpointGroup_TimeoutsRef {
        ComputeRegionNetworkEndpointGroup_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionNetworkEndpointGroup_TimeoutsRef {
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
}
