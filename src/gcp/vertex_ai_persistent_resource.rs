#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiPersistentResourceData_ {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std::option::Option<std::vec::Vec<VertexAiPersistentResource_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_interface_config: std::option::Option<std::vec::Vec<VertexAiPersistentResource_PscInterfaceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_pools: std::option::Option<std::vec::Vec<VertexAiPersistentResource_ResourcePools>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_runtime_spec: std::option::Option<std::vec::Vec<VertexAiPersistentResource_ResourceRuntimeSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiPersistentResource_Timeouts>,
    dynamic: VertexAiPersistentResourceDynamic,
}

struct VertexAiPersistentResource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiPersistentResourceData_>,
}

#[derive(Clone)]
pub struct VertexAiPersistentResource(std::rc::Rc<VertexAiPersistentResource_>);

impl VertexAiPersistentResource {
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
        "Set the field `display_name`.\nThe display name of the PersistentResource.\nThe name can be up to 128 characters long and can consist of any UTF-8\ncharacters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels with user-defined metadata to organize PersistentResource.\n\nLabel keys and values can be no longer than 64 characters\n(Unicode codepoints), can only contain lowercase letters, numeric\ncharacters, underscores and dashes. International characters are allowed.\n\nSee https://goo.gl/xmQnxf for more information and examples of labels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location of the PersistentResource. eg us-central1"]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe full name of the Compute Engine\n[network](/compute/docs/networks-and-firewalls#networks) to peered with\nVertex AI to host the persistent resources.\nFor example, 'projects/12345/global/networks/myVPC'.\n[Format](/compute/docs/reference/rest/v1/networks/insert)\nis of the form 'projects/{project}/global/networks/{network}'.\nWhere {project} is a project number, as in '12345', and {network} is a\nnetwork name.\n\nTo specify this field, you must have already [configured VPC Network\nPeering for Vertex\nAI](https://cloud.google.com/vertex-ai/docs/general/vpc-peering).\n\nIf this field is left unspecified, the resources aren't peered with any\nnetwork."]
    pub fn set_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reserved_ip_ranges`.\nA list of names for the reserved IP ranges under the VPC network\nthat can be used for this persistent resource.\n\nIf set, we will deploy the persistent resource within the provided IP\nranges. Otherwise, the persistent resource is deployed to any IP\nranges under the provided VPC network.\n\nExample: ['vertex-ai-ip-range']."]
    pub fn set_reserved_ip_ranges(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().reserved_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiPersistentResource_EncryptionSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `psc_interface_config`.\n"]
    pub fn set_psc_interface_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiPersistentResource_PscInterfaceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_interface_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_interface_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_pools`.\n"]
    pub fn set_resource_pools(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiPersistentResource_ResourcePools>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_pools = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_pools = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_runtime_spec`.\n"]
    pub fn set_resource_runtime_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiPersistentResource_ResourceRuntimeSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_runtime_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_runtime_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiPersistentResource_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime when the PersistentResource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the PersistentResource.\nThe name can be up to 128 characters long and can consist of any UTF-8\ncharacters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error` after provisioning.\nThe 'Status' type defines a logical error model that is suitable for\ndifferent programming environments, including REST APIs and RPC APIs. It is\nused by [gRPC](https://github.com/grpc). Each 'Status' message contains\nthree pieces of data: error code, error message, and error details.\n\nYou can find out more about this error model and how to work with it in the\n[API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub fn error(&self) -> terrars::ListRef<VertexAiPersistentResource_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize PersistentResource.\n\nLabel keys and values can be no longer than 64 characters\n(Unicode codepoints), can only contain lowercase letters, numeric\ncharacters, underscores and dashes. International characters are allowed.\n\nSee https://goo.gl/xmQnxf for more information and examples of labels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the PersistentResource. eg us-central1"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe ID to use for the PersistentResource, which become the final component\nof the PersistentResource's resource name.\n\nThe maximum length is 63 characters, and valid characters\nare '/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the Compute Engine\n[network](/compute/docs/networks-and-firewalls#networks) to peered with\nVertex AI to host the persistent resources.\nFor example, 'projects/12345/global/networks/myVPC'.\n[Format](/compute/docs/reference/rest/v1/networks/insert)\nis of the form 'projects/{project}/global/networks/{network}'.\nWhere {project} is a project number, as in '12345', and {network} is a\nnetwork name.\n\nTo specify this field, you must have already [configured VPC Network\nPeering for Vertex\nAI](https://cloud.google.com/vertex-ai/docs/general/vpc-peering).\n\nIf this field is left unspecified, the resources aren't peered with any\nnetwork."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_ip_ranges` after provisioning.\nA list of names for the reserved IP ranges under the VPC network\nthat can be used for this persistent resource.\n\nIf set, we will deploy the persistent resource within the provided IP\nranges. Otherwise, the persistent resource is deployed to any IP\nranges under the provided VPC network.\n\nExample: ['vertex-ai-ip-range']."]
    pub fn reserved_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reserved_ip_ranges", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_runtime` after provisioning.\nPersistent Cluster runtime information as output"]
    pub fn resource_runtime(&self) -> terrars::ListRef<VertexAiPersistentResource_ResourceRuntimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_runtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `satisfies_pzi` after provisioning.\nReserved for future use."]
    pub fn satisfies_pzi(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzi", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `satisfies_pzs` after provisioning.\nReserved for future use."]
    pub fn satisfies_pzs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nTime when the PersistentResource for the first time entered the 'RUNNING'\nstate."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe detailed state of a PersistentResource.\nPossible values:\nPROVISIONING\nRUNNING\nSTOPPING\nERROR\nREBOOTING\nUPDATING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime when the PersistentResource was most recently updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiPersistentResource_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_interface_config` after provisioning.\n"]
    pub fn psc_interface_config(&self) -> terrars::ListRef<VertexAiPersistentResource_PscInterfaceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_interface_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_pools` after provisioning.\n"]
    pub fn resource_pools(&self) -> terrars::ListRef<VertexAiPersistentResource_ResourcePoolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_pools", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_runtime_spec` after provisioning.\n"]
    pub fn resource_runtime_spec(&self) -> terrars::ListRef<VertexAiPersistentResource_ResourceRuntimeSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_runtime_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiPersistentResource_TimeoutsRef {
        VertexAiPersistentResource_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VertexAiPersistentResource {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiPersistentResource { }

impl terrars::ToListMappable for VertexAiPersistentResource {
    type O = terrars::ListRef<VertexAiPersistentResourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiPersistentResource_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_persistent_resource".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiPersistentResource {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the PersistentResource, which become the final component\nof the PersistentResource's resource name.\n\nThe maximum length is 63 characters, and valid characters\nare '/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiPersistentResource {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiPersistentResource {
        let out = VertexAiPersistentResource(std::rc::Rc::new(VertexAiPersistentResource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiPersistentResourceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                project: core::default::Default::default(),
                reserved_ip_ranges: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                psc_interface_config: core::default::Default::default(),
                resource_pools: core::default::Default::default(),
                resource_runtime_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiPersistentResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiPersistentResourceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime when the PersistentResource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the PersistentResource.\nThe name can be up to 128 characters long and can consist of any UTF-8\ncharacters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error` after provisioning.\nThe 'Status' type defines a logical error model that is suitable for\ndifferent programming environments, including REST APIs and RPC APIs. It is\nused by [gRPC](https://github.com/grpc). Each 'Status' message contains\nthree pieces of data: error code, error message, and error details.\n\nYou can find out more about this error model and how to work with it in the\n[API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub fn error(&self) -> terrars::ListRef<VertexAiPersistentResource_ErrorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize PersistentResource.\n\nLabel keys and values can be no longer than 64 characters\n(Unicode codepoints), can only contain lowercase letters, numeric\ncharacters, underscores and dashes. International characters are allowed.\n\nSee https://goo.gl/xmQnxf for more information and examples of labels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the PersistentResource. eg us-central1"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe ID to use for the PersistentResource, which become the final component\nof the PersistentResource's resource name.\n\nThe maximum length is 63 characters, and valid characters\nare '/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the Compute Engine\n[network](/compute/docs/networks-and-firewalls#networks) to peered with\nVertex AI to host the persistent resources.\nFor example, 'projects/12345/global/networks/myVPC'.\n[Format](/compute/docs/reference/rest/v1/networks/insert)\nis of the form 'projects/{project}/global/networks/{network}'.\nWhere {project} is a project number, as in '12345', and {network} is a\nnetwork name.\n\nTo specify this field, you must have already [configured VPC Network\nPeering for Vertex\nAI](https://cloud.google.com/vertex-ai/docs/general/vpc-peering).\n\nIf this field is left unspecified, the resources aren't peered with any\nnetwork."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_ip_ranges` after provisioning.\nA list of names for the reserved IP ranges under the VPC network\nthat can be used for this persistent resource.\n\nIf set, we will deploy the persistent resource within the provided IP\nranges. Otherwise, the persistent resource is deployed to any IP\nranges under the provided VPC network.\n\nExample: ['vertex-ai-ip-range']."]
    pub fn reserved_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reserved_ip_ranges", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_runtime` after provisioning.\nPersistent Cluster runtime information as output"]
    pub fn resource_runtime(&self) -> terrars::ListRef<VertexAiPersistentResource_ResourceRuntimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_runtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `satisfies_pzi` after provisioning.\nReserved for future use."]
    pub fn satisfies_pzi(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzi", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `satisfies_pzs` after provisioning.\nReserved for future use."]
    pub fn satisfies_pzs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.satisfies_pzs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nTime when the PersistentResource for the first time entered the 'RUNNING'\nstate."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe detailed state of a PersistentResource.\nPossible values:\nPROVISIONING\nRUNNING\nSTOPPING\nERROR\nREBOOTING\nUPDATING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime when the PersistentResource was most recently updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiPersistentResource_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_interface_config` after provisioning.\n"]
    pub fn psc_interface_config(&self) -> terrars::ListRef<VertexAiPersistentResource_PscInterfaceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_interface_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_pools` after provisioning.\n"]
    pub fn resource_pools(&self) -> terrars::ListRef<VertexAiPersistentResource_ResourcePoolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_pools", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_runtime_spec` after provisioning.\n"]
    pub fn resource_runtime_spec(&self) -> terrars::ListRef<VertexAiPersistentResource_ResourceRuntimeSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_runtime_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiPersistentResource_TimeoutsRef {
        VertexAiPersistentResource_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VertexAiPersistentResourceDynamic {
    encryption_spec: std::option::Option<terrars::DynamicBlock<VertexAiPersistentResource_EncryptionSpec>>,
    psc_interface_config: std::option::Option<terrars::DynamicBlock<VertexAiPersistentResource_PscInterfaceConfig>>,
    resource_pools: std::option::Option<terrars::DynamicBlock<VertexAiPersistentResource_ResourcePools>>,
    resource_runtime_spec: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiPersistentResource_ResourceRuntimeSpec>>,
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_Error {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiPersistentResource_Error {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiPersistentResource_Error {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_Error>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_Error {}

impl BuildVertexAiPersistentResource_Error {
    pub fn build(self) -> VertexAiPersistentResource_Error {
        VertexAiPersistentResource_Error {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_ErrorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_ErrorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiPersistentResource_ErrorRef {
        VertexAiPersistentResource_ErrorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_ErrorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_ResourceRuntime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_uris: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl VertexAiPersistentResource_ResourceRuntime {
    #[doc = "Set the field `access_uris`.\n"]
    pub fn set_access_uris(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access_uris = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiPersistentResource_ResourceRuntime {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_ResourceRuntime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_ResourceRuntime {}

impl BuildVertexAiPersistentResource_ResourceRuntime {
    pub fn build(self) -> VertexAiPersistentResource_ResourceRuntime {
        VertexAiPersistentResource_ResourceRuntime { access_uris: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_ResourceRuntimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_ResourceRuntimeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiPersistentResource_ResourceRuntimeRef {
        VertexAiPersistentResource_ResourceRuntimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_ResourceRuntimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_uris` after provisioning.\n"]
    pub fn access_uris(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.access_uris", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_EncryptionSpec {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl VertexAiPersistentResource_EncryptionSpec { }

impl terrars::ToListMappable for VertexAiPersistentResource_EncryptionSpec {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_EncryptionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_EncryptionSpec {
    #[doc =
        "Resource name of the Cloud KMS key used to protect the resource.\n\nThe Cloud KMS key must be in the same region as the resource. It must have\nthe format\n'projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}'."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiPersistentResource_EncryptionSpec {
    pub fn build(self) -> VertexAiPersistentResource_EncryptionSpec {
        VertexAiPersistentResource_EncryptionSpec { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_EncryptionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_EncryptionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiPersistentResource_EncryptionSpecRef {
        VertexAiPersistentResource_EncryptionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_EncryptionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of the Cloud KMS key used to protect the resource.\n\nThe Cloud KMS key must be in the same region as the resource. It must have\nthe format\n'projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}'."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs {
    domain: terrars::PrimField<std::string::String>,
    target_network: terrars::PrimField<std::string::String>,
    target_project: terrars::PrimField<std::string::String>,
}

impl VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs { }

impl terrars::ToListMappable for VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs {
    #[doc =
        "The DNS name suffix of the zone being peered to, e.g.,\n\"my-internal-domain.corp.\". Must end with a dot."]
    pub domain: terrars::PrimField<std::string::String>,
    #[doc = "The VPC network name\nin the target_project where the DNS zone specified by 'domain' is\nvisible."]
    pub target_network: terrars::PrimField<std::string::String>,
    #[doc =
        "The project ID hosting the Cloud DNS managed zone that\ncontains the 'domain'. The Vertex AI Service Agent requires the\ndns.peer role on this project."]
    pub target_project: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs {
    pub fn build(self) -> VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs {
        VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs {
            domain: self.domain,
            target_network: self.target_network,
            target_project: self.target_project,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigsRef {
        VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domain` after provisioning.\nThe DNS name suffix of the zone being peered to, e.g.,\n\"my-internal-domain.corp.\". Must end with a dot."]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_network` after provisioning.\nThe VPC network name\nin the target_project where the DNS zone specified by 'domain' is\nvisible."]
    pub fn target_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_project` after provisioning.\nThe project ID hosting the Cloud DNS managed zone that\ncontains the 'domain'. The Vertex AI Service Agent requires the\ndns.peer role on this project."]
    pub fn target_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_project", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiPersistentResource_PscInterfaceConfigDynamic {
    dns_peering_configs: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs>>,
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_PscInterfaceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_peering_configs: std
    ::option
    ::Option<std::vec::Vec<VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs>>,
    dynamic: VertexAiPersistentResource_PscInterfaceConfigDynamic,
}

impl VertexAiPersistentResource_PscInterfaceConfig {
    #[doc =
        "Set the field `network_attachment`.\nThe name of the Compute Engine\n[network\nattachment](https://cloud.google.com/vpc/docs/about-network-attachments) to\nattach to the resource within the region and user project.\nTo specify this field, you must have already [created a network attachment]\n(https://cloud.google.com/vpc/docs/create-manage-network-attachments#create-network-attachments).\nThis field is only used for resources using PSC-I."]
    pub fn set_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_peering_configs`.\n"]
    pub fn set_dns_peering_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns_peering_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_peering_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiPersistentResource_PscInterfaceConfig {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_PscInterfaceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_PscInterfaceConfig {}

impl BuildVertexAiPersistentResource_PscInterfaceConfig {
    pub fn build(self) -> VertexAiPersistentResource_PscInterfaceConfig {
        VertexAiPersistentResource_PscInterfaceConfig {
            network_attachment: core::default::Default::default(),
            dns_peering_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_PscInterfaceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_PscInterfaceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiPersistentResource_PscInterfaceConfigRef {
        VertexAiPersistentResource_PscInterfaceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_PscInterfaceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network_attachment` after provisioning.\nThe name of the Compute Engine\n[network\nattachment](https://cloud.google.com/vpc/docs/about-network-attachments) to\nattach to the resource within the region and user project.\nTo specify this field, you must have already [created a network attachment]\n(https://cloud.google.com/vpc/docs/create-manage-network-attachments#create-network-attachments).\nThis field is only used for resources using PSC-I."]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_peering_configs` after provisioning.\n"]
    pub fn dns_peering_configs(
        &self,
    ) -> terrars::ListRef<VertexAiPersistentResource_PscInterfaceConfig_DnsPeeringConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_peering_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_ResourcePools_AutoscalingSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replica_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_replica_count: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiPersistentResource_ResourcePools_AutoscalingSpec {
    #[doc =
        "Set the field `max_replica_count`.\nmax replicas in the node pool,\nmust be ≥ replica_count and > min_replica_count or will throw error"]
    pub fn set_max_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_replica_count`.\nmin replicas in the node pool,\nmust be ≤ replica_count and < max_replica_count or will throw error.\nFor autoscaling enabled Ray-on-Vertex, we allow min_replica_count of a\nresource_pool to be 0 to match the OSS Ray\nbehavior(https://docs.ray.io/en/latest/cluster/vms/user-guides/configuring-autoscaling.html#cluster-config-parameters).\nAs for Persistent Resource, the min_replica_count must be > 0, we added\na corresponding validation inside\nCreatePersistentResourceRequestValidator.java."]
    pub fn set_min_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_replica_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiPersistentResource_ResourcePools_AutoscalingSpec {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_ResourcePools_AutoscalingSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_ResourcePools_AutoscalingSpec {}

impl BuildVertexAiPersistentResource_ResourcePools_AutoscalingSpec {
    pub fn build(self) -> VertexAiPersistentResource_ResourcePools_AutoscalingSpec {
        VertexAiPersistentResource_ResourcePools_AutoscalingSpec {
            max_replica_count: core::default::Default::default(),
            min_replica_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_ResourcePools_AutoscalingSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_ResourcePools_AutoscalingSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiPersistentResource_ResourcePools_AutoscalingSpecRef {
        VertexAiPersistentResource_ResourcePools_AutoscalingSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_ResourcePools_AutoscalingSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_replica_count` after provisioning.\nmax replicas in the node pool,\nmust be ≥ replica_count and > min_replica_count or will throw error"]
    pub fn max_replica_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replica_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_replica_count` after provisioning.\nmin replicas in the node pool,\nmust be ≤ replica_count and < max_replica_count or will throw error.\nFor autoscaling enabled Ray-on-Vertex, we allow min_replica_count of a\nresource_pool to be 0 to match the OSS Ray\nbehavior(https://docs.ray.io/en/latest/cluster/vms/user-guides/configuring-autoscaling.html#cluster-config-parameters).\nAs for Persistent Resource, the min_replica_count must be > 0, we added\na corresponding validation inside\nCreatePersistentResourceRequestValidator.java."]
    pub fn min_replica_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_replica_count", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_ResourcePools_DiskSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiPersistentResource_ResourcePools_DiskSpec {
    #[doc = "Set the field `boot_disk_size_gb`.\nSize in GB of the boot disk (default is 100GB)."]
    pub fn set_boot_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boot_disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boot_disk_type`.\nType of the boot disk. For non-A3U machines, the default value is\n\"pd-ssd\", for A3U machines, the default value is \"hyperdisk-balanced\".\nValid values: \"pd-ssd\" (Persistent Disk Solid State Drive),\n\"pd-standard\" (Persistent Disk Hard Disk Drive) or \"hyperdisk-balanced\"."]
    pub fn set_boot_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.boot_disk_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiPersistentResource_ResourcePools_DiskSpec {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_ResourcePools_DiskSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_ResourcePools_DiskSpec {}

impl BuildVertexAiPersistentResource_ResourcePools_DiskSpec {
    pub fn build(self) -> VertexAiPersistentResource_ResourcePools_DiskSpec {
        VertexAiPersistentResource_ResourcePools_DiskSpec {
            boot_disk_size_gb: core::default::Default::default(),
            boot_disk_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_ResourcePools_DiskSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_ResourcePools_DiskSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiPersistentResource_ResourcePools_DiskSpecRef {
        VertexAiPersistentResource_ResourcePools_DiskSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_ResourcePools_DiskSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_size_gb` after provisioning.\nSize in GB of the boot disk (default is 100GB)."]
    pub fn boot_disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boot_disk_type` after provisioning.\nType of the boot disk. For non-A3U machines, the default value is\n\"pd-ssd\", for A3U machines, the default value is \"hyperdisk-balanced\".\nValid values: \"pd-ssd\" (Persistent Disk Solid State Drive),\n\"pd-standard\" (Persistent Disk Hard Disk Drive) or \"hyperdisk-balanced\"."]
    pub fn boot_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot_disk_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_ResourcePools_MachineSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiPersistentResource_ResourcePools_MachineSpec {
    #[doc = "Set the field `accelerator_count`.\nThe number of accelerators to attach to the machine."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `accelerator_type`.\nThe type of accelerator(s) that may be attached to the machine.\nPossible values:\nNVIDIA_TESLA_K80\nNVIDIA_TESLA_P100\nNVIDIA_TESLA_V100\nNVIDIA_TESLA_P4\nNVIDIA_TESLA_T4\nNVIDIA_TESLA_A100\nNVIDIA_A100_80GB\nNVIDIA_L4\nNVIDIA_H100_80GB\nNVIDIA_H100_MEGA_80GB\nNVIDIA_H200_141GB\nNVIDIA_B200\nNVIDIA_GB200\nNVIDIA_RTX_PRO_6000\nTPU_V2\nTPU_V3\nTPU_V4_POD\nTPU_V5_LITEPOD"]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nThe type of the machine.\n\nSee the [list of machine types supported for\nprediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)\n\nSee the [list of machine types supported for custom\ntraining](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types)."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiPersistentResource_ResourcePools_MachineSpec {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_ResourcePools_MachineSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_ResourcePools_MachineSpec {}

impl BuildVertexAiPersistentResource_ResourcePools_MachineSpec {
    pub fn build(self) -> VertexAiPersistentResource_ResourcePools_MachineSpec {
        VertexAiPersistentResource_ResourcePools_MachineSpec {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_ResourcePools_MachineSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_ResourcePools_MachineSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiPersistentResource_ResourcePools_MachineSpecRef {
        VertexAiPersistentResource_ResourcePools_MachineSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_ResourcePools_MachineSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of accelerators to attach to the machine."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of accelerator(s) that may be attached to the machine.\nPossible values:\nNVIDIA_TESLA_K80\nNVIDIA_TESLA_P100\nNVIDIA_TESLA_V100\nNVIDIA_TESLA_P4\nNVIDIA_TESLA_T4\nNVIDIA_TESLA_A100\nNVIDIA_A100_80GB\nNVIDIA_L4\nNVIDIA_H100_80GB\nNVIDIA_H100_MEGA_80GB\nNVIDIA_H200_141GB\nNVIDIA_B200\nNVIDIA_GB200\nNVIDIA_RTX_PRO_6000\nTPU_V2\nTPU_V3\nTPU_V4_POD\nTPU_V5_LITEPOD"]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe type of the machine.\n\nSee the [list of machine types supported for\nprediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)\n\nSee the [list of machine types supported for custom\ntraining](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types)."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiPersistentResource_ResourcePoolsDynamic {
    autoscaling_spec: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiPersistentResource_ResourcePools_AutoscalingSpec>>,
    disk_spec: std::option::Option<terrars::DynamicBlock<VertexAiPersistentResource_ResourcePools_DiskSpec>>,
    machine_spec: std::option::Option<terrars::DynamicBlock<VertexAiPersistentResource_ResourcePools_MachineSpec>>,
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_ResourcePools {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_spec: std::option::Option<std::vec::Vec<VertexAiPersistentResource_ResourcePools_AutoscalingSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_spec: std::option::Option<std::vec::Vec<VertexAiPersistentResource_ResourcePools_DiskSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_spec: std::option::Option<std::vec::Vec<VertexAiPersistentResource_ResourcePools_MachineSpec>>,
    dynamic: VertexAiPersistentResource_ResourcePoolsDynamic,
}

impl VertexAiPersistentResource_ResourcePools {
    #[doc =
        "Set the field `id`.\nThe unique ID in a PersistentResource for referring to this resource pool.\nUser can specify it if necessary. Otherwise, it's generated\nautomatically."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_count`.\nThe total number of machines to use for this resource pool."]
    pub fn set_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_spec`.\n"]
    pub fn set_autoscaling_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiPersistentResource_ResourcePools_AutoscalingSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_spec`.\n"]
    pub fn set_disk_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiPersistentResource_ResourcePools_DiskSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiPersistentResource_ResourcePools_MachineSpec>>,
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

impl terrars::ToListMappable for VertexAiPersistentResource_ResourcePools {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_ResourcePools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_ResourcePools {}

impl BuildVertexAiPersistentResource_ResourcePools {
    pub fn build(self) -> VertexAiPersistentResource_ResourcePools {
        VertexAiPersistentResource_ResourcePools {
            id: core::default::Default::default(),
            replica_count: core::default::Default::default(),
            autoscaling_spec: core::default::Default::default(),
            disk_spec: core::default::Default::default(),
            machine_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_ResourcePoolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_ResourcePoolsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiPersistentResource_ResourcePoolsRef {
        VertexAiPersistentResource_ResourcePoolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_ResourcePoolsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique ID in a PersistentResource for referring to this resource pool.\nUser can specify it if necessary. Otherwise, it's generated\nautomatically."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replica_count` after provisioning.\nThe total number of machines to use for this resource pool."]
    pub fn replica_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `used_replica_count` after provisioning.\nThe number of machines currently in use by training jobs for this resource\npool. Will replace idle_replica_count."]
    pub fn used_replica_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.used_replica_count", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_spec` after provisioning.\n"]
    pub fn autoscaling_spec(&self) -> terrars::ListRef<VertexAiPersistentResource_ResourcePools_AutoscalingSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_spec` after provisioning.\n"]
    pub fn disk_spec(&self) -> terrars::ListRef<VertexAiPersistentResource_ResourcePools_DiskSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(&self) -> terrars::ListRef<VertexAiPersistentResource_ResourcePools_MachineSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_ResourceRuntimeSpec_RaySpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiPersistentResource_ResourceRuntimeSpec_RaySpec {
    #[doc =
        "Set the field `image_uri`.\nDefault image for user to choose a preferred ML framework\n(for example, TensorFlow or Pytorch) by choosing from [Vertex prebuilt\nimages](https://cloud.google.com/vertex-ai/docs/training/pre-built-containers).\nEither this or the resource_pool_images is required. Use this field if\nyou need all the resource pools to have the same Ray image. Otherwise, use\nthe 'resource_pool_images' field."]
    pub fn set_image_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiPersistentResource_ResourceRuntimeSpec_RaySpec {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_ResourceRuntimeSpec_RaySpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_ResourceRuntimeSpec_RaySpec {}

impl BuildVertexAiPersistentResource_ResourceRuntimeSpec_RaySpec {
    pub fn build(self) -> VertexAiPersistentResource_ResourceRuntimeSpec_RaySpec {
        VertexAiPersistentResource_ResourceRuntimeSpec_RaySpec { image_uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_ResourceRuntimeSpec_RaySpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_ResourceRuntimeSpec_RaySpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiPersistentResource_ResourceRuntimeSpec_RaySpecRef {
        VertexAiPersistentResource_ResourceRuntimeSpec_RaySpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_ResourceRuntimeSpec_RaySpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image_uri` after provisioning.\nDefault image for user to choose a preferred ML framework\n(for example, TensorFlow or Pytorch) by choosing from [Vertex prebuilt\nimages](https://cloud.google.com/vertex-ai/docs/training/pre-built-containers).\nEither this or the resource_pool_images is required. Use this field if\nyou need all the resource pools to have the same Ray image. Otherwise, use\nthe 'resource_pool_images' field."]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec {
    enable_custom_service_account: terrars::PrimField<bool>,
}

impl VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec { }

impl terrars::ToListMappable for VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec {
    #[doc =
        "If true, custom user-managed service account is enforced to run any\nworkloads (for example, Vertex Jobs) on the resource.\nOtherwise, uses the [Vertex AI Custom Code Service\nAgent](https://cloud.google.com/vertex-ai/docs/general/access-control#service-agents)."]
    pub enable_custom_service_account: terrars::PrimField<bool>,
}

impl BuildVertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec {
    pub fn build(self) -> VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec {
        VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec {
            enable_custom_service_account: self.enable_custom_service_account,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpecRef {
        VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_custom_service_account` after provisioning.\nIf true, custom user-managed service account is enforced to run any\nworkloads (for example, Vertex Jobs) on the resource.\nOtherwise, uses the [Vertex AI Custom Code Service\nAgent](https://cloud.google.com/vertex-ai/docs/general/access-control#service-agents)."]
    pub fn enable_custom_service_account(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_custom_service_account", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiPersistentResource_ResourceRuntimeSpecDynamic {
    ray_spec: std::option::Option<terrars::DynamicBlock<VertexAiPersistentResource_ResourceRuntimeSpec_RaySpec>>,
    service_account_spec: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec>>,
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_ResourceRuntimeSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ray_spec: std::option::Option<std::vec::Vec<VertexAiPersistentResource_ResourceRuntimeSpec_RaySpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_spec: std
    ::option
    ::Option<std::vec::Vec<VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec>>,
    dynamic: VertexAiPersistentResource_ResourceRuntimeSpecDynamic,
}

impl VertexAiPersistentResource_ResourceRuntimeSpec {
    #[doc = "Set the field `ray_spec`.\n"]
    pub fn set_ray_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiPersistentResource_ResourceRuntimeSpec_RaySpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ray_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ray_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_account_spec`.\n"]
    pub fn set_service_account_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpec>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_account_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiPersistentResource_ResourceRuntimeSpec {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_ResourceRuntimeSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_ResourceRuntimeSpec {}

impl BuildVertexAiPersistentResource_ResourceRuntimeSpec {
    pub fn build(self) -> VertexAiPersistentResource_ResourceRuntimeSpec {
        VertexAiPersistentResource_ResourceRuntimeSpec {
            ray_spec: core::default::Default::default(),
            service_account_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_ResourceRuntimeSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_ResourceRuntimeSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiPersistentResource_ResourceRuntimeSpecRef {
        VertexAiPersistentResource_ResourceRuntimeSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_ResourceRuntimeSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ray_spec` after provisioning.\n"]
    pub fn ray_spec(&self) -> terrars::ListRef<VertexAiPersistentResource_ResourceRuntimeSpec_RaySpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ray_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_spec` after provisioning.\n"]
    pub fn service_account_spec(
        &self,
    ) -> terrars::ListRef<VertexAiPersistentResource_ResourceRuntimeSpec_ServiceAccountSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiPersistentResource_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiPersistentResource_Timeouts {
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

impl terrars::ToListMappable for VertexAiPersistentResource_Timeouts {
    type O = terrars::BlockAssignable<VertexAiPersistentResource_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiPersistentResource_Timeouts {}

impl BuildVertexAiPersistentResource_Timeouts {
    pub fn build(self) -> VertexAiPersistentResource_Timeouts {
        VertexAiPersistentResource_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiPersistentResource_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiPersistentResource_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiPersistentResource_TimeoutsRef {
        VertexAiPersistentResource_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiPersistentResource_TimeoutsRef {
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
