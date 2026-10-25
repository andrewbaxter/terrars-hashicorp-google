#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudQuotasQuotaPreferenceData_ {
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
    contact_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_safety_checks: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    justification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_config: std::option::Option<std::vec::Vec<CloudQuotasQuotaPreference_QuotaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudQuotasQuotaPreference_Timeouts>,
    dynamic: CloudQuotasQuotaPreferenceDynamic,
}

struct CloudQuotasQuotaPreference_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudQuotasQuotaPreferenceData_>,
}

#[derive(Clone)]
pub struct CloudQuotasQuotaPreference(std::rc::Rc<CloudQuotasQuotaPreference_>);

impl CloudQuotasQuotaPreference {
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
        "Set the field `contact_email`.\nAn email address that can be used for quota related communication between the Google Cloud and the user in case the Google Cloud needs further information to make a decision on whether the user preferred quota can be granted.\n\nThe Google account for the email address must have quota update permission for the project, folder or organization this quota preference is for."]
    pub fn set_contact_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().contact_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dimensions`.\nThe dimensions that this quota preference applies to. The key of the map entry is the name of a dimension, such as \"region\", \"zone\", \"network_id\", and the value of the map entry is the dimension value. If a dimension is missing from the map of dimensions, the quota preference applies to all the dimension values except for those that have other quota preferences configured for the specific value.\n\nNOTE: QuotaPreferences can only be applied across all values of \"user\" and \"resource\" dimension. Do not set values for \"user\" or \"resource\" in the dimension map.\n\nExample: '{\"provider\": \"Foo Inc\"}' where \"provider\" is a service specific dimension."]
    pub fn set_dimensions(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_safety_checks`.\nThe list of quota safety checks to be ignored. Default value: \"QUOTA_SAFETY_CHECK_UNSPECIFIED\" Possible values: [\"QUOTA_SAFETY_CHECK_UNSPECIFIED\", \"QUOTA_DECREASE_BELOW_USAGE\", \"QUOTA_DECREASE_PERCENTAGE_TOO_HIGH\"]"]
    pub fn set_ignore_safety_checks(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ignore_safety_checks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `justification`.\nThe reason / justification for this quota preference."]
    pub fn set_justification(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().justification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe resource name of the quota preference. Required except in the CREATE requests."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parent`.\nThe parent of the quota preference. Allowed parents are \"projects/[project-id / number]\" or \"folders/[folder-id / number]\" or \"organizations/[org-id / number]\"."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `quota_id`.\nThe id of the quota to which the quota preference is applied. A quota id is unique in the service.\nExample: 'CPUS-per-project-region'."]
    pub fn set_quota_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().quota_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\nThe name of the service to which the quota preference is applied."]
    pub fn set_service(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quota_config`.\n"]
    pub fn set_quota_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudQuotasQuotaPreference_QuotaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().quota_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.quota_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudQuotasQuotaPreference_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `contact_email` after provisioning.\nAn email address that can be used for quota related communication between the Google Cloud and the user in case the Google Cloud needs further information to make a decision on whether the user preferred quota can be granted.\n\nThe Google account for the email address must have quota update permission for the project, folder or organization this quota preference is for."]
    pub fn contact_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreate time stamp.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dimensions` after provisioning.\nThe dimensions that this quota preference applies to. The key of the map entry is the name of a dimension, such as \"region\", \"zone\", \"network_id\", and the value of the map entry is the dimension value. If a dimension is missing from the map of dimensions, the quota preference applies to all the dimension values except for those that have other quota preferences configured for the specific value.\n\nNOTE: QuotaPreferences can only be applied across all values of \"user\" and \"resource\" dimension. Do not set values for \"user\" or \"resource\" in the dimension map.\n\nExample: '{\"provider\": \"Foo Inc\"}' where \"provider\" is a service specific dimension."]
    pub fn dimensions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dimensions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe current etag of the quota preference. If an etag is provided on update and does not match the current server's etag of the quota preference, the request will be blocked and an ABORTED error will be returned. See https://google.aip.dev/134#etags for more details on etags."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_safety_checks` after provisioning.\nThe list of quota safety checks to be ignored. Default value: \"QUOTA_SAFETY_CHECK_UNSPECIFIED\" Possible values: [\"QUOTA_SAFETY_CHECK_UNSPECIFIED\", \"QUOTA_DECREASE_BELOW_USAGE\", \"QUOTA_DECREASE_PERCENTAGE_TOO_HIGH\"]"]
    pub fn ignore_safety_checks(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_safety_checks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `justification` after provisioning.\nThe reason / justification for this quota preference."]
    pub fn justification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.justification", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the quota preference. Required except in the CREATE requests."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the quota preference. Allowed parents are \"projects/[project-id / number]\" or \"folders/[folder-id / number]\" or \"organizations/[org-id / number]\"."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quota_id` after provisioning.\nThe id of the quota to which the quota preference is applied. A quota id is unique in the service.\nExample: 'CPUS-per-project-region'."]
    pub fn quota_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIs the quota preference pending Google Cloud approval and fulfillment."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of the service to which the quota preference is applied."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nUpdate time stamp.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_config` after provisioning.\n"]
    pub fn quota_config(&self) -> terrars::ListRef<CloudQuotasQuotaPreference_QuotaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudQuotasQuotaPreference_TimeoutsRef {
        CloudQuotasQuotaPreference_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CloudQuotasQuotaPreference {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudQuotasQuotaPreference { }

impl terrars::ToListMappable for CloudQuotasQuotaPreference {
    type O = terrars::ListRef<CloudQuotasQuotaPreferenceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudQuotasQuotaPreference_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_quotas_quota_preference".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudQuotasQuotaPreference {
    pub tf_id: std::string::String,
}

impl BuildCloudQuotasQuotaPreference {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudQuotasQuotaPreference {
        let out = CloudQuotasQuotaPreference(std::rc::Rc::new(CloudQuotasQuotaPreference_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudQuotasQuotaPreferenceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                contact_email: core::default::Default::default(),
                dimensions: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_safety_checks: core::default::Default::default(),
                justification: core::default::Default::default(),
                name: core::default::Default::default(),
                parent: core::default::Default::default(),
                quota_id: core::default::Default::default(),
                service: core::default::Default::default(),
                quota_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudQuotasQuotaPreferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudQuotasQuotaPreferenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudQuotasQuotaPreferenceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contact_email` after provisioning.\nAn email address that can be used for quota related communication between the Google Cloud and the user in case the Google Cloud needs further information to make a decision on whether the user preferred quota can be granted.\n\nThe Google account for the email address must have quota update permission for the project, folder or organization this quota preference is for."]
    pub fn contact_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nCreate time stamp.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dimensions` after provisioning.\nThe dimensions that this quota preference applies to. The key of the map entry is the name of a dimension, such as \"region\", \"zone\", \"network_id\", and the value of the map entry is the dimension value. If a dimension is missing from the map of dimensions, the quota preference applies to all the dimension values except for those that have other quota preferences configured for the specific value.\n\nNOTE: QuotaPreferences can only be applied across all values of \"user\" and \"resource\" dimension. Do not set values for \"user\" or \"resource\" in the dimension map.\n\nExample: '{\"provider\": \"Foo Inc\"}' where \"provider\" is a service specific dimension."]
    pub fn dimensions(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.dimensions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe current etag of the quota preference. If an etag is provided on update and does not match the current server's etag of the quota preference, the request will be blocked and an ABORTED error will be returned. See https://google.aip.dev/134#etags for more details on etags."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_safety_checks` after provisioning.\nThe list of quota safety checks to be ignored. Default value: \"QUOTA_SAFETY_CHECK_UNSPECIFIED\" Possible values: [\"QUOTA_SAFETY_CHECK_UNSPECIFIED\", \"QUOTA_DECREASE_BELOW_USAGE\", \"QUOTA_DECREASE_PERCENTAGE_TOO_HIGH\"]"]
    pub fn ignore_safety_checks(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_safety_checks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `justification` after provisioning.\nThe reason / justification for this quota preference."]
    pub fn justification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.justification", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the quota preference. Required except in the CREATE requests."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the quota preference. Allowed parents are \"projects/[project-id / number]\" or \"folders/[folder-id / number]\" or \"organizations/[org-id / number]\"."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quota_id` after provisioning.\nThe id of the quota to which the quota preference is applied. A quota id is unique in the service.\nExample: 'CPUS-per-project-region'."]
    pub fn quota_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reconciling` after provisioning.\nIs the quota preference pending Google Cloud approval and fulfillment."]
    pub fn reconciling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reconciling", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of the service to which the quota preference is applied."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nUpdate time stamp.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: '2014-10-02T15:01:23Z' and '2014-10-02T15:01:23.045123456Z'."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_config` after provisioning.\n"]
    pub fn quota_config(&self) -> terrars::ListRef<CloudQuotasQuotaPreference_QuotaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudQuotasQuotaPreference_TimeoutsRef {
        CloudQuotasQuotaPreference_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudQuotasQuotaPreferenceDynamic {
    quota_config: std::option::Option<terrars::DynamicBlock<CloudQuotasQuotaPreference_QuotaConfig>>,
}

#[derive(Serialize)]
pub struct CloudQuotasQuotaPreference_QuotaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    preferred_value: terrars::PrimField<std::string::String>,
}

impl CloudQuotasQuotaPreference_QuotaConfig {
    #[doc =
        "Set the field `annotations`.\nThe annotations map for clients to store small amounts of arbitrary data. Do not put PII or other sensitive information here. See https://google.aip.dev/128#annotations.\n\nAn object containing a list of \"key: value\" pairs. Example: '{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn set_annotations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.annotations = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudQuotasQuotaPreference_QuotaConfig {
    type O = terrars::BlockAssignable<CloudQuotasQuotaPreference_QuotaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudQuotasQuotaPreference_QuotaConfig {
    #[doc =
        "The preferred value. Must be greater than or equal to -1. If set to -1, it means the value is \"unlimited\"."]
    pub preferred_value: terrars::PrimField<std::string::String>,
}

impl BuildCloudQuotasQuotaPreference_QuotaConfig {
    pub fn build(self) -> CloudQuotasQuotaPreference_QuotaConfig {
        CloudQuotasQuotaPreference_QuotaConfig {
            annotations: core::default::Default::default(),
            preferred_value: self.preferred_value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudQuotasQuotaPreference_QuotaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudQuotasQuotaPreference_QuotaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudQuotasQuotaPreference_QuotaConfigRef {
        CloudQuotasQuotaPreference_QuotaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudQuotasQuotaPreference_QuotaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nThe annotations map for clients to store small amounts of arbitrary data. Do not put PII or other sensitive information here. See https://google.aip.dev/128#annotations.\n\nAn object containing a list of \"key: value\" pairs. Example: '{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc = "Get a reference to the value of field `granted_value` after provisioning.\nGranted quota value."]
    pub fn granted_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.granted_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preferred_value` after provisioning.\nThe preferred value. Must be greater than or equal to -1. If set to -1, it means the value is \"unlimited\"."]
    pub fn preferred_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_origin` after provisioning.\nThe origin of the quota preference request."]
    pub fn request_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_origin", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state_detail` after provisioning.\nOptional details about the state of this quota preference."]
    pub fn state_detail(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_detail", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trace_id` after provisioning.\nThe trace id that the Google Cloud uses to provision the requested quota. This trace id may be used by the client to contact Cloud support to track the state of a quota preference request. The trace id is only produced for increase requests and is unique for each request. The quota decrease requests do not have a trace id."]
    pub fn trace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trace_id", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudQuotasQuotaPreference_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudQuotasQuotaPreference_Timeouts {
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

impl terrars::ToListMappable for CloudQuotasQuotaPreference_Timeouts {
    type O = terrars::BlockAssignable<CloudQuotasQuotaPreference_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudQuotasQuotaPreference_Timeouts {}

impl BuildCloudQuotasQuotaPreference_Timeouts {
    pub fn build(self) -> CloudQuotasQuotaPreference_Timeouts {
        CloudQuotasQuotaPreference_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudQuotasQuotaPreference_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudQuotasQuotaPreference_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudQuotasQuotaPreference_TimeoutsRef {
        CloudQuotasQuotaPreference_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudQuotasQuotaPreference_TimeoutsRef {
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
