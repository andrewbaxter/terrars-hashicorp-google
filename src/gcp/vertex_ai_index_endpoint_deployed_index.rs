#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiIndexEndpointDeployedIndexData_ {
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
    deployed_index_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_access_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    index: terrars::PrimField<std::string::String>,
    index_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_resources: std::option::Option<std::vec::Vec<VertexAiIndexEndpointDeployedIndex_AutomaticResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_resources: std::option::Option<std::vec::Vec<VertexAiIndexEndpointDeployedIndex_DedicatedResources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployed_index_auth_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiIndexEndpointDeployedIndex_Timeouts>,
    dynamic: VertexAiIndexEndpointDeployedIndexDynamic,
}

struct VertexAiIndexEndpointDeployedIndex_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiIndexEndpointDeployedIndexData_>,
}

#[derive(Clone)]
pub struct VertexAiIndexEndpointDeployedIndex(std::rc::Rc<VertexAiIndexEndpointDeployedIndex_>);

impl VertexAiIndexEndpointDeployedIndex {
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
        "Set the field `deployment_group`.\nThe deployment group can be no longer than 64 characters (eg: 'test', 'prod'). If not set, we will use the 'default' deployment group.\nCreating deployment_groups with reserved_ip_ranges is a recommended practice when the peered network has multiple peering ranges. This creates your deployments from predictable IP spaces for easier traffic administration. Also, one deployment_group (except 'default') can only be used with the same reserved_ip_ranges which means if the deployment_group has been used with reserved_ip_ranges: [a, b, c], using it with [a, b] or [d, e] is disallowed. [See the official documentation here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#DeployedIndex.FIELDS.deployment_group).\nNote: we only support up to 5 deployment groups (not including 'default')."]
    pub fn set_deployment_group(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deployment_group = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_access_logging`.\nIf true, private endpoint's access logs are sent to Cloud Logging."]
    pub fn set_enable_access_logging(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_access_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region of the index endpoint deployment. eg us-central1"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reserved_ip_ranges`.\nA list of reserved ip ranges under the VPC network that can be used for this DeployedIndex.\nIf set, we will deploy the index within the provided ip ranges. Otherwise, the index might be deployed to any ip ranges under the provided VPC network.\n\nThe value should be the name of the address (https://cloud.google.com/compute/docs/reference/rest/v1/addresses) Example: ['vertex-ai-ip-range'].\n\nFor more information about subnets and network IP ranges, please see https://cloud.google.com/vpc/docs/subnets#manually_created_subnet_ip_ranges."]
    pub fn set_reserved_ip_ranges(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().reserved_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_resources`.\n"]
    pub fn set_automatic_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_AutomaticResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automatic_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automatic_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dedicated_resources`.\n"]
    pub fn set_dedicated_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_DedicatedResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dedicated_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dedicated_resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployed_index_auth_config`.\n"]
    pub fn set_deployed_index_auth_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployed_index_auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployed_index_auth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiIndexEndpointDeployedIndex_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_index_id` after provisioning.\nThe user specified ID of the DeployedIndex. The ID can be up to 128 characters long and must start with a letter and only contain letters, numbers, and underscores. The ID must be unique within the project it is created in."]
    pub fn deployed_index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_index_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_group` after provisioning.\nThe deployment group can be no longer than 64 characters (eg: 'test', 'prod'). If not set, we will use the 'default' deployment group.\nCreating deployment_groups with reserved_ip_ranges is a recommended practice when the peered network has multiple peering ranges. This creates your deployments from predictable IP spaces for easier traffic administration. Also, one deployment_group (except 'default') can only be used with the same reserved_ip_ranges which means if the deployment_group has been used with reserved_ip_ranges: [a, b, c], using it with [a, b] or [d, e] is disallowed. [See the official documentation here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#DeployedIndex.FIELDS.deployment_group).\nNote: we only support up to 5 deployment groups (not including 'default')."]
    pub fn deployment_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_access_logging` after provisioning.\nIf true, private endpoint's access logs are sent to Cloud Logging."]
    pub fn enable_access_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_access_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index` after provisioning.\nThe name of the Index this is the deployment of."]
    pub fn index(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_endpoint` after provisioning.\nIdentifies the index endpoint. Must be in the format\n'projects/{{project}}/locations/{{region}}/indexEndpoints/{{indexEndpoint}}'"]
    pub fn index_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_sync_time` after provisioning.\nThe DeployedIndex may depend on various data on its original Index. Additionally when certain changes to the original Index are being done (e.g. when what the Index contains is being changed) the DeployedIndex may be asynchronously updated in the background to reflect these changes. If this timestamp's value is at least the [Index.update_time](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexes#Index.FIELDS.update_time) of the original Index, it means that this DeployedIndex and the original Index are in sync. If this timestamp is older, then to see which updates this DeployedIndex already contains (and which it does not), one must [list](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.operations/list#google.longrunning.Operations.ListOperations) the operations that are running on the original Index. Only the successfully completed Operations with updateTime equal or before this sync time are contained in this DeployedIndex.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples:\u{a0}\"2014-10-02T15:01:23Z\"\u{a0}and\u{a0}\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn index_sync_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_sync_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the DeployedIndex resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoints` after provisioning.\nProvides paths for users to send requests directly to the deployed index services running on Cloud via private services access. This field is populated if [network](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#IndexEndpoint.FIELDS.network) is configured."]
    pub fn private_endpoints(&self) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_PrivateEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the index endpoint deployment. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_ip_ranges` after provisioning.\nA list of reserved ip ranges under the VPC network that can be used for this DeployedIndex.\nIf set, we will deploy the index within the provided ip ranges. Otherwise, the index might be deployed to any ip ranges under the provided VPC network.\n\nThe value should be the name of the address (https://cloud.google.com/compute/docs/reference/rest/v1/addresses) Example: ['vertex-ai-ip-range'].\n\nFor more information about subnets and network IP ranges, please see https://cloud.google.com/vpc/docs/subnets#manually_created_subnet_ip_ranges."]
    pub fn reserved_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reserved_ip_ranges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_resources` after provisioning.\n"]
    pub fn automatic_resources(&self) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_AutomaticResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(&self) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_DedicatedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployed_index_auth_config` after provisioning.\n"]
    pub fn deployed_index_auth_config(
        &self,
    ) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployed_index_auth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiIndexEndpointDeployedIndex_TimeoutsRef {
        VertexAiIndexEndpointDeployedIndex_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VertexAiIndexEndpointDeployedIndex {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiIndexEndpointDeployedIndex { }

impl terrars::ToListMappable for VertexAiIndexEndpointDeployedIndex {
    type O = terrars::ListRef<VertexAiIndexEndpointDeployedIndexRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiIndexEndpointDeployedIndex_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_index_endpoint_deployed_index".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiIndexEndpointDeployedIndex {
    pub tf_id: std::string::String,
    #[doc =
        "The user specified ID of the DeployedIndex. The ID can be up to 128 characters long and must start with a letter and only contain letters, numbers, and underscores. The ID must be unique within the project it is created in."]
    pub deployed_index_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Index this is the deployment of."]
    pub index: terrars::PrimField<std::string::String>,
    #[doc =
        "Identifies the index endpoint. Must be in the format\n'projects/{{project}}/locations/{{region}}/indexEndpoints/{{indexEndpoint}}'"]
    pub index_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiIndexEndpointDeployedIndex {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiIndexEndpointDeployedIndex {
        let out = VertexAiIndexEndpointDeployedIndex(std::rc::Rc::new(VertexAiIndexEndpointDeployedIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiIndexEndpointDeployedIndexData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deployed_index_id: self.deployed_index_id,
                deployment_group: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enable_access_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                index: self.index,
                index_endpoint: self.index_endpoint,
                region: core::default::Default::default(),
                reserved_ip_ranges: core::default::Default::default(),
                automatic_resources: core::default::Default::default(),
                dedicated_resources: core::default::Default::default(),
                deployed_index_auth_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiIndexEndpointDeployedIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndexEndpointDeployedIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiIndexEndpointDeployedIndexRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_index_id` after provisioning.\nThe user specified ID of the DeployedIndex. The ID can be up to 128 characters long and must start with a letter and only contain letters, numbers, and underscores. The ID must be unique within the project it is created in."]
    pub fn deployed_index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_index_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_group` after provisioning.\nThe deployment group can be no longer than 64 characters (eg: 'test', 'prod'). If not set, we will use the 'default' deployment group.\nCreating deployment_groups with reserved_ip_ranges is a recommended practice when the peered network has multiple peering ranges. This creates your deployments from predictable IP spaces for easier traffic administration. Also, one deployment_group (except 'default') can only be used with the same reserved_ip_ranges which means if the deployment_group has been used with reserved_ip_ranges: [a, b, c], using it with [a, b] or [d, e] is disallowed. [See the official documentation here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#DeployedIndex.FIELDS.deployment_group).\nNote: we only support up to 5 deployment groups (not including 'default')."]
    pub fn deployment_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_access_logging` after provisioning.\nIf true, private endpoint's access logs are sent to Cloud Logging."]
    pub fn enable_access_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_access_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index` after provisioning.\nThe name of the Index this is the deployment of."]
    pub fn index(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_endpoint` after provisioning.\nIdentifies the index endpoint. Must be in the format\n'projects/{{project}}/locations/{{region}}/indexEndpoints/{{indexEndpoint}}'"]
    pub fn index_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_sync_time` after provisioning.\nThe DeployedIndex may depend on various data on its original Index. Additionally when certain changes to the original Index are being done (e.g. when what the Index contains is being changed) the DeployedIndex may be asynchronously updated in the background to reflect these changes. If this timestamp's value is at least the [Index.update_time](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexes#Index.FIELDS.update_time) of the original Index, it means that this DeployedIndex and the original Index are in sync. If this timestamp is older, then to see which updates this DeployedIndex already contains (and which it does not), one must [list](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.operations/list#google.longrunning.Operations.ListOperations) the operations that are running on the original Index. Only the successfully completed Operations with updateTime equal or before this sync time are contained in this DeployedIndex.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples:\u{a0}\"2014-10-02T15:01:23Z\"\u{a0}and\u{a0}\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn index_sync_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_sync_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the DeployedIndex resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoints` after provisioning.\nProvides paths for users to send requests directly to the deployed index services running on Cloud via private services access. This field is populated if [network](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#IndexEndpoint.FIELDS.network) is configured."]
    pub fn private_endpoints(&self) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_PrivateEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoints", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the index endpoint deployment. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_ip_ranges` after provisioning.\nA list of reserved ip ranges under the VPC network that can be used for this DeployedIndex.\nIf set, we will deploy the index within the provided ip ranges. Otherwise, the index might be deployed to any ip ranges under the provided VPC network.\n\nThe value should be the name of the address (https://cloud.google.com/compute/docs/reference/rest/v1/addresses) Example: ['vertex-ai-ip-range'].\n\nFor more information about subnets and network IP ranges, please see https://cloud.google.com/vpc/docs/subnets#manually_created_subnet_ip_ranges."]
    pub fn reserved_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reserved_ip_ranges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_resources` after provisioning.\n"]
    pub fn automatic_resources(&self) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_AutomaticResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(&self) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_DedicatedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployed_index_auth_config` after provisioning.\n"]
    pub fn deployed_index_auth_config(
        &self,
    ) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployed_index_auth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiIndexEndpointDeployedIndex_TimeoutsRef {
        VertexAiIndexEndpointDeployedIndex_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VertexAiIndexEndpointDeployedIndexDynamic {
    automatic_resources: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiIndexEndpointDeployedIndex_AutomaticResources>>,
    dedicated_resources: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiIndexEndpointDeployedIndex_DedicatedResources>>,
    deployed_index_auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpoints {
    #[doc = "Set the field `match_address`.\n"]
    pub fn set_match_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpoints {
    type O = terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpoints {}

impl BuildVertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpoints {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpoints {
        VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpoints {
            match_address: core::default::Default::default(),
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpointsRef {
        VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_address` after provisioning.\n"]
    pub fn match_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_address", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndex_PrivateEndpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_grpc_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_automated_endpoints: std
    ::option
    ::Option<terrars::ListField<VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiIndexEndpointDeployedIndex_PrivateEndpoints {
    #[doc = "Set the field `match_grpc_address`.\n"]
    pub fn set_match_grpc_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.match_grpc_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_automated_endpoints`.\n"]
    pub fn set_psc_automated_endpoints(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpoints>,
                    >,
    ) -> Self {
        self.psc_automated_endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_attachment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiIndexEndpointDeployedIndex_PrivateEndpoints {
    type O = terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_PrivateEndpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndexEndpointDeployedIndex_PrivateEndpoints {}

impl BuildVertexAiIndexEndpointDeployedIndex_PrivateEndpoints {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndex_PrivateEndpoints {
        VertexAiIndexEndpointDeployedIndex_PrivateEndpoints {
            match_grpc_address: core::default::Default::default(),
            psc_automated_endpoints: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndexEndpointDeployedIndex_PrivateEndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndexEndpointDeployedIndex_PrivateEndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndexEndpointDeployedIndex_PrivateEndpointsRef {
        VertexAiIndexEndpointDeployedIndex_PrivateEndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndexEndpointDeployedIndex_PrivateEndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_grpc_address` after provisioning.\n"]
    pub fn match_grpc_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_grpc_address", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_automated_endpoints` after provisioning.\n"]
    pub fn psc_automated_endpoints(
        &self,
    ) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_PrivateEndpoints_PscAutomatedEndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_automated_endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndex_AutomaticResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_replica_count: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiIndexEndpointDeployedIndex_AutomaticResources {
    #[doc =
        "Set the field `max_replica_count`.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If maxReplicaCount is not set, the default value is minReplicaCount. The max allowed replica count is 1000.\n\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, a no upper bound for scaling under heavy traffic will be assume, though Vertex AI may be unable to scale beyond certain replica number."]
    pub fn set_max_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_replica_count`.\nThe minimum number of replicas this DeployedModel will be always deployed on. If minReplicaCount is not set, the default value is 2 (we don't provide SLA when minReplicaCount=1).\n\nIf traffic against it increases, it may dynamically be deployed onto more replicas up to [maxReplicaCount](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/AutomaticResources#FIELDS.max_replica_count), and as traffic decreases, some of these extra replicas may be freed. If the requested value is too large, the deployment will error."]
    pub fn set_min_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_replica_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiIndexEndpointDeployedIndex_AutomaticResources {
    type O = terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_AutomaticResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndexEndpointDeployedIndex_AutomaticResources {}

impl BuildVertexAiIndexEndpointDeployedIndex_AutomaticResources {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndex_AutomaticResources {
        VertexAiIndexEndpointDeployedIndex_AutomaticResources {
            max_replica_count: core::default::Default::default(),
            min_replica_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndexEndpointDeployedIndex_AutomaticResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndexEndpointDeployedIndex_AutomaticResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndexEndpointDeployedIndex_AutomaticResourcesRef {
        VertexAiIndexEndpointDeployedIndex_AutomaticResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndexEndpointDeployedIndex_AutomaticResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_replica_count` after provisioning.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If maxReplicaCount is not set, the default value is minReplicaCount. The max allowed replica count is 1000.\n\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, a no upper bound for scaling under heavy traffic will be assume, though Vertex AI may be unable to scale beyond certain replica number."]
    pub fn max_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replica_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_replica_count` after provisioning.\nThe minimum number of replicas this DeployedModel will be always deployed on. If minReplicaCount is not set, the default value is 2 (we don't provide SLA when minReplicaCount=1).\n\nIf traffic against it increases, it may dynamically be deployed onto more replicas up to [maxReplicaCount](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/AutomaticResources#FIELDS.max_replica_count), and as traffic decreases, some of these extra replicas may be freed. If the requested value is too large, the deployment will error."]
    pub fn min_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_replica_count", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec {
    #[doc =
        "Set the field `machine_type`.\nThe type of the machine.\n\nSee the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)\n\nSee the [list of machine types supported for custom training](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types).\n\nFor [DeployedModel](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.endpoints#DeployedModel) this field is optional, and the default value is n1-standard-2. For [BatchPredictionJob](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.batchPredictionJobs#BatchPredictionJob) or as part of [WorkerPoolSpec](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/CustomJobSpec#WorkerPoolSpec) this field is required."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec {
    type O = terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec {}

impl BuildVertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec {
        VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec {
            machine_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpecRef {
        VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe type of the machine.\n\nSee the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)\n\nSee the [list of machine types supported for custom training](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types).\n\nFor [DeployedModel](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.endpoints#DeployedModel) this field is optional, and the default value is n1-standard-2. For [BatchPredictionJob](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.batchPredictionJobs#BatchPredictionJob) or as part of [WorkerPoolSpec](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/CustomJobSpec#WorkerPoolSpec) this field is required."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiIndexEndpointDeployedIndex_DedicatedResourcesDynamic {
    machine_spec: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec>>,
}

#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndex_DedicatedResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replica_count: std::option::Option<terrars::PrimField<f64>>,
    min_replica_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_spec: std::option::Option<std::vec::Vec<VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec>>,
    dynamic: VertexAiIndexEndpointDeployedIndex_DedicatedResourcesDynamic,
}

impl VertexAiIndexEndpointDeployedIndex_DedicatedResources {
    #[doc =
        "Set the field `max_replica_count`.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If maxReplicaCount is not set, the default value is minReplicaCount"]
    pub fn set_max_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.machine_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiIndexEndpointDeployedIndex_DedicatedResources {
    type O = terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_DedicatedResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndexEndpointDeployedIndex_DedicatedResources {
    #[doc =
        "The minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1."]
    pub min_replica_count: terrars::PrimField<f64>,
}

impl BuildVertexAiIndexEndpointDeployedIndex_DedicatedResources {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndex_DedicatedResources {
        VertexAiIndexEndpointDeployedIndex_DedicatedResources {
            max_replica_count: core::default::Default::default(),
            min_replica_count: self.min_replica_count,
            machine_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndexEndpointDeployedIndex_DedicatedResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndexEndpointDeployedIndex_DedicatedResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndexEndpointDeployedIndex_DedicatedResourcesRef {
        VertexAiIndexEndpointDeployedIndex_DedicatedResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndexEndpointDeployedIndex_DedicatedResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_replica_count` after provisioning.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If maxReplicaCount is not set, the default value is minReplicaCount"]
    pub fn max_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replica_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_replica_count` after provisioning.\nThe minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1."]
    pub fn min_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_replica_count", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(&self) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_DedicatedResources_MachineSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_issuers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audiences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider {
    #[doc =
        "Set the field `allowed_issuers`.\nA list of allowed JWT issuers. Each entry must be a valid Google service account, in the following format: service-account-name@project-id.iam.gserviceaccount.com"]
    pub fn set_allowed_issuers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_issuers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `audiences`.\nThe list of JWT audiences. that are allowed to access. A JWT containing any of these audiences will be accepted."]
    pub fn set_audiences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.audiences = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider {
    type O = terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider {}

impl BuildVertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider {
        VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider {
            allowed_issuers: core::default::Default::default(),
            audiences: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProviderRef {
        VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_issuers` after provisioning.\nA list of allowed JWT issuers. Each entry must be a valid Google service account, in the following format: service-account-name@project-id.iam.gserviceaccount.com"]
    pub fn allowed_issuers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_issuers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `audiences` after provisioning.\nThe list of JWT audiences. that are allowed to access. A JWT containing any of these audiences will be accepted."]
    pub fn audiences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audiences", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfigDynamic {
    auth_provider: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider>>,
}

#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_provider: std
    ::option
    ::Option<std::vec::Vec<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider>>,
    dynamic: VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfigDynamic,
}

impl VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig {
    #[doc = "Set the field `auth_provider`.\n"]
    pub fn set_auth_provider(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProvider>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_provider = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig {
    type O = terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig {}

impl BuildVertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig {
        VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig {
            auth_provider: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfigRef {
        VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_provider` after provisioning.\n"]
    pub fn auth_provider(
        &self,
    ) -> terrars::ListRef<VertexAiIndexEndpointDeployedIndex_DeployedIndexAuthConfig_AuthProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_provider", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndex_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiIndexEndpointDeployedIndex_Timeouts {
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

impl terrars::ToListMappable for VertexAiIndexEndpointDeployedIndex_Timeouts {
    type O = terrars::BlockAssignable<VertexAiIndexEndpointDeployedIndex_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndexEndpointDeployedIndex_Timeouts {}

impl BuildVertexAiIndexEndpointDeployedIndex_Timeouts {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndex_Timeouts {
        VertexAiIndexEndpointDeployedIndex_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndexEndpointDeployedIndex_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndexEndpointDeployedIndex_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndexEndpointDeployedIndex_TimeoutsRef {
        VertexAiIndexEndpointDeployedIndex_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndexEndpointDeployedIndex_TimeoutsRef {
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
