#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ClouddomainsRegistrationData_ {
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
    contact_notices: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_notices: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contact_settings: std::option::Option<std::vec::Vec<ClouddomainsRegistration_ContactSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_settings: std::option::Option<std::vec::Vec<ClouddomainsRegistration_DnsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management_settings: std::option::Option<std::vec::Vec<ClouddomainsRegistration_ManagementSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ClouddomainsRegistration_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    yearly_price: std::option::Option<std::vec::Vec<ClouddomainsRegistration_YearlyPrice>>,
    dynamic: ClouddomainsRegistrationDynamic,
}

struct ClouddomainsRegistration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ClouddomainsRegistrationData_>,
}

#[derive(Clone)]
pub struct ClouddomainsRegistration(std::rc::Rc<ClouddomainsRegistration_>);

impl ClouddomainsRegistration {
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
        "Set the field `contact_notices`.\nThe list of contact notices that the caller acknowledges. Possible value is PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT"]
    pub fn set_contact_notices(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().contact_notices = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `domain_notices`.\nThe list of domain notices that you acknowledge. Possible value is HSTS_PRELOADED"]
    pub fn set_domain_notices(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().domain_notices = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nSet of labels associated with the Registration.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `contact_settings`.\n"]
    pub fn set_contact_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().contact_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.contact_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dns_settings`.\n"]
    pub fn set_dns_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddomainsRegistration_DnsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dns_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dns_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `management_settings`.\n"]
    pub fn set_management_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddomainsRegistration_ManagementSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().management_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.management_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ClouddomainsRegistration_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `yearly_price`.\n"]
    pub fn set_yearly_price(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddomainsRegistration_YearlyPrice>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().yearly_price = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.yearly_price = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `contact_notices` after provisioning.\nThe list of contact notices that the caller acknowledges. Possible value is PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT"]
    pub fn contact_notices(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contact_notices", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the automation was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_name` after provisioning.\nRequired. The domain name. Unicode domain names must be expressed in Punycode format."]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_notices` after provisioning.\nThe list of domain notices that you acknowledge. Possible value is HSTS_PRELOADED"]
    pub fn domain_notices(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_notices", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nOutput only. Time at which the automation was updated."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issues` after provisioning.\nOutput only. The set of issues with the Registration that require attention."]
    pub fn issues(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issues", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the Registration.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the Registration resource, in the format projects/*/locations/*/registrations/<domain_name>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `register_failure_reason` after provisioning.\nOutput only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state."]
    pub fn register_failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.register_failure_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the Registration."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_privacy` after provisioning.\nOutput only. Set of options for the contactSettings.privacy field that this Registration supports."]
    pub fn supported_privacy(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_privacy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_settings` after provisioning.\n"]
    pub fn contact_settings(&self) -> terrars::ListRef<ClouddomainsRegistration_ContactSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contact_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_settings` after provisioning.\n"]
    pub fn dns_settings(&self) -> terrars::ListRef<ClouddomainsRegistration_DnsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management_settings` after provisioning.\n"]
    pub fn management_settings(&self) -> terrars::ListRef<ClouddomainsRegistration_ManagementSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddomainsRegistration_TimeoutsRef {
        ClouddomainsRegistration_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `yearly_price` after provisioning.\n"]
    pub fn yearly_price(&self) -> terrars::ListRef<ClouddomainsRegistration_YearlyPriceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yearly_price", self.extract_ref()))
    }
}

impl terrars::Referable for ClouddomainsRegistration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ClouddomainsRegistration { }

impl terrars::ToListMappable for ClouddomainsRegistration {
    type O = terrars::ListRef<ClouddomainsRegistrationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ClouddomainsRegistration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_clouddomains_registration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildClouddomainsRegistration {
    pub tf_id: std::string::String,
    #[doc = "Required. The domain name. Unicode domain names must be expressed in Punycode format."]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildClouddomainsRegistration {
    pub fn build(self, stack: &mut terrars::Stack) -> ClouddomainsRegistration {
        let out = ClouddomainsRegistration(std::rc::Rc::new(ClouddomainsRegistration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ClouddomainsRegistrationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                contact_notices: core::default::Default::default(),
                domain_name: self.domain_name,
                domain_notices: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                contact_settings: core::default::Default::default(),
                dns_settings: core::default::Default::default(),
                management_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                yearly_price: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ClouddomainsRegistrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ClouddomainsRegistrationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contact_notices` after provisioning.\nThe list of contact notices that the caller acknowledges. Possible value is PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT"]
    pub fn contact_notices(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contact_notices", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the automation was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_name` after provisioning.\nRequired. The domain name. Unicode domain names must be expressed in Punycode format."]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `domain_notices` after provisioning.\nThe list of domain notices that you acknowledge. Possible value is HSTS_PRELOADED"]
    pub fn domain_notices(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.domain_notices", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nOutput only. Time at which the automation was updated."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `issues` after provisioning.\nOutput only. The set of issues with the Registration that require attention."]
    pub fn issues(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.issues", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the Registration.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the Registration resource, in the format projects/*/locations/*/registrations/<domain_name>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `register_failure_reason` after provisioning.\nOutput only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state."]
    pub fn register_failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.register_failure_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the Registration."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `supported_privacy` after provisioning.\nOutput only. Set of options for the contactSettings.privacy field that this Registration supports."]
    pub fn supported_privacy(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_privacy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_settings` after provisioning.\n"]
    pub fn contact_settings(&self) -> terrars::ListRef<ClouddomainsRegistration_ContactSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.contact_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_settings` after provisioning.\n"]
    pub fn dns_settings(&self) -> terrars::ListRef<ClouddomainsRegistration_DnsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management_settings` after provisioning.\n"]
    pub fn management_settings(&self) -> terrars::ListRef<ClouddomainsRegistration_ManagementSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddomainsRegistration_TimeoutsRef {
        ClouddomainsRegistration_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `yearly_price` after provisioning.\n"]
    pub fn yearly_price(&self) -> terrars::ListRef<ClouddomainsRegistration_YearlyPriceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.yearly_price", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ClouddomainsRegistrationDynamic {
    contact_settings: std::option::Option<terrars::DynamicBlock<ClouddomainsRegistration_ContactSettings>>,
    dns_settings: std::option::Option<terrars::DynamicBlock<ClouddomainsRegistration_DnsSettings>>,
    management_settings: std::option::Option<terrars::DynamicBlock<ClouddomainsRegistration_ManagementSettings>>,
    yearly_price: std::option::Option<terrars::DynamicBlock<ClouddomainsRegistration_YearlyPrice>>,
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_lines: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    administrative_area: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recipients: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    region_code: terrars::PrimField<std::string::String>,
}

impl ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress {
    #[doc =
        "Set the field `address_lines`.\nUnstructured address lines describing the lower levels of an address.\nBecause values in addressLines do not have type information and may sometimes contain multiple values in a single\nfield (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be\n\"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language\nis used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way,\nthe most specific line of an address can be selected based on the language."]
    pub fn set_address_lines(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.address_lines = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `administrative_area`.\nHighest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state,\na province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community\n(e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland\nthis should be left unpopulated."]
    pub fn set_administrative_area(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.administrative_area = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `locality`.\nGenerally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world\nwhere localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines."]
    pub fn set_locality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.locality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organization`.\nThe name of the organization at the address."]
    pub fn set_organization(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.organization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `postal_code`.\nPostal code of the address. Not all countries use or require postal codes to be present, but where they are used,\nthey may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
    pub fn set_postal_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `recipients`.\nThe recipient at the address. This field may, under certain circumstances, contain multiline information. For example,\nit might contain \"care of\" information."]
    pub fn set_recipients(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.recipients = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress {
    #[doc =
        "Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to\nensure the value is correct. See https://cldr.unicode.org/ and\nhttps://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
    pub region_code: terrars::PrimField<std::string::String>,
}

impl BuildClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress {
    pub fn build(self) -> ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress {
        ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress {
            address_lines: core::default::Default::default(),
            administrative_area: core::default::Default::default(),
            locality: core::default::Default::default(),
            organization: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            recipients: core::default::Default::default(),
            region_code: self.region_code,
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddressRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddressRef {
        ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address_lines` after provisioning.\nUnstructured address lines describing the lower levels of an address.\nBecause values in addressLines do not have type information and may sometimes contain multiple values in a single\nfield (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be\n\"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language\nis used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way,\nthe most specific line of an address can be selected based on the language."]
    pub fn address_lines(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_lines", self.base))
    }

    #[doc =
        "Get a reference to the value of field `administrative_area` after provisioning.\nHighest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state,\na province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community\n(e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland\nthis should be left unpopulated."]
    pub fn administrative_area(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.administrative_area", self.base))
    }

    #[doc =
        "Get a reference to the value of field `locality` after provisioning.\nGenerally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world\nwhere localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines."]
    pub fn locality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }

    #[doc =
        "Get a reference to the value of field `organization` after provisioning.\nThe name of the organization at the address."]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `postal_code` after provisioning.\nPostal code of the address. Not all countries use or require postal codes to be present, but where they are used,\nthey may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `recipients` after provisioning.\nThe recipient at the address. This field may, under certain circumstances, contain multiline information. For example,\nit might contain \"care of\" information."]
    pub fn recipients(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recipients", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region_code` after provisioning.\nRequired. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to\nensure the value is correct. See https://cldr.unicode.org/ and\nhttps://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
    pub fn region_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_code", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddomainsRegistration_ContactSettings_AdminContactDynamic {
    postal_address: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress>>,
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_ContactSettings_AdminContact {
    email: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fax_number: std::option::Option<terrars::PrimField<std::string::String>>,
    phone_number: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_address: std
    ::option
    ::Option<std::vec::Vec<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress>>,
    dynamic: ClouddomainsRegistration_ContactSettings_AdminContactDynamic,
}

impl ClouddomainsRegistration_ContactSettings_AdminContact {
    #[doc =
        "Set the field `fax_number`.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn set_fax_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fax_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `postal_address`.\n"]
    pub fn set_postal_address(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postal_address = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postal_address = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_ContactSettings_AdminContact {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings_AdminContact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_ContactSettings_AdminContact {
    #[doc = "Required. Email address of the contact."]
    pub email: terrars::PrimField<std::string::String>,
    #[doc = "Required. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildClouddomainsRegistration_ContactSettings_AdminContact {
    pub fn build(self) -> ClouddomainsRegistration_ContactSettings_AdminContact {
        ClouddomainsRegistration_ContactSettings_AdminContact {
            email: self.email,
            fax_number: core::default::Default::default(),
            phone_number: self.phone_number,
            postal_address: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_ContactSettings_AdminContactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_ContactSettings_AdminContactRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddomainsRegistration_ContactSettings_AdminContactRef {
        ClouddomainsRegistration_ContactSettings_AdminContactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_ContactSettings_AdminContactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contact."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fax_number` after provisioning.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn fax_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax_number", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nRequired. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_address` after provisioning.\n"]
    pub fn postal_address(
        &self,
    ) -> terrars::ListRef<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postal_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddomainsRegistration_ContactSettings_RegistrantContactDynamic {
    postal_address: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress>>,
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_ContactSettings_RegistrantContact {
    email: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fax_number: std::option::Option<terrars::PrimField<std::string::String>>,
    phone_number: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_address: std
    ::option
    ::Option<std::vec::Vec<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress>>,
    dynamic: ClouddomainsRegistration_ContactSettings_RegistrantContactDynamic,
}

impl ClouddomainsRegistration_ContactSettings_RegistrantContact {
    #[doc =
        "Set the field `fax_number`.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn set_fax_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fax_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `postal_address`.\n"]
    pub fn set_postal_address(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postal_address = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postal_address = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_ContactSettings_RegistrantContact {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings_RegistrantContact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_ContactSettings_RegistrantContact {
    #[doc = "Required. Email address of the contact."]
    pub email: terrars::PrimField<std::string::String>,
    #[doc = "Required. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildClouddomainsRegistration_ContactSettings_RegistrantContact {
    pub fn build(self) -> ClouddomainsRegistration_ContactSettings_RegistrantContact {
        ClouddomainsRegistration_ContactSettings_RegistrantContact {
            email: self.email,
            fax_number: core::default::Default::default(),
            phone_number: self.phone_number,
            postal_address: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_ContactSettings_RegistrantContactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_ContactSettings_RegistrantContactRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddomainsRegistration_ContactSettings_RegistrantContactRef {
        ClouddomainsRegistration_ContactSettings_RegistrantContactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_ContactSettings_RegistrantContactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contact."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fax_number` after provisioning.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn fax_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax_number", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nRequired. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_address` after provisioning.\n"]
    pub fn postal_address(
        &self,
    ) -> terrars::ListRef<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postal_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddomainsRegistration_ContactSettings_TechnicalContactDynamic {
    postal_address: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress>>,
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_ContactSettings_TechnicalContact {
    email: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fax_number: std::option::Option<terrars::PrimField<std::string::String>>,
    phone_number: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postal_address: std
    ::option
    ::Option<std::vec::Vec<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress>>,
    dynamic: ClouddomainsRegistration_ContactSettings_TechnicalContactDynamic,
}

impl ClouddomainsRegistration_ContactSettings_TechnicalContact {
    #[doc =
        "Set the field `fax_number`.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn set_fax_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fax_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `postal_address`.\n"]
    pub fn set_postal_address(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddress>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postal_address = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postal_address = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_ContactSettings_TechnicalContact {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings_TechnicalContact>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_ContactSettings_TechnicalContact {
    #[doc = "Required. Email address of the contact."]
    pub email: terrars::PrimField<std::string::String>,
    #[doc = "Required. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub phone_number: terrars::PrimField<std::string::String>,
}

impl BuildClouddomainsRegistration_ContactSettings_TechnicalContact {
    pub fn build(self) -> ClouddomainsRegistration_ContactSettings_TechnicalContact {
        ClouddomainsRegistration_ContactSettings_TechnicalContact {
            email: self.email,
            fax_number: core::default::Default::default(),
            phone_number: self.phone_number,
            postal_address: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_ContactSettings_TechnicalContactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_ContactSettings_TechnicalContactRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddomainsRegistration_ContactSettings_TechnicalContactRef {
        ClouddomainsRegistration_ContactSettings_TechnicalContactRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_ContactSettings_TechnicalContactRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contact."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fax_number` after provisioning.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn fax_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fax_number", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nRequired. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc = "Get a reference to the value of field `postal_address` after provisioning.\n"]
    pub fn postal_address(
        &self,
    ) -> terrars::ListRef<ClouddomainsRegistration_ContactSettings_AdminContact_PostalAddressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postal_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddomainsRegistration_ContactSettingsDynamic {
    admin_contact: std::option::Option<terrars::DynamicBlock<ClouddomainsRegistration_ContactSettings_AdminContact>>,
    registrant_contact: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddomainsRegistration_ContactSettings_RegistrantContact>>,
    technical_contact: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddomainsRegistration_ContactSettings_TechnicalContact>>,
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_ContactSettings {
    privacy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_contact: std::option::Option<std::vec::Vec<ClouddomainsRegistration_ContactSettings_AdminContact>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registrant_contact: std::option::Option<std::vec::Vec<ClouddomainsRegistration_ContactSettings_RegistrantContact>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    technical_contact: std::option::Option<std::vec::Vec<ClouddomainsRegistration_ContactSettings_TechnicalContact>>,
    dynamic: ClouddomainsRegistration_ContactSettingsDynamic,
}

impl ClouddomainsRegistration_ContactSettings {
    #[doc = "Set the field `admin_contact`.\n"]
    pub fn set_admin_contact(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings_AdminContact>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.admin_contact = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.admin_contact = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `registrant_contact`.\n"]
    pub fn set_registrant_contact(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings_RegistrantContact>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.registrant_contact = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.registrant_contact = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `technical_contact`.\n"]
    pub fn set_technical_contact(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings_TechnicalContact>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.technical_contact = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.technical_contact = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_ContactSettings {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_ContactSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_ContactSettings {
    #[doc =
        "Required. Privacy setting for the contacts associated with the Registration.\nValues are PUBLIC_CONTACT_DATA, PRIVATE_CONTACT_DATA, and REDACTED_CONTACT_DATA"]
    pub privacy: terrars::PrimField<std::string::String>,
}

impl BuildClouddomainsRegistration_ContactSettings {
    pub fn build(self) -> ClouddomainsRegistration_ContactSettings {
        ClouddomainsRegistration_ContactSettings {
            privacy: self.privacy,
            admin_contact: core::default::Default::default(),
            registrant_contact: core::default::Default::default(),
            technical_contact: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_ContactSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_ContactSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddomainsRegistration_ContactSettingsRef {
        ClouddomainsRegistration_ContactSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_ContactSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `privacy` after provisioning.\nRequired. Privacy setting for the contacts associated with the Registration.\nValues are PUBLIC_CONTACT_DATA, PRIVATE_CONTACT_DATA, and REDACTED_CONTACT_DATA"]
    pub fn privacy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.privacy", self.base))
    }

    #[doc = "Get a reference to the value of field `admin_contact` after provisioning.\n"]
    pub fn admin_contact(&self) -> terrars::ListRef<ClouddomainsRegistration_ContactSettings_AdminContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_contact", self.base))
    }

    #[doc = "Get a reference to the value of field `registrant_contact` after provisioning.\n"]
    pub fn registrant_contact(&self) -> terrars::ListRef<ClouddomainsRegistration_ContactSettings_RegistrantContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registrant_contact", self.base))
    }

    #[doc = "Get a reference to the value of field `technical_contact` after provisioning.\n"]
    pub fn technical_contact(&self) -> terrars::ListRef<ClouddomainsRegistration_ContactSettings_TechnicalContactRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.technical_contact", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_DnsSettings_CustomDns_DsRecords {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digest: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digest_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_tag: std::option::Option<terrars::PrimField<f64>>,
}

impl ClouddomainsRegistration_DnsSettings_CustomDns_DsRecords {
    #[doc = "Set the field `algorithm`.\nThe algorithm used to generate the referenced DNSKEY."]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `digest`.\nThe digest generated from the referenced DNSKEY."]
    pub fn set_digest(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.digest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `digest_type`.\nThe hash function used to generate the digest of the referenced DNSKEY."]
    pub fn set_digest_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.digest_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_tag`.\nThe key tag of the record. Must be set in range 0 -- 65535."]
    pub fn set_key_tag(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.key_tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_DnsSettings_CustomDns_DsRecords {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_DnsSettings_CustomDns_DsRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_DnsSettings_CustomDns_DsRecords {}

impl BuildClouddomainsRegistration_DnsSettings_CustomDns_DsRecords {
    pub fn build(self) -> ClouddomainsRegistration_DnsSettings_CustomDns_DsRecords {
        ClouddomainsRegistration_DnsSettings_CustomDns_DsRecords {
            algorithm: core::default::Default::default(),
            digest: core::default::Default::default(),
            digest_type: core::default::Default::default(),
            key_tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_DnsSettings_CustomDns_DsRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_DnsSettings_CustomDns_DsRecordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddomainsRegistration_DnsSettings_CustomDns_DsRecordsRef {
        ClouddomainsRegistration_DnsSettings_CustomDns_DsRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_DnsSettings_CustomDns_DsRecordsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `algorithm` after provisioning.\nThe algorithm used to generate the referenced DNSKEY."]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc =
        "Get a reference to the value of field `digest` after provisioning.\nThe digest generated from the referenced DNSKEY."]
    pub fn digest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }

    #[doc =
        "Get a reference to the value of field `digest_type` after provisioning.\nThe hash function used to generate the digest of the referenced DNSKEY."]
    pub fn digest_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_tag` after provisioning.\nThe key tag of the record. Must be set in range 0 -- 65535."]
    pub fn key_tag(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddomainsRegistration_DnsSettings_CustomDnsDynamic {
    ds_records: std::option::Option<terrars::DynamicBlock<ClouddomainsRegistration_DnsSettings_CustomDns_DsRecords>>,
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_DnsSettings_CustomDns {
    name_servers: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ds_records: std::option::Option<std::vec::Vec<ClouddomainsRegistration_DnsSettings_CustomDns_DsRecords>>,
    dynamic: ClouddomainsRegistration_DnsSettings_CustomDnsDynamic,
}

impl ClouddomainsRegistration_DnsSettings_CustomDns {
    #[doc = "Set the field `ds_records`.\n"]
    pub fn set_ds_records(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ClouddomainsRegistration_DnsSettings_CustomDns_DsRecords>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ds_records = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ds_records = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_DnsSettings_CustomDns {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_DnsSettings_CustomDns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_DnsSettings_CustomDns {
    #[doc =
        "Required. A list of name servers that store the DNS zone for this domain. Each name server is a domain\nname, with Unicode domain names expressed in Punycode format."]
    pub name_servers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildClouddomainsRegistration_DnsSettings_CustomDns {
    pub fn build(self) -> ClouddomainsRegistration_DnsSettings_CustomDns {
        ClouddomainsRegistration_DnsSettings_CustomDns {
            name_servers: self.name_servers,
            ds_records: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_DnsSettings_CustomDnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_DnsSettings_CustomDnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddomainsRegistration_DnsSettings_CustomDnsRef {
        ClouddomainsRegistration_DnsSettings_CustomDnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_DnsSettings_CustomDnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name_servers` after provisioning.\nRequired. A list of name servers that store the DNS zone for this domain. Each name server is a domain\nname, with Unicode domain names expressed in Punycode format."]
    pub fn name_servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `ds_records` after provisioning.\n"]
    pub fn ds_records(&self) -> terrars::ListRef<ClouddomainsRegistration_DnsSettings_CustomDns_DsRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ds_records", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_DnsSettings_GlueRecords {
    host_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ClouddomainsRegistration_DnsSettings_GlueRecords {
    #[doc =
        "Set the field `ipv4_addresses`.\nList of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).\nAt least one of ipv4_address and ipv6_address must be set."]
    pub fn set_ipv4_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv4_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv6_addresses`.\nList of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).\nAt least one of ipv4_address and ipv6_address must be set."]
    pub fn set_ipv6_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv6_addresses = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_DnsSettings_GlueRecords {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_DnsSettings_GlueRecords>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_DnsSettings_GlueRecords {
    #[doc = "Required. Domain name of the host in Punycode format."]
    pub host_name: terrars::PrimField<std::string::String>,
}

impl BuildClouddomainsRegistration_DnsSettings_GlueRecords {
    pub fn build(self) -> ClouddomainsRegistration_DnsSettings_GlueRecords {
        ClouddomainsRegistration_DnsSettings_GlueRecords {
            host_name: self.host_name,
            ipv4_addresses: core::default::Default::default(),
            ipv6_addresses: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_DnsSettings_GlueRecordsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_DnsSettings_GlueRecordsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddomainsRegistration_DnsSettings_GlueRecordsRef {
        ClouddomainsRegistration_DnsSettings_GlueRecordsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_DnsSettings_GlueRecordsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_name` after provisioning.\nRequired. Domain name of the host in Punycode format."]
    pub fn host_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv4_addresses` after provisioning.\nList of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).\nAt least one of ipv4_address and ipv6_address must be set."]
    pub fn ipv4_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_addresses` after provisioning.\nList of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).\nAt least one of ipv4_address and ipv6_address must be set."]
    pub fn ipv6_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddomainsRegistration_DnsSettingsDynamic {
    custom_dns: std::option::Option<terrars::DynamicBlock<ClouddomainsRegistration_DnsSettings_CustomDns>>,
    glue_records: std::option::Option<terrars::DynamicBlock<ClouddomainsRegistration_DnsSettings_GlueRecords>>,
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_DnsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_dns: std::option::Option<std::vec::Vec<ClouddomainsRegistration_DnsSettings_CustomDns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glue_records: std::option::Option<std::vec::Vec<ClouddomainsRegistration_DnsSettings_GlueRecords>>,
    dynamic: ClouddomainsRegistration_DnsSettingsDynamic,
}

impl ClouddomainsRegistration_DnsSettings {
    #[doc = "Set the field `custom_dns`.\n"]
    pub fn set_custom_dns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddomainsRegistration_DnsSettings_CustomDns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_dns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_dns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `glue_records`.\n"]
    pub fn set_glue_records(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddomainsRegistration_DnsSettings_GlueRecords>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.glue_records = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.glue_records = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_DnsSettings {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_DnsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_DnsSettings {}

impl BuildClouddomainsRegistration_DnsSettings {
    pub fn build(self) -> ClouddomainsRegistration_DnsSettings {
        ClouddomainsRegistration_DnsSettings {
            custom_dns: core::default::Default::default(),
            glue_records: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_DnsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_DnsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddomainsRegistration_DnsSettingsRef {
        ClouddomainsRegistration_DnsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_DnsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_dns` after provisioning.\n"]
    pub fn custom_dns(&self) -> terrars::ListRef<ClouddomainsRegistration_DnsSettings_CustomDnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_dns", self.base))
    }

    #[doc = "Get a reference to the value of field `glue_records` after provisioning.\n"]
    pub fn glue_records(&self) -> terrars::ListRef<ClouddomainsRegistration_DnsSettings_GlueRecordsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glue_records", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_ManagementSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_renewal_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_lock_state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddomainsRegistration_ManagementSettings {
    #[doc =
        "Set the field `preferred_renewal_method`.\nThe desired renewal method for this Registration. The actual renewalMethod is automatically updated to reflect this choice.\nIf unset or equal to RENEWAL_METHOD_UNSPECIFIED, the actual renewalMethod is treated as if it were set to AUTOMATIC_RENEWAL.\nYou cannot use RENEWAL_DISABLED during resource creation, and you can update the renewal status only when the Registration\nresource has state ACTIVE or SUSPENDED.\n\nWhen preferredRenewalMethod is set to AUTOMATIC_RENEWAL, the actual renewalMethod can be set to RENEWAL_DISABLED in case of\nproblems with the billing account or reported domain abuse. In such cases, check the issues field on the Registration. After\nthe problem is resolved, the renewalMethod is automatically updated to preferredRenewalMethod in a few hours."]
    pub fn set_preferred_renewal_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.preferred_renewal_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transfer_lock_state`.\nControls whether the domain can be transferred to another registrar. Values are UNLOCKED or LOCKED."]
    pub fn set_transfer_lock_state(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transfer_lock_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_ManagementSettings {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_ManagementSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_ManagementSettings {}

impl BuildClouddomainsRegistration_ManagementSettings {
    pub fn build(self) -> ClouddomainsRegistration_ManagementSettings {
        ClouddomainsRegistration_ManagementSettings {
            preferred_renewal_method: core::default::Default::default(),
            transfer_lock_state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_ManagementSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_ManagementSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddomainsRegistration_ManagementSettingsRef {
        ClouddomainsRegistration_ManagementSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_ManagementSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `preferred_renewal_method` after provisioning.\nThe desired renewal method for this Registration. The actual renewalMethod is automatically updated to reflect this choice.\nIf unset or equal to RENEWAL_METHOD_UNSPECIFIED, the actual renewalMethod is treated as if it were set to AUTOMATIC_RENEWAL.\nYou cannot use RENEWAL_DISABLED during resource creation, and you can update the renewal status only when the Registration\nresource has state ACTIVE or SUSPENDED.\n\nWhen preferredRenewalMethod is set to AUTOMATIC_RENEWAL, the actual renewalMethod can be set to RENEWAL_DISABLED in case of\nproblems with the billing account or reported domain abuse. In such cases, check the issues field on the Registration. After\nthe problem is resolved, the renewalMethod is automatically updated to preferredRenewalMethod in a few hours."]
    pub fn preferred_renewal_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_renewal_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `renewal_method` after provisioning.\nOutput only. The actual renewal method for this Registration. When preferredRenewalMethod is set to AUTOMATIC_RENEWAL,\nthe actual renewalMethod can be equal to RENEWAL_DISABLED—for example, when there are problems with the billing account\nor reported domain abuse. In such cases, check the issues field on the Registration. After the problem is resolved, the\nrenewalMethod is automatically updated to preferredRenewalMethod in a few hours."]
    pub fn renewal_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.renewal_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transfer_lock_state` after provisioning.\nControls whether the domain can be transferred to another registrar. Values are UNLOCKED or LOCKED."]
    pub fn transfer_lock_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_lock_state", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddomainsRegistration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddomainsRegistration_Timeouts {
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

impl terrars::ToListMappable for ClouddomainsRegistration_Timeouts {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_Timeouts {}

impl BuildClouddomainsRegistration_Timeouts {
    pub fn build(self) -> ClouddomainsRegistration_Timeouts {
        ClouddomainsRegistration_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddomainsRegistration_TimeoutsRef {
        ClouddomainsRegistration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_TimeoutsRef {
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

#[derive(Serialize)]
pub struct ClouddomainsRegistration_YearlyPrice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    units: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddomainsRegistration_YearlyPrice {
    #[doc = "Set the field `currency_code`.\nThe three-letter currency code defined in ISO 4217."]
    pub fn set_currency_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `units`.\nThe whole units of the amount. For example if currencyCode is \"USD\", then 1 unit is one US dollar."]
    pub fn set_units(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddomainsRegistration_YearlyPrice {
    type O = terrars::BlockAssignable<ClouddomainsRegistration_YearlyPrice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddomainsRegistration_YearlyPrice {}

impl BuildClouddomainsRegistration_YearlyPrice {
    pub fn build(self) -> ClouddomainsRegistration_YearlyPrice {
        ClouddomainsRegistration_YearlyPrice {
            currency_code: core::default::Default::default(),
            units: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddomainsRegistration_YearlyPriceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddomainsRegistration_YearlyPriceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddomainsRegistration_YearlyPriceRef {
        ClouddomainsRegistration_YearlyPriceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddomainsRegistration_YearlyPriceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `currency_code` after provisioning.\nThe three-letter currency code defined in ISO 4217."]
    pub fn currency_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `units` after provisioning.\nThe whole units of the amount. For example if currencyCode is \"USD\", then 1 unit is one US dollar."]
    pub fn units(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}
