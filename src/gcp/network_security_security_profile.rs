#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkSecuritySecurityProfileData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_intercept_profile: std::option::Option<std::vec::Vec<NetworkSecuritySecurityProfile_CustomInterceptProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_mirroring_profile: std::option::Option<std::vec::Vec<NetworkSecuritySecurityProfile_CustomMirroringProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_prevention_profile: std::option::Option<std::vec::Vec<NetworkSecuritySecurityProfile_ThreatPreventionProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkSecuritySecurityProfile_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_filtering_profile: std::option::Option<std::vec::Vec<NetworkSecuritySecurityProfile_UrlFilteringProfile>>,
    dynamic: NetworkSecuritySecurityProfileDynamic,
}

struct NetworkSecuritySecurityProfile_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkSecuritySecurityProfileData_>,
}

#[derive(Clone)]
pub struct NetworkSecuritySecurityProfile(std::rc::Rc<NetworkSecuritySecurityProfile_>);

impl NetworkSecuritySecurityProfile {
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
        "Set the field `description`.\nAn optional description of the security profile. The Max length is 512 characters."]
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
        "Set the field `labels`.\nA map of key/value label pairs to assign to the resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location of the security profile.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parent`.\nThe name of the parent this security profile belongs to.\nFormat: 'organizations/{organization_id}' or 'projects/{project_id}'."]
    pub fn set_parent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_intercept_profile`.\n"]
    pub fn set_custom_intercept_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecuritySecurityProfile_CustomInterceptProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_intercept_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_intercept_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_mirroring_profile`.\n"]
    pub fn set_custom_mirroring_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecuritySecurityProfile_CustomMirroringProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_mirroring_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_mirroring_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_prevention_profile`.\n"]
    pub fn set_threat_prevention_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecuritySecurityProfile_ThreatPreventionProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().threat_prevention_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.threat_prevention_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkSecuritySecurityProfile_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url_filtering_profile`.\n"]
    pub fn set_url_filtering_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkSecuritySecurityProfile_UrlFilteringProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().url_filtering_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.url_filtering_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the security profile was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the security profile. The Max length is 512 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields,\nand may be sent on update and delete requests to ensure the client has an up-to-date\nvalue before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA map of key/value label pairs to assign to the resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the security profile.\nThe default value is 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the security profile resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe name of the parent this security profile belongs to.\nFormat: 'organizations/{organization_id}' or 'projects/{project_id}'."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of security profile. 'WILDFIRE_ANALYSIS' is beta-only. Possible values: [\"THREAT_PREVENTION\", \"URL_FILTERING\", \"CUSTOM_MIRRORING\", \"CUSTOM_INTERCEPT\", \"WILDFIRE_ANALYSIS\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the security profile was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_intercept_profile` after provisioning.\n"]
    pub fn custom_intercept_profile(&self) -> terrars::ListRef<NetworkSecuritySecurityProfile_CustomInterceptProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_intercept_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_mirroring_profile` after provisioning.\n"]
    pub fn custom_mirroring_profile(&self) -> terrars::ListRef<NetworkSecuritySecurityProfile_CustomMirroringProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_mirroring_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threat_prevention_profile` after provisioning.\n"]
    pub fn threat_prevention_profile(
        &self,
    ) -> terrars::ListRef<NetworkSecuritySecurityProfile_ThreatPreventionProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threat_prevention_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecuritySecurityProfile_TimeoutsRef {
        NetworkSecuritySecurityProfile_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `url_filtering_profile` after provisioning.\n"]
    pub fn url_filtering_profile(&self) -> terrars::ListRef<NetworkSecuritySecurityProfile_UrlFilteringProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_filtering_profile", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkSecuritySecurityProfile {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkSecuritySecurityProfile { }

impl terrars::ToListMappable for NetworkSecuritySecurityProfile {
    type O = terrars::ListRef<NetworkSecuritySecurityProfileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkSecuritySecurityProfile_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_security_security_profile".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkSecuritySecurityProfile {
    pub tf_id: std::string::String,
    #[doc = "The name of the security profile resource."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of security profile. 'WILDFIRE_ANALYSIS' is beta-only. Possible values: [\"THREAT_PREVENTION\", \"URL_FILTERING\", \"CUSTOM_MIRRORING\", \"CUSTOM_INTERCEPT\", \"WILDFIRE_ANALYSIS\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecuritySecurityProfile {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkSecuritySecurityProfile {
        let out = NetworkSecuritySecurityProfile(std::rc::Rc::new(NetworkSecuritySecurityProfile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkSecuritySecurityProfileData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                parent: core::default::Default::default(),
                type_: self.type_,
                custom_intercept_profile: core::default::Default::default(),
                custom_mirroring_profile: core::default::Default::default(),
                threat_prevention_profile: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                url_filtering_profile: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkSecuritySecurityProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecuritySecurityProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkSecuritySecurityProfileRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the security profile was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the security profile. The Max length is 512 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThis checksum is computed by the server based on the value of other fields,\nand may be sent on update and delete requests to ensure the client has an up-to-date\nvalue before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA map of key/value label pairs to assign to the resource.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the security profile.\nThe default value is 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the security profile resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe name of the parent this security profile belongs to.\nFormat: 'organizations/{organization_id}' or 'projects/{project_id}'."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of security profile. 'WILDFIRE_ANALYSIS' is beta-only. Possible values: [\"THREAT_PREVENTION\", \"URL_FILTERING\", \"CUSTOM_MIRRORING\", \"CUSTOM_INTERCEPT\", \"WILDFIRE_ANALYSIS\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the security profile was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_intercept_profile` after provisioning.\n"]
    pub fn custom_intercept_profile(&self) -> terrars::ListRef<NetworkSecuritySecurityProfile_CustomInterceptProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_intercept_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_mirroring_profile` after provisioning.\n"]
    pub fn custom_mirroring_profile(&self) -> terrars::ListRef<NetworkSecuritySecurityProfile_CustomMirroringProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_mirroring_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threat_prevention_profile` after provisioning.\n"]
    pub fn threat_prevention_profile(
        &self,
    ) -> terrars::ListRef<NetworkSecuritySecurityProfile_ThreatPreventionProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threat_prevention_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkSecuritySecurityProfile_TimeoutsRef {
        NetworkSecuritySecurityProfile_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `url_filtering_profile` after provisioning.\n"]
    pub fn url_filtering_profile(&self) -> terrars::ListRef<NetworkSecuritySecurityProfile_UrlFilteringProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_filtering_profile", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecuritySecurityProfileDynamic {
    custom_intercept_profile: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecuritySecurityProfile_CustomInterceptProfile>>,
    custom_mirroring_profile: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecuritySecurityProfile_CustomMirroringProfile>>,
    threat_prevention_profile: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecuritySecurityProfile_ThreatPreventionProfile>>,
    url_filtering_profile: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecuritySecurityProfile_UrlFilteringProfile>>,
}

#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfile_CustomInterceptProfile {
    intercept_endpoint_group: terrars::PrimField<std::string::String>,
}

impl NetworkSecuritySecurityProfile_CustomInterceptProfile { }

impl terrars::ToListMappable for NetworkSecuritySecurityProfile_CustomInterceptProfile {
    type O = terrars::BlockAssignable<NetworkSecuritySecurityProfile_CustomInterceptProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecuritySecurityProfile_CustomInterceptProfile {
    #[doc =
        "The Intercept Endpoint Group to which matching traffic should be intercepted.\nFormat: projects/{project_id}/locations/global/interceptEndpointGroups/{endpoint_group_id}"]
    pub intercept_endpoint_group: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecuritySecurityProfile_CustomInterceptProfile {
    pub fn build(self) -> NetworkSecuritySecurityProfile_CustomInterceptProfile {
        NetworkSecuritySecurityProfile_CustomInterceptProfile {
            intercept_endpoint_group: self.intercept_endpoint_group,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecuritySecurityProfile_CustomInterceptProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecuritySecurityProfile_CustomInterceptProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecuritySecurityProfile_CustomInterceptProfileRef {
        NetworkSecuritySecurityProfile_CustomInterceptProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecuritySecurityProfile_CustomInterceptProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `intercept_endpoint_group` after provisioning.\nThe Intercept Endpoint Group to which matching traffic should be intercepted.\nFormat: projects/{project_id}/locations/global/interceptEndpointGroups/{endpoint_group_id}"]
    pub fn intercept_endpoint_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.intercept_endpoint_group", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfile_CustomMirroringProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mirroring_deployment_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    mirroring_endpoint_group: terrars::PrimField<std::string::String>,
}

impl NetworkSecuritySecurityProfile_CustomMirroringProfile {
    #[doc =
        "Set the field `mirroring_deployment_groups`.\nThe target downstream Mirroring Deployment Groups.\nThis field is used for Packet Broker mirroring endpoint groups to specify\nthe deployment groups that the packet should be mirrored to by the broker.\nFormat: projects/{project_id}/locations/global/mirroringDeploymentGroups/{deployment_group_id}"]
    pub fn set_mirroring_deployment_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.mirroring_deployment_groups = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecuritySecurityProfile_CustomMirroringProfile {
    type O = terrars::BlockAssignable<NetworkSecuritySecurityProfile_CustomMirroringProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecuritySecurityProfile_CustomMirroringProfile {
    #[doc =
        "The target Mirroring Endpoint Group.\nWhen a mirroring rule with this security profile attached matches a packet,\na replica will be mirrored to the location-local target in this group.\nFormat: projects/{project_id}/locations/global/mirroringEndpointGroups/{endpoint_group_id}"]
    pub mirroring_endpoint_group: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecuritySecurityProfile_CustomMirroringProfile {
    pub fn build(self) -> NetworkSecuritySecurityProfile_CustomMirroringProfile {
        NetworkSecuritySecurityProfile_CustomMirroringProfile {
            mirroring_deployment_groups: core::default::Default::default(),
            mirroring_endpoint_group: self.mirroring_endpoint_group,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecuritySecurityProfile_CustomMirroringProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecuritySecurityProfile_CustomMirroringProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecuritySecurityProfile_CustomMirroringProfileRef {
        NetworkSecuritySecurityProfile_CustomMirroringProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecuritySecurityProfile_CustomMirroringProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mirroring_deployment_groups` after provisioning.\nThe target downstream Mirroring Deployment Groups.\nThis field is used for Packet Broker mirroring endpoint groups to specify\nthe deployment groups that the packet should be mirrored to by the broker.\nFormat: projects/{project_id}/locations/global/mirroringDeploymentGroups/{deployment_group_id}"]
    pub fn mirroring_deployment_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mirroring_deployment_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mirroring_endpoint_group` after provisioning.\nThe target Mirroring Endpoint Group.\nWhen a mirroring rule with this security profile attached matches a packet,\na replica will be mirrored to the location-local target in this group.\nFormat: projects/{project_id}/locations/global/mirroringEndpointGroups/{endpoint_group_id}"]
    pub fn mirroring_endpoint_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirroring_endpoint_group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mirroring_endpoint_group_type` after provisioning.\nThe type of the mirroring endpoint group this profile is attached to.\nPossible values:\nDIRECT\nBROKER"]
    pub fn mirroring_endpoint_group_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mirroring_endpoint_group_type", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides {
    action: terrars::PrimField<std::string::String>,
    protocol: terrars::PrimField<std::string::String>,
}

impl NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides { }

impl terrars::ToListMappable for NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides {
    type O = terrars::BlockAssignable<NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides {
    #[doc =
        "Threat action override. For some threat types, only a subset of actions applies. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub action: terrars::PrimField<std::string::String>,
    #[doc =
        "Required protocol to match. Possible values: [\"SMTP\", \"SMB\", \"POP3\", \"IMAP\", \"HTTP2\", \"HTTP\", \"FTP\"]"]
    pub protocol: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides {
    pub fn build(self) -> NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides {
        NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides {
            action: self.action,
            protocol: self.protocol,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverridesRef {
        NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThreat action override. For some threat types, only a subset of actions applies. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nRequired protocol to match. Possible values: [\"SMTP\", \"SMB\", \"POP3\", \"IMAP\", \"HTTP2\", \"HTTP\", \"FTP\"]"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides {
    action: terrars::PrimField<std::string::String>,
    severity: terrars::PrimField<std::string::String>,
}

impl NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides { }

impl terrars::ToListMappable for NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides {
    type O = terrars::BlockAssignable<NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides {
    #[doc = "Threat action override. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub action: terrars::PrimField<std::string::String>,
    #[doc =
        "Severity level to match. Possible values: [\"CRITICAL\", \"HIGH\", \"INFORMATIONAL\", \"LOW\", \"MEDIUM\"]"]
    pub severity: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides {
    pub fn build(self) -> NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides {
        NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides {
            action: self.action,
            severity: self.severity,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverridesRef {
        NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThreat action override. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `severity` after provisioning.\nSeverity level to match. Possible values: [\"CRITICAL\", \"HIGH\", \"INFORMATIONAL\", \"LOW\", \"MEDIUM\"]"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides {
    action: terrars::PrimField<std::string::String>,
    threat_id: terrars::PrimField<std::string::String>,
}

impl NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides { }

impl terrars::ToListMappable for NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides {
    type O = terrars::BlockAssignable<NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides {
    #[doc = "Threat action. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = "Vendor-specific ID of a threat to override."]
    pub threat_id: terrars::PrimField<std::string::String>,
}

impl BuildNetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides {
    pub fn build(self) -> NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides {
        NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides {
            action: self.action,
            threat_id: self.threat_id,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverridesRef {
        NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThreat action. Possible values: [\"ALERT\", \"ALLOW\", \"DEFAULT_ACTION\", \"DENY\"]"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `threat_id` after provisioning.\nVendor-specific ID of a threat to override."]
    pub fn threat_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threat_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of threat."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecuritySecurityProfile_ThreatPreventionProfileDynamic {
    antivirus_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides>>,
    severity_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides>>,
    threat_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides>>,
}

#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfile_ThreatPreventionProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    antivirus_overrides: std
    ::option
    ::Option<std::vec::Vec<NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity_overrides: std
    ::option
    ::Option<std::vec::Vec<NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_overrides: std
    ::option
    ::Option<std::vec::Vec<NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides>>,
    dynamic: NetworkSecuritySecurityProfile_ThreatPreventionProfileDynamic,
}

impl NetworkSecuritySecurityProfile_ThreatPreventionProfile {
    #[doc = "Set the field `antivirus_overrides`.\n"]
    pub fn set_antivirus_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkSecuritySecurityProfile_ThreatPreventionProfile_AntivirusOverrides>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.antivirus_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.antivirus_overrides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `severity_overrides`.\n"]
    pub fn set_severity_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkSecuritySecurityProfile_ThreatPreventionProfile_SeverityOverrides>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.severity_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.severity_overrides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_overrides`.\n"]
    pub fn set_threat_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<NetworkSecuritySecurityProfile_ThreatPreventionProfile_ThreatOverrides>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecuritySecurityProfile_ThreatPreventionProfile {
    type O = terrars::BlockAssignable<NetworkSecuritySecurityProfile_ThreatPreventionProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecuritySecurityProfile_ThreatPreventionProfile {}

impl BuildNetworkSecuritySecurityProfile_ThreatPreventionProfile {
    pub fn build(self) -> NetworkSecuritySecurityProfile_ThreatPreventionProfile {
        NetworkSecuritySecurityProfile_ThreatPreventionProfile {
            antivirus_overrides: core::default::Default::default(),
            severity_overrides: core::default::Default::default(),
            threat_overrides: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecuritySecurityProfile_ThreatPreventionProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecuritySecurityProfile_ThreatPreventionProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecuritySecurityProfile_ThreatPreventionProfileRef {
        NetworkSecuritySecurityProfile_ThreatPreventionProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecuritySecurityProfile_ThreatPreventionProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfile_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkSecuritySecurityProfile_Timeouts {
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

impl terrars::ToListMappable for NetworkSecuritySecurityProfile_Timeouts {
    type O = terrars::BlockAssignable<NetworkSecuritySecurityProfile_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecuritySecurityProfile_Timeouts {}

impl BuildNetworkSecuritySecurityProfile_Timeouts {
    pub fn build(self) -> NetworkSecuritySecurityProfile_Timeouts {
        NetworkSecuritySecurityProfile_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecuritySecurityProfile_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecuritySecurityProfile_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkSecuritySecurityProfile_TimeoutsRef {
        NetworkSecuritySecurityProfile_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecuritySecurityProfile_TimeoutsRef {
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
pub struct NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters {
    filtering_action: terrars::PrimField<std::string::String>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    urls: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters {
    #[doc =
        "Set the field `urls`.\nA list of domain matcher strings that a domain name gets compared with to determine if the filter is applicable.\nA domain name must match with at least one of the strings in the list for a filter to be applicable."]
    pub fn set_urls(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.urls = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters {
    type O = terrars::BlockAssignable<NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters {
    #[doc = "The action to take when the filter is applied. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub filtering_action: terrars::PrimField<std::string::String>,
    #[doc =
        "The priority of the filter within the URL filtering profile.\nMust be an integer from 0 and 2147483647, inclusive. Lower integers indicate higher priorities.\nThe priority of a filter must be unique within a URL filtering profile."]
    pub priority: terrars::PrimField<f64>,
}

impl BuildNetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters {
    pub fn build(self) -> NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters {
        NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters {
            filtering_action: self.filtering_action,
            priority: self.priority,
            urls: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFiltersRef {
        NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filtering_action` after provisioning.\nThe action to take when the filter is applied. Possible values: [\"ALLOW\", \"DENY\"]"]
    pub fn filtering_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filtering_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nThe priority of the filter within the URL filtering profile.\nMust be an integer from 0 and 2147483647, inclusive. Lower integers indicate higher priorities.\nThe priority of a filter must be unique within a URL filtering profile."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `urls` after provisioning.\nA list of domain matcher strings that a domain name gets compared with to determine if the filter is applicable.\nA domain name must match with at least one of the strings in the list for a filter to be applicable."]
    pub fn urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.urls", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkSecuritySecurityProfile_UrlFilteringProfileDynamic {
    url_filters: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters>>,
}

#[derive(Serialize)]
pub struct NetworkSecuritySecurityProfile_UrlFilteringProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_filters: std::option::Option<std::vec::Vec<NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters>>,
    dynamic: NetworkSecuritySecurityProfile_UrlFilteringProfileDynamic,
}

impl NetworkSecuritySecurityProfile_UrlFilteringProfile {
    #[doc = "Set the field `url_filters`.\n"]
    pub fn set_url_filters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkSecuritySecurityProfile_UrlFilteringProfile_UrlFilters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_filters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkSecuritySecurityProfile_UrlFilteringProfile {
    type O = terrars::BlockAssignable<NetworkSecuritySecurityProfile_UrlFilteringProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkSecuritySecurityProfile_UrlFilteringProfile {}

impl BuildNetworkSecuritySecurityProfile_UrlFilteringProfile {
    pub fn build(self) -> NetworkSecuritySecurityProfile_UrlFilteringProfile {
        NetworkSecuritySecurityProfile_UrlFilteringProfile {
            url_filters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkSecuritySecurityProfile_UrlFilteringProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkSecuritySecurityProfile_UrlFilteringProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkSecuritySecurityProfile_UrlFilteringProfileRef {
        NetworkSecuritySecurityProfile_UrlFilteringProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkSecuritySecurityProfile_UrlFilteringProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}
