use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkConnectivityServiceConnectionPolicyData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    network: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    service_class: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std::option::Option<std::vec::Vec<NetworkConnectivityServiceConnectionPolicyPscConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkConnectivityServiceConnectionPolicyTimeoutsEl>,
    dynamic: NetworkConnectivityServiceConnectionPolicyDynamic,
}

struct NetworkConnectivityServiceConnectionPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkConnectivityServiceConnectionPolicyData>,
}

#[derive(Clone)]
pub struct NetworkConnectivityServiceConnectionPolicy(std::rc::Rc<NetworkConnectivityServiceConnectionPolicy_>);

impl NetworkConnectivityServiceConnectionPolicy {
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

    #[doc = "Set the field `description`.\nFree-text description of the resource."]
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
        "Set the field `labels`.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkConnectivityServiceConnectionPolicyPscConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkConnectivityServiceConnectionPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nFree-text description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `infrastructure` after provisioning.\nThe type of underlying resources used to create the connection."]
    pub fn infrastructure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infrastructure", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the ServiceConnectionPolicy."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_connections` after provisioning.\nInformation about each Private Service Connect connection."]
    pub fn psc_connections(&self) -> terrars::ListRef<NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_class` after provisioning.\nThe service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass.\nIt is provided by the Service Producer. Google services have a prefix of gcp. For example, gcp-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx."]
    pub fn service_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_class", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> terrars::ListRef<NetworkConnectivityServiceConnectionPolicyPscConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
        NetworkConnectivityServiceConnectionPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkConnectivityServiceConnectionPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkConnectivityServiceConnectionPolicy { }

impl terrars::ToListMappable for NetworkConnectivityServiceConnectionPolicy {
    type O = terrars::ListRef<NetworkConnectivityServiceConnectionPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkConnectivityServiceConnectionPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_connectivity_service_connection_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkConnectivityServiceConnectionPolicy {
    pub tf_id: std::string::String,
    #[doc = "The location of the ServiceConnectionPolicy."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}."]
    pub network: terrars::PrimField<std::string::String>,
    #[doc =
        "The service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass.\nIt is provided by the Service Producer. Google services have a prefix of gcp. For example, gcp-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx."]
    pub service_class: terrars::PrimField<std::string::String>,
}

impl BuildNetworkConnectivityServiceConnectionPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkConnectivityServiceConnectionPolicy {
        let out =
            NetworkConnectivityServiceConnectionPolicy(std::rc::Rc::new(NetworkConnectivityServiceConnectionPolicy_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(NetworkConnectivityServiceConnectionPolicyData {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
                    name: self.name,
                    network: self.network,
                    project: core::default::Default::default(),
                    service_class: self.service_class,
                    psc_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkConnectivityServiceConnectionPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityServiceConnectionPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkConnectivityServiceConnectionPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nFree-text description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `infrastructure` after provisioning.\nThe type of underlying resources used to create the connection."]
    pub fn infrastructure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infrastructure", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the ServiceConnectionPolicy."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_connections` after provisioning.\nInformation about each Private Service Connect connection."]
    pub fn psc_connections(&self) -> terrars::ListRef<NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_class` after provisioning.\nThe service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass.\nIt is provided by the Service Producer. Google services have a prefix of gcp. For example, gcp-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx."]
    pub fn service_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_class", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> terrars::ListRef<NetworkConnectivityServiceConnectionPolicyPscConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
        NetworkConnectivityServiceConnectionPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
    type O = terrars::BlockAssignable<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {}

impl BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
    pub fn build(self) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
    type O = terrars::BlockAssignable<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {}

impl BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
    pub fn build(self) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
            domain: core::default::Default::default(),
            metadata: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}

pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_forwarding_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_target_project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error: std::option::Option<terrars::ListField<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_info: std
    ::option
    ::Option<terrars::ListField<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_operation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
    #[doc = "Set the field `consumer_address`.\n"]
    pub fn set_consumer_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_forwarding_rule`.\n"]
    pub fn set_consumer_forwarding_rule(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consumer_forwarding_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_target_project`.\n"]
    pub fn set_consumer_target_project(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consumer_target_project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl>>,
    ) -> Self {
        self.error = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_info`.\n"]
    pub fn set_error_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl>>,
    ) -> Self {
        self.error_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_type`.\n"]
    pub fn set_error_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gce_operation`.\n"]
    pub fn set_gce_operation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gce_operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.psc_connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
    type O = terrars::BlockAssignable<NetworkConnectivityServiceConnectionPolicyPscConnectionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsEl {}

impl BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
    pub fn build(self) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
            consumer_address: core::default::Default::default(),
            consumer_forwarding_rule: core::default::Default::default(),
            consumer_target_project: core::default::Default::default(),
            error: core::default::Default::default(),
            error_info: core::default::Default::default(),
            error_type: core::default::Default::default(),
            gce_operation: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumer_address` after provisioning.\n"]
    pub fn consumer_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_address", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_forwarding_rule` after provisioning.\n"]
    pub fn consumer_forwarding_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_forwarding_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_target_project` after provisioning.\n"]
    pub fn consumer_target_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_target_project", self.base))
    }

    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> terrars::ListRef<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }

    #[doc = "Get a reference to the value of field `error_info` after provisioning.\n"]
    pub fn error_info(
        &self,
    ) -> terrars::ListRef<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_info", self.base))
    }

    #[doc = "Get a reference to the value of field `error_type` after provisioning.\n"]
    pub fn error_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_type", self.base))
    }

    #[doc = "Get a reference to the value of field `gce_operation` after provisioning.\n"]
    pub fn gce_operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gce_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_connection_id", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivityServiceConnectionPolicyPscConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<std::string::String>>,
    subnetworks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivityServiceConnectionPolicyPscConfigEl {
    #[doc = "Set the field `limit`.\nMax number of PSC connections for this policy."]
    pub fn set_limit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkConnectivityServiceConnectionPolicyPscConfigEl {
    type O = terrars::BlockAssignable<NetworkConnectivityServiceConnectionPolicyPscConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivityServiceConnectionPolicyPscConfigEl {
    #[doc = "IDs of the subnetworks or fully qualified identifiers for the subnetworks"]
    pub subnetworks: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildNetworkConnectivityServiceConnectionPolicyPscConfigEl {
    pub fn build(self) -> NetworkConnectivityServiceConnectionPolicyPscConfigEl {
        NetworkConnectivityServiceConnectionPolicyPscConfigEl {
            limit: core::default::Default::default(),
            subnetworks: self.subnetworks,
        }
    }
}

pub struct NetworkConnectivityServiceConnectionPolicyPscConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityServiceConnectionPolicyPscConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivityServiceConnectionPolicyPscConfigElRef {
        NetworkConnectivityServiceConnectionPolicyPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivityServiceConnectionPolicyPscConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nMax number of PSC connections for this policy."]
    pub fn limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetworks` after provisioning.\nIDs of the subnetworks or fully qualified identifiers for the subnetworks"]
    pub fn subnetworks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnetworks", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkConnectivityServiceConnectionPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkConnectivityServiceConnectionPolicyTimeoutsEl {
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

impl terrars::ToListMappable for NetworkConnectivityServiceConnectionPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<NetworkConnectivityServiceConnectionPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkConnectivityServiceConnectionPolicyTimeoutsEl {}

impl BuildNetworkConnectivityServiceConnectionPolicyTimeoutsEl {
    pub fn build(self) -> NetworkConnectivityServiceConnectionPolicyTimeoutsEl {
        NetworkConnectivityServiceConnectionPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
        NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct NetworkConnectivityServiceConnectionPolicyDynamic {
    psc_config: std::option::Option<terrars::DynamicBlock<NetworkConnectivityServiceConnectionPolicyPscConfigEl>>,
}
