#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct LookerInstanceData_ {
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
    consumer_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    controlled_egress_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fips_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gemini_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_edition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ip_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reserved_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_settings: std::option::Option<std::vec::Vec<LookerInstance_AdminSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    controlled_egress_config: std::option::Option<std::vec::Vec<LookerInstance_ControlledEgressConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_domain: std::option::Option<std::vec::Vec<LookerInstance_CustomDomain>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_maintenance_period: std::option::Option<std::vec::Vec<LookerInstance_DenyMaintenancePeriod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<LookerInstance_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<LookerInstance_MaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std::option::Option<std::vec::Vec<LookerInstance_OauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    periodic_export_config: std::option::Option<std::vec::Vec<LookerInstance_PeriodicExportConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_config: std::option::Option<std::vec::Vec<LookerInstance_PscConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LookerInstance_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_metadata: std::option::Option<std::vec::Vec<LookerInstance_UserMetadata>>,
    dynamic: LookerInstanceDynamic,
}

struct LookerInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LookerInstanceData_>,
}

#[derive(Clone)]
pub struct LookerInstance(std::rc::Rc<LookerInstance_>);

impl LookerInstance {
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
        "Set the field `consumer_network`.\nNetwork name in the consumer project in the format of: projects/{project}/global/networks/{network}\nNote that the consumer network may be in a different GCP project than the consumer\nproject that is hosting the Looker Instance."]
    pub fn set_consumer_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().consumer_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `controlled_egress_enabled`.\nWhether controlled egress is enabled on the Looker instance."]
    pub fn set_controlled_egress_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().controlled_egress_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/looker_instance.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fips_enabled`.\nFIPS 140-2 Encryption enablement for Looker (Google Cloud Core)."]
    pub fn set_fips_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().fips_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gemini_enabled`.\nGemini enablement for Looker (Google Cloud Core)."]
    pub fn set_gemini_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().gemini_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `platform_edition`.\nPlatform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of these values:\n- LOOKER_CORE_TRIAL: trial instance (Currently Unavailable)\n- LOOKER_CORE_STANDARD: pay as you go standard instance (Currently Unavailable)\n- LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance\n- LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance\n- LOOKER_CORE_EMBED_ANNUAL: subscription embed instance\n- LOOKER_CORE_NONPROD_STANDARD_ANNUAL: nonprod subscription standard instance\n- LOOKER_CORE_NONPROD_ENTERPRISE_ANNUAL: nonprod subscription enterprise instance\n- LOOKER_CORE_NONPROD_EMBED_ANNUAL: nonprod subscription embed instance\n- LOOKER_CORE_TRIAL_STANDARD: A standard trial edition of Looker (Google Cloud core) product.\n- LOOKER_CORE_TRIAL_ENTERPRISE: An enterprise trial edition of Looker (Google Cloud core) product.\n- LOOKER_CORE_TRIAL_EMBED: An embed trial edition of Looker (Google Cloud core) product. Default value: \"LOOKER_CORE_TRIAL\" Possible values: [\"LOOKER_CORE_TRIAL\", \"LOOKER_CORE_STANDARD\", \"LOOKER_CORE_STANDARD_ANNUAL\", \"LOOKER_CORE_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_EMBED_ANNUAL\", \"LOOKER_CORE_NONPROD_STANDARD_ANNUAL\", \"LOOKER_CORE_NONPROD_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_NONPROD_EMBED_ANNUAL\", \"LOOKER_CORE_TRIAL_STANDARD\", \"LOOKER_CORE_TRIAL_ENTERPRISE\", \"LOOKER_CORE_TRIAL_EMBED\"]"]
    pub fn set_platform_edition(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().platform_edition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_enabled`.\nWhether private IP is enabled on the Looker instance."]
    pub fn set_private_ip_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_ip_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `psc_enabled`.\nWhether Public Service Connect (PSC) is enabled on the Looker instance"]
    pub fn set_psc_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().psc_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_ip_enabled`.\nWhether public IP is enabled on the Looker instance."]
    pub fn set_public_ip_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public_ip_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe name of the Looker region of the instance."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reserved_range`.\nName of a reserved IP address range within the consumer network, to be used for\nprivate service access connection. User may or may not specify this in a request."]
    pub fn set_reserved_range(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().reserved_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `admin_settings`.\n"]
    pub fn set_admin_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_AdminSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().admin_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.admin_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `controlled_egress_config`.\n"]
    pub fn set_controlled_egress_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_ControlledEgressConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().controlled_egress_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.controlled_egress_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_domain`.\n"]
    pub fn set_custom_domain(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_CustomDomain>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_domain = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_domain = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deny_maintenance_period`.\n"]
    pub fn set_deny_maintenance_period(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_DenyMaintenancePeriod>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deny_maintenance_period = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deny_maintenance_period = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_EncryptionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_MaintenanceWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_OauthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().oauth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.oauth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `periodic_export_config`.\n"]
    pub fn set_periodic_export_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_PeriodicExportConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().periodic_export_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.periodic_export_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(self, v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_PscConfig>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<LookerInstance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_metadata`.\n"]
    pub fn set_user_metadata(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_UserMetadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `consumer_network` after provisioning.\nNetwork name in the consumer project in the format of: projects/{project}/global/networks/{network}\nNote that the consumer network may be in a different GCP project than the consumer\nproject that is hosting the Looker Instance."]
    pub fn consumer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `controlled_egress_enabled` after provisioning.\nWhether controlled egress is enabled on the Looker instance."]
    pub fn controlled_egress_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.controlled_egress_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/looker_instance.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_public_ip` after provisioning.\nPublic Egress IP (IPv4)."]
    pub fn egress_public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress_public_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fips_enabled` after provisioning.\nFIPS 140-2 Encryption enablement for Looker (Google Cloud Core)."]
    pub fn fips_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fips_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gemini_enabled` after provisioning.\nGemini enablement for Looker (Google Cloud Core)."]
    pub fn gemini_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gemini_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingress_private_ip` after provisioning.\nPrivate Ingress IP (IPv4)."]
    pub fn ingress_private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_public_ip` after provisioning.\nPublic Ingress IP (IPv4)."]
    pub fn ingress_public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_public_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `looker_uri` after provisioning.\nLooker instance URI which can be used to access the Looker Instance UI."]
    pub fn looker_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.looker_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `looker_version` after provisioning.\nThe Looker version that the instance is using."]
    pub fn looker_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.looker_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `platform_edition` after provisioning.\nPlatform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of these values:\n- LOOKER_CORE_TRIAL: trial instance (Currently Unavailable)\n- LOOKER_CORE_STANDARD: pay as you go standard instance (Currently Unavailable)\n- LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance\n- LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance\n- LOOKER_CORE_EMBED_ANNUAL: subscription embed instance\n- LOOKER_CORE_NONPROD_STANDARD_ANNUAL: nonprod subscription standard instance\n- LOOKER_CORE_NONPROD_ENTERPRISE_ANNUAL: nonprod subscription enterprise instance\n- LOOKER_CORE_NONPROD_EMBED_ANNUAL: nonprod subscription embed instance\n- LOOKER_CORE_TRIAL_STANDARD: A standard trial edition of Looker (Google Cloud core) product.\n- LOOKER_CORE_TRIAL_ENTERPRISE: An enterprise trial edition of Looker (Google Cloud core) product.\n- LOOKER_CORE_TRIAL_EMBED: An embed trial edition of Looker (Google Cloud core) product. Default value: \"LOOKER_CORE_TRIAL\" Possible values: [\"LOOKER_CORE_TRIAL\", \"LOOKER_CORE_STANDARD\", \"LOOKER_CORE_STANDARD_ANNUAL\", \"LOOKER_CORE_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_EMBED_ANNUAL\", \"LOOKER_CORE_NONPROD_STANDARD_ANNUAL\", \"LOOKER_CORE_NONPROD_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_NONPROD_EMBED_ANNUAL\", \"LOOKER_CORE_TRIAL_STANDARD\", \"LOOKER_CORE_TRIAL_ENTERPRISE\", \"LOOKER_CORE_TRIAL_EMBED\"]"]
    pub fn platform_edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_edition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ip_enabled` after provisioning.\nWhether private IP is enabled on the Looker instance."]
    pub fn private_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_enabled` after provisioning.\nWhether Public Service Connect (PSC) is enabled on the Looker instance"]
    pub fn psc_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_enabled` after provisioning.\nWhether public IP is enabled on the Looker instance."]
    pub fn public_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the Looker region of the instance."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_range` after provisioning.\nName of a reserved IP address range within the consumer network, to be used for\nprivate service access connection. User may or may not specify this in a request."]
    pub fn reserved_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the instance was updated in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_settings` after provisioning.\n"]
    pub fn admin_settings(&self) -> terrars::ListRef<LookerInstance_AdminSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `controlled_egress_config` after provisioning.\n"]
    pub fn controlled_egress_config(&self) -> terrars::ListRef<LookerInstance_ControlledEgressConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.controlled_egress_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domain` after provisioning.\n"]
    pub fn custom_domain(&self) -> terrars::ListRef<LookerInstance_CustomDomainRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(&self) -> terrars::ListRef<LookerInstance_DenyMaintenancePeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny_maintenance_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<LookerInstance_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<LookerInstance_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<LookerInstance_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `periodic_export_config` after provisioning.\n"]
    pub fn periodic_export_config(&self) -> terrars::ListRef<LookerInstance_PeriodicExportConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.periodic_export_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> terrars::ListRef<LookerInstance_PscConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LookerInstance_TimeoutsRef {
        LookerInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_metadata` after provisioning.\n"]
    pub fn user_metadata(&self) -> terrars::ListRef<LookerInstance_UserMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_metadata", self.extract_ref()))
    }
}

impl terrars::Referable for LookerInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LookerInstance { }

impl terrars::ToListMappable for LookerInstance {
    type O = terrars::ListRef<LookerInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LookerInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_looker_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLookerInstance {
    pub tf_id: std::string::String,
    #[doc = "The ID of the instance or a fully qualified identifier for the instance."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLookerInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> LookerInstance {
        let out = LookerInstance(std::rc::Rc::new(LookerInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LookerInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                consumer_network: core::default::Default::default(),
                controlled_egress_enabled: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                fips_enabled: core::default::Default::default(),
                gemini_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                platform_edition: core::default::Default::default(),
                private_ip_enabled: core::default::Default::default(),
                project: core::default::Default::default(),
                psc_enabled: core::default::Default::default(),
                public_ip_enabled: core::default::Default::default(),
                region: core::default::Default::default(),
                reserved_range: core::default::Default::default(),
                admin_settings: core::default::Default::default(),
                controlled_egress_config: core::default::Default::default(),
                custom_domain: core::default::Default::default(),
                deny_maintenance_period: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                oauth_config: core::default::Default::default(),
                periodic_export_config: core::default::Default::default(),
                psc_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_metadata: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LookerInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LookerInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `consumer_network` after provisioning.\nNetwork name in the consumer project in the format of: projects/{project}/global/networks/{network}\nNote that the consumer network may be in a different GCP project than the consumer\nproject that is hosting the Looker Instance."]
    pub fn consumer_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `controlled_egress_enabled` after provisioning.\nWhether controlled egress is enabled on the Looker instance."]
    pub fn controlled_egress_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.controlled_egress_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/looker_instance.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_public_ip` after provisioning.\nPublic Egress IP (IPv4)."]
    pub fn egress_public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress_public_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fips_enabled` after provisioning.\nFIPS 140-2 Encryption enablement for Looker (Google Cloud Core)."]
    pub fn fips_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fips_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gemini_enabled` after provisioning.\nGemini enablement for Looker (Google Cloud Core)."]
    pub fn gemini_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gemini_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ingress_private_ip` after provisioning.\nPrivate Ingress IP (IPv4)."]
    pub fn ingress_private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_public_ip` after provisioning.\nPublic Ingress IP (IPv4)."]
    pub fn ingress_public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_public_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `looker_uri` after provisioning.\nLooker instance URI which can be used to access the Looker Instance UI."]
    pub fn looker_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.looker_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `looker_version` after provisioning.\nThe Looker version that the instance is using."]
    pub fn looker_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.looker_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `platform_edition` after provisioning.\nPlatform editions for a Looker instance. Each edition maps to a set of instance features, like its size. Must be one of these values:\n- LOOKER_CORE_TRIAL: trial instance (Currently Unavailable)\n- LOOKER_CORE_STANDARD: pay as you go standard instance (Currently Unavailable)\n- LOOKER_CORE_STANDARD_ANNUAL: subscription standard instance\n- LOOKER_CORE_ENTERPRISE_ANNUAL: subscription enterprise instance\n- LOOKER_CORE_EMBED_ANNUAL: subscription embed instance\n- LOOKER_CORE_NONPROD_STANDARD_ANNUAL: nonprod subscription standard instance\n- LOOKER_CORE_NONPROD_ENTERPRISE_ANNUAL: nonprod subscription enterprise instance\n- LOOKER_CORE_NONPROD_EMBED_ANNUAL: nonprod subscription embed instance\n- LOOKER_CORE_TRIAL_STANDARD: A standard trial edition of Looker (Google Cloud core) product.\n- LOOKER_CORE_TRIAL_ENTERPRISE: An enterprise trial edition of Looker (Google Cloud core) product.\n- LOOKER_CORE_TRIAL_EMBED: An embed trial edition of Looker (Google Cloud core) product. Default value: \"LOOKER_CORE_TRIAL\" Possible values: [\"LOOKER_CORE_TRIAL\", \"LOOKER_CORE_STANDARD\", \"LOOKER_CORE_STANDARD_ANNUAL\", \"LOOKER_CORE_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_EMBED_ANNUAL\", \"LOOKER_CORE_NONPROD_STANDARD_ANNUAL\", \"LOOKER_CORE_NONPROD_ENTERPRISE_ANNUAL\", \"LOOKER_CORE_NONPROD_EMBED_ANNUAL\", \"LOOKER_CORE_TRIAL_STANDARD\", \"LOOKER_CORE_TRIAL_ENTERPRISE\", \"LOOKER_CORE_TRIAL_EMBED\"]"]
    pub fn platform_edition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_edition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_ip_enabled` after provisioning.\nWhether private IP is enabled on the Looker instance."]
    pub fn private_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `psc_enabled` after provisioning.\nWhether Public Service Connect (PSC) is enabled on the Looker instance"]
    pub fn psc_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.psc_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_ip_enabled` after provisioning.\nWhether public IP is enabled on the Looker instance."]
    pub fn public_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe name of the Looker region of the instance."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reserved_range` after provisioning.\nName of a reserved IP address range within the consumer network, to be used for\nprivate service access connection. User may or may not specify this in a request."]
    pub fn reserved_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reserved_range", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the instance was updated in RFC3339 UTC \"Zulu\" format,\naccurate to nanoseconds."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_settings` after provisioning.\n"]
    pub fn admin_settings(&self) -> terrars::ListRef<LookerInstance_AdminSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.admin_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `controlled_egress_config` after provisioning.\n"]
    pub fn controlled_egress_config(&self) -> terrars::ListRef<LookerInstance_ControlledEgressConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.controlled_egress_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domain` after provisioning.\n"]
    pub fn custom_domain(&self) -> terrars::ListRef<LookerInstance_CustomDomainRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deny_maintenance_period` after provisioning.\n"]
    pub fn deny_maintenance_period(&self) -> terrars::ListRef<LookerInstance_DenyMaintenancePeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny_maintenance_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<LookerInstance_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<LookerInstance_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> terrars::ListRef<LookerInstance_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `periodic_export_config` after provisioning.\n"]
    pub fn periodic_export_config(&self) -> terrars::ListRef<LookerInstance_PeriodicExportConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.periodic_export_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> terrars::ListRef<LookerInstance_PscConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LookerInstance_TimeoutsRef {
        LookerInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_metadata` after provisioning.\n"]
    pub fn user_metadata(&self) -> terrars::ListRef<LookerInstance_UserMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_metadata", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LookerInstanceDynamic {
    admin_settings: std::option::Option<terrars::DynamicBlock<LookerInstance_AdminSettings>>,
    controlled_egress_config: std::option::Option<terrars::DynamicBlock<LookerInstance_ControlledEgressConfig>>,
    custom_domain: std::option::Option<terrars::DynamicBlock<LookerInstance_CustomDomain>>,
    deny_maintenance_period: std::option::Option<terrars::DynamicBlock<LookerInstance_DenyMaintenancePeriod>>,
    encryption_config: std::option::Option<terrars::DynamicBlock<LookerInstance_EncryptionConfig>>,
    maintenance_window: std::option::Option<terrars::DynamicBlock<LookerInstance_MaintenanceWindow>>,
    oauth_config: std::option::Option<terrars::DynamicBlock<LookerInstance_OauthConfig>>,
    periodic_export_config: std::option::Option<terrars::DynamicBlock<LookerInstance_PeriodicExportConfig>>,
    psc_config: std::option::Option<terrars::DynamicBlock<LookerInstance_PscConfig>>,
    user_metadata: std::option::Option<terrars::DynamicBlock<LookerInstance_UserMetadata>>,
}

#[derive(Serialize)]
pub struct LookerInstance_AdminSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_email_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl LookerInstance_AdminSettings {
    #[doc =
        "Set the field `allowed_email_domains`.\nEmail domain allowlist for the instance.\n\nDefine the email domains to which your users can deliver Looker (Google Cloud core) content.\nUpdating this list will restart the instance. Updating the allowed email domains from terraform\nmeans the value provided will be considered as the entire list and not an amendment to the\nexisting list of allowed email domains."]
    pub fn set_allowed_email_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_email_domains = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstance_AdminSettings {
    type O = terrars::BlockAssignable<LookerInstance_AdminSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_AdminSettings {}

impl BuildLookerInstance_AdminSettings {
    pub fn build(self) -> LookerInstance_AdminSettings {
        LookerInstance_AdminSettings { allowed_email_domains: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_AdminSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_AdminSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_AdminSettingsRef {
        LookerInstance_AdminSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_AdminSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_email_domains` after provisioning.\nEmail domain allowlist for the instance.\n\nDefine the email domains to which your users can deliver Looker (Google Cloud core) content.\nUpdating this list will restart the instance. Updating the allowed email domains from terraform\nmeans the value provided will be considered as the entire list and not an amendment to the\nexisting list of allowed email domains."]
    pub fn allowed_email_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_email_domains", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstance_ControlledEgressConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_fqdns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    marketplace_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl LookerInstance_ControlledEgressConfig {
    #[doc =
        "Set the field `egress_fqdns`.\nList of fully qualified domain names to be added to the allowlist for\noutbound traffic."]
    pub fn set_egress_fqdns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.egress_fqdns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `marketplace_enabled`.\nWhether the Looker Marketplace is enabled."]
    pub fn set_marketplace_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.marketplace_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstance_ControlledEgressConfig {
    type O = terrars::BlockAssignable<LookerInstance_ControlledEgressConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_ControlledEgressConfig {}

impl BuildLookerInstance_ControlledEgressConfig {
    pub fn build(self) -> LookerInstance_ControlledEgressConfig {
        LookerInstance_ControlledEgressConfig {
            egress_fqdns: core::default::Default::default(),
            marketplace_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_ControlledEgressConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_ControlledEgressConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_ControlledEgressConfigRef {
        LookerInstance_ControlledEgressConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_ControlledEgressConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `egress_fqdns` after provisioning.\nList of fully qualified domain names to be added to the allowlist for\noutbound traffic."]
    pub fn egress_fqdns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_fqdns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `marketplace_enabled` after provisioning.\nWhether the Looker Marketplace is enabled."]
    pub fn marketplace_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.marketplace_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstance_CustomDomain {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LookerInstance_CustomDomain {
    #[doc = "Set the field `domain`.\nDomain name"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstance_CustomDomain {
    type O = terrars::BlockAssignable<LookerInstance_CustomDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_CustomDomain {}

impl BuildLookerInstance_CustomDomain {
    pub fn build(self) -> LookerInstance_CustomDomain {
        LookerInstance_CustomDomain { domain: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_CustomDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_CustomDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_CustomDomainRef {
        LookerInstance_CustomDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_CustomDomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain name"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nStatus of the custom domain."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstance_DenyMaintenancePeriod_EndDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl LookerInstance_DenyMaintenancePeriod_EndDate {
    #[doc =
        "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstance_DenyMaintenancePeriod_EndDate {
    type O = terrars::BlockAssignable<LookerInstance_DenyMaintenancePeriod_EndDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_DenyMaintenancePeriod_EndDate {}

impl BuildLookerInstance_DenyMaintenancePeriod_EndDate {
    pub fn build(self) -> LookerInstance_DenyMaintenancePeriod_EndDate {
        LookerInstance_DenyMaintenancePeriod_EndDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_DenyMaintenancePeriod_EndDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_DenyMaintenancePeriod_EndDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LookerInstance_DenyMaintenancePeriod_EndDateRef {
        LookerInstance_DenyMaintenancePeriod_EndDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_DenyMaintenancePeriod_EndDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstance_DenyMaintenancePeriod_StartDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl LookerInstance_DenyMaintenancePeriod_StartDate {
    #[doc =
        "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstance_DenyMaintenancePeriod_StartDate {
    type O = terrars::BlockAssignable<LookerInstance_DenyMaintenancePeriod_StartDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_DenyMaintenancePeriod_StartDate {}

impl BuildLookerInstance_DenyMaintenancePeriod_StartDate {
    pub fn build(self) -> LookerInstance_DenyMaintenancePeriod_StartDate {
        LookerInstance_DenyMaintenancePeriod_StartDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_DenyMaintenancePeriod_StartDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_DenyMaintenancePeriod_StartDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LookerInstance_DenyMaintenancePeriod_StartDateRef {
        LookerInstance_DenyMaintenancePeriod_StartDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_DenyMaintenancePeriod_StartDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't significant."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstance_DenyMaintenancePeriod_Time {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl LookerInstance_DenyMaintenancePeriod_Time {
    #[doc = "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstance_DenyMaintenancePeriod_Time {
    type O = terrars::BlockAssignable<LookerInstance_DenyMaintenancePeriod_Time>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_DenyMaintenancePeriod_Time {}

impl BuildLookerInstance_DenyMaintenancePeriod_Time {
    pub fn build(self) -> LookerInstance_DenyMaintenancePeriod_Time {
        LookerInstance_DenyMaintenancePeriod_Time {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_DenyMaintenancePeriod_TimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_DenyMaintenancePeriod_TimeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_DenyMaintenancePeriod_TimeRef {
        LookerInstance_DenyMaintenancePeriod_TimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_DenyMaintenancePeriod_TimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct LookerInstance_DenyMaintenancePeriodDynamic {
    end_date: std::option::Option<terrars::DynamicBlock<LookerInstance_DenyMaintenancePeriod_EndDate>>,
    start_date: std::option::Option<terrars::DynamicBlock<LookerInstance_DenyMaintenancePeriod_StartDate>>,
    time: std::option::Option<terrars::DynamicBlock<LookerInstance_DenyMaintenancePeriod_Time>>,
}

#[derive(Serialize)]
pub struct LookerInstance_DenyMaintenancePeriod {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std::option::Option<std::vec::Vec<LookerInstance_DenyMaintenancePeriod_EndDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<std::vec::Vec<LookerInstance_DenyMaintenancePeriod_StartDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time: std::option::Option<std::vec::Vec<LookerInstance_DenyMaintenancePeriod_Time>>,
    dynamic: LookerInstance_DenyMaintenancePeriodDynamic,
}

impl LookerInstance_DenyMaintenancePeriod {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_DenyMaintenancePeriod_EndDate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_DenyMaintenancePeriod_StartDate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time`.\n"]
    pub fn set_time(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_DenyMaintenancePeriod_Time>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LookerInstance_DenyMaintenancePeriod {
    type O = terrars::BlockAssignable<LookerInstance_DenyMaintenancePeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_DenyMaintenancePeriod {}

impl BuildLookerInstance_DenyMaintenancePeriod {
    pub fn build(self) -> LookerInstance_DenyMaintenancePeriod {
        LookerInstance_DenyMaintenancePeriod {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_DenyMaintenancePeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_DenyMaintenancePeriodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_DenyMaintenancePeriodRef {
        LookerInstance_DenyMaintenancePeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_DenyMaintenancePeriodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::ListRef<LookerInstance_DenyMaintenancePeriod_EndDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::ListRef<LookerInstance_DenyMaintenancePeriod_StartDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> terrars::ListRef<LookerInstance_DenyMaintenancePeriod_TimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstance_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LookerInstance_EncryptionConfig {
    #[doc = "Set the field `kms_key_name`.\nName of the customer managed encryption key (CMEK) in KMS."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstance_EncryptionConfig {
    type O = terrars::BlockAssignable<LookerInstance_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_EncryptionConfig {}

impl BuildLookerInstance_EncryptionConfig {
    pub fn build(self) -> LookerInstance_EncryptionConfig {
        LookerInstance_EncryptionConfig { kms_key_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_EncryptionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_EncryptionConfigRef {
        LookerInstance_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nName of the customer managed encryption key (CMEK) in KMS."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name_version` after provisioning.\nFull name and version of the CMEK key currently in use to encrypt Looker data."]
    pub fn kms_key_name_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_state` after provisioning.\nStatus of the customer managed encryption key (CMEK) in KMS."]
    pub fn kms_key_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_state", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstance_MaintenanceWindow_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl LookerInstance_MaintenanceWindow_StartTime {
    #[doc = "Set the field `hours`.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nanos`.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstance_MaintenanceWindow_StartTime {
    type O = terrars::BlockAssignable<LookerInstance_MaintenanceWindow_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_MaintenanceWindow_StartTime {}

impl BuildLookerInstance_MaintenanceWindow_StartTime {
    pub fn build(self) -> LookerInstance_MaintenanceWindow_StartTime {
        LookerInstance_MaintenanceWindow_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_MaintenanceWindow_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_MaintenanceWindow_StartTimeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_MaintenanceWindow_StartTimeRef {
        LookerInstance_MaintenanceWindow_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_MaintenanceWindow_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of day in 24 hour format. Should be from 0 to 23."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of hour of day. Must be from 0 to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of minutes of the time. Must normally be from 0 to 59."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct LookerInstance_MaintenanceWindowDynamic {
    start_time: std::option::Option<terrars::DynamicBlock<LookerInstance_MaintenanceWindow_StartTime>>,
}

#[derive(Serialize)]
pub struct LookerInstance_MaintenanceWindow {
    day_of_week: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<std::vec::Vec<LookerInstance_MaintenanceWindow_StartTime>>,
    dynamic: LookerInstance_MaintenanceWindowDynamic,
}

impl LookerInstance_MaintenanceWindow {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_MaintenanceWindow_StartTime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LookerInstance_MaintenanceWindow {
    type O = terrars::BlockAssignable<LookerInstance_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_MaintenanceWindow {
    #[doc =
        "Required. Day of the week for this MaintenanceWindow (in UTC).\n\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: terrars::PrimField<std::string::String>,
}

impl BuildLookerInstance_MaintenanceWindow {
    pub fn build(self) -> LookerInstance_MaintenanceWindow {
        LookerInstance_MaintenanceWindow {
            day_of_week: self.day_of_week,
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_MaintenanceWindowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_MaintenanceWindowRef {
        LookerInstance_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day_of_week` after provisioning.\nRequired. Day of the week for this MaintenanceWindow (in UTC).\n\n- MONDAY: Monday\n- TUESDAY: Tuesday\n- WEDNESDAY: Wednesday\n- THURSDAY: Thursday\n- FRIDAY: Friday\n- SATURDAY: Saturday\n- SUNDAY: Sunday Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::ListRef<LookerInstance_MaintenanceWindow_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstance_OauthConfig {
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
}

impl LookerInstance_OauthConfig { }

impl terrars::ToListMappable for LookerInstance_OauthConfig {
    type O = terrars::BlockAssignable<LookerInstance_OauthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_OauthConfig {
    #[doc = "The client ID for the Oauth config."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = "The client secret for the Oauth config."]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildLookerInstance_OauthConfig {
    pub fn build(self) -> LookerInstance_OauthConfig {
        LookerInstance_OauthConfig {
            client_id: self.client_id,
            client_secret: self.client_secret,
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_OauthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_OauthConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_OauthConfigRef {
        LookerInstance_OauthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_OauthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe client ID for the Oauth config."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nThe client secret for the Oauth config."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize, Default)]
struct LookerInstance_PeriodicExportConfigDynamic {
    start_time: std::option::Option<terrars::DynamicBlock<LookerInstance_MaintenanceWindow_StartTime>>,
}

#[derive(Serialize)]
pub struct LookerInstance_PeriodicExportConfig {
    gcs_uri: terrars::PrimField<std::string::String>,
    kms_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<std::vec::Vec<LookerInstance_MaintenanceWindow_StartTime>>,
    dynamic: LookerInstance_PeriodicExportConfigDynamic,
}

impl LookerInstance_PeriodicExportConfig {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_MaintenanceWindow_StartTime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LookerInstance_PeriodicExportConfig {
    type O = terrars::BlockAssignable<LookerInstance_PeriodicExportConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_PeriodicExportConfig {
    #[doc = "Cloud Storage bucket URI for periodic export.\nFormat: gs://{bucket_name}"]
    pub gcs_uri: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the CMEK key in KMS.\nFormat:\nprojects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}"]
    pub kms_key: terrars::PrimField<std::string::String>,
}

impl BuildLookerInstance_PeriodicExportConfig {
    pub fn build(self) -> LookerInstance_PeriodicExportConfig {
        LookerInstance_PeriodicExportConfig {
            gcs_uri: self.gcs_uri,
            kms_key: self.kms_key,
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_PeriodicExportConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_PeriodicExportConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_PeriodicExportConfigRef {
        LookerInstance_PeriodicExportConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_PeriodicExportConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcs_uri` after provisioning.\nCloud Storage bucket URI for periodic export.\nFormat: gs://{bucket_name}"]
    pub fn gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nName of the CMEK key in KMS.\nFormat:\nprojects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::ListRef<LookerInstance_MaintenanceWindow_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstance_PscConfig_ServiceAttachments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_service_attachment_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LookerInstance_PscConfig_ServiceAttachments {
    #[doc =
        "Set the field `local_fqdn`.\nFully qualified domain name that will be used in the private DNS record created for the service attachment."]
    pub fn set_local_fqdn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_fqdn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_service_attachment_uri`.\nURI of the service attachment to connect to."]
    pub fn set_target_service_attachment_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_service_attachment_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstance_PscConfig_ServiceAttachments {
    type O = terrars::BlockAssignable<LookerInstance_PscConfig_ServiceAttachments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_PscConfig_ServiceAttachments {}

impl BuildLookerInstance_PscConfig_ServiceAttachments {
    pub fn build(self) -> LookerInstance_PscConfig_ServiceAttachments {
        LookerInstance_PscConfig_ServiceAttachments {
            local_fqdn: core::default::Default::default(),
            target_service_attachment_uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_PscConfig_ServiceAttachmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_PscConfig_ServiceAttachmentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LookerInstance_PscConfig_ServiceAttachmentsRef {
        LookerInstance_PscConfig_ServiceAttachmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_PscConfig_ServiceAttachmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_status` after provisioning.\nStatus of the service attachment connection."]
    pub fn connection_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_status", self.base))
    }

    #[doc =
        "Get a reference to the value of field `local_fqdn` after provisioning.\nFully qualified domain name that will be used in the private DNS record created for the service attachment."]
    pub fn local_fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_fqdn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_service_attachment_uri` after provisioning.\nURI of the service attachment to connect to."]
    pub fn target_service_attachment_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_service_attachment_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct LookerInstance_PscConfigDynamic {
    service_attachments: std::option::Option<terrars::DynamicBlock<LookerInstance_PscConfig_ServiceAttachments>>,
}

#[derive(Serialize)]
pub struct LookerInstance_PscConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_vpcs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachments: std::option::Option<std::vec::Vec<LookerInstance_PscConfig_ServiceAttachments>>,
    dynamic: LookerInstance_PscConfigDynamic,
}

impl LookerInstance_PscConfig {
    #[doc = "Set the field `allowed_vpcs`.\nList of VPCs that are allowed ingress into the Looker instance."]
    pub fn set_allowed_vpcs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_vpcs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_attachments`.\n"]
    pub fn set_service_attachments(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LookerInstance_PscConfig_ServiceAttachments>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_attachments = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_attachments = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LookerInstance_PscConfig {
    type O = terrars::BlockAssignable<LookerInstance_PscConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_PscConfig {}

impl BuildLookerInstance_PscConfig {
    pub fn build(self) -> LookerInstance_PscConfig {
        LookerInstance_PscConfig {
            allowed_vpcs: core::default::Default::default(),
            service_attachments: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_PscConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_PscConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_PscConfigRef {
        LookerInstance_PscConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_PscConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_vpcs` after provisioning.\nList of VPCs that are allowed ingress into the Looker instance."]
    pub fn allowed_vpcs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_vpcs", self.base))
    }

    #[doc =
        "Get a reference to the value of field `looker_service_attachment_uri` after provisioning.\nURI of the Looker service attachment."]
    pub fn looker_service_attachment_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.looker_service_attachment_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `service_attachments` after provisioning.\n"]
    pub fn service_attachments(&self) -> terrars::ListRef<LookerInstance_PscConfig_ServiceAttachmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_attachments", self.base))
    }
}

#[derive(Serialize)]
pub struct LookerInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LookerInstance_Timeouts {
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

impl terrars::ToListMappable for LookerInstance_Timeouts {
    type O = terrars::BlockAssignable<LookerInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_Timeouts {}

impl BuildLookerInstance_Timeouts {
    pub fn build(self) -> LookerInstance_Timeouts {
        LookerInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_TimeoutsRef {
        LookerInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_TimeoutsRef {
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
pub struct LookerInstance_UserMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_developer_user_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_standard_user_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_viewer_user_count: std::option::Option<terrars::PrimField<f64>>,
}

impl LookerInstance_UserMetadata {
    #[doc =
        "Set the field `additional_developer_user_count`.\nNumber of additional Developer Users to allocate to the Looker Instance."]
    pub fn set_additional_developer_user_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.additional_developer_user_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `additional_standard_user_count`.\nNumber of additional Standard Users to allocate to the Looker Instance."]
    pub fn set_additional_standard_user_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.additional_standard_user_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `additional_viewer_user_count`.\nNumber of additional Viewer Users to allocate to the Looker Instance."]
    pub fn set_additional_viewer_user_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.additional_viewer_user_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LookerInstance_UserMetadata {
    type O = terrars::BlockAssignable<LookerInstance_UserMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLookerInstance_UserMetadata {}

impl BuildLookerInstance_UserMetadata {
    pub fn build(self) -> LookerInstance_UserMetadata {
        LookerInstance_UserMetadata {
            additional_developer_user_count: core::default::Default::default(),
            additional_standard_user_count: core::default::Default::default(),
            additional_viewer_user_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LookerInstance_UserMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LookerInstance_UserMetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LookerInstance_UserMetadataRef {
        LookerInstance_UserMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LookerInstance_UserMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_developer_user_count` after provisioning.\nNumber of additional Developer Users to allocate to the Looker Instance."]
    pub fn additional_developer_user_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_developer_user_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `additional_standard_user_count` after provisioning.\nNumber of additional Standard Users to allocate to the Looker Instance."]
    pub fn additional_standard_user_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_standard_user_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `additional_viewer_user_count` after provisioning.\nNumber of additional Viewer Users to allocate to the Looker Instance."]
    pub fn additional_viewer_user_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_viewer_user_count", self.base))
    }
}
