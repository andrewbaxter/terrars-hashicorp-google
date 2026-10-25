#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeApiProductData_ {
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
    api_resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approval_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environments: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    org_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxies: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_counter_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_time_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    space: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<std::vec::Vec<ApigeeApiProduct_Attributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    graphql_operation_group: std::option::Option<std::vec::Vec<ApigeeApiProduct_GraphqlOperationGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_operation_group: std::option::Option<std::vec::Vec<ApigeeApiProduct_GrpcOperationGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_group: std::option::Option<std::vec::Vec<ApigeeApiProduct_OperationGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApigeeApiProduct_Timeouts>,
    dynamic: ApigeeApiProductDynamic,
}

struct ApigeeApiProduct_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApigeeApiProductData_>,
}

#[derive(Clone)]
pub struct ApigeeApiProduct(std::rc::Rc<ApigeeApiProduct_>);

impl ApigeeApiProduct {
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
        "Set the field `api_resources`.\nComma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the proxy.pathsuffix variable.\nThe proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the apiResources element is defined to be /forecastrss and the base path defined for the API proxy is /weather, then only requests to /weather/forecastrss are permitted by the API product."]
    pub fn set_api_resources(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().api_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `approval_type`.\nFlag that specifies how API keys are approved to access the APIs defined by the API product.\nValid values are 'auto' or 'manual'. Possible values: [\"auto\", \"manual\"]"]
    pub fn set_approval_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().approval_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nDescription of the API product. Include key information about the API product that is not captured by other fields."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `environments`.\nComma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected.\nBy specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment."]
    pub fn set_environments(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().environments = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proxies`.\nComma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies.\nApigee rejects requests to API proxies that are not listed."]
    pub fn set_proxies(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().proxies = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `quota`.\nNumber of request messages permitted per app by this API product for the specified quotaInterval and quotaTimeUnit.\nFor example, a quota of 50, for a quotaInterval of 12 and a quotaTimeUnit of hours means 50 requests are allowed every 12 hours."]
    pub fn set_quota(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().quota = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `quota_counter_scope`.\nScope of the quota decides how the quota counter gets applied and evaluate for quota violation. If the Scope is set as PROXY, then all the operations defined for the APIproduct that are associated with the same proxy will share the same quota counter set at the APIproduct level, making it a global counter at a proxy level. If the Scope is set as OPERATION, then each operations get the counter set at the API product dedicated, making it a local counter. Note that, the QuotaCounterScope applies only when an operation does not have dedicated quota set for itself. Possible values: [\"QUOTA_COUNTER_SCOPE_UNSPECIFIED\", \"PROXY\", \"OPERATION\"]"]
    pub fn set_quota_counter_scope(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().quota_counter_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quota_interval`.\nTime interval over which the number of request messages is calculated."]
    pub fn set_quota_interval(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().quota_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `quota_time_unit`.\nTime unit defined for the quotaInterval. Valid values include second, minute, hour, day, month or year."]
    pub fn set_quota_time_unit(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().quota_time_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scopes`.\nComma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product."]
    pub fn set_scopes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `space`.\nOptional. The resource ID of the parent Space. If not set, the parent resource will be the Organization."]
    pub fn set_space(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().space = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_Attributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `graphql_operation_group`.\n"]
    pub fn set_graphql_operation_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_GraphqlOperationGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().graphql_operation_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.graphql_operation_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `grpc_operation_group`.\n"]
    pub fn set_grpc_operation_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_GrpcOperationGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grpc_operation_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grpc_operation_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operation_group`.\n"]
    pub fn set_operation_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_OperationGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().operation_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.operation_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApigeeApiProduct_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `api_resources` after provisioning.\nComma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the proxy.pathsuffix variable.\nThe proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the apiResources element is defined to be /forecastrss and the base path defined for the API proxy is /weather, then only requests to /weather/forecastrss are permitted by the API product."]
    pub fn api_resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.api_resources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approval_type` after provisioning.\nFlag that specifies how API keys are approved to access the APIs defined by the API product.\nValid values are 'auto' or 'manual'. Possible values: [\"auto\", \"manual\"]"]
    pub fn approval_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nResponse only. Creation time of this environment as milliseconds since epoch."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the API product. Include key information about the API product that is not captured by other fields."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nName displayed in the UI or developer portal to developers registering for API access."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environments` after provisioning.\nComma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected.\nBy specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment."]
    pub fn environments(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.environments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_at` after provisioning.\nResponse only. Modified time of this environment as milliseconds since epoch."]
    pub fn last_modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nInternal name of the API product."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee API product,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxies` after provisioning.\nComma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies.\nApigee rejects requests to API proxies that are not listed."]
    pub fn proxies(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.proxies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quota` after provisioning.\nNumber of request messages permitted per app by this API product for the specified quotaInterval and quotaTimeUnit.\nFor example, a quota of 50, for a quotaInterval of 12 and a quotaTimeUnit of hours means 50 requests are allowed every 12 hours."]
    pub fn quota(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quota_counter_scope` after provisioning.\nScope of the quota decides how the quota counter gets applied and evaluate for quota violation. If the Scope is set as PROXY, then all the operations defined for the APIproduct that are associated with the same proxy will share the same quota counter set at the APIproduct level, making it a global counter at a proxy level. If the Scope is set as OPERATION, then each operations get the counter set at the API product dedicated, making it a local counter. Note that, the QuotaCounterScope applies only when an operation does not have dedicated quota set for itself. Possible values: [\"QUOTA_COUNTER_SCOPE_UNSPECIFIED\", \"PROXY\", \"OPERATION\"]"]
    pub fn quota_counter_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_counter_scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quota_interval` after provisioning.\nTime interval over which the number of request messages is calculated."]
    pub fn quota_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quota_time_unit` after provisioning.\nTime unit defined for the quotaInterval. Valid values include second, minute, hour, day, month or year."]
    pub fn quota_time_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_time_unit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nComma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `space` after provisioning.\nOptional. The resource ID of the parent Space. If not set, the parent resource will be the Organization."]
    pub fn space(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `graphql_operation_group` after provisioning.\n"]
    pub fn graphql_operation_group(&self) -> terrars::ListRef<ApigeeApiProduct_GraphqlOperationGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.graphql_operation_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grpc_operation_group` after provisioning.\n"]
    pub fn grpc_operation_group(&self) -> terrars::ListRef<ApigeeApiProduct_GrpcOperationGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_operation_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_group` after provisioning.\n"]
    pub fn operation_group(&self) -> terrars::ListRef<ApigeeApiProduct_OperationGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeApiProduct_TimeoutsRef {
        ApigeeApiProduct_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApigeeApiProduct {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApigeeApiProduct { }

impl terrars::ToListMappable for ApigeeApiProduct {
    type O = terrars::ListRef<ApigeeApiProductRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApigeeApiProduct_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apigee_api_product".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeApiProduct {
    pub tf_id: std::string::String,
    #[doc = "Name displayed in the UI or developer portal to developers registering for API access."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "Internal name of the API product."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The Apigee Organization associated with the Apigee API product,\nin the format 'organizations/{{org_name}}'."]
    pub org_id: terrars::PrimField<std::string::String>,
}

impl BuildApigeeApiProduct {
    pub fn build(self, stack: &mut terrars::Stack) -> ApigeeApiProduct {
        let out = ApigeeApiProduct(std::rc::Rc::new(ApigeeApiProduct_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApigeeApiProductData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_resources: core::default::Default::default(),
                approval_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                environments: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                org_id: self.org_id,
                proxies: core::default::Default::default(),
                quota: core::default::Default::default(),
                quota_counter_scope: core::default::Default::default(),
                quota_interval: core::default::Default::default(),
                quota_time_unit: core::default::Default::default(),
                scopes: core::default::Default::default(),
                space: core::default::Default::default(),
                attributes: core::default::Default::default(),
                graphql_operation_group: core::default::Default::default(),
                grpc_operation_group: core::default::Default::default(),
                operation_group: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeApiProductRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProductRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeApiProductRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_resources` after provisioning.\nComma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the proxy.pathsuffix variable.\nThe proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the apiResources element is defined to be /forecastrss and the base path defined for the API proxy is /weather, then only requests to /weather/forecastrss are permitted by the API product."]
    pub fn api_resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.api_resources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `approval_type` after provisioning.\nFlag that specifies how API keys are approved to access the APIs defined by the API product.\nValid values are 'auto' or 'manual'. Possible values: [\"auto\", \"manual\"]"]
    pub fn approval_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nResponse only. Creation time of this environment as milliseconds since epoch."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the API product. Include key information about the API product that is not captured by other fields."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nName displayed in the UI or developer portal to developers registering for API access."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environments` after provisioning.\nComma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected.\nBy specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment."]
    pub fn environments(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.environments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_at` after provisioning.\nResponse only. Modified time of this environment as milliseconds since epoch."]
    pub fn last_modified_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nInternal name of the API product."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee API product,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxies` after provisioning.\nComma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies.\nApigee rejects requests to API proxies that are not listed."]
    pub fn proxies(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.proxies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quota` after provisioning.\nNumber of request messages permitted per app by this API product for the specified quotaInterval and quotaTimeUnit.\nFor example, a quota of 50, for a quotaInterval of 12 and a quotaTimeUnit of hours means 50 requests are allowed every 12 hours."]
    pub fn quota(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quota_counter_scope` after provisioning.\nScope of the quota decides how the quota counter gets applied and evaluate for quota violation. If the Scope is set as PROXY, then all the operations defined for the APIproduct that are associated with the same proxy will share the same quota counter set at the APIproduct level, making it a global counter at a proxy level. If the Scope is set as OPERATION, then each operations get the counter set at the API product dedicated, making it a local counter. Note that, the QuotaCounterScope applies only when an operation does not have dedicated quota set for itself. Possible values: [\"QUOTA_COUNTER_SCOPE_UNSPECIFIED\", \"PROXY\", \"OPERATION\"]"]
    pub fn quota_counter_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_counter_scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quota_interval` after provisioning.\nTime interval over which the number of request messages is calculated."]
    pub fn quota_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quota_time_unit` after provisioning.\nTime unit defined for the quotaInterval. Valid values include second, minute, hour, day, month or year."]
    pub fn quota_time_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_time_unit", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nComma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `space` after provisioning.\nOptional. The resource ID of the parent Space. If not set, the parent resource will be the Organization."]
    pub fn space(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `graphql_operation_group` after provisioning.\n"]
    pub fn graphql_operation_group(&self) -> terrars::ListRef<ApigeeApiProduct_GraphqlOperationGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.graphql_operation_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grpc_operation_group` after provisioning.\n"]
    pub fn grpc_operation_group(&self) -> terrars::ListRef<ApigeeApiProduct_GrpcOperationGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_operation_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operation_group` after provisioning.\n"]
    pub fn operation_group(&self) -> terrars::ListRef<ApigeeApiProduct_OperationGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operation_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeApiProduct_TimeoutsRef {
        ApigeeApiProduct_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ApigeeApiProductDynamic {
    attributes: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_Attributes>>,
    graphql_operation_group: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_GraphqlOperationGroup>>,
    grpc_operation_group: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_GrpcOperationGroup>>,
    operation_group: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_OperationGroup>>,
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_Attributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeApiProduct_Attributes {
    #[doc = "Set the field `name`.\nKey of the attribute."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nValue of the attribute."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeApiProduct_Attributes {
    type O = terrars::BlockAssignable<ApigeeApiProduct_Attributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_Attributes {}

impl BuildApigeeApiProduct_Attributes {
    pub fn build(self) -> ApigeeApiProduct_Attributes {
        ApigeeApiProduct_Attributes {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_AttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_AttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeApiProduct_AttributesRef {
        ApigeeApiProduct_AttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_AttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nKey of the attribute."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the attribute."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations {
    #[doc =
        "Set the field `operation`.\nGraphQL operation name. The name and operation type will be used to apply quotas. If no name is specified, the quota will be applied to all GraphQL operations irrespective of their operation names in the payload."]
    pub fn set_operation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `operation_types`.\nRequired. GraphQL operation types. Valid values include query or mutation.\nNote: Apigee does not currently support subscription types."]
    pub fn set_operation_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.operation_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations {
    type O = terrars::BlockAssignable<ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations {}

impl BuildApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations {
    pub fn build(self) -> ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations {
        ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations {
            operation: core::default::Default::default(),
            operation_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_OperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_OperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_OperationsRef {
        ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_OperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_OperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operation` after provisioning.\nGraphQL operation name. The name and operation type will be used to apply quotas. If no name is specified, the quota will be applied to all GraphQL operations irrespective of their operation names in the payload."]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operation_types` after provisioning.\nRequired. GraphQL operation types. Valid values include query or mutation.\nNote: Apigee does not currently support subscription types."]
    pub fn operation_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.operation_types", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_OperationGroup_OperationConfigs_Quota {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeApiProduct_OperationGroup_OperationConfigs_Quota {
    #[doc =
        "Set the field `interval`.\nRequired. Time interval over which the number of request messages is calculated."]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `limit`.\nRequired. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected."]
    pub fn set_limit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.limit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_unit`.\nTime unit defined for the interval. Valid values include second, minute, hour, day, month or year. If limit and interval are valid, the default value is hour; otherwise, the default is null."]
    pub fn set_time_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeApiProduct_OperationGroup_OperationConfigs_Quota {
    type O = terrars::BlockAssignable<ApigeeApiProduct_OperationGroup_OperationConfigs_Quota>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_OperationGroup_OperationConfigs_Quota {}

impl BuildApigeeApiProduct_OperationGroup_OperationConfigs_Quota {
    pub fn build(self) -> ApigeeApiProduct_OperationGroup_OperationConfigs_Quota {
        ApigeeApiProduct_OperationGroup_OperationConfigs_Quota {
            interval: core::default::Default::default(),
            limit: core::default::Default::default(),
            time_unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_OperationGroup_OperationConfigs_QuotaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_OperationGroup_OperationConfigs_QuotaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeApiProduct_OperationGroup_OperationConfigs_QuotaRef {
        ApigeeApiProduct_OperationGroup_OperationConfigs_QuotaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_OperationGroup_OperationConfigs_QuotaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nRequired. Time interval over which the number of request messages is calculated."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `limit` after provisioning.\nRequired. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected."]
    pub fn limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_unit` after provisioning.\nTime unit defined for the interval. Valid values include second, minute, hour, day, month or year. If limit and interval are valid, the default value is hour; otherwise, the default is null."]
    pub fn time_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeApiProduct_GraphqlOperationGroup_OperationConfigsDynamic {
    attributes: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_Attributes>>,
    operations: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations>>,
    quota: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_OperationGroup_OperationConfigs_Quota>>,
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<std::vec::Vec<ApigeeApiProduct_Attributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std::option::Option<std::vec::Vec<ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota: std::option::Option<std::vec::Vec<ApigeeApiProduct_OperationGroup_OperationConfigs_Quota>>,
    dynamic: ApigeeApiProduct_GraphqlOperationGroup_OperationConfigsDynamic,
}

impl ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs {
    #[doc =
        "Set the field `api_source`.\nRequired. Name of the API proxy endpoint or remote service with which the GraphQL operation and quota are associated."]
    pub fn set_api_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_Attributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs_Operations>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `quota`.\n"]
    pub fn set_quota(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_OperationGroup_OperationConfigs_Quota>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.quota = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.quota = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs {
    type O = terrars::BlockAssignable<ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_GraphqlOperationGroup_OperationConfigs {}

impl BuildApigeeApiProduct_GraphqlOperationGroup_OperationConfigs {
    pub fn build(self) -> ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs {
        ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs {
            api_source: core::default::Default::default(),
            attributes: core::default::Default::default(),
            operations: core::default::Default::default(),
            quota: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_GraphqlOperationGroup_OperationConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_GraphqlOperationGroup_OperationConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeApiProduct_GraphqlOperationGroup_OperationConfigsRef {
        ApigeeApiProduct_GraphqlOperationGroup_OperationConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_GraphqlOperationGroup_OperationConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_source` after provisioning.\nRequired. Name of the API proxy endpoint or remote service with which the GraphQL operation and quota are associated."]
    pub fn api_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_source", self.base))
    }

    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(&self) -> terrars::ListRef<ApigeeApiProduct_OperationGroup_OperationConfigs_QuotaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeApiProduct_GraphqlOperationGroupDynamic {
    operation_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs>>,
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_GraphqlOperationGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_config_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_configs: std::option::Option<std::vec::Vec<ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs>>,
    dynamic: ApigeeApiProduct_GraphqlOperationGroupDynamic,
}

impl ApigeeApiProduct_GraphqlOperationGroup {
    #[doc =
        "Set the field `operation_config_type`.\nFlag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values include proxy or remoteservice. Defaults to proxy. Set to proxy when Apigee API proxies are associated with the API product. Set to remoteservice when non-Apigee proxies like Istio-Envoy are associated with the API product. Possible values: [\"proxy\", \"remoteservice\"]"]
    pub fn set_operation_config_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.operation_config_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operation_configs`.\n"]
    pub fn set_operation_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApigeeApiProduct_GraphqlOperationGroup_OperationConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operation_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operation_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeApiProduct_GraphqlOperationGroup {
    type O = terrars::BlockAssignable<ApigeeApiProduct_GraphqlOperationGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_GraphqlOperationGroup {}

impl BuildApigeeApiProduct_GraphqlOperationGroup {
    pub fn build(self) -> ApigeeApiProduct_GraphqlOperationGroup {
        ApigeeApiProduct_GraphqlOperationGroup {
            operation_config_type: core::default::Default::default(),
            operation_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_GraphqlOperationGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_GraphqlOperationGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeApiProduct_GraphqlOperationGroupRef {
        ApigeeApiProduct_GraphqlOperationGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_GraphqlOperationGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operation_config_type` after provisioning.\nFlag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values include proxy or remoteservice. Defaults to proxy. Set to proxy when Apigee API proxies are associated with the API product. Set to remoteservice when non-Apigee proxies like Istio-Envoy are associated with the API product. Possible values: [\"proxy\", \"remoteservice\"]"]
    pub fn operation_config_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation_config_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeApiProduct_GrpcOperationGroup_OperationConfigsDynamic {
    attributes: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_Attributes>>,
    quota: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_OperationGroup_OperationConfigs_Quota>>,
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_GrpcOperationGroup_OperationConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<std::vec::Vec<ApigeeApiProduct_Attributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota: std::option::Option<std::vec::Vec<ApigeeApiProduct_OperationGroup_OperationConfigs_Quota>>,
    dynamic: ApigeeApiProduct_GrpcOperationGroup_OperationConfigsDynamic,
}

impl ApigeeApiProduct_GrpcOperationGroup_OperationConfigs {
    #[doc =
        "Set the field `api_source`.\nRequired. Name of the API proxy with which the gRPC operation and quota are associated."]
    pub fn set_api_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_source = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `methods`.\nList of unqualified gRPC method names for the proxy to which quota will be applied. If this field is empty, the Quota will apply to all operations on the gRPC service defined on the proxy.\n\nExample: Given a proxy that is configured to serve com.petstore.PetService, the methods com.petstore.PetService.ListPets and com.petstore.PetService.GetPet would be specified here as simply [\"ListPets\", \"GetPet\"].\n\nNote: Currently, you can specify only a single GraphQLOperation. Specifying more than one will cause the operation to fail."]
    pub fn set_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service`.\nRequired. gRPC Service name associated to be associated with the API proxy, on which quota rules can be applied upon."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_Attributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `quota`.\n"]
    pub fn set_quota(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_OperationGroup_OperationConfigs_Quota>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.quota = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.quota = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeApiProduct_GrpcOperationGroup_OperationConfigs {
    type O = terrars::BlockAssignable<ApigeeApiProduct_GrpcOperationGroup_OperationConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_GrpcOperationGroup_OperationConfigs {}

impl BuildApigeeApiProduct_GrpcOperationGroup_OperationConfigs {
    pub fn build(self) -> ApigeeApiProduct_GrpcOperationGroup_OperationConfigs {
        ApigeeApiProduct_GrpcOperationGroup_OperationConfigs {
            api_source: core::default::Default::default(),
            methods: core::default::Default::default(),
            service: core::default::Default::default(),
            attributes: core::default::Default::default(),
            quota: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_GrpcOperationGroup_OperationConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_GrpcOperationGroup_OperationConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeApiProduct_GrpcOperationGroup_OperationConfigsRef {
        ApigeeApiProduct_GrpcOperationGroup_OperationConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_GrpcOperationGroup_OperationConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_source` after provisioning.\nRequired. Name of the API proxy with which the gRPC operation and quota are associated."]
    pub fn api_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_source", self.base))
    }

    #[doc =
        "Get a reference to the value of field `methods` after provisioning.\nList of unqualified gRPC method names for the proxy to which quota will be applied. If this field is empty, the Quota will apply to all operations on the gRPC service defined on the proxy.\n\nExample: Given a proxy that is configured to serve com.petstore.PetService, the methods com.petstore.PetService.ListPets and com.petstore.PetService.GetPet would be specified here as simply [\"ListPets\", \"GetPet\"].\n\nNote: Currently, you can specify only a single GraphQLOperation. Specifying more than one will cause the operation to fail."]
    pub fn methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nRequired. gRPC Service name associated to be associated with the API proxy, on which quota rules can be applied upon."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(&self) -> terrars::ListRef<ApigeeApiProduct_OperationGroup_OperationConfigs_QuotaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeApiProduct_GrpcOperationGroupDynamic {
    operation_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeApiProduct_GrpcOperationGroup_OperationConfigs>>,
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_GrpcOperationGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_configs: std::option::Option<std::vec::Vec<ApigeeApiProduct_GrpcOperationGroup_OperationConfigs>>,
    dynamic: ApigeeApiProduct_GrpcOperationGroupDynamic,
}

impl ApigeeApiProduct_GrpcOperationGroup {
    #[doc = "Set the field `operation_configs`.\n"]
    pub fn set_operation_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_GrpcOperationGroup_OperationConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operation_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operation_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeApiProduct_GrpcOperationGroup {
    type O = terrars::BlockAssignable<ApigeeApiProduct_GrpcOperationGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_GrpcOperationGroup {}

impl BuildApigeeApiProduct_GrpcOperationGroup {
    pub fn build(self) -> ApigeeApiProduct_GrpcOperationGroup {
        ApigeeApiProduct_GrpcOperationGroup {
            operation_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_GrpcOperationGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_GrpcOperationGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeApiProduct_GrpcOperationGroupRef {
        ApigeeApiProduct_GrpcOperationGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_GrpcOperationGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_OperationGroup_OperationConfigs_Operations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    methods: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeApiProduct_OperationGroup_OperationConfigs_Operations {
    #[doc =
        "Set the field `methods`.\nMethods refers to the REST verbs, when none specified, all verb types are allowed."]
    pub fn set_methods(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.methods = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource`.\nRequired. REST resource path associated with the API proxy or remote service."]
    pub fn set_resource(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeApiProduct_OperationGroup_OperationConfigs_Operations {
    type O = terrars::BlockAssignable<ApigeeApiProduct_OperationGroup_OperationConfigs_Operations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_OperationGroup_OperationConfigs_Operations {}

impl BuildApigeeApiProduct_OperationGroup_OperationConfigs_Operations {
    pub fn build(self) -> ApigeeApiProduct_OperationGroup_OperationConfigs_Operations {
        ApigeeApiProduct_OperationGroup_OperationConfigs_Operations {
            methods: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_OperationGroup_OperationConfigs_OperationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_OperationGroup_OperationConfigs_OperationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeApiProduct_OperationGroup_OperationConfigs_OperationsRef {
        ApigeeApiProduct_OperationGroup_OperationConfigs_OperationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_OperationGroup_OperationConfigs_OperationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `methods` after provisioning.\nMethods refers to the REST verbs, when none specified, all verb types are allowed."]
    pub fn methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource` after provisioning.\nRequired. REST resource path associated with the API proxy or remote service."]
    pub fn resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeApiProduct_OperationGroup_OperationConfigsDynamic {
    attributes: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_Attributes>>,
    operations: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeApiProduct_OperationGroup_OperationConfigs_Operations>>,
    quota: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_OperationGroup_OperationConfigs_Quota>>,
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_OperationGroup_OperationConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<std::vec::Vec<ApigeeApiProduct_Attributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std::option::Option<std::vec::Vec<ApigeeApiProduct_OperationGroup_OperationConfigs_Operations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota: std::option::Option<std::vec::Vec<ApigeeApiProduct_OperationGroup_OperationConfigs_Quota>>,
    dynamic: ApigeeApiProduct_OperationGroup_OperationConfigsDynamic,
}

impl ApigeeApiProduct_OperationGroup_OperationConfigs {
    #[doc =
        "Set the field `api_source`.\nRequired. Name of the API proxy or remote service with which the resources, methods, and quota are associated."]
    pub fn set_api_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_Attributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApigeeApiProduct_OperationGroup_OperationConfigs_Operations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `quota`.\n"]
    pub fn set_quota(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_OperationGroup_OperationConfigs_Quota>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.quota = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.quota = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeApiProduct_OperationGroup_OperationConfigs {
    type O = terrars::BlockAssignable<ApigeeApiProduct_OperationGroup_OperationConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_OperationGroup_OperationConfigs {}

impl BuildApigeeApiProduct_OperationGroup_OperationConfigs {
    pub fn build(self) -> ApigeeApiProduct_OperationGroup_OperationConfigs {
        ApigeeApiProduct_OperationGroup_OperationConfigs {
            api_source: core::default::Default::default(),
            attributes: core::default::Default::default(),
            operations: core::default::Default::default(),
            quota: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_OperationGroup_OperationConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_OperationGroup_OperationConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeApiProduct_OperationGroup_OperationConfigsRef {
        ApigeeApiProduct_OperationGroup_OperationConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_OperationGroup_OperationConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_source` after provisioning.\nRequired. Name of the API proxy or remote service with which the resources, methods, and quota are associated."]
    pub fn api_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_source", self.base))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(&self) -> terrars::ListRef<ApigeeApiProduct_OperationGroup_OperationConfigs_OperationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }

    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(&self) -> terrars::ListRef<ApigeeApiProduct_OperationGroup_OperationConfigs_QuotaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeApiProduct_OperationGroupDynamic {
    operation_configs: std::option::Option<terrars::DynamicBlock<ApigeeApiProduct_OperationGroup_OperationConfigs>>,
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_OperationGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_config_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation_configs: std::option::Option<std::vec::Vec<ApigeeApiProduct_OperationGroup_OperationConfigs>>,
    dynamic: ApigeeApiProduct_OperationGroupDynamic,
}

impl ApigeeApiProduct_OperationGroup {
    #[doc =
        "Set the field `operation_config_type`.\nFlag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values include proxy or remoteservice. Defaults to proxy. Set to proxy when Apigee API proxies are associated with the API product. Set to remoteservice when non-Apigee proxies like Istio-Envoy are associated with the API product. Possible values: [\"proxy\", \"remoteservice\"]"]
    pub fn set_operation_config_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.operation_config_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operation_configs`.\n"]
    pub fn set_operation_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeApiProduct_OperationGroup_OperationConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operation_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operation_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeApiProduct_OperationGroup {
    type O = terrars::BlockAssignable<ApigeeApiProduct_OperationGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_OperationGroup {}

impl BuildApigeeApiProduct_OperationGroup {
    pub fn build(self) -> ApigeeApiProduct_OperationGroup {
        ApigeeApiProduct_OperationGroup {
            operation_config_type: core::default::Default::default(),
            operation_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_OperationGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_OperationGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeApiProduct_OperationGroupRef {
        ApigeeApiProduct_OperationGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_OperationGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operation_config_type` after provisioning.\nFlag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values include proxy or remoteservice. Defaults to proxy. Set to proxy when Apigee API proxies are associated with the API product. Set to remoteservice when non-Apigee proxies like Istio-Envoy are associated with the API product. Possible values: [\"proxy\", \"remoteservice\"]"]
    pub fn operation_config_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation_config_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeApiProduct_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeApiProduct_Timeouts {
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

impl terrars::ToListMappable for ApigeeApiProduct_Timeouts {
    type O = terrars::BlockAssignable<ApigeeApiProduct_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeApiProduct_Timeouts {}

impl BuildApigeeApiProduct_Timeouts {
    pub fn build(self) -> ApigeeApiProduct_Timeouts {
        ApigeeApiProduct_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeApiProduct_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeApiProduct_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeApiProduct_TimeoutsRef {
        ApigeeApiProduct_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeApiProduct_TimeoutsRef {
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
