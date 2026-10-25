#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeOrganizationData_ {
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
    analytics_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_consumer_data_encryption_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_consumer_data_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorized_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_plane_encryption_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_vpc_peering: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    project_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_database_encryption_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<std::vec::Vec<ApigeeOrganization_Properties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApigeeOrganization_Timeouts>,
    dynamic: ApigeeOrganizationDynamic,
}

struct ApigeeOrganization_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApigeeOrganizationData_>,
}

#[derive(Clone)]
pub struct ApigeeOrganization(std::rc::Rc<ApigeeOrganization_>);

impl ApigeeOrganization {
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
        "Set the field `analytics_region`.\nPrimary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org)."]
    pub fn set_analytics_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().analytics_region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `api_consumer_data_encryption_key_name`.\nCloud KMS key name used for encrypting API consumer data."]
    pub fn set_api_consumer_data_encryption_key_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().api_consumer_data_encryption_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `api_consumer_data_location`.\nThis field is needed only for customers using non-default data residency regions.\nApigee stores some control plane data only in single region.\nThis field determines which single region Apigee should use."]
    pub fn set_api_consumer_data_location(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().api_consumer_data_location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `authorized_network`.\nCompute Engine network used for Service Networking to be peered with Apigee runtime instances.\nSee [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).\nValid only when 'RuntimeType' is set to CLOUD. The value can be updated only when there are no runtime instances. For example: \"default\"."]
    pub fn set_authorized_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authorized_network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `billing_type`.\nBilling type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing)."]
    pub fn set_billing_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().billing_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `control_plane_encryption_key_name`.\nCloud KMS key name used for encrypting control plane data that is stored in a multi region.\nOnly used for the data residency region \"US\" or \"EU\"."]
    pub fn set_control_plane_encryption_key_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().control_plane_encryption_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the Apigee organization."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_vpc_peering`.\nFlag that specifies whether the VPC Peering through Private Google Access should be\ndisabled between the consumer network and Apigee. Required if an 'authorizedNetwork'\non the consumer project is not provided, in which case the flag should be set to 'true'.\nValid only when 'RuntimeType' is set to CLOUD. The value must be set before the creation\nof any Apigee runtime instance and can be updated only when there are no runtime instances."]
    pub fn set_disable_vpc_peering(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_vpc_peering = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe display name of the Apigee organization."]
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
        "Set the field `retention`.\nOptional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType\nis not EVALUATION). It controls how long Organization data will be retained after the initial delete\noperation completes. During this period, the Organization may be restored to its last known state.\nAfter this period, the Organization will no longer be able to be restored. Default value: \"DELETION_RETENTION_UNSPECIFIED\" Possible values: [\"DELETION_RETENTION_UNSPECIFIED\", \"MINIMUM\"]"]
    pub fn set_retention(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().retention = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `runtime_database_encryption_key_name`.\nCloud KMS key name used for encrypting the data that is stored and replicated across runtime instances.\nUpdate is not allowed after the organization is created.\nIf not specified, a Google-Managed encryption key will be used.\nValid only when 'RuntimeType' is CLOUD. For example: 'projects/foo/locations/us/keyRings/bar/cryptoKeys/baz'."]
    pub fn set_runtime_database_encryption_key_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().runtime_database_encryption_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `runtime_type`.\nRuntime type of the Apigee organization based on the Apigee subscription purchased. Default value: \"CLOUD\" Possible values: [\"CLOUD\", \"HYBRID\"]"]
    pub fn set_runtime_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().runtime_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeOrganization_Properties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApigeeOrganization_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `analytics_region` after provisioning.\nPrimary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org)."]
    pub fn analytics_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analytics_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `api_consumer_data_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting API consumer data."]
    pub fn api_consumer_data_encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_consumer_data_encryption_key_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `api_consumer_data_location` after provisioning.\nThis field is needed only for customers using non-default data residency regions.\nApigee stores some control plane data only in single region.\nThis field determines which single region Apigee should use."]
    pub fn api_consumer_data_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_consumer_data_location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `apigee_project_id` after provisioning.\nOutput only. Project ID of the Apigee Tenant Project."]
    pub fn apigee_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apigee_project_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorized_network` after provisioning.\nCompute Engine network used for Service Networking to be peered with Apigee runtime instances.\nSee [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).\nValid only when 'RuntimeType' is set to CLOUD. The value can be updated only when there are no runtime instances. For example: \"default\"."]
    pub fn authorized_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorized_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_type` after provisioning.\nBilling type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing)."]
    pub fn billing_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate` after provisioning.\nOutput only. Base64-encoded public certificate for the root CA of the Apigee organization.\nValid only when 'RuntimeType' is CLOUD. A base64-encoded string."]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting control plane data that is stored in a multi region.\nOnly used for the data residency region \"US\" or \"EU\"."]
    pub fn control_plane_encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_encryption_key_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the Apigee organization."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_vpc_peering` after provisioning.\nFlag that specifies whether the VPC Peering through Private Google Access should be\ndisabled between the consumer network and Apigee. Required if an 'authorizedNetwork'\non the consumer project is not provided, in which case the flag should be set to 'true'.\nValid only when 'RuntimeType' is set to CLOUD. The value must be set before the creation\nof any Apigee runtime instance and can be updated only when there are no runtime instances."]
    pub fn disable_vpc_peering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_vpc_peering", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Apigee organization."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the Apigee organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe project ID associated with the Apigee organization."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention` after provisioning.\nOptional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType\nis not EVALUATION). It controls how long Organization data will be retained after the initial delete\noperation completes. During this period, the Organization may be restored to its last known state.\nAfter this period, the Organization will no longer be able to be restored. Default value: \"DELETION_RETENTION_UNSPECIFIED\" Possible values: [\"DELETION_RETENTION_UNSPECIFIED\", \"MINIMUM\"]"]
    pub fn retention(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_database_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting the data that is stored and replicated across runtime instances.\nUpdate is not allowed after the organization is created.\nIf not specified, a Google-Managed encryption key will be used.\nValid only when 'RuntimeType' is CLOUD. For example: 'projects/foo/locations/us/keyRings/bar/cryptoKeys/baz'."]
    pub fn runtime_database_encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_database_encryption_key_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `runtime_type` after provisioning.\nRuntime type of the Apigee organization based on the Apigee subscription purchased. Default value: \"CLOUD\" Possible values: [\"CLOUD\", \"HYBRID\"]"]
    pub fn runtime_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_type` after provisioning.\nOutput only. Subscription type of the Apigee organization.\nValid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased)."]
    pub fn subscription_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<ApigeeOrganization_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeOrganization_TimeoutsRef {
        ApigeeOrganization_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApigeeOrganization {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApigeeOrganization { }

impl terrars::ToListMappable for ApigeeOrganization {
    type O = terrars::ListRef<ApigeeOrganizationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApigeeOrganization_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apigee_organization".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeOrganization {
    pub tf_id: std::string::String,
    #[doc = "The project ID associated with the Apigee organization."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildApigeeOrganization {
    pub fn build(self, stack: &mut terrars::Stack) -> ApigeeOrganization {
        let out = ApigeeOrganization(std::rc::Rc::new(ApigeeOrganization_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApigeeOrganizationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                analytics_region: core::default::Default::default(),
                api_consumer_data_encryption_key_name: core::default::Default::default(),
                api_consumer_data_location: core::default::Default::default(),
                authorized_network: core::default::Default::default(),
                billing_type: core::default::Default::default(),
                control_plane_encryption_key_name: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_vpc_peering: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                project_id: self.project_id,
                retention: core::default::Default::default(),
                runtime_database_encryption_key_name: core::default::Default::default(),
                runtime_type: core::default::Default::default(),
                properties: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeOrganizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeOrganizationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `analytics_region` after provisioning.\nPrimary GCP region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org)."]
    pub fn analytics_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analytics_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `api_consumer_data_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting API consumer data."]
    pub fn api_consumer_data_encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_consumer_data_encryption_key_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `api_consumer_data_location` after provisioning.\nThis field is needed only for customers using non-default data residency regions.\nApigee stores some control plane data only in single region.\nThis field determines which single region Apigee should use."]
    pub fn api_consumer_data_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_consumer_data_location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `apigee_project_id` after provisioning.\nOutput only. Project ID of the Apigee Tenant Project."]
    pub fn apigee_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.apigee_project_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `authorized_network` after provisioning.\nCompute Engine network used for Service Networking to be peered with Apigee runtime instances.\nSee [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started).\nValid only when 'RuntimeType' is set to CLOUD. The value can be updated only when there are no runtime instances. For example: \"default\"."]
    pub fn authorized_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorized_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_type` after provisioning.\nBilling type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing)."]
    pub fn billing_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ca_certificate` after provisioning.\nOutput only. Base64-encoded public certificate for the root CA of the Apigee organization.\nValid only when 'RuntimeType' is CLOUD. A base64-encoded string."]
    pub fn ca_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `control_plane_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting control plane data that is stored in a multi region.\nOnly used for the data residency region \"US\" or \"EU\"."]
    pub fn control_plane_encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.control_plane_encryption_key_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the Apigee organization."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_vpc_peering` after provisioning.\nFlag that specifies whether the VPC Peering through Private Google Access should be\ndisabled between the consumer network and Apigee. Required if an 'authorizedNetwork'\non the consumer project is not provided, in which case the flag should be set to 'true'.\nValid only when 'RuntimeType' is set to CLOUD. The value must be set before the creation\nof any Apigee runtime instance and can be updated only when there are no runtime instances."]
    pub fn disable_vpc_peering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_vpc_peering", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Apigee organization."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the Apigee organization."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe project ID associated with the Apigee organization."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention` after provisioning.\nOptional. This setting is applicable only for organizations that are soft-deleted (i.e., BillingType\nis not EVALUATION). It controls how long Organization data will be retained after the initial delete\noperation completes. During this period, the Organization may be restored to its last known state.\nAfter this period, the Organization will no longer be able to be restored. Default value: \"DELETION_RETENTION_UNSPECIFIED\" Possible values: [\"DELETION_RETENTION_UNSPECIFIED\", \"MINIMUM\"]"]
    pub fn retention(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_database_encryption_key_name` after provisioning.\nCloud KMS key name used for encrypting the data that is stored and replicated across runtime instances.\nUpdate is not allowed after the organization is created.\nIf not specified, a Google-Managed encryption key will be used.\nValid only when 'RuntimeType' is CLOUD. For example: 'projects/foo/locations/us/keyRings/bar/cryptoKeys/baz'."]
    pub fn runtime_database_encryption_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_database_encryption_key_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `runtime_type` after provisioning.\nRuntime type of the Apigee organization based on the Apigee subscription purchased. Default value: \"CLOUD\" Possible values: [\"CLOUD\", \"HYBRID\"]"]
    pub fn runtime_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_type` after provisioning.\nOutput only. Subscription type of the Apigee organization.\nValid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased)."]
    pub fn subscription_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<ApigeeOrganization_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeOrganization_TimeoutsRef {
        ApigeeOrganization_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ApigeeOrganizationDynamic {
    properties: std::option::Option<terrars::DynamicBlock<ApigeeOrganization_Properties>>,
}

#[derive(Serialize)]
pub struct ApigeeOrganization_Properties_Property {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeOrganization_Properties_Property {
    #[doc = "Set the field `name`.\nName of the property."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nValue of the property."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeOrganization_Properties_Property {
    type O = terrars::BlockAssignable<ApigeeOrganization_Properties_Property>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeOrganization_Properties_Property {}

impl BuildApigeeOrganization_Properties_Property {
    pub fn build(self) -> ApigeeOrganization_Properties_Property {
        ApigeeOrganization_Properties_Property {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeOrganization_Properties_PropertyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeOrganization_Properties_PropertyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeOrganization_Properties_PropertyRef {
        ApigeeOrganization_Properties_PropertyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeOrganization_Properties_PropertyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the property."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the property."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeOrganization_PropertiesDynamic {
    property: std::option::Option<terrars::DynamicBlock<ApigeeOrganization_Properties_Property>>,
}

#[derive(Serialize)]
pub struct ApigeeOrganization_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property: std::option::Option<std::vec::Vec<ApigeeOrganization_Properties_Property>>,
    dynamic: ApigeeOrganization_PropertiesDynamic,
}

impl ApigeeOrganization_Properties {
    #[doc = "Set the field `property`.\n"]
    pub fn set_property(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeOrganization_Properties_Property>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.property = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeOrganization_Properties {
    type O = terrars::BlockAssignable<ApigeeOrganization_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeOrganization_Properties {}

impl BuildApigeeOrganization_Properties {
    pub fn build(self) -> ApigeeOrganization_Properties {
        ApigeeOrganization_Properties {
            property: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeOrganization_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeOrganization_PropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeOrganization_PropertiesRef {
        ApigeeOrganization_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeOrganization_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `property` after provisioning.\n"]
    pub fn property(&self) -> terrars::ListRef<ApigeeOrganization_Properties_PropertyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.property", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeOrganization_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeOrganization_Timeouts {
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

impl terrars::ToListMappable for ApigeeOrganization_Timeouts {
    type O = terrars::BlockAssignable<ApigeeOrganization_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeOrganization_Timeouts {}

impl BuildApigeeOrganization_Timeouts {
    pub fn build(self) -> ApigeeOrganization_Timeouts {
        ApigeeOrganization_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeOrganization_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeOrganization_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeOrganization_TimeoutsRef {
        ApigeeOrganization_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeOrganization_TimeoutsRef {
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
